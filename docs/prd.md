# Product Requirements Document

## Product Name

MD Star

## Date

2026-04-25

## Executive Summary

MD Star is an open-source Markdown viewer/editor for macOS and Linux. It
combines a Tauri desktop app, a terminal `md` workflow, and shared Rust parsing
and rendering components.

## Problem Statement

Developers need a local Markdown tool that is fast, reliable, attractive, and
comfortable in both GUI and terminal workflows. Public project quality matters:
the first GitHub release should look maintained, testable, and intentional.

## Goals

- Deliver professional Markdown rendering in desktop and terminal contexts.
- Keep one shared semantic core for all surfaces.
- Support file-manager workflows such as macOS "Open With".
- Ship credible GitHub release artifacts.
- Keep the repository approachable for contributors.

## Non-Goals For First Public Release

- Cloud sync or collaboration.
- Full WYSIWYG editing.
- Signed/notarized macOS distribution.
- Completed Quick Look extension.
- Completed Linux MIME installer flow.

## Target Users

- Engineers writing READMEs, specs, and project docs.
- OSS maintainers checking Markdown before publishing.
- Terminal-first users who want a better `cat`/pager workflow for Markdown.

## User Stories

### Desktop Open And Preview

User opens a `.md`, `.markdown`, or `.txt` file in the desktop app and sees a
rendered preview, outline, and document metadata.

Acceptance:

- file picker opens supported files,
- drag/drop opens supported files,
- macOS "Open With" passes the file path into the app,
- malformed or missing files report errors without crashing.

### Desktop Edit

User edits source and sees the preview update.

Acceptance:

- edit pane opens and closes cleanly,
- save writes to the selected file,
- external changes reload when the buffer is not dirty.

### CLI View

User runs `md <file.md>` and gets readable terminal output.

Acceptance:

- output handles headings, lists, tables, code, and Mermaid fences,
- pager behavior is explicit and controllable,
- `--plain`, `--no-tui`, `--no-color`, and `--ascii-mermaid` work.

### Release Install

User can download an artifact from a GitHub release.

Acceptance:

- macOS DMG is attached to the release,
- CLI archives and checksums are attached,
- installer script points at the real GitHub repository.

## Non-Functional Requirements

- Quality: no known P0/P1 bugs before public release.
- Reliability: all user-facing IO errors are handled.
- Performance: common Markdown files open quickly.
- Security: preview HTML must avoid active script behavior where possible.
- Maintainability: crate boundaries remain clear and tests cover core flows.

## Current Risks

- macOS signing/notarization is not configured.
- Tauri file association behavior must be validated from an installed app.
- Linux packaging needs clean-runner verification.
- The legacy `mdstar-cli` crate can confuse contributors if not clearly documented.
