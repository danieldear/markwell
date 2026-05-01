# Milestone Status

Date: 2026-04-26

## Progress Board

```text
[M0] Foundations             | DONE
[M1] Core engine             | DONE
[M2] CLI/TUI MVP             | DONE
[M3] Tauri desktop MVP       | IN REVIEW
[M4] macOS file association  | IN REVIEW
[M5] Linux integration       | PLANNED
[M6] Public release          | IN PROGRESS
[M7] Reliability hardening   | PLANNED
[M8] Core UX parity          | PLANNED
[M9] Differentiators         | PLANNED
[M10] Ecosystem + scale      | PLANNED
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

- macOS signing/notarization is not configured.
- Linux package artifacts have not been validated on a clean release runner.
- Quick Look extension scaffold exists in `macos/MarkwellQuickLook`; host-app embedding and signing remain pending.
- Roadmap execution after first release has now been documented in
  [roadmap.md](./roadmap.md) and needs milestone-by-milestone tracking.

## Release Gate

First public release is ready when:

```text
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cd crates/mdstar-app && cargo tauri build
```

all pass, and the draft GitHub release contains the expected artifacts.
