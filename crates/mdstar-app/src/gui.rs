use mdstar_core::{Block, inlines_to_plain_text, parse_markdown_with_diagnostics};
use mdstar_render_html::render_html;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use serde::Serialize;
use std::ffi::{OsStr, OsString};
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

#[derive(Debug, Serialize, Clone)]
struct WorkspaceFileEntry {
    path: String,
    relative_path: String,
    depth: usize,
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
async fn list_workspace_files(path: String) -> Result<Vec<WorkspaceFileEntry>, String> {
    let input_path = PathBuf::from(&path);
    let root = if input_path.is_dir() {
        input_path
    } else {
        input_path
            .parent()
            .map(PathBuf::from)
            .ok_or_else(|| "file has no parent directory".to_string())?
    };

    let mut files = Vec::new();
    collect_workspace_files_recursive(&root, &root, &mut files)
        .map_err(|e| format!("could not scan workspace: {e}"))?;

    files.sort_by_key(|f| f.relative_path.to_lowercase());
    Ok(files)
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
            list_workspace_files,
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
        .expect("error while building MD Star");

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
    collect_startup_file_paths(std::env::args_os().skip(1))
}

fn collect_startup_file_paths(args: impl IntoIterator<Item = OsString>) -> Vec<String> {
    normalize_supported_paths(
        args.into_iter()
            .filter(|arg| {
                let value = arg.to_string_lossy();
                !value.starts_with("-psn_") && value != "--app" && value != "--gui"
            })
            .filter_map(|arg| path_from_input(&arg)),
    )
}

fn path_from_input(arg: &OsStr) -> Option<PathBuf> {
    let value = arg.to_string_lossy();
    if value.starts_with("file://") {
        return tauri::Url::parse(&value)
            .ok()
            .and_then(|url| url.to_file_path().ok());
    }
    Some(PathBuf::from(arg))
}

fn drain_open_paths(state: &State<'_, OpenPathsState>) -> Vec<String> {
    let mut paths = state.0.lock().expect("open paths state poisoned");
    std::mem::take(&mut *paths)
}

fn normalize_supported_paths(paths: impl IntoIterator<Item = PathBuf>) -> Vec<String> {
    paths
        .into_iter()
        .filter(|path| is_supported_document(path))
        .map(|path| path.to_string_lossy().to_string())
        .collect()
}

fn queue_open_paths(queue: &mut Vec<String>, paths: &[String]) {
    queue.extend(paths.iter().cloned());
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
    let paths = normalize_supported_paths(paths);

    if paths.is_empty() {
        return;
    }

    if let Some(state) = app_handle.try_state::<OpenPathsState>() {
        let mut queued = state.0.lock().expect("open paths state poisoned");
        queue_open_paths(&mut queued, &paths);
    }

    let _ = app_handle.emit("open-paths", paths);
    if let Some(window) = app_handle.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

fn collect_workspace_files_recursive(
    root: &Path,
    dir: &Path,
    files: &mut Vec<WorkspaceFileEntry>,
) -> std::io::Result<()> {
    let mut entries = fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .collect::<Vec<_>>();
    entries.sort_by_key(|entry| entry.file_name().to_string_lossy().to_lowercase());

    for entry in entries {
        let path = entry.path();
        let file_type = match entry.file_type() {
            Ok(t) => t,
            Err(_) => continue,
        };

        if file_type.is_dir() {
            if entry
                .file_name()
                .to_str()
                .is_some_and(|name| name.starts_with('.'))
            {
                continue;
            }
            collect_workspace_files_recursive(root, &path, files)?;
            continue;
        }

        if !file_type.is_file() || !is_supported_document(&path) {
            continue;
        }

        let relative = match path.strip_prefix(root) {
            Ok(rel) => rel.to_path_buf(),
            Err(_) => continue,
        };
        let depth = relative.components().count().saturating_sub(1);

        files.push(WorkspaceFileEntry {
            path: path.to_string_lossy().to_string(),
            relative_path: relative.to_string_lossy().to_string(),
            depth,
        });
    }

    Ok(())
}

fn extract_headings(doc: &mdstar_core::Document) -> Vec<HeadingEntry> {
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
    // Runtime fallback for macOS where config-driven window effects may not
    // apply consistently across builds/environments.
    apply_vibrancy(window, NSVisualEffectMaterial::Sidebar, None, None)
        .unwrap_or_else(|e| eprintln!("vibrancy unavailable: {e}"));
}

#[cfg(test)]
mod tests {
    use super::{collect_startup_file_paths, normalize_supported_paths, queue_open_paths};
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn startup_paths_filter_flags_and_unsupported_extensions() {
        let paths = collect_startup_file_paths(vec![
            OsString::from("-psn_0_12345"),
            OsString::from("--app"),
            OsString::from("/tmp/notes.md"),
            OsString::from("/tmp/image.png"),
            OsString::from("/tmp/readme.MARKDOWN"),
        ]);
        assert_eq!(paths, vec!["/tmp/notes.md", "/tmp/readme.MARKDOWN"]);
    }

    #[test]
    fn startup_paths_normalize_file_urls_to_real_paths() {
        let paths = collect_startup_file_paths(vec![OsString::from(
            "file:///Users/neo/Documents/Project%20Guide.md",
        )]);
        assert_eq!(paths, vec!["/Users/neo/Documents/Project Guide.md"]);
    }

    #[test]
    fn startup_paths_ignore_invalid_file_urls() {
        let paths = collect_startup_file_paths(vec![OsString::from("file://%%invalid%%.md")]);
        assert!(paths.is_empty());
    }

    #[test]
    fn startup_paths_accept_supported_extensions_case_insensitive() {
        let paths = collect_startup_file_paths(vec![
            OsString::from("/tmp/doc.Md"),
            OsString::from("/tmp/wiki.MarkDown"),
            OsString::from("/tmp/note.TXT"),
            OsString::from("/tmp/photo.jpg"),
        ]);
        assert_eq!(
            paths,
            vec!["/tmp/doc.Md", "/tmp/wiki.MarkDown", "/tmp/note.TXT"]
        );
    }

    #[test]
    fn normalize_supported_paths_filters_and_preserves_order() {
        let normalized = normalize_supported_paths(vec![
            PathBuf::from("/tmp/1.md"),
            PathBuf::from("/tmp/2.png"),
            PathBuf::from("/tmp/3.markdown"),
            PathBuf::from("/tmp/4.txt"),
        ]);
        assert_eq!(
            normalized,
            vec!["/tmp/1.md", "/tmp/3.markdown", "/tmp/4.txt"]
        );
    }

    #[test]
    fn queue_open_paths_appends_payload_in_order() {
        let mut queue = vec!["/tmp/existing.md".to_string()];
        let new_paths = vec!["/tmp/new-a.md".to_string(), "/tmp/new-b.txt".to_string()];
        queue_open_paths(&mut queue, &new_paths);
        assert_eq!(
            queue,
            vec!["/tmp/existing.md", "/tmp/new-a.md", "/tmp/new-b.txt"]
        );
    }
}
