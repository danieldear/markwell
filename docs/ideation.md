# Ideation

## Product Vision

MD Star should feel like a practical open-source Markdown workbench:
pleasant for reading, direct for editing, and useful from the command line.

## Product Shape

```text
One Rust core
  -> terminal renderer for `md`
  -> HTML renderer for desktop preview
  -> FFI bridge for future native integrations
```

## Current Product Direction

- Tauri desktop app for the native window, file association, and packaging path.
- Static frontend inside `crates/mdstar-app/ui/`.
- Unified `md` binary that runs CLI/TUI behavior in terminal contexts and GUI
  behavior in app contexts.
- GitHub Pages-ready static public page under `docs/`.

## Near-Term Focus

- Make macOS "Open With" load the selected file reliably.
- Make window dragging feel native.
- Clean public docs.
- Prepare GitHub repository and draft release artifacts.

## Later Ideas

- Quick Look extension backed by `mdstar-ffi`.
- Linux MIME registration tooling.
- Syntax highlighting in HTML preview.
- Configurable themes.
- Optional signed/notarized release channel.
