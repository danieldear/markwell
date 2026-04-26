// On Windows release builds, suppress the console window when running as GUI.
// On macOS/Linux this attribute is ignored — the terminal always works.
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod cli;
mod gui;
use std::io::IsTerminal;

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
/// 2. `--cli` / `--no-gui`     → CLI  (explicit override)
/// 3. `-psn_*` argument        → GUI  (macOS Finder launch)
/// 4. Running in a terminal    → CLI  (shell-first behavior)
/// 5. Inside a `.app` bundle   → GUI  (desktop launch behavior)
/// 6. Everything else          → CLI
fn use_gui_mode() -> bool {
    let args: Vec<String> = std::env::args().collect();
    let has_terminal = std::io::stdin().is_terminal()
        || std::io::stdout().is_terminal()
        || std::io::stderr().is_terminal();

    // Detect if we are running inside a macOS .app bundle.
    #[cfg(target_os = "macos")]
    let in_app_bundle = std::env::current_exe()
        .ok()
        .and_then(|p| p.to_str().map(|s| s.contains(".app/Contents/")))
        .unwrap_or(false);

    #[cfg(not(target_os = "macos"))]
    let in_app_bundle = false;

    decide_gui_mode(&args, has_terminal, in_app_bundle)
}

fn decide_gui_mode(args: &[String], has_terminal: bool, in_app_bundle: bool) -> bool {
    // Explicit opt-in
    if args.iter().any(|a| a == "--app" || a == "--gui") {
        return true;
    }

    // Explicit opt-out
    if args.iter().any(|a| a == "--cli" || a == "--no-gui") {
        return false;
    }

    // macOS adds -psn_XXXXXXX when launching from Finder / dock / file-open
    if args.iter().any(|a| a.starts_with("-psn_")) {
        return true;
    }

    // Shell usage should stay in CLI mode even if the executable path is inside
    // a bundled .app (e.g. /Applications/Markwell.app/Contents/MacOS/md).
    if has_terminal {
        return false;
    }

    // Desktop launch fallback
    if in_app_bundle {
        return true;
    }

    false
}

#[cfg(test)]
mod tests {
    use super::decide_gui_mode;

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn gui_flag_forces_gui_mode() {
        assert!(decide_gui_mode(&args(&["md", "--app"]), true, false));
        assert!(decide_gui_mode(&args(&["md", "--gui"]), true, false));
    }

    #[test]
    fn cli_flags_force_cli_mode() {
        assert!(!decide_gui_mode(&args(&["md", "--cli"]), false, true));
        assert!(!decide_gui_mode(&args(&["md", "--no-gui"]), false, true));
    }

    #[test]
    fn psn_argument_uses_gui_mode() {
        assert!(decide_gui_mode(&args(&["md", "-psn_0_12345"]), false, false));
    }

    #[test]
    fn terminal_defaults_to_cli_mode() {
        assert!(!decide_gui_mode(&args(&["md", "README.md"]), true, true));
    }

    #[test]
    fn app_bundle_defaults_to_gui_without_terminal() {
        assert!(decide_gui_mode(&args(&["md"]), false, true));
    }

    #[test]
    fn defaults_to_cli_mode_without_terminal_or_bundle() {
        assert!(!decide_gui_mode(&args(&["md"]), false, false));
    }
}
