# Milestone Status

Date: 2026-04-25

## Progress Board

```text
[M0] Foundations             | DONE
[M1] Core engine             | DONE
[M2] CLI/TUI MVP             | DONE
[M3] Tauri desktop MVP       | IN REVIEW
[M4] macOS file association  | IN REVIEW
[M5] Linux integration       | PLANNED
[M6] Public release          | IN PROGRESS
```

## Completed

- Workspace, CI, licenses, contribution docs, and GitHub templates.
- Parser adapter, semantic IR, diagnostics, and fixture harness.
- Terminal renderer with mdansi and Mermaid text preprocessing.
- Direct `md <path>` invocation plus legacy `md view <path>`.
- Interactive ratatui viewer and split source edit mode.
- Tauri desktop app with rendered preview, outline, metadata, open/save/reload,
  file watching, and Mermaid diagrams.
- Tauri bundling for `.app` and `.dmg` on macOS.

## In Review

- macOS "Open With" path handoff into the running app.
- Custom titlebar dragging.
- Public documentation and static project page.
- Release workflow artifact validation.

## Known Gaps

- `install.sh` currently requires `REPO=owner/repository` until the final GitHub path is known.
- macOS signing/notarization is not configured.
- Linux package artifacts have not been validated on a clean release runner.
- Quick Look extension scaffold exists in `macos/MarkwellQuickLook`; host-app embedding and signing remain pending.

## Release Gate

First public release is ready when:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cd crates/markdown-app && cargo tauri build
```

all pass, and the draft GitHub release contains the expected artifacts.
