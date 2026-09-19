# Project Context

## Purpose

vrdx is a Rust CLI for engineering decisions stored as standalone Markdown files. The user explicitly replaced the earlier terminal editor on 2026-09-19. Markdown and Git are authoritative; the graph is rebuilt in memory. A future web interface may consume the same library or JSON contract, but is not implemented.

## Stack and Architecture

- Rust 2024, stable 1.97.1 pinned in Cargo.toml, rust-toolchain.toml and mise.toml.
- Dagger 0.21.9 with Dang orchestrates Linux checks; mise manages tools/tasks and nextest runs tests. Native macOS checks remain separate.
- src/main.rs: CLI entry point; src/records/cli.rs: clap workflows and human/versioned JSON transport.
- src/records/mod.rs: metadata, parsing, no-clobber creation, graph and validation.
- serde/serde_json, toml, ulid, minimal jiff, tempfile and clap are runtime dependencies.
- Full 26-character uppercase ULIDs live in metadata. Filenames use date, UTC time and title, without IDs.
- TOML +++ metadata carries schema version, title, ID, date, status, optional tags and relationship arrays; body Markdown is unrestricted.
- One flat collection per command. No existing-file mutation, database, cache, web service, terminal UI or automatic Git commands.

## Quality and Workflow

Use a topic branch and Conventional Commits. OpenSpec tracks changes; the user's explicit design-and-implement request authorizes implementation. Preserve unrelated local changes. references/ remains read-only historical context.

Run mise run ci for containerized Linux verification through Dagger/Dang, or mise run ci-native for formatting, compilation, strict Clippy, nextest, doctests and separately installed CLI checks on the host. Production unsafe code, unwrap/panic/indexing/unchecked arithmetic restrictions remain. Test fixtures may fail immediately on invalid setup or unexpected response shapes. CI covers macOS natively and Linux through the same Dagger pipeline used locally. No Python code, interpreter or package manager is required.

## Data and Limits

Sorted maps and sets give deterministic output. Invalid collections emit findings and cannot provide authoritative context. Creation syncs a temporary file and publishes without clobbering. Manual edits across files are not an atomic snapshot; filesystem race and power-loss directory durability limitations are documented. Old embedded records and code remain in Git history rather than being silently migrated.
