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
- `pytest`, `pytest-asyncio` (dev dependencies via `uv add --dev`). Optional UI tooling such as `textual-devtools` should be installed manually once a compatible distribution is published, ensuring developers can still inspect layouts without blocking the core dependency set.
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
- `status_links.py`: maintain supersedes/deprecated cross references and surface descriptive `DecisionParseError` messages when the canonical fields are missing or malformed.
- Add documentation snippet describing marker block structure and parsing assumptions.
- Unit tests with fixture files covering parsing edge cases and serialization.

### Milestone 4 – State Management and Commands
- `state.py`: define `Decision`, `DecisionLink`, `AppState`.
- `commands.py`: operations (new decision, edit existing, reorder, delete,
  refresh).
- Integrate parser/persistence calls with state updates.
- Tests verifying state transitions and command side effects.

### Milestone 5 – Textual UI Skeleton
- `ui/app.py`: instantiate panes, handle pane focus, status bar updates.
- `ui/panes/*.py`: basic widgets reflecting milestone 3 data.
- Implement numeric focus (`1`–`4`), navigation keys, `space` actions.
- Implement `?` overlay with key map.

### Milestone 6 – Editor and Preview Features
- Editor pane with Textual `TextArea`/`Input` integration.
- Status picker interaction (modal or inline).
- Markdown preview rendering using `markdown-it-py`.
- Decision reorder (`J/K`) and delete (`d`) interactions.
- Save (`s`) command with persistence flush and status bar updates.
- New decision (`n`) flow hooking template.

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
  present UI feedback when references invalid.
