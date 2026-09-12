# Project Context

## Purpose

vrdx manages engineering decisions directly in repository Markdown files through a keyboard-first terminal UI. Files remain readable and reviewable with ordinary editors and Git. Runtime behavior does not depend on hosted services or perform automatic Git operations.

## Stack and Development

- Rust 2024, pinned to nightly-2026-09-06 at the user's request; there was no pre-existing Rust MSRV. No stable MSRV is claimed.
- Ratatui with Crossterm for rendering, terminal lifecycle, keyboard/mouse/paste input, and resize events.
- Premise (library name patterns) for named records, keyed lookup, and code-attached rationale in the domain core.
- pulldown-cmark for source-offset Markdown context, tempfile for atomic publication, unicode-width for terminal cells, and signal-hook for clean interrupt handling.
- mise manages the pinned compiler and development utilities. Cargo.lock is committed. No Nix, devenv, justfile, Python, or uv runtime is required.

Use mise install followed by mise run ci for local verification. Rust Analyzer settings are checked in. Bacon and watchexec provide check/test feedback without automatically launching the interactive application. nextest executes unit and integration tests; cargo test --doc runs documentation tests. Criterion benchmarks parser throughput.

## Architecture

- src/main.rs: argument parsing and terminal/event lifecycle.
- src/document.rs: validated Premise-compatible records, source-preserving parsing, and persistence.
- src/app.rs: repository discovery, file/record selection, draft state, input handling, and text editing.
- src/ui.rs: responsive Ratatui rendering and mouse hit regions.
- tests/: executable pseudo-terminal journeys and documentation checks.

Document operations work without a TUI. The app owns one draft, and saves commit domain state only after persistence succeeds. Record IDs are unique per file. Core records retain full u64 identity through Premise's text field encoding rather than its saturating integer conversion.

## Data and Reliability

The canonical block uses <!-- vrdx start --> and <!-- vrdx end --> outside Markdown code examples. Records start with ### <ID> <Title> and have Status, Decision, Context, and Consequences labels. Title and status must be nonempty; narrative values may be empty. New records are inserted at the top. Unknown status text remains readable.

Preserve untouched source, newline styles, and existing records. Duplicate/missing labels, duplicate IDs, malformed markers, invalid UTF-8, or ambiguous generated structure must yield diagnostics rather than silent rewrites. No-op saves do not write. Atomic saves compare the original raw source before preparation and before replacement. External-editor races between the final check and rename remain a documented limitation. New files use no-clobber publication. Failed saves retain drafts.

## Testing and Quality

Formatting, all-target compilation, strict Clippy, nextest, doc tests, and installed-binary checks run in CI on Linux and macOS. Production Rust forbids unsafe code and the requested panic/index/arithmetic/cast restrictions. Any allowance must be narrowly scoped and explained. Tests may use assertions and explicit test-only allowances.

Ratatui TestBackend tests cover rendering and focus at compact and large sizes. portable-pty tests launch the actual executable, type UTF-8 characters and terminal sequences, decode the VT100 screen, inspect saved files, and check terminal cleanup. Tests use temporary repositories, bounded waits, and child-process cleanup. Installed tests run from outside the checkout.

## UI Contract

Normal mode uses 1 Decisions, 2 Files, 3 Editor, 4 Preview; j/k or arrows navigate, Enter/Space edits, n creates, r reloads, ? shows help, q quits. Editing uses Tab/Shift+Tab, Ctrl+S, Escape, and Ctrl+Q for guarded quit. Alt+1–4 switches panes during editing without consuming ordinary digit input. Status selection preserves other fields. Save and Cancel return to a read-only view. Save/Discard/Stay guards protect drafts when abandoning them.

Minimum supported size is 80×24. Compact layout keeps actions reachable and preview accessible; smaller terminals request resizing without discarding work. NO_COLOR is honored, and focus is visible without color.

## Workflow and Scope

Use topic branches and Conventional Commits. OpenSpec tracks approved changes; keep specifications synchronized with implementation and archive after delivery. The user's Rust/Ratatui/mise/nightly/Premise instructions supersede historical Python-only distribution decisions. references/ and archived specifications remain read-only historical context.

Headless JSON commands, search, persistent file-and-ID relationships, repository templates, deletion/reordering, field-level three-way merge, and Git-history inspection are implemented capabilities. Supported platforms remain macOS/Linux. Premise Fielded/Keyed contracts are shared by the parser, persistence, and agent transport; Rust documentation tests remain part of the standard crate architecture.
