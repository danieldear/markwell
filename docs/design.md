# System Design

## Scope

MD Star currently targets:

- a Tauri desktop app for opening, reading, editing, and previewing Markdown,
- a terminal `md` workflow for shell users,
- shared Rust parsing and rendering components,
- release automation for GitHub-hosted public artifacts.

Historical GPUI planning remains in `docs/ui-gpui-design.md`, but the current
implementation is Tauri 2.

## Component Responsibilities

### `mdstar-core`

- Parse Markdown into a semantic document model.
- Collect recoverable diagnostics.
- Provide stable data types for all renderers.

### `mdstar-render-terminal`

- Render terminal-friendly Markdown with mdansi.
- Improve code block presentation.
- Convert Mermaid fences to text diagrams.
- Support ASCII-only Mermaid output.

### `mdstar-render-html`

- Render semantic HTML for desktop preview.
- Keep HTML generation deterministic for tests.
- Avoid runtime dependencies on frontend frameworks.

### `mdstar-app`

- Own the active `md` binary.
- Route terminal launches into CLI/TUI behavior.
- Route app launches into the Tauri GUI.
- Handle open/save/reload/watch/file association flows.

### `mdstar-ffi`

- Provide a C ABI surface for future Quick Look and native integration adapters.

## Desktop Interaction Model

```text
Open file
  -> render HTML preview
  -> populate outline and metadata
  -> watch file for external changes

Edit mode
  -> source pane opens
  -> input debounces render_source
  -> save writes the current file
```

## Quality Bar

- No known P0/P1 defects before public release.
- `cargo fmt`, `cargo clippy`, and `cargo test` must pass.
- Public docs must describe what exists today, not only future architecture.
- Release artifacts must be reproducible through GitHub Actions.
