# MD Star Roadmap

Date: 2026-04-26

This roadmap defines what to build after first-public-release readiness so
MD Star can move from "usable" to "competitive" for developer workflows.

## Product Position

Primary lane:

```text
MD Star = local-first Markdown app + terminal-native workflow
```

The strategy is to win on reliability, speed, and shell/desktop integration
before expanding into a broader plugin ecosystem.

## 12-Month Roadmap

```text
Q2 2026 (Phase 1)  -> Trust + Reliability
Q3 2026 (Phase 2)  -> Developer Core UX Parity
Q4 2026 (Phase 3)  -> Differentiators
Q1 2027 (Phase 4)  -> Ecosystem + Scale
```

---

## Phase 1: Trust + Reliability (Q2 2026)

Outcome:

```text
Any Markdown file opens reliably from Finder/shell/drag-drop, app remains stable,
and install/update paths are production-grade.
```

Deliverables:

1. File-open reliability hardening:
   - macOS `Open With` handoff validation for installed app bundle.
   - drag-drop reliability validation against installed binary, not dev build only.
   - CLI handoff behavior (`md file.md`) deterministic for TUI vs app mode.
2. Editing safety:
   - autosave policy.
   - unsaved-change guard rails.
   - crash recovery buffer for the active document.
3. Release quality:
   - deterministic installer outputs on CI.
   - release artifact verification checklist per platform.
4. macOS distribution baseline:
   - app signing and notarization pipeline design (implementation may continue into
     Phase 2 depending on certificate readiness).

Exit criteria:

- 99%+ success in local scripted open tests for `Open With`, drag-drop, CLI open.
- No known P0/P1 data-loss bug.
- Release checklist runs clean on CI for tagged builds.

Risks:

- macOS runtime behavior differences between dev mode and installed app bundles.
- signing/notarization credentials setup delays.

---

## Phase 2: Developer Core UX Parity (Q3 2026)

Outcome:

```text
MD Star becomes a daily-driver editor for developer note/docs workflows.
```

Deliverables:

1. Workspace usability:
   - multi-tab editing.
   - session restore.
   - recent files and quick reopen.
2. Navigation and productivity:
   - command palette.
   - customizable keyboard shortcuts.
   - fast in-document search/replace.
3. Editing polish:
   - predictable split/source/preview behavior.
   - improved outline navigation and heading jump.
4. Performance:
   - benchmark suite for large markdown files and frequent re-render cycles.

Exit criteria:

- Supports sustained daily usage without workflow-blocking regressions.
- Large file baseline met (define target in benchmark doc and track in CI).
- Keyboard-first flows complete for top 10 user actions.

Risks:

- feature scope creep into "all editor features" instead of developer-focused core.

---

## Phase 3: Differentiators (Q4 2026)

Outcome:

```text
MD Star has clear reasons to choose it over general-purpose markdown editors.
```

Deliverables:

1. Shell/Desktop bridge:
   - open in existing running app instance from CLI.
   - file focus handoff and tab activation.
2. Dev-focused markdown intelligence:
   - markdown lint checks.
   - frontmatter schema validation.
   - document diagnostics surface.
3. Git-aware workflow:
   - changed-section preview for markdown docs.
   - lightweight diff preview for active file.
4. Diagram experience:
   - robust Mermaid authoring/preview behavior and error diagnostics.

Exit criteria:

- At least 2 marquee features unavailable (or materially worse) in most competing
  markdown editors used by developer audiences.
- Feature adoption measured via docs examples and user feedback issues.

Risks:

- building differentiators before Phase 1/2 reliability baseline is locked.

---

## Phase 4: Ecosystem + Scale (Q1 2027)

Outcome:

```text
MD Star is extensible, community-friendly, and maintainable as a public project.
```

Deliverables:

1. Extension model:
   - small, stable plugin API surface.
   - security boundary and versioning policy.
2. Import/export polish:
   - predictable HTML/PDF export behavior.
   - documented compatibility matrix.
3. Community operations:
   - public roadmap board.
   - contributor quickstart improvements.
   - release cadence and changelog discipline.

Exit criteria:

- External contributors can build and test without maintainer handholding.
- Plugin API marked stable for at least one minor series.

Risks:

- plugin API freeze too early; maintainability burden for solo maintainer.

---

## Execution Rules

```text
1) Reliability before feature breadth.
2) One primary lane: local-first + terminal-native markdown workflows.
3) Every phase ships with measurable exit criteria.
4) No major new subsystem without tests and release notes impact review.
```

## Next 4-Week Sprint (Start Now)

1. Lock Phase 1 open-path test matrix and automate it.
2. Finalize release artifact verification for macOS/Linux/Windows.
3. Ship autosave + unsaved-change protection.
4. Publish v0.1.x with release notes that reflect these reliability guarantees.
