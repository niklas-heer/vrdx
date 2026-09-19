## MODIFIED Requirements

### Requirement: Source authority and safe creation
The system SHALL preserve Markdown authority and publish new files without clobbering existing paths. Queries and validation SHALL perform no persistent writes. Explicit formatting MAY atomically replace existing files while preserving metadata semantics, permissions, comments and exact Markdown body bytes. Editor authoring SHALL stage a new record and preserve recoverable work on failure.

#### Scenario: Filename collision
- **WHEN** creation targets an existing path
- **THEN** the command SHALL fail and preserve the existing contents

## ADDED Requirements

### Requirement: Measured standalone CLI workflows
The application SHALL remain one native executable with bundled dashboard and guidance assets, requiring no runtime interpreter or external AI service. Verification SHALL cover structured and human workflows through the executable outside the checkout. A reproducible opt-in latency benchmark SHALL report release-command timings and workload size without treating machine-specific measurements as universal guarantees.

#### Scenario: Relocated executable
- **WHEN** only the executable is copied into an otherwise empty working directory
- **THEN** guide, prompt, JSON creation, validation and formatting SHALL work without accessing repository assets or invoking a runtime interpreter
