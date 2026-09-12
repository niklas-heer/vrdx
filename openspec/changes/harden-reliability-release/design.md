## Context

The user selected a Rust migration with Ratatui, Premise, mise, and nightly Rust. There was no existing Cargo manifest, Rust lockfile, or MSRV. nightly-2026-09-06 is pinned because it is installed and was explicitly requested; no unstable language feature is introduced merely to use nightly. No stable MSRV is claimed.

## Architecture

- document: domain records, source-aware Markdown parsing, shared validation, and atomic persistence with explicit errors.
- app: repository discovery, selection, draft transitions, text editing, and input dispatch, independent of a live terminal.
- ui: Ratatui rendering and mouse hit regions, tested with TestBackend.
- main: argument parsing, terminal lifecycle, event polling, and interrupt cleanup.

Premise (registry package premise; library patterns) supplies the core Field/Fielded representation and Keyed lookup. Domain records implement those traits, preserving IDs as decimal text because upstream's u64 field conversion saturates beyond i64::MAX. Restoring fields validates names, types, uniqueness, and record values. Rationale macros attach reasons to important domain choices. Ratatui and operating-system I/O remain explicit adapters. This integrates the library without claiming upstream checker/zone compliance or adopting unrelated money/web3/cloud modules.

## Persistence

Capture exact UTF-8 source bytes, original newline style, marker bounds, records, and source spans. Use pulldown-cmark offsets to exclude inline/fenced examples. Reject duplicate IDs, duplicate/missing canonical labels, invalid titles/statuses, and structural ambiguity. Empty narrative values are valid. Preserve unedited records, surrounding prose, paragraphs, indentation, CRLF, and BOM. A semantic no-op does not write.

Save against the original whole-file baseline. Validate generated source before writing; prepare and fsync a temporary sibling, preserve permission bits, recheck the baseline, and atomically replace. Publish new files with no-clobber semantics. Failures before replacement retain the source and draft. Replacement is the commit point; there is no post-replace error presented as an untouched file. Directory metadata durability after power loss is not guaranteed. Whole-file conflict checks are conservative and cannot exclude an uncooperative external writer racing the last check and rename.

## Interaction

Keep one active draft with original/current values and a source baseline. Successful save returns to the decision list and read-only view; Cancel restores the saved state. Navigation/new/refresh/quit use Save/Discard/Stay when a draft is dirty. Ordinary letters remain text during editing. Ctrl+S saves, Escape cancels, Ctrl+Q requests guarded quit, and Ctrl+C interrupts cleanly. Alt+1–4 accesses panes during editing; normal 1–4 remains available while browsing. Tab traverses fields and buttons. Status selection is keyboard accessible.

First-run n offers root DECISIONS.md; markerless-file initialization is confirmed. File/scaffold creation is deferred until the first successful save, with root containment and no-clobber checks. Refresh reloads disk contents and retains selection by identity where possible. Invalid files show diagnostics without preventing other files from loading. At 80×24, compact layout keeps fields and actions reachable and exposes Preview on demand. Below minimum size, a resize message preserves work. Use terminal-relative colors and NO_COLOR rather than relying on hardcoded backgrounds.

## Testing and Tooling

Unit tests cover parsing, field round trips, source preservation, conflict/write failures, and input state transitions. TestBackend checks rendered geometry and focus at multiple sizes. portable-pty launches the actual Cargo-built or independently installed executable; vt100 decodes output so tests can await actual screen states before sending UTF-8 characters, control keys, paste sequences, and resize events. Tests assert disk records and terminal-mode restoration, impose bounded deadlines, and kill children on failure.

mise pins nightly Rust and developer binaries. rust-toolchain.toml supports direct Cargo and editor users. rustfmt and the requested strict Clippy restrictions apply to production; documented test allowances keep assertions readable. CI on macOS/Linux runs compilation, lint, nextest, doc tests, and an installed-binary journey outside the checkout. Criterion measures parser throughput without imposing an unmeasured threshold. Bacon/watchexec watch checks; they do not launch interactive applications automatically. Scaffolding/discovery tools are developer binaries, not application dependencies.

## Scope and Migration

Replace Python sources/tests, pyproject/uv lock, justfile, Earthfile, and generated package metadata after Rust parity checks pass. Preserve references/, existing user changes, valid Markdown files, and historical OpenSpec archives. Update README, project conventions, and baseline specs with the implementation; archive this change only after delivery. No package registry publication or automatic Git operation is part of runtime behavior.
