## MODIFIED Requirements

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
