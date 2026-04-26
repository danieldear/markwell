// On Windows release builds, suppress the console window when running as GUI.
// On macOS/Linux this attribute is ignored — the terminal always works.
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cli;
mod gui;

fn main() {
    if use_gui_mode() {
        gui::run();
    } else {
        if let Err(e) = cli::run() {
            eprintln!("error: {e:#}");
            std::process::exit(1);
        }
    }
}

/// Decide at runtime whether to open the GUI window or run as a CLI tool.
///
/// Rules (first match wins):
/// 1. `--app` / `--gui` flag   → GUI  (explicit override)
/// 2. `-psn_*` argument        → GUI  (macOS Finder launch)
/// 3. Inside a `.app` bundle   → GUI  (macOS bundle detection)
/// 4. Everything else          → CLI
fn use_gui_mode() -> bool {
    let args: Vec<String> = std::env::args().collect();

    // Explicit opt-in
    if args.iter().any(|a| a == "--app" || a == "--gui") {
        return true;
    }

    // macOS adds -psn_XXXXXXX when launching from Finder / dock / file-open
    if args.iter().any(|a| a.starts_with("-psn_")) {
        return true;
    }

    // Detect if we are running inside a macOS .app bundle
    #[cfg(target_os = "macos")]
    if std::env::current_exe()
        .ok()
        .and_then(|p| p.to_str().map(|s| s.contains(".app/Contents/")))
        .unwrap_or(false)
    {
        return true;
    }

    false
}
