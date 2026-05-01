# Architecture

## Goals

- Keep one semantic Markdown model across desktop, CLI, and system adapters.
- Keep platform integration thin and testable.
- Ship a small native app without a JavaScript build pipeline.
- Maintain a clean public repository suitable for outside contributors.

## Current Runtime Topology

```text
                        +----------------------+
                        | mdstar-core        |
                        | parse + semantic IR  |
                        | diagnostics          |
                        +----------+-----------+
                                   |
              +--------------------+--------------------+
              |                                         |
              v                                         v
    +--------------------------+              +----------------------+
    | mdstar-render-terminal |              | mdstar-render-html |
    | mdansi + Mermaid text    |              | preview HTML         |
    +------------+-------------+              +----------+-----------+
                 |                                       |
                 v                                       v
        +----------------+                      +----------------+
        | md CLI / TUI   |                      | Tauri desktop  |
        | mdstar-app   |                      | mdstar-app   |
        +----------------+                      +----------------+
```

`mdstar-app` is the active product binary. It runs as a CLI in terminal
contexts and as a Tauri desktop app when launched with `--app`, from Finder, or
from inside the macOS `.app` bundle.

## Workspace Topology

```text
.
+-- crates/
|   +-- mdstar-core/
|   +-- mdstar-render-terminal/
|   +-- mdstar-render-html/
|   +-- mdstar-app/
|   +-- mdstar-cli/        legacy crate, retained for reference
|   `-- mdstar-ffi/
+-- docs/
+-- tests/fixtures/
`-- .github/workflows/
```

## Core Contracts

- `mdstar-core` parses source and returns a `Document` plus diagnostics.
- `mdstar-render-terminal` renders Markdown source for terminal output,
  including Mermaid preprocessing.
- `mdstar-render-html` renders semantic HTML for the desktop preview.
- `mdstar-ffi` exposes a minimal C ABI for future system integrations.

## Desktop Flow

```text
Finder/Open With/CLI flag/dialog
   |
   v
Tauri backend render_file/render_source
   |
   v
mdstar-core -> mdstar-render-html
   |
   v
frontend preview + outline + metadata
```

macOS "Open With" uses the Tauri `RunEvent::Opened` event. Startup file
arguments are also captured so Linux/Windows-style launches can pass a file path
to the GUI.

## CLI Flow

```text
md <path>
   |
   v
read file -> mdstar-render-terminal -> stdout/pager or ratatui view
```

The TUI edit mode writes directly to disk and refreshes the rendered preview
from the shared terminal renderer.

## Packaging

Tauri owns application packaging:

```bash
cd crates/mdstar-app
cargo tauri build
cargo tauri build --bundles dmg
```

The GitHub release workflow builds platform artifacts and attaches them to a
draft release for version tags.

## Release Risks

- macOS signing/notarization is not configured yet.
- Linux packages need validation on GitHub Actions runners.
- Quick Look and Linux MIME registration are planned but not complete.
- `install.sh` still needs the real GitHub owner/repository before public use.
