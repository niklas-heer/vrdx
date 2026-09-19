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
The project SHALL pin stable Rust consistently in Cargo.toml, rust-toolchain.toml and mise.toml, with Cargo.lock and mise.lock checked in. Dagger with the Dang SDK SHALL orchestrate containerized Linux checks, reusing mise tasks for formatting, compilation, strict Clippy, nextest, doctests and installed CLI tests. Native macOS verification SHALL remain separate. The application and build SHALL require no Python interpreter or package configuration.

#### Scenario: Installed artifact
- **WHEN** CI installs the binary outside the checkout
- **THEN** subprocess journeys SHALL verify persisted Markdown, queries, graph findings and structured output without source-tree runtime dependencies

#### Scenario: Reproducible local and hosted CI
- **WHEN** a contributor runs mise run ci or the Linux CI job runs
- **THEN** the same Dang module SHALL execute the native quality tasks in a pinned container with filtered source inputs
- **AND** failing checks SHALL propagate a nonzero result

#### Scenario: Native platform coverage
- **WHEN** the macOS job runs mise run ci-native
- **THEN** checks SHALL execute on macOS rather than substituting Linux-container coverage

### Requirement: Source authority and safe creation
The system SHALL preserve Markdown authority and publish new files without clobbering existing paths. Queries and validation SHALL perform no persistent writes. Explicit formatting MAY atomically replace existing files while preserving metadata semantics, permissions, comments and exact Markdown body bytes. Editor authoring SHALL stage a new record and preserve recoverable work on failure.

#### Scenario: Filename collision
- **WHEN** creation targets an existing path
- **THEN** the command SHALL fail and preserve the existing contents

### Requirement: Reproducible usage simulation
The project SHALL exercise realistic mixed-topic decision collections and repeatable lifecycle, rename, invalid-edit and recovery sequences through the public executable. Assertions SHALL cover source identity, current applicability, replacement history, evidence retrieval and deterministic results. Failures SHALL identify the scenario or reproducible seed. Reported results SHALL distinguish modeled coverage from real adoption.

#### Scenario: Recover from a broken replacement edit
- **WHEN** an edited collection temporarily contains an inconsistent replacement and the edit is then completed or reverted
- **THEN** validation and evidence queries SHALL reject the invalid state and recover using the same stable record identities

### Requirement: Verified native distribution
The project SHALL distribute versioned Linux and macOS native executable archives with the project license, usage documentation and SHA-256 checksums. Release automation SHALL build from the selected source revision, verify the version and exercise each native executable outside the source checkout before publishing. The documented platform and architecture SHALL match the artifact.

#### Scenario: Install without a Rust toolchain
- **WHEN** a user verifies and extracts a supported release archive
- **THEN** the executable SHALL provide guide, creation, validation and retrieval workflows without a source checkout, database, Python or Rust toolchain

### Requirement: Measured standalone CLI workflows
The application SHALL remain one native executable with bundled dashboard and guidance assets, requiring no runtime interpreter or external AI service. Verification SHALL cover structured and human workflows through the executable outside the checkout. A reproducible opt-in latency benchmark SHALL report release-command timings and workload size without treating machine-specific measurements as universal guarantees.

#### Scenario: Relocated executable
- **WHEN** only the executable is copied into an otherwise empty working directory
- **THEN** guide, prompt, JSON creation, validation and formatting SHALL work without accessing repository assets or invoking a runtime interpreter

### Requirement: Tested Homebrew distribution
The release SHALL be installable through the existing niklas-heer/tap/vrdx formula using checksummed native archives. Automated formula updates SHALL select a published stable release, require all supported archives and valid checksums, and pass native installation tests before updating the tap. Tag publication SHALL require the full quality workflow and all native archive checks.

#### Scenario: Homebrew installation
- **WHEN** a user installs niklas-heer/tap/vrdx on a supported platform
- **THEN** Homebrew SHALL install the matching verified executable without a Rust toolchain

#### Scenario: Failed release or formula checks
- **WHEN** a quality, packaging or native formula test fails
- **THEN** the corresponding publication SHALL not proceed
