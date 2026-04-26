# Milestones

## Milestone Table

| Milestone | Goal | Status | Exit Criteria |
|---|---|---|---|
| M0 | Foundations | Done | workspace, CI, docs baseline |
| M1 | Core engine | Done | parser, semantic IR, diagnostics, fixtures |
| M2 | CLI/TUI MVP | Done | `md <path>`, pager, ratatui view/edit, tests |
| M3 | Tauri desktop MVP | In review | open/edit/save/reload/watch and preview stable |
| M4 | macOS integration | In review | file association path handoff validated |
| M5 | Linux integration | Planned | `.desktop`, MIME/default app validation |
| M6 | First public release | In progress | CI green, artifacts verified, docs polished |

## Feature Breakdown

### M0 Foundations

- Rust workspace scaffold.
- CI workflow.
- OSS baseline docs and templates.

### M1 Core

- Parser adapter abstraction.
- Semantic IR model.
- Metadata and diagnostics.
- Fixture harness.

### M2 CLI/TUI

- Terminal renderer.
- Direct command routing with `md <path>`.
- Legacy `md view <path>` compatibility.
- Pager support.
- Ratatui viewer and split edit mode.
- CLI snapshots and integration tests.

### M3 Desktop

- Tauri app shell.
- HTML preview from shared renderer.
- Source edit pane.
- Save, reload, file watch, and outline behavior.
- Mermaid diagram rendering in preview.

### M4 macOS

- Document type association in the app bundle.
- "Open With" file path handoff.
- Quick Look extension scaffold added under `macos/MarkwellQuickLook`.
- Future: Finder action.

### M5 Linux

- `.desktop` file.
- MIME registration.
- Install/uninstall/validation scripts.

### M6 Release

- Clean docs and website.
- Release workflow validation.
- CLI archive installer validation.
- GitHub release notes and artifact inspection.

## Recommended Execution Order

```text
M0 -> M1 -> M2/M3 -> M4 review -> M6 public release -> M5/Linux polish
```
