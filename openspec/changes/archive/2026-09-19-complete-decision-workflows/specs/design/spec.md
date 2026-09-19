## ADDED Requirements
### Requirement: Complete Decision Operations
The system SHALL expose decision deletion with confirmation, order changes preserving identifiers, reusable templates, persistent file-and-ID relationships, search, Git history inspection, and conservative three-way conflict merging through the appropriate TUI and command-line workflows.

#### Scenario: Delete and reorder preserve surrounding data
- **WHEN** a decision is deleted or moved
- **THEN** unaffected records and surrounding source SHALL remain intact and concurrent changes SHALL not be silently overwritten
- **AND** a persisted high-water identifier SHALL prevent deleted decision IDs from being reused

#### Scenario: Independent edits merge safely
- **WHEN** local and external edits affect different fields or records
- **THEN** an explicit merge action SHALL preserve both changes
- **AND** divergent changes to the same field SHALL fail while preserving the local draft and disk bytes

#### Scenario: Agent interface works without a terminal
- **WHEN** a caller lists, searches, reads, creates, updates, deletes, reorders, or validates decisions using headless commands
- **THEN** results and failures SHALL have deterministic machine-readable JSON representations and meaningful exit codes

#### Scenario: Templates relationships and history
- **WHEN** users select a repository template, follow a persisted decision relationship, or inspect Git history
- **THEN** the requested content SHALL be available without relying on external services or executing repository configuration

### Requirement: Premise Record Contracts
The domain core and agent transport SHALL share Premise's named-field representation and keyed identity. Executable tests SHALL verify representation round trips, field order independence, invalid-shape rejection, bulk transport, Unicode, and integer boundaries. Rust documentation tests and the standard Rust crate architecture SHALL remain in force; the separate upstream four-law framework is not this project's conformance standard.

#### Scenario: Machine transport preserves identity
- **WHEN** a record crosses the JSON and Premise boundaries
- **THEN** every field and the complete unsigned identifier SHALL retain its value without numeric coercion

### Requirement: Optimistic Agent Mutations
Headless mutations SHALL require a SHA-256 snapshot token for an existing file or the explicit missing token for new-file creation. A success response SHALL describe the committed snapshot without requiring another filesystem read. JSON requests SHALL reject unknown, duplicated, or incorrectly typed record fields.

#### Scenario: A file changes after it was read
- **WHEN** a mutation token or merge snapshot no longer matches disk
- **THEN** the command SHALL fail with a conflict status and preserve disk contents
