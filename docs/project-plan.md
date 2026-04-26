# Project Plan

## Release Target

The next target is a first public GitHub release that feels credible for an
open-source developer tool.

## Completed Work

- Rust workspace and CI scaffold.
- Shared Markdown parser, semantic model, diagnostics, and fixtures.
- Terminal renderer with polished code/table output and Mermaid text rendering.
- Unified `md` app binary with CLI, TUI, and Tauri desktop behavior.
- Desktop open/save/reload/watch, outline, metadata, and live preview.
- macOS app bundle and DMG generation through Tauri.
- OSS baseline files: licenses, contribution guide, code of conduct, security
  policy, issue templates, and PR template.

## Remaining Before Public Push

1. Replace placeholder release metadata.
2. Confirm GitHub repository owner/name in `install.sh`.
3. Validate macOS "Open With" using an installed `.app`.
4. Validate `cargo tauri build` artifacts on GitHub Actions.
5. Review docs and website for stale implementation claims.
6. Tag `v0.1.0` only after CI passes on the pushed branch.

## Release Commands

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cd crates/markdown-app && cargo tauri build
```

## GitHub Release Flow

```text
push branch
  -> open PR or merge to main
  -> wait for CI
  -> create tag v0.1.0
  -> GitHub Actions creates draft release
  -> inspect artifacts
  -> publish release manually
```

## Post-Release Roadmap

The canonical post-release roadmap now lives in:

- [Roadmap](./roadmap.md)

Immediate post-release focus remains:

1. Reliability hardening for file-open workflows (`Open With`, drag-drop,
   CLI/app routing).
2. Signing/notarization planning and rollout for macOS distribution.
3. Developer UX baseline (workspace ergonomics and keyboard-first flow).
