# MD Star Documentation

These documents describe the product direction, current implementation, and
release plan for MD Star.

## Current Scope

- Platforms: macOS and Linux first.
- Desktop app: Tauri 2 native shell with an HTML/CSS frontend.
- CLI: unified `md` binary from `mdstar-app`.
- Rendering: shared Rust core with terminal and HTML renderers.
- System integration: macOS file associations now; Quick Look and Linux MIME
  integration planned after the first public release.

## Documents

- [Public static page](./index.html)
- [Research](./research.md)
- [Ideation](./ideation.md)
- [PRD](./prd.md)
- [Architecture](./architecture.md)
- [Design](./design.md)
- [Historical GPUI design notes](./ui-gpui-design.md)
- [Project plan](./project-plan.md)
- [Roadmap](./roadmap.md)
- [Milestones](./milestones.md)
- [Milestone status](./milestone-status.md)
- [Release checklist](./release-checklist.md)

## Build And Release Commands

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cd crates/mdstar-app && cargo tauri build
```

The release workflow creates draft GitHub releases when a `vMAJOR.MINOR.PATCH`
tag is pushed.
