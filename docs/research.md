# Research Notes

Date: 2026-04-25

## Summary

The implementation has moved from early GPUI planning to a working Tauri 2
desktop app with a unified Rust binary. The shared-core architecture remains the
right direction: it keeps CLI, desktop preview, and future system integrations
from drifting.

## Current Technical Findings

1. Tauri 2 can generate app bundles and installers through `cargo tauri build`.
2. Plain `cargo build --release` builds the binary but does not create installer
   artifacts.
3. macOS "Open With" needs explicit Tauri `RunEvent::Opened` handling; file
   associations alone only launch the app.
4. Tauri custom titlebar drag is more reliable with manual `startDragging()` when
   the titlebar contains nested controls.
5. `mdansi` and `mermaid-text` are working choices for terminal rendering.
6. A static GitHub Pages site can live under `docs/` without introducing a
   frontend build system.

## Sources To Track

- Tauri v2 distribution and DMG documentation.
- Tauri v2 window customization documentation.
- Tauri `RunEvent::Opened` API behavior for macOS.
- CommonMark and GFM specifications.
- freedesktop MIME/default app specifications.
- Apple signing, notarization, and Quick Look documentation.

## Open Research Items

- Whether to sign/notarize before the first public binary release or clearly mark
  binaries as unsigned.
- Which Linux package formats should be considered first-class for v0.1.x.
- Whether `markdown-cli` should be removed entirely or retained as a reference
  crate outside the workspace.
