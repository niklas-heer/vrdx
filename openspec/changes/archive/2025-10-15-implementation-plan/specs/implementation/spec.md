## ADDED Requirements
### Requirement: Package Structure and Responsibilities
The system SHALL maintain the vrdx package layout outlined in the historical implementation plan, separating CLI entrypoints, application orchestration, parser utilities, and Textual UI panes into the directories `vrdx/cli.py`, `vrdx/app/`, `vrdx/parser/`, and `vrdx/ui/`, respectively.

#### Scenario: CLI entrypoint bootstraps the app layer
- **GIVEN** the CLI module `vrdx/cli.py`
- **WHEN** the user executes `vrdx` from the command line
- **THEN** the module SHALL resolve arguments, configure logging, construct `AppState`, and invoke the Textual application defined under `vrdx/ui/app.py`

#### Scenario: Parser utilities are isolated from UI widgets
- **GIVEN** a need to detect or serialize decision markers
- **WHEN** the application processes Markdown files
- **THEN** the logic SHALL execute within `vrdx/parser/markers.py`, `vrdx/parser/decisions.py`, or related helpers, avoiding duplication inside UI or command modules

### Requirement: Dependency and Tooling Strategy
The project SHALL manage runtime dependencies (Textual, Rich, markdown-it-py, mdurl, Pydantic) and development tooling (pytest, pytest-asyncio, textual-dev, ruff, mypy, ptw) through uv, keeping configuration in `pyproject.toml` and orchestrated commands in `justfile`.

#### Scenario: Syncing dependencies with uv
- **GIVEN** a developer cloning the repository
- **WHEN** they run `uv sync`
- **THEN** the runtime and development dependencies defined in `pyproject.toml` SHALL be installed into the uv-managed environment

#### Scenario: Standard developer workflows
- **GIVEN** the `justfile` recipes
- **WHEN** a developer runs `just dev`, `just test`, or `just lint`
- **THEN** each recipe SHALL execute the corresponding uv command (`textual run --dev`, `pytest`, `ruff check`) without requiring manual virtual environment management

### Requirement: Milestone-Driven Delivery
Implementation SHALL proceed through the seven historical milestones: (1) infrastructure scaffolding, (2) discovery and marker management, (3) parsing and serialization, (4) state and commands, (5) TUI skeleton, (6) editor/preview enhancements, and (7) polish and packaging, ensuring each milestone’s deliverables are satisfied before advancing.

#### Scenario: Completing Milestone 2
- **GIVEN** milestone sequencing
- **WHEN** Milestone 2 work concludes
- **THEN** discovery utilities SHALL locate Markdown files with appropriate ignores, marker detection SHALL prompt for scaffold insertion, and tests SHALL cover directory filtering and prompt conditions

#### Scenario: Milestone 6 editor enhancements
- **GIVEN** the transition from TUI skeleton to richer editing capabilities
- **WHEN** Milestone 6 tasks are executed
- **THEN** the Textual editor pane SHALL use a `TextArea`, support new-decision templates, saving, and status feedback while pending UI polish items remain scoped to later work

### Requirement: Testing and Quality Gates
The implementation SHALL uphold the testing strategy described in the historical plan, including unit tests for parser/state modules, Textual `AppTest` coverage for interaction flows, integration tests using Markdown fixtures, and continuous integration via GitHub Actions and Earthly.

#### Scenario: Running local test suite
- **GIVEN** a developer preparing a change
- **WHEN** they execute `uv run pytest -v`
- **THEN** parser, discovery, state, persistence, and UI tests SHALL run successfully, providing confidence before opening a pull request

#### Scenario: CI enforcement
- **GIVEN** a pull request or push to main
- **WHEN** GitHub Actions triggers
- **THEN** the workflow SHALL run uv-based test jobs (including Earthly multi-distro runs where configured) and report failures before merge

### Requirement: Risk Tracking and Mitigation
The project SHALL preserve awareness of key risks—Markdown parsing ambiguity, Textual API changes, Python distribution assumptions, and decision cross-link integrity—and document the recommended mitigations for each.

#### Scenario: Handling parsing ambiguity
- **GIVEN** the risk that Markdown edge cases could defeat marker parsing
- **WHEN** new Markdown patterns appear in repositories
- **THEN** developers SHALL reference the archived mitigation (explicit markers, structured headings, targeted unit tests, descriptive parser errors) before altering parser logic

#### Scenario: Managing dependency drift
- **GIVEN** potential Textual or dependency updates
- **WHEN** compatibility issues arise
- **THEN** maintainers SHALL revisit the mitigation guidance (pinning versions, monitoring release notes, validating with existing tests) before upgrading

### Requirement: Tooling and Quality Gates Adoption
The team SHALL maintain linting (`ruff`), optional static typing (`mypy`), and fast feedback loops (ptw, textual console) as first-class aspects of the implementation roadmap, encouraging contributors to integrate these tools into everyday development.

#### Scenario: Enforcing linting
- **GIVEN** code changes that introduce style violations
- **WHEN** `uv run ruff check .` is executed locally or in CI
- **THEN** the lint step SHALL surface issues so they can be resolved prior to merge

#### Scenario: Encouraging rapid iteration
- **GIVEN** the availability of watch-mode tooling such as `ptw` and Textual’s console
- **WHEN** developers iterate on UI changes
- **THEN** they SHALL be able to run `just dev-watch` or `just console` to obtain immediate feedback, aligning with the roadmap’s focus on rapid TUI iteration
