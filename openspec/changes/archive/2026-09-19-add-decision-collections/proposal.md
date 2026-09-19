## Why

The user explicitly rejected the existing editor and requested a focused CLI for portable Markdown decision files, with compact filenames and metadata-only ULIDs.

## What Changes

- **BREAKING** Replace the TUI, embedded numeric-record parser and `vrdx agent` commands with top-level CLI workflows.
- Store standalone Markdown with TOML metadata, full ULIDs, tags, lifecycle and typed relationships.
- Use timestamp/title filenames without ULIDs; preserve identity across renames.
- Rebuild a deterministic in-memory graph and provide versioned JSON and concise AI evidence.
- Remove terminal/Premise/benchmark code, dependencies, tests and obsolete development tasks; add collection subprocess coverage and installed CLI verification.

## Impact

- Affected specs: decision-collections (new), implementation; retire design, ui-layout and decision-markers.
- Affected code: Rust library/binary, Cargo dependencies, tests, README, mise tasks and install check.
- No automatic migration, existing-file mutation, database or web app. The user identified a web interface as a possible later direction, not a present requirement.
- Implementation and breaking scope authorized by the user's 2026-09-19 instructions; existing uncommitted AGENTS.md changes are excluded.
