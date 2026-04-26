use markdown_core::{Block, inlines_to_plain_text, parse_markdown_with_diagnostics};
use markdown_render_html::render_html;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::{Mutex, mpsc};
use tauri::{Emitter, Manager, State, WebviewWindow};

// ─── Data types sent to the frontend ─────────────────────────────────────────

#[derive(Debug, Serialize, Clone)]
struct HeadingEntry {
    level: u8,
    text: String,
    id: String,
}

#[derive(Debug, Serialize)]
struct RenderResult {
    html: String,
    source: String,
    headings: Vec<HeadingEntry>,
    word_count: usize,
    line_count: usize,
    read_minutes: usize,
    warning_count: usize,
    file_name: String,
    path: String,
}

#[derive(Debug, Serialize, Clone)]
struct FileChangedPayload {
    path: String,
}

struct OpenPathsState(Mutex<Vec<String>>);

// ─── Tauri commands ───────────────────────────────────────────────────────────

#[tauri::command]
async fn render_file(path: String) -> Result<RenderResult, String> {
    let path_buf = PathBuf::from(&path);
    let source = fs::read_to_string(&path_buf).map_err(|e| format!("could not read file: {e}"))?;

    let output =
        parse_markdown_with_diagnostics(&source).map_err(|e| format!("parse error: {e}"))?;

    let html = render_html(&output.document);
    let headings = extract_headings(&output.document);
    let word_count = source.split_whitespace().count();
    let line_count = source.lines().count();
    let read_minutes = (word_count / 200).max(1);
    let warning_count = output.diagnostics.len();

    let file_name = path_buf
        .file_name()
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_else(|| "Untitled".to_string());

    Ok(RenderResult {
        html,
        source,
        headings,
        word_count,
        line_count,
        read_minutes,
        warning_count,
        file_name,
        path,
    })
}

#[tauri::command]
async fn render_source(source: String) -> Result<RenderResult, String> {
    let output =
        parse_markdown_with_diagnostics(&source).map_err(|e| format!("parse error: {e}"))?;

    let html = render_html(&output.document);
    let headings = extract_headings(&output.document);
    let word_count = source.split_whitespace().count();
    let line_count = source.lines().count();
    let read_minutes = (word_count / 200).max(1);
    let warning_count = output.diagnostics.len();

    Ok(RenderResult {
        html,
        source: String::new(),
        headings,
        word_count,
        line_count,
        read_minutes,
        warning_count,
        file_name: String::new(),
        path: String::new(),
    })
}

#[tauri::command]
async fn save_file(path: String, content: String) -> Result<(), String> {
    fs::write(&path, content).map_err(|e| format!("could not save file: {e}"))
}

#[tauri::command]
async fn pick_file() -> Option<String> {
    rfd::AsyncFileDialog::new()
        .set_title("Open Markdown File")
        .add_filter("Markdown", &["md", "markdown", "txt"])
        .pick_file()
        .await
        .map(|f| f.path().to_string_lossy().to_string())
}

#[tauri::command]
fn initial_open_paths(state: State<'_, OpenPathsState>) -> Vec<String> {
    drain_open_paths(&state)
}

#[tauri::command]
fn watch_file(path: String, window: WebviewWindow) {
    let path_buf = PathBuf::from(path.clone());
    std::thread::spawn(move || {
        let (tx, rx) = mpsc::channel();
        let mut watcher = match RecommendedWatcher::new(tx, Config::default()) {
            Ok(w) => w,
            Err(_) => return,
        };
        if watcher
            .watch(&path_buf, RecursiveMode::NonRecursive)
            .is_err()
        {
            return;
        }
        for event in rx.into_iter().flatten() {
            if matches!(
                event.kind,
                EventKind::Modify(_) | EventKind::Create(_) | EventKind::Remove(_)
            ) {
                let _ = window.emit("file-changed", FileChangedPayload { path: path.clone() });
            }
        }
    });
}

// ─── GUI entry point ──────────────────────────────────────────────────────────

pub fn run() {
    let initial_paths = startup_file_paths();
    let app = tauri::Builder::default()
        .manage(OpenPathsState(Mutex::new(initial_paths)))
        .invoke_handler(tauri::generate_handler![
            render_file,
            render_source,
            save_file,
            pick_file,
            initial_open_paths,
            watch_file
        ])
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();

            #[cfg(target_os = "macos")]
            apply_macos_window_style(&window);

            Ok(())
        })
        .build(tauri::generate_context!())
        .expect("error while building Markwell");

    app.run(|app_handle, event| {
        if let tauri::RunEvent::WindowEvent { event, .. } = &event
            && let tauri::WindowEvent::DragDrop(tauri::DragDropEvent::Drop { paths, .. }) = event
        {
            emit_open_paths(app_handle, paths.clone());
        }

        if let tauri::RunEvent::WebviewEvent { event, .. } = &event
            && let tauri::WebviewEvent::DragDrop(tauri::DragDropEvent::Drop { paths, .. }) = event
        {
            emit_open_paths(app_handle, paths.clone());
        }

        #[cfg(target_os = "macos")]
        if let tauri::RunEvent::Opened { urls } = event {
            let paths = urls
                .into_iter()
                .filter_map(|url| url.to_file_path().ok())
                .filter(|path| is_supported_document(path))
                .collect::<Vec<_>>();
            emit_open_paths(app_handle, paths);
        }
    });
}

// ─── Helpers ─────────────────────────────────────────────────────────────────

fn startup_file_paths() -> Vec<String> {
    std::env::args_os()
        .skip(1)
        .filter(|arg| {
            let value = arg.to_string_lossy();
            !value.starts_with("-psn_") && value != "--app" && value != "--gui"
        })
        .map(PathBuf::from)
        .filter(|path| is_supported_document(path))
        .map(|path| path.to_string_lossy().to_string())
        .collect()
}

fn drain_open_paths(state: &State<'_, OpenPathsState>) -> Vec<String> {
    let mut paths = state.0.lock().expect("open paths state poisoned");
    std::mem::take(&mut *paths)
}

fn is_supported_document(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            let ext = ext.to_ascii_lowercase();
            matches!(ext.as_str(), "md" | "markdown" | "txt")
        })
        .unwrap_or(false)
}

fn emit_open_paths<R: tauri::Runtime>(app_handle: &tauri::AppHandle<R>, paths: Vec<PathBuf>) {
    let paths = paths
        .into_iter()
        .filter(|path| is_supported_document(path))
        .map(|path| path.to_string_lossy().to_string())
        .collect::<Vec<_>>();

    if paths.is_empty() {
        return;
    }

    if let Some(state) = app_handle.try_state::<OpenPathsState>() {
        let mut queued = state.0.lock().expect("open paths state poisoned");
        queued.extend(paths.clone());
    }

    let _ = app_handle.emit("open-paths", paths);
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn extract_headings(doc: &markdown_core::Document) -> Vec<HeadingEntry> {
    doc.blocks
        .iter()
        .filter_map(|block| {
            if let Block::Heading { level, children } = block {
                let text = inlines_to_plain_text(children);
                let id = slugify(&text);
                Some(HeadingEntry {
                    level: *level,
                    text,
                    id,
                })
            } else {
                None
            }
        })
        .collect()
}

fn slugify(text: &str) -> String {
    text.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(target_os = "macos")]
fn apply_macos_window_style(window: &WebviewWindow) {
    use window_vibrancy::{NSVisualEffectMaterial, apply_vibrancy};
    apply_vibrancy(window, NSVisualEffectMaterial::Sidebar, None, None)
        .unwrap_or_else(|e| eprintln!("vibrancy unavailable: {e}"));
}
