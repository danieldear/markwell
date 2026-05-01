use std::env;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{IsTerminal, Stdout, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::time::Duration;

use ansi_to_tui::IntoText;
use anyhow::{Context, Result};
use clap::Parser;
use crossterm::event::{
    DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers,
    poll, read,
};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode, size,
};
use mdstar_render_terminal::{RenderOptions, render_markdown};
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};
use ratatui::{Frame, Terminal};
use ratatui_textarea::TextArea;

#[derive(Debug, Parser)]
#[command(
    name = "md",
    version,
    about = "MD Star CLI viewer (`md <file>`). Compatibility alias: `md view <file>`."
)]
struct Cli {
    /// Path to markdown file.
    path: PathBuf,
    /// Wrap width for terminal output.
    #[arg(long)]
    width: Option<usize>,
    /// Disable ANSI color output.
    #[arg(long)]
    no_color: bool,
    /// Render Mermaid diagrams in ASCII (no Unicode box drawing).
    #[arg(long)]
    ascii_mermaid: bool,
    /// Disable interactive TUI mode, always print output.
    #[arg(long)]
    no_tui: bool,
    /// Force plain text mode (implies no color, no TUI).
    #[arg(long)]
    plain: bool,
    /// Do not pipe output through pager (non-TUI mode only).
    #[arg(long)]
    no_pager: bool,
    /// Pager command override (for example: "less -R" or "bat -pp").
    #[arg(long)]
    pager: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum UiMode {
    Preview,
    Edit,
}

struct TuiApp {
    path: PathBuf,
    render_options: RenderOptions,
    source: String,
    rendered_ansi: String,
    rendered_text: Text<'static>,
    preview_scroll: u16,
    status: String,
    mode: UiMode,
    editor: TextArea<'static>,
    dirty: bool,
}

impl TuiApp {
    fn load(path: PathBuf, render_options: RenderOptions) -> Result<Self> {
        let source = fs::read_to_string(&path)
            .with_context(|| format!("failed to read markdown file: {}", path.display()))?;

        let mut app = Self {
            path,
            render_options,
            source,
            rendered_ansi: String::new(),
            rendered_text: Text::raw(String::new()),
            preview_scroll: 0,
            status: String::new(),
            mode: UiMode::Preview,
            editor: TextArea::default(),
            dirty: false,
        };
        app.editor = build_editor(&app.source);
        app.refresh_preview();
        app.set_status("loaded");
        Ok(app)
    }

    fn refresh_preview(&mut self) {
        self.rendered_ansi = render_markdown(&self.source, self.render_options);
        self.rendered_text = self
            .rendered_ansi
            .as_str()
            .into_text()
            .unwrap_or_else(|_| Text::raw(self.rendered_ansi.clone()));
    }

    fn file_summary(&self) -> String {
        format!(
            "{} lines, {} bytes",
            self.source.split('\n').count(),
            self.source.len()
        )
    }

    fn set_status(&mut self, message: &str) {
        self.status = format!("{message} | {}", self.file_summary());
    }

    fn reload_from_disk(&mut self) -> Result<()> {
        self.source = fs::read_to_string(&self.path)
            .with_context(|| format!("failed to read markdown file: {}", self.path.display()))?;
        self.editor = build_editor(&self.source);
        self.refresh_preview();
        self.preview_scroll = 0;
        self.dirty = false;
        self.set_status("reloaded from disk");
        Ok(())
    }

    fn save_to_disk(&mut self) -> Result<()> {
        fs::write(&self.path, &self.source)
            .with_context(|| format!("failed to write markdown file: {}", self.path.display()))?;
        self.dirty = false;
        self.set_status("saved");
        Ok(())
    }

    fn enter_edit_mode(&mut self) {
        self.mode = UiMode::Edit;
        self.editor = build_editor(&self.source);
        self.set_status("edit mode: live preview");
    }

    fn exit_edit_mode(&mut self) -> Result<()> {
        let autosaved = self.dirty;
        if autosaved {
            self.save_to_disk()?;
        }
        self.mode = UiMode::Preview;
        if autosaved {
            self.set_status("edit mode closed; auto-saved");
        } else {
            self.set_status("edit mode closed");
        }
        Ok(())
    }

    fn apply_editor_change(&mut self) {
        self.source = self.editor.lines().join("\n");
        self.refresh_preview();
        self.dirty = true;
        self.set_status("editing");
    }

