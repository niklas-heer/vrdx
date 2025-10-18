# Project Context

## Purpose
vrdx is a standalone CLI/TUI for curating architecture and engineering decision records directly inside a repository. It emulates the ergonomics of tools like lazygit while focusing on discovering Markdown files, parsing structured decision blocks, and helping practitioners compose, review, and update decisions without leaving the terminal.
The project targets engineering teams that prefer Markdown-based documentation and want a lightweight workflow for decision governance without introducing heavyweight web tooling or bespoke storage formats.

## Tech Stack
- **Language & Runtime:** Python 3.13 managed with uv for reproducible environments.
- **TUI Framework:** Textual (with Rich) powers the four-pane terminal interface and async event loop.
- **Markdown Processing:** markdown-it-py and mdurl provide targeted parsing around decision markers.
- **Data Modeling:** Pydantic models ensure structured decision records with validation.
- **Packaging & Distribution:** Hatchling backend with uv tool install for delivery (no binary bundling).
- **Testing & Tooling:** pytest, pytest-asyncio, textual-dev for UI tests, ptw for watch mode, and ruff for linting and formatting enforcement.

## Project Conventions

### Code Style
- Write fully type-annotated Python; prefer explicit dataclasses or Pydantic models for shared data.
- Keep modules single-purpose (e.g., discovery, parsing, state) to align with the layered architecture.
- Enforce style with ruff (see `just lint` / `just lint-fix`) and maintain descriptive docstrings for public functions.
- Avoid silent modifications to user files; surface errors with actionable log messages via the centralized logging helpers.

### Architecture Patterns
- CLI entrypoints (`main.py`, `cli.py`) handle argument parsing, logging configuration, and `AppState` wiring.
- App runner bootstraps a Textual `VrdxApp` that orchestrates pane focus, state transitions, and command routing.
- Discovery and persistence layers isolate filesystem scanning, marker detection, and write-back so UI logic remains declarative.
- Parser layer converts marker blocks into structured decision models and guarantees canonical serialization for round-trip edits.
- State and command modules centralize mutations (create/update/reorder/delete) and maintain cross-pane synchronization.
- UI pane widgets follow a lazygit-inspired layout (decisions, editor, preview, files) sized to operate within an 80×24 terminal while scaling up gracefully.

### Testing Strategy
- Unit tests cover CLI resolution, discovery filters, marker parsing, decision serialization, state transitions, and command behaviors.
- Integration fixtures exercise end-to-end flows that read, edit, and persist decision blocks across Markdown files.
- Textual `AppTest`-based tests simulate key bindings and pane focus changes to guard the TUI interaction model.
- Watch-mode (`ptw`) and CI runs (`uv run pytest -v`, Earthly multi-distro) ensure rapid feedback.

### Git Workflow
- The canonical branch is `main`; CI runs on pushes to `main` and on every pull request.
- Contributors develop changes on topic branches and submit pull requests; Earthly plus GitHub Actions must pass before merging.
- Commit style is conventional-but-unenforced—use concise, imperative subject lines that describe behavior, not implementation details.
- Release artifacts align with `main` and are distributed via `uv tool install` (no separate release branch or binary packaging).

## Domain Context
- Decision records live inside Markdown marker blocks bounded by `<!-- vrdx start -->` and `<!-- vrdx end -->` delimiters.
- Each decision starts with a heading in the form `### <ID> <Title>` and includes bullet-labeled Status, Decision, Context, and Consequences fields.
- Status values are curated (📝 Draft, ✅ Accepted, ❌ Rejected, ⛔ Deprecated by …, ⬆️ Supersedes …) with reciprocal link management handled by the command layer.
- The UI presents four panes (decisions list, editor, preview, files) with numeric focus shortcuts (`1`–`4`), lazygit-style navigation (`j`/`k` or arrows), and a contextual help overlay (`?`).
- New decisions are inserted at the top of the marker block so the most recent decisions remain visible.

## Important Constraints
- Official support targets macOS and Linux; Windows support is presently out of scope.
- Distribution is Python-only via uv; binary bundlers (PyInstaller, Nuitka, etc.) are intentionally unsupported.
- The application assumes Markdown files with canonical markers; malformed or duplicated markers surface explicit errors and require user correction before persistence.
- Automatic Git operations are intentionally omitted—users manage commit, stage, and push steps manually.
- Terminal layout is optimized for 80×24; larger terminals expand, but smaller sizes degrade the experience.

## External Dependencies
- GitHub Actions with Earthly orchestrate CI across macOS and Linux.
- uv provides runtime management, tool installation, and upgrade flows for end users.
- Textual and related libraries (Rich, markdown-it-py, mdurl, Pydantic) are bundled as part of the Python distribution—no external web services are required at runtime.
