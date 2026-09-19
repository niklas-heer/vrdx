# decision-collections Specification

## Purpose

Portable Markdown decisions, lifecycle graphs and human/AI discovery.
## Requirements
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

### Requirement: Local visual decision dashboard
The CLI SHALL offer a read-only loopback web dashboard generated from the current Markdown collection, with search, lifecycle/tag filtering, relationship graph navigation and full record details. Browser content SHALL treat record data as untrusted text and expose collection findings clearly.

#### Scenario: Refresh after a Markdown change
- **WHEN** a user edits a decision in the selected directory and refreshes the dashboard
- **THEN** the dashboard SHALL show the updated graph without a database or persistent index

#### Scenario: Invalid collection
- **WHEN** a collection contains an invalid record or relationship
- **THEN** the dashboard SHALL display diagnostics and mark the graph invalid rather than implying current policy is authoritative

### Requirement: Self-describing AI workflow
The CLI SHALL provide human and versioned JSON guidance without requiring a collection, including commands, metadata, examples, lifecycle semantics and decision writing style.

#### Scenario: Agent onboarding
- **WHEN** an AI calls guide before any decisions directory exists
- **THEN** it SHALL receive sufficient instructions to discover, propose, write and validate a decision without inventing acceptance or evidence

### Requirement: Explained relationship suggestions
The CLI SHALL suggest potentially related decisions deterministically with observable reasons, lifecycle and source identity. Suggestions SHALL exclude the input record and existing direct relationships, and SHALL NOT mutate Markdown or create graph edges.

#### Scenario: Topical overlap
- **WHEN** two unlinked records share meaningful tags or words
- **THEN** suggest SHALL expose the overlap and score as an advisory candidate rather than a confirmed relationship

### Requirement: Graph-first exploration
The dashboard SHALL open in graph view and retain a records view. Pointer hover and keyboard focus SHALL distinguish the highlighted decision, its direct neighbors and connecting edges. Selecting a decision SHALL show its immediate neighborhood with named relationship directions and access to full reasoning. Context outside current filters SHALL be labeled, and clearing selection SHALL restore the filtered overview. Isolated decisions SHALL have an explicit empty connection state.

#### Scenario: Explore a filtered decision
- **WHEN** a user selects a decision whose direct neighbor does not match the current filters
- **THEN** the neighborhood SHALL include that neighbor as context and label it as outside the filters
- **AND** the user SHALL be able to navigate the relationship, read full reasoning and return to the filtered overview

### Requirement: Dashboard appearance
The dashboard SHALL support light, dark and system appearance, use system preference initially, and remember explicit choices when browser storage is available. Controls, records, graph connections, validation findings and full record details SHALL remain readable and keyboard accessible in both themes.

#### Scenario: Remember dark appearance
- **WHEN** a user chooses dark appearance and reloads the page with browser storage available
- **THEN** the dashboard SHALL retain dark appearance
- **AND** choosing system appearance SHALL follow subsequent operating-system appearance changes
