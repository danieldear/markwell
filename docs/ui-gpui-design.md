# Historical GPUI Desktop UX Design

Date: 2026-04-24

This document is retained as early product/design research. The current desktop
implementation uses Tauri 2 with static HTML/CSS/JavaScript in
`crates/markdown-app/ui/`. Treat this file as design background, not as the
active implementation contract.

## Design Direction

**Theme name:** Editorial Workbench

The UI should feel like a professional writing desk, not a browser clone:

- left side for structure and navigation,
- center for source editing,
- right side for rendered output,
- bottom strip for diagnostics and parser health.

Visual intent:

- neutral, print-like base colors,
- high-contrast text for long sessions,
- restrained accent color for active focus and actions,
- minimal but purposeful motion.

## 1) Information Architecture

```text
+----------------------------------------------------------------------------------+
| Titlebar: file name | dirty marker | project path | global actions              |
+----------------------------+-----------------------+-----------------------------+
| Sidebar                    | Editor Pane           | Preview Pane                |
| - File tree (optional)     | - Source markdown     | - Rendered markdown         |
| - Document outline / TOC   | - line numbers        | - scroll + links            |
| - Quick symbols            | - diagnostics marks   | - code blocks + tables      |
+----------------------------+-----------------------+-----------------------------+
| Diagnostics + status: parser warnings | file encoding | line/col | grammar mode  |
+----------------------------------------------------------------------------------+
```

Primary navigation modes:

- **Browse mode:** TOC and file navigation.
- **Author mode:** focused editor + live preview.
- **Inspect mode:** diagnostics and rendering parity checks.

## 2) Interaction Model

Keyboard-first baseline:

- `Cmd/Ctrl+O`: open file.
- `Cmd/Ctrl+S`: save.
- `Cmd/Ctrl+P`: command palette.
- `Cmd/Ctrl+Shift+O`: outline quick jump.
- `Cmd/Ctrl+\\`: toggle split orientation.
- `Cmd/Ctrl+1`, `2`, `3`: focus sidebar/editor/preview.
- `Cmd/Ctrl+Shift+D`: toggle diagnostics panel.
- `Alt+Up/Down`: move heading block selection in editor (future).

Live preview sync:

- source and preview scroll are softly synchronized with anchor alignment,
- cursor-in-heading highlights current TOC node,
- clicking preview heading jumps source to matched range.

Command palette actions:

- Open file
- Save / Save As
- Toggle word wrap
- Toggle preview pin
- Export HTML (phase 2)
- Toggle strict CommonMark mode (phase 2)

## 3) Visual System Tokens

```text
COLOR TOKENS
  bg.canvas          = #F3F1EC
  bg.surface         = #FBFAF7
  bg.panel           = #ECE8DE
  fg.primary         = #1F2328
  fg.muted           = #5D6670
  accent.primary     = #0B6E4F
  accent.warning     = #A35A00
  accent.error       = #A32323
  border.soft        = #D9D2C2
  border.strong      = #BEB39A

TYPOGRAPHY TOKENS
  font.ui            = "IBM Plex Sans"
  font.code          = "IBM Plex Mono"
  font.preview_body  = "Source Serif 4"
  scale.h1           = 32px / 1.2
  scale.h2           = 26px / 1.25
  scale.h3           = 22px / 1.3
  scale.body         = 16px / 1.6
  scale.small        = 13px / 1.4

SPACING TOKENS
  space.1 = 4
  space.2 = 8
  space.3 = 12
  space.4 = 16
  space.5 = 24
  space.6 = 32
  radius.s = 4
  radius.m = 8
  radius.l = 12
```

Accessibility baselines:

- minimum text contrast 4.5:1 for primary content,
- keyboard navigation for all interactive controls,
- visible focus ring on actionable elements,
- no information by color alone.

## 4) Component Inventory

Core components:

- `MainWindow`
- `TitlebarView`
- `SidebarView`
- `OutlineTreeView`
- `EditorView`
- `PreviewView`
- `DiagnosticsPanel`
- `StatusBar`
- `CommandPalette`
- `ToastCenter`

Editor-specific widgets:

- line gutter with diagnostics marks,
- heading breadcrumbs,
- selection and search highlights,
- inline lint badges (non-blocking).

Preview-specific widgets:

- semantic blocks with stable anchors,
- code block container with language badge,
- table container with horizontal overflow handling.

## 5) Motion and Transitions

Motion should be subtle and informative:

- panel open/close: `120ms` ease-out,
- palette entry and overlay fade: `100ms`,
- diagnostics item highlight pulse: `160ms`,
- focus ring transition: `80ms`.

Rules:

- no continuous idle animation,
- no large spring effects in writing surfaces,
- animation must preserve readability and cursor stability.

## 6) Adaptive Layout Behavior

Large windows (`>= 1400px`):

- default three-column layout (sidebar/editor/preview),
- diagnostics docked bottom and always visible (small height),
- editor and preview split approximately `50/50`.

Medium windows (`1000px - 1399px`):

- sidebar collapsible by default,
- editor/preview split remains, diagnostics collapsible.

Small windows (`< 1000px`):

- single-main-pane mode with quick toggle between editor and preview,
- overlay TOC drawer,
- diagnostics as modal/panel sheet.

## GPUI Implementation Notes

Suggested state shape:

```text
AppState
  - active_file: PathBuf
  - buffer: Rope/String
  - parse_result: Document + diagnostics
  - ui_layout: { sidebar_open, preview_mode, diagnostics_open, split_ratio }
  - focus: { sidebar|editor|preview|palette|diagnostics }
  - theme: token set id
```

Suggested rendering loop:

```text
on_buffer_change
   -> debounce(60-120ms)
   -> parse_markdown in background task
   -> update semantic document + diagnostics
   -> rerender preview + outline
```

Performance guidance:

- avoid full preview rebuild for small edits where possible,
- cache heading map and block offsets,
- cap expensive syntax-highlighting updates to visible ranges first.

## Implementation Priority

1. Window shell + split panes + status bar.
2. Editor + preview live wiring.
3. TOC and diagnostics panel.
4. Command palette and keymap completion.
5. polish pass: motion, tokens, focus/contrast audit.
