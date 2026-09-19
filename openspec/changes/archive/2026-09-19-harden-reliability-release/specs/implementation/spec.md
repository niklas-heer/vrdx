## MODIFIED Requirements

### Requirement: Package Structure and Responsibilities
The system SHALL provide a Rust binary and library separating terminal lifecycle and CLI, application/draft state, source-aware decision parsing and persistence, and Ratatui rendering. Premise SHALL supply the domain's named record representation and keyed lookup.

#### Scenario: CLI entrypoint bootstraps the app layer
- **WHEN** the user invokes vrdx with an optional repository directory
- **THEN** the Rust entrypoint SHALL initialize application state and run the Ratatui event loop
- **AND** terminal modes SHALL be restored on normal exit, interrupt, and handled error

#### Scenario: Parser utilities are isolated from UI widgets
- **WHEN** a decision is parsed, validated, or persisted
- **THEN** the operation SHALL use the document layer without requiring a terminal

#### Scenario: Premise record boundary
- **WHEN** a domain record is represented as Premise named fields and restored
- **THEN** all values, including every valid u64 identifier and Unicode text, SHALL remain exact
- **AND** invalid or ambiguous field shapes SHALL be rejected

### Requirement: Dependency and Tooling Strategy
The project SHALL use mise and Cargo with pinned nightly Rust, a committed Cargo.lock, rustfmt, Clippy, Rust Analyzer, nextest, Bacon, watchexec, cargo-generate, cargo-seek, and Criterion. Development executables SHALL NOT be runtime dependencies. Application crates SHALL have minimal required features. The user-selected Rust/mise distribution supersedes Python-only uv distribution and justfile/Earthly workflows.

#### Scenario: Reproducible setup
- **WHEN** a contributor follows the documented mise setup
- **THEN** the pinned compiler, components, and development tools SHALL be available
- **AND** setup SHALL NOT mutate application dependencies or lockfiles

#### Scenario: Standard developer workflows
- **WHEN** a developer runs the documented mise formatting, check, lint, test, doc-test, benchmark, or run tasks
- **THEN** the task SHALL execute against the pinned toolchain and committed dependencies
- **AND** watch tasks SHALL run checks rather than repeatedly launch the interactive TUI

### Requirement: Milestone-Driven Delivery
Implementation SHALL deliver Rust foundation, file integrity, terminal workflows, real verification, and migration in dependency order, maintaining working behavior and tests for each completed slice.

#### Scenario: File-integrity milestone
- **WHEN** file-integrity work completes
- **THEN** parser and persistence tests SHALL verify source preservation, conflicts, validation, and exclusive creation

#### Scenario: Terminal-workflow milestone
- **WHEN** terminal work completes
- **THEN** users SHALL create, edit, save, reopen, cancel, refresh, and navigate decisions through the actual Rust UI

### Requirement: Testing and Quality Gates
The project SHALL execute unit, integration, Ratatui TestBackend, and real pseudo-terminal tests in macOS/Linux CI. Real terminal tests SHALL launch the actual binary, send characters and control sequences through an OS pseudo-terminal, decode the displayed screen, and assert persisted output. A mocked event handler alone SHALL NOT satisfy end-to-end coverage.

#### Scenario: Running local test suite
- **WHEN** a contributor invokes the mise test task
- **THEN** nextest SHALL execute unit and integration tests including the real terminal journeys
- **AND** a separate doc-test task SHALL verify documentation tests

#### Scenario: CI enforcement
- **WHEN** a pull request or main push triggers CI
- **THEN** rustfmt, all-target compilation, strict Clippy with warnings denied, nextest, doc tests, and installed-binary checks SHALL run and fail on errors

#### Scenario: Character-driven terminal journeys
- **WHEN** the terminal suite runs
- **THEN** it SHALL cover individual UTF-8 character input, bracketed paste, control keys, resize, save/reopen, cancelled drafts, and stale-file errors
- **AND** tests SHALL wait on observable screen states, enforce timeouts, clean up children, and verify terminal restoration

#### Scenario: Installed artifact outside checkout
- **GIVEN** a separately installed vrdx binary
- **WHEN** smoke and terminal tests run outside the checkout
- **THEN** help/version and create/edit/save/reopen SHALL succeed without Python or source-tree runtime imports

### Requirement: Risk Tracking and Mitigation
The project SHALL document and test Markdown ambiguity, observed concurrent edits, terminal restoration, and dependency compatibility. It SHALL preserve user data on detected failures and state limitations accurately.

#### Scenario: Handling parsing ambiguity
- **WHEN** source cannot be safely interpreted
- **THEN** the file SHALL have a visible diagnostic and SHALL NOT be silently rewritten

#### Scenario: Managing dependency drift
- **WHEN** the pinned compiler or runtime dependencies are updated
- **THEN** all relevant compilation, lint, and real-terminal checks SHALL pass before delivery

### Requirement: Tooling and Quality Gates Adoption
Production Rust SHALL forbid unsafe code and enforce the requested pedantic, nursery, and safety-focused Clippy restrictions. Any allowance SHALL be narrowly scoped and justified; tests MAY allow unwrap/expect/panic/indexing for assertions. Optional tools SHALL be explicit developer tasks rather than hidden hooks.

#### Scenario: Enforcing linting
- **WHEN** Clippy checks all targets and features with warnings denied
- **THEN** prohibited production unwraps, panics, unchecked arithmetic, indexing, casts, placeholders, and exit calls SHALL fail the check

#### Scenario: Encouraging rapid iteration
- **WHEN** contributors launch Bacon or watchexec through mise
- **THEN** changes SHALL produce compiler/lint/test feedback without taking control of the terminal to run vrdx

## ADDED Requirements

### Requirement: Documentation Matches Shipped Behavior
The README SHALL describe Rust/mise setup, supported commands and bindings, copyable Markdown records, and current features. Historical Python distribution decisions SHALL be marked superseded by the user's Rust decision.

#### Scenario: Following the documented interface
- **WHEN** users follow quick-start instructions and copy the decision example
- **THEN** the installed binary SHALL support the documented behavior and the production parser SHALL recognize the example
