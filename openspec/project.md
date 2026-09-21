# Project Context

## Purpose

vrdx is a Rust CLI for engineering decisions stored as standalone Markdown files. The user explicitly replaced the earlier terminal editor on 2026-09-19. Markdown and Git are authoritative; the graph is rebuilt in memory. A bundled read-only loopback dashboard visualizes current files; CLI guide, context and explained suggestions provide a versioned AI interface.

## Stack and Architecture

- Rust 2024, stable 1.97.1 pinned in Cargo.toml, rust-toolchain.toml and mise.toml.
- Dagger 0.21.9 with Dang orchestrates Linux checks; mise manages tools/tasks and nextest runs tests. Native macOS checks remain separate.
- src/main.rs: CLI entry point; src/records/cli.rs: clap commands, dispatch and the versioned JSON envelope; src/records/render.rs: human output.
- src/records/mod.rs: metadata, parsing, no-clobber creation, graph and validation.
- src/records/ai.rs: embedded authoring guide, question-ranked context and deterministic lexical/tag suggestions.
- src/records/init.rs: installs the bundled agent skill from .agents/skills/vrdx and the managed AGENTS.md block.
- src/records/dashboard.rs: tiny_http loopback server; web/ contains dependency-free browser assets.
- serde/serde_json, toml, ulid, minimal jiff, tempfile, clap, tiny_http, toml_edit and shlex are runtime dependencies.
- Full 26-character uppercase ULIDs live in metadata. Filenames use date, UTC time and title, without IDs.
- TOML +++ metadata carries schema version, title, ID, date, status, optional tags and relationship arrays; body Markdown is unrestricted.
- One flat collection per command. Only explicit metadata formatting mutates existing files; no database, cache, remote service, terminal UI or automatic Git commands.

## Quality and Workflow

Use a topic branch and Conventional Commits. OpenSpec tracks changes; the user's explicit design-and-implement request authorizes implementation. Preserve unrelated local changes.

Run mise run ci for containerized Linux verification through Dagger/Dang, or mise run ci-native for formatting, compilation, strict Clippy, nextest, doctests and separately installed CLI checks on the host. Production unsafe code, unwrap/panic/indexing/unchecked arithmetic restrictions remain. Test fixtures may fail immediately on invalid setup or unexpected response shapes. CI covers macOS natively and Linux through the same Dagger pipeline used locally. No Python code, interpreter or package manager is required.

## Data and Limits

Sorted maps and sets give deterministic output. Invalid collections emit findings and cannot provide authoritative context. Creation syncs a temporary file and publishes without clobbering. Manual edits across files are not an atomic snapshot; filesystem race and power-loss directory durability limitations are documented. Old embedded records and code remain in Git history rather than being silently migrated.
