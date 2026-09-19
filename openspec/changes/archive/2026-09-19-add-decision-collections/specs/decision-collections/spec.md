## ADDED Requirements

### Requirement: Portable standalone records
The system SHALL create and read human-readable Markdown decisions with stable time-aware identity, title, date, lifecycle, optional tags, explicit relationships and unrestricted reasoning. Metadata SHALL remain authoritative after a file rename. Creation SHALL NOT overwrite existing files.

#### Scenario: Rename and reread
- **WHEN** a generated decision file is renamed
- **THEN** its ID and relationships SHALL resolve without path-dependent migration

### Requirement: Deterministic collection graph
The system SHALL rebuild the graph entirely from Markdown and return deterministic nodes, edges and diagnostics. Validation SHALL identify malformed records, duplicate identities, missing references, self-references, ambiguous replacements, inconsistent lifecycle states and supersession cycles.

#### Scenario: Invalid replacement chain
- **WHEN** records form a supersession cycle or a superseded record has no replacement
- **THEN** validation SHALL return machine-readable findings and a nonzero exit code

### Requirement: Human and machine workflows
The CLI SHALL provide create, show, list, search, filters, relationships, chain, validate, rebuild and context commands with human-readable and versioned JSON output. Tags SHALL support exact case-insensitive filtering independently of full-text queries and lifecycle.

#### Scenario: Discover and follow decisions
- **WHEN** a caller filters by tags and status, then follows a matching record's replacement chain
- **THEN** results SHALL preserve stable IDs, metadata, source paths and typed relationships

### Requirement: AI context from source evidence
AI context SHALL identify current applicability, historical status, rationale, trade-offs, tags and replacement relationships. Question relevance SHALL use documented deterministic scoring; excerpts and selection limits SHALL be explicit.

#### Scenario: Relevant historical decision
- **WHEN** a question matches a superseded decision
- **THEN** context SHALL also identify its replacement chain and current lifecycle states
- **AND** invalid collections SHALL fail instead of emitting misleading authoritative context
