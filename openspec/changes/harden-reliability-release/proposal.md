# Change: Migrate vrdx to Rust, Ratatui, and Premise with reliable terminal editing

## Why

The existing Python editor has demonstrated save, draft, parser, and installation failures. The user approved implementation and explicitly selected Rust, Ratatui, nightly Rust, mise, Premise as the core pattern library, and end-to-end tests that send characters through a real terminal.

## What Changes

- **BREAKING**: Replace the Python/Textual runtime and uv distribution with a Rust binary. Replace justfile and Earthly/Python tooling with mise and Cargo workflows on macOS/Linux.
- Pin nightly-2026-09-06, Ratatui/Crossterm, and Premise. Use Premise's named record representation, keyed lookup, and attached rationale in the domain core.
- Preserve the existing Markdown decision format while supporting empty narrative fields, rejecting ambiguity, preserving unedited source, and detecting observed external changes before atomic saving.
- Implement reliable draft transitions, first-run initialization, status selection, navigation, refresh, Unicode editing, paste, mouse actions, and compact terminal layouts.
- Exercise the actual binary through a pseudo-terminal with individual UTF-8 characters, control keys, paste, resizing, and VT100 screen assertions, in addition to unit and Ratatui TestBackend tests.
- Configure rustfmt, strict Clippy, Rust Analyzer, nextest, documentation tests, Criterion, Bacon, watchexec, cargo-generate, cargo-seek, and binary installation checks through mise and CI.

## Impact

Affected specifications: design, decision-markers, ui-layout, implementation. The implementation lives in Cargo.toml, src/, Rust tests, mise configuration, and CI. Existing valid Markdown files need no migration. The Python package and development artifacts are retired; references/ and historical OpenSpec archives remain unchanged.

Premise is a core dependency, not a claim that this application implements the upstream repository's entire four-law checker discipline. The project's existing OpenSpec and contributor documentation remain authoritative. No unrelated database, web, blockchain, or cloud crates are introduced.

## Acceptance

Every accepted edit survives reopening; ordinary save failures retain draft and original bytes; observed concurrent changes refuse the save; untouched source remains byte-identical. The empty-repository create/save/reopen flow works at 80×24. Real terminal tests send characters to the executable, assert on decoded screen and saved files, and verify terminal cleanup. Formatting, all-target compilation, Clippy with warnings denied, nextest, documentation tests, and installed-binary tests pass on the pinned toolchain.

## Approval

Approved for implementation by the user's Rust/Ratatui/mise request, subsequently refined to pinned nightly Rust and Premise. These explicit instructions supersede the previous Python-only distribution constraint. No additional implementation approval is required.

## Follow-up Completion

The approved complete-decision-workflows change delivers search/JSON commands, durable decision relationships, templates, Git-history navigation, three-way conflict merging, and the original delete/reorder requirements. No claim of portable compare-and-swap is made for independent external editors.
