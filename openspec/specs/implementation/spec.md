# implementation Specification

## Purpose

Focused Rust CLI architecture, safe source handling and reproducible verification.

## Requirements

### Requirement: Portable CLI architecture
The application SHALL provide a Rust binary and library for standalone Markdown collection parsing, validation, creation, deterministic graph rebuilding and human/JSON CLI workflows. It SHALL NOT require a terminal, database, persistent index or hosted service.

#### Scenario: Headless use
- **WHEN** the binary runs without a terminal
- **THEN** all collection commands SHALL work with the selected directory and documented exit codes

### Requirement: Reproducible CLI verification
The project SHALL retain its pinned Rust toolchain, Cargo.lock and mise setup, enforcing formatting, compilation, strict Clippy, nextest, doctests and installed CLI tests on macOS and Linux.

#### Scenario: Installed artifact
- **WHEN** CI installs the binary outside the checkout
- **THEN** subprocess journeys SHALL verify persisted Markdown, queries, graph findings and structured output without source-tree runtime dependencies

### Requirement: Source authority and safe creation
The system SHALL preserve Markdown authority, publish new files without clobbering existing paths, and leave existing files unchanged. Validation and read commands SHALL perform no persistent writes.

#### Scenario: Filename collision
- **WHEN** creation targets an existing path
- **THEN** the command SHALL fail and preserve the existing contents