    fn ensure_preview_width(&mut self, width: usize) {
        let clamped = width.max(40);
        if self.render_options.width != clamped {
            self.render_options.width = clamped;
            self.refresh_preview();
        }
    }
}

fn build_editor(source: &str) -> TextArea<'static> {
    let mut textarea = TextArea::from(source.split('\n'));
    textarea.set_block(
        Block::default()
            .title("Source Editor")
            .borders(Borders::ALL),
    );
    textarea.set_cursor_line_style(Style::default().bg(Color::DarkGray));
    textarea
}

fn main() -> Result<()> {
    let cli = Cli::parse_from(normalize_legacy_args(env::args_os()));
    let options = build_render_options(&cli);

    if should_use_tui(&cli) {
        return run_tui(cli.path, options);
    }

    let source = fs::read_to_string(&cli.path)
        .with_context(|| format!("failed to read markdown file: {}", cli.path.display()))?;
    let output = render_markdown(&source, options);

    let use_pager = should_use_pager(cli.no_pager || cli.plain, cli.pager.is_some());
    emit_output(output, use_pager, cli.pager)?;
    Ok(())
}

fn build_render_options(cli: &Cli) -> RenderOptions {
    let mut options = RenderOptions::default();
    if let Some(width) = cli.width {
        options.width = width;
    } else if std::io::stdout().is_terminal() && let Ok((cols, _)) = size() {
        options.width = usize::from(cols).saturating_sub(4).max(40);
    }
    options.color = !(cli.no_color || cli.plain);
    options.ascii_mermaid = cli.ascii_mermaid;
    options
}

fn normalize_legacy_args(args: impl IntoIterator<Item = OsString>) -> Vec<OsString> {
    let mut normalized: Vec<OsString> = args.into_iter().collect();
    if normalized
        .get(1)
        .is_some_and(|arg| arg == OsStr::new("view"))
    {
        normalized.remove(1);
    }
    normalized
}

fn should_use_tui(cli: &Cli) -> bool {
    if cli.no_tui || cli.plain || cli.pager.is_some() {
        return false;
    }
    std::io::stdout().is_terminal() && std::io::stdin().is_terminal()
}

fn run_tui(path: PathBuf, render_options: RenderOptions) -> Result<()> {
    enable_raw_mode().context("failed to enable raw mode")?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)
        .context("failed to enter alternate screen")?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("failed to initialize terminal backend")?;
    terminal.clear().context("failed to clear terminal")?;

    let mut app = TuiApp::load(path, render_options)?;
    let result = run_tui_loop(&mut terminal, &mut app);

    let restore_result = restore_terminal(&mut terminal);
    result.and(restore_result)
}

fn run_tui_loop(terminal: &mut Terminal<CrosstermBackend<Stdout>>, app: &mut TuiApp) -> Result<()> {
    loop {
        terminal
            .draw(|frame| draw_ui(frame, app))
            .context("failed to draw TUI frame")?;

        if !poll(Duration::from_millis(200)).context("failed to poll terminal events")? {
            continue;
        }

        let event = read().context("failed to read terminal event")?;
        let Event::Key(key) = event else {
            continue;
        };
        if key.kind != KeyEventKind::Press {
            continue;
        }

        match app.mode {
            UiMode::Preview => {
                if handle_preview_key(app, key)? {
                    break;
                }
            }
            UiMode::Edit => {
                if handle_edit_key(app, key)? {
                    break;
                }
            }
        }
    }

    Ok(())
}

fn handle_preview_key(app: &mut TuiApp, key: KeyEvent) -> Result<bool> {
    match key.code {
        KeyCode::Char('q') | KeyCode::Esc => return Ok(true),
        KeyCode::Down | KeyCode::Char('j') => {
            app.preview_scroll = app.preview_scroll.saturating_add(1)
        }
        KeyCode::Up | KeyCode::Char('k') => {
            app.preview_scroll = app.preview_scroll.saturating_sub(1)
        }
        KeyCode::PageDown => app.preview_scroll = app.preview_scroll.saturating_add(12),
        KeyCode::PageUp => app.preview_scroll = app.preview_scroll.saturating_sub(12),
        KeyCode::Home => app.preview_scroll = 0,
        KeyCode::End => app.preview_scroll = u16::MAX,
        KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::SHIFT) => {
            app.preview_scroll = u16::MAX;
        }
        KeyCode::Char('g') => app.preview_scroll = 0,
        KeyCode::Char('r') => app.reload_from_disk()?,
        KeyCode::Char('e') => app.enter_edit_mode(),
        _ => {}
    }
    Ok(false)
}

