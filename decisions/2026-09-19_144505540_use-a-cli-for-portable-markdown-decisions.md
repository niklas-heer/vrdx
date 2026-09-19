+++
schema_version = 1
id = "01M2X21HW4QXRHR3EPWP4TV7YS"
title = "Use a CLI for portable Markdown decisions"
date = "2026-09-19"
status = "accepted"
tags = ["architecture", "tooling"]
supersedes = []
superseded_by = []
depends_on = []
related_to = []
+++

## Decision

Replace the terminal editor and embedded numeric-record interface with a CLI for standalone Markdown decisions. Keep full 26-character ULIDs in metadata and use a short timestamp/title filename. Markdown and Git are authoritative; rebuild the graph in memory for every command.

## Context

On 2026-09-19 Niklas explicitly asked to discard the editor, preferred a CLI, and accepted ULIDs provided they stay out of filenames. A web interface was suggested as a possible future consumer. This supersedes the interface direction of the earlier Rust/Ratatui implementation; the Rust and mise tooling choices remain.

The implementation uses TOML metadata, five explicit lifecycle states, adaptable tags and typed relationships. Supersession is declared on either endpoint and normalized into one graph edge. Deterministic validation and versioned JSON serve people, scripts and AI tools without an external service.

## Alternatives

Keeping the old TUI and adding a second interface was rejected by Niklas. UUIDv7 is suitable but would require either longer text or a custom compact encoding; standard ULIDs provide the requested time-aware identity directly. No database or persistent index is needed for the present collection size. Tags do not replace lifecycle states or typed relationships.

## Consequences

Files stay portable and references survive renaming. A CLI and library offer a clear base for a future web app. The old TUI and agent commands are intentionally removed, which is a breaking interface change. Existing embedded logs are not silently converted; their history remains in Git.

People edit status, links and reasoning in Markdown. Validation catches malformed metadata and graph inconsistencies, but cannot judge the quality of reasoning or prove historical transitions. AI context uses lexical ranking and explicit excerpts; callers should use show for complete source evidence. No semantic search, service infrastructure or multi-file edit transaction is introduced.

## Verification

The local CI workflow covers formatting, compilation, strict Clippy, CLI subprocess tests, doctests and an installed binary outside the source checkout. CLI fixtures exercise renames, malformed data, collisions, Unicode, deterministic graph/context output and a 128-record supersession history. See ../README.md for the interface and deliberate limitations.
