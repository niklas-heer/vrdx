# Implementation Plan

## 1. Overview

This document translates the high-level design in `docs/DESIGN.md` into concrete
engineering tasks. It enumerates the modules, dependencies, and milestones
required to deliver the vrdx TUI, parser, and build pipeline.

## 2. Project Structure

```
vrdx/
├── __init__.py
├── main.py                # CLI entry point
├── cli.py                 # Argument parsing, environment setup
├── app/
│   ├── __init__.py
│   ├── state.py           # AppState, Decision models, status registry
│   ├── commands.py        # High-level actions (new, edit, reorder, delete)
│   ├── persistence.py     # File I/O, marker insertion, writing updates
│   ├── discovery.py       # Markdown discovery, caching, refresh
│   └── status_links.py    # Cross-link validation for supersedes/deprecated
├── parser/
│   ├── __init__.py
│   ├── markers.py         # Marker detection, scaffold creation
│   ├── decisions.py       # Parse decision sections, serialize updates
│   └── template.py        # New decision template helpers
└── ui/
    ├── __init__.py
    ├── app.py             # Textual App subclass, pane lifecycle
    ├── panes/
    │   ├── __init__.py
    │   ├── decisions.py   # Pane 1 widget (list + interactions)
    │   ├── files.py       # Pane 4 widget (file list)
    │   ├── editor.py      # Pane 2 widget (editable text area)
    │   └── preview.py     # Pane 3 widget (rendered markdown)
    ├── dialogs.py         # Prompts (marker insertion, delete confirm, help)
    └── keymap.py          # Key binding definitions, overlay content
```

Tests live under `tests/` mirroring the package structure. Fixture markdown
files reside in `tests/fixtures/`.

## 3. Dependencies

Add with `uv add`:

- `textual` (TUI)
- `rich` (already a dependency of textual but useful for logging)
- `markdown-it-py` (preview rendering) and `mdurl`
- `pydantic` for decision models and schema validation
- `watchfiles` (optional refresh enhancement; consider later)
- `pytest`, `pytest-asyncio` (dev dependencies via `uv add --dev`). Optional UI tooling—such as Textual-specific testing helpers or future `textual-devtools` builds—should be installed manually once compatible distributions are available so developers can inspect layouts without bloating the core dependency set.
- `ruff` and `mypy` for linting/type checking (dev)

The toolchain targets macOS and Linux; Windows support is currently out of scope.

Update `pyproject.toml` to reflect main optional entry points and configure
scripts:

```
[project.scripts]
vrdx = "vrdx.main:main"
```

## 4. Milestones

### Milestone 1 – Infrastructure and Utilities
- Scaffold package layout (`__init__`, directories).
- Implement `cli.py` basic argument parsing (cwd override, `--version`).
- Implement logging setup (per design; possibly `vrdx.log` under cache dir).
- Unit tests for CLI, logging, and app initialization.

### Milestone 2 – File Discovery and Marker Management
- `discovery.py`: find Markdown files from working directory.
- `markers.py`: detect `<!-- vrdx start -->` / `<!-- vrdx end -->`, prompt logic
  (hook for UI).
- `persistence.py`: function to insert marker scaffold with confirmation.
- Document discovery expectations with a brief snippet illustrating how results feed the UI.
- Tests ensuring discovery respects nested directories and prompt logic is
  triggered only when markers missing.

### Milestone 3 – Decision Parsing and Serialization
- `decisions.py`: parse decisions from marker regions into structured `DecisionRecord` (Pydantic) models that retain IDs, titles, statuses, narrative fields, and the original markdown slice.
- `template.py`: generate next ID, default status, and stub sections.
- Round-trip serialization (insert new, reorder, delete) implemented centrally, preserving the host file’s newline style when regenerating the decision block.
- Expose descriptive `DecisionParseError` messages so later milestones can surface actionable UI feedback when canonical fields are missing or malformed.
- Add documentation snippet describing marker block structure and parsing assumptions.
- Unit tests with fixture files covering parsing edge cases and serialization.

### Milestone 4 – State Management and Commands
- `state.py`: defines `DecisionState`, `FileState`, and `AppState`, capturing selection, modification, and pane focus (implemented).
- `commands.py`: provides create/update/delete flows, ordering, linking, navigation helpers, and serialization hooks (implemented).
- `status_links.py`: slated for a focused follow-up to encapsulate cross-link rules once UI workflows surface them.
- Parser/persistence wiring is exercised through command paths to keep marker-backed bodies in sync.
- Tests validate state transitions, linking reciprocity, navigation helpers, and error propagation.

### Milestone 5 – Textual UI Skeleton
- `ui/app.py`: skeleton Textual shell wiring AppState to the live decision/file lists, pane focus, and dirty indicator (implemented).
- `ui/panes.py`: lightweight pane descriptors to support upcoming richer widgets (implemented).
- Numeric focus (`1`–`4`), navigation keys (`j/k` and arrows), `space` stubs, and the `?` help overlay are handled at the app layer (implemented; editor interactions deferred).
- `ui/styles.tcss`: initial stylesheet to lay out the left column, editor, and preview panes (added).

### Milestone 6 – Editor and Preview Features
- **Implemented**: Upgraded the editor pane to a Textual `TextArea`, allowing read-only browsing versus editable drafts, seeding new decision templates, and wiring the save flow (`s`) through the command layer with status feedback.
- **Remaining**: Add curated status selection UI, render the preview pane with live Markdown, and support in-app reordering/deletion (`J/K`, `d`) with confirmations.

### Milestone 7 – Polish and Packaging
- Visual polish: highlight active pane, ensure colors accessible.
- Logging for errors (file write failures, parse issues).
- Wire `main.py` to run the Textual app.
- Update `justfile` targets to use `uv run`.
- Verify Nuitka emits the static standalone binary at `bin/vrdx`.
- Add `just test`, `just lint`, `just typecheck`, `just build`.
- Document usage in README and design doc references (link to help overlay).

## 5. Testing Strategy

- **Unit tests**: parser, template, status link logic, commands.
- **Integration tests**: run CLI on fixture directories; assert file modifications.
- **TUI tests**: use Textual’s `AppTest` utilities to simulate key presses.
- **Regression tests**: ensure cross-linking updates both sides correctly.
- Add GitHub-friendly snapshots for serialized decision blocks.

## 6. Tooling and Quality Gates

- Configure `ruff` (lint + formatting) and `mypy`.
- Add `pytest.ini` / `pyproject` entries to control test discovery.
- Pre-commit config (optional) to run `ruff`, `mypy`, `pytest` (fast subset).
- Document dev workflow in README (install, run, test, build, lint).

## 7. Deliverables

1. Functional CLI binary supporting the specified TUI interactions, packaged as a static standalone binary at `bin/vrdx`.
2. Automated tests passing locally (`just test`) and during CI.
3. Updated documentation:
   - README quick-start with `uv` commands and Nuitka build note.
   - `docs/DESIGN.md` kept in sync with implementation changes.
   - This implementation guide updated as milestones complete.

## 8. Risks & Mitigations

- **Markdown parsing ambiguity**: rely on explicit markers and consistent
  H3 structure; add tests for malformed cases and fallback error messaging.
- **Textual API changes**: pin version in `pyproject.toml` and monitor release
  notes.
- **Binary size/compatibility**: verify Nuitka build on macOS, consider CI
  job capturing output; add smoke test for binary execution.
- **Cross-link data integrity**: enforce invariants in `status_links.py` and
  present UI feedback when references are invalid or missing.