fn handle_edit_key(app: &mut TuiApp, key: KeyEvent) -> Result<bool> {
    let ctrl = key.modifiers.contains(KeyModifiers::CONTROL);

    if ctrl && matches!(key.code, KeyCode::Char('q')) {
        return Ok(true);
    }
    if ctrl && matches!(key.code, KeyCode::Char('s')) {
        app.save_to_disk()?;
        return Ok(false);
    }
    if ctrl && matches!(key.code, KeyCode::Char('r')) {
        app.reload_from_disk()?;
        return Ok(false);
    }
    if matches!(key.code, KeyCode::Esc) {
        app.exit_edit_mode()?;
        return Ok(false);
    }

    if app.editor.input(key) {
        app.apply_editor_change();
    }

    Ok(false)
}

fn draw_ui(frame: &mut Frame, app: &mut TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(1), Constraint::Length(1)])
        .split(frame.area());

    match app.mode {
        UiMode::Preview => {
            app.ensure_preview_width(usize::from(chunks[0].width.saturating_sub(4)));
            let title = format!("{}  [q quit, e split-edit, r reload]", app.path.display());
            let content = Paragraph::new(app.rendered_text.clone())
                .block(Block::default().title(title).borders(Borders::ALL))
                .scroll((app.preview_scroll, 0))
                .wrap(Wrap { trim: false });
            frame.render_widget(content, chunks[0]);
        }
        UiMode::Edit => {
            let panes = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
                .split(chunks[0]);

            app.ensure_preview_width(usize::from(panes[1].width.saturating_sub(4)));

            frame.render_widget(&app.editor, panes[0]);

            let preview_scroll = u16::try_from(app.editor.cursor().0).unwrap_or(u16::MAX);
            let preview_title = "Live Preview";
            let preview = Paragraph::new(app.rendered_text.clone())
                .block(Block::default().title(preview_title).borders(Borders::ALL))
                .scroll((preview_scroll, 0))
                .wrap(Wrap { trim: false });
            frame.render_widget(preview, panes[1]);
        }
    }

    let dirty_flag = if app.dirty { "*" } else { "" };
    let mode_label = match app.mode {
        UiMode::Preview => "VIEW",
        UiMode::Edit => "EDIT",
    };
    let hint = match app.mode {
        UiMode::Preview => "j/k scroll | PgUp/PgDn jump | g/G top/bottom",
        UiMode::Edit => "type to edit | Ctrl+S save | Esc close-edit | Ctrl+Q quit",
    };
    let status = Paragraph::new(format!(
        "[{mode_label}{dirty_flag}] {} | {} | {hint}",
        app.path.display(),
        app.status
    ))
    .style(Style::default().fg(Color::Black).bg(Color::Cyan));
    frame.render_widget(status, chunks[1]);
}

fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )
    .context("failed to leave alternate screen")?;
    terminal.show_cursor().context("failed to restore cursor")?;
    Ok(())
}

fn should_use_pager(no_pager: bool, pager_override: bool) -> bool {
    if no_pager {
        return false;
    }
    pager_override || std::io::stdout().is_terminal()
}

fn emit_output(output: String, use_pager: bool, pager_override: Option<String>) -> Result<()> {
    if !use_pager {
        print!("{output}");
        return Ok(());
    }

    let pager_spec = pager_override
        .or_else(|| env::var("PAGER").ok())
        .unwrap_or_else(|| "less -R".to_string());

    match pipe_to_pager(&pager_spec, &output) {
        Ok(()) => Ok(()),
        Err(error) => {
            eprintln!("warning: pager unavailable ({error}), writing to stdout");
            print!("{output}");
            Ok(())
        }
    }
}

fn pipe_to_pager(pager_spec: &str, output: &str) -> Result<()> {
    let parts = shell_words::split(pager_spec)
        .with_context(|| format!("invalid pager command: {pager_spec}"))?;
    if parts.is_empty() {
        anyhow::bail!("pager command is empty");
    }

    let mut child = Command::new(&parts[0])
        .args(&parts[1..])
        .stdin(Stdio::piped())
        .spawn()
        .with_context(|| format!("failed to start pager '{}'", parts[0]))?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .context("failed to open stdin for pager process")?;
        stdin
            .write_all(output.as_bytes())
            .context("failed to write output to pager")?;
    }

    let status = child.wait().context("failed waiting for pager process")?;
    if !status.success() {
        anyhow::bail!("pager exited with non-zero status: {status}");
    }

    Ok(())
}
