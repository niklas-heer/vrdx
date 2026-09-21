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

### Requirement: Read reasoning alongside connections
Selecting a graph decision SHALL immediately display its full Markdown body, title, lifecycle, date and source path in a reading pane without opening a modal. The graph SHALL remain available for navigation. Selecting another decision SHALL update the reader and its neighborhood together. The reader SHALL reuse safe Markdown rendering, expose invalid-collection warnings and adapt to narrow screens without hiding content behind an additional action.

#### Scenario: Follow a connection while reading
- **WHEN** a user selects a graph decision and then one of its neighbors
- **THEN** the visible full reasoning and graph neighborhood SHALL both belong to the newly selected decision
- **AND** the user SHALL be able to read the content and continue navigating without opening or dismissing a dialog

### Requirement: Concise human and agent authoring
The CLI SHALL generate concise proposed records with a short title, one decision, its reasons and consequences. JSON creation SHALL accept a documented strict input object from a file or stdin, generate identity/date defaults, reject invalid input without writing a record and return the created identity and path. An explicit editor option SHALL use VISUAL or EDITOR, stage the draft before publication and retain a recoverable draft on failure. A prompt command SHALL return copyable authoring instructions without a collection, network access or file creation.

#### Scenario: Agent creates a decision
- **WHEN** an agent submits valid JSON using the guide's input contract
- **THEN** creation SHALL publish a formatted proposed record and return versioned JSON with its identity and path
- **AND** unknown fields or missing required content SHALL produce actionable errors without a published record

#### Scenario: Editor fails
- **WHEN** the editor exits unsuccessfully or saves invalid metadata
- **THEN** the CLI SHALL retain the staged draft, report its recovery path and leave the collection unchanged

### Requirement: Conservative explicit formatting
The CLI SHALL offer fmt and fmt --check for deterministic metadata ordering and spacing. Formatting SHALL preserve comments, record identity, lifecycle, relationships and Markdown body bytes. Check-only mode SHALL perform no writes and report files needing formatting with a nonzero exit status. A second formatting pass SHALL make no changes. Invalid collections SHALL be rejected before writes; replacement SHALL be atomic per file and preserve permissions. No multi-file transaction is promised.

#### Scenario: Format and check
- **WHEN** a valid record with irregular metadata spacing is formatted
- **THEN** its Markdown body and metadata values SHALL remain unchanged and a subsequent check SHALL pass

### Requirement: Actionable diagnostics
Validation SHALL report deterministic findings with the affected file, stable code, explanation and repair hint in both human and JSON output. Errors SHALL give enough context to choose the next action without exposing source bodies as instructions. Validation SHALL not infer decision acceptance, require verbose prose or mutate records.

#### Scenario: Broken relationship
- **WHEN** validation finds an unresolved target or inconsistent replacement lifecycle
- **THEN** output SHALL identify the affected records and suggest the relevant reference or status correction

### Requirement: Agent skill installation
The CLI SHALL provide an `init` command that installs a bundled Agent Skills
`SKILL.md` for consulting and recording decisions at `.agents/skills/vrdx/`,
links it for Claude Code at `.claude/skills/vrdx`, and maintains one marked block
in `AGENTS.md` that names the collection directory. The command SHALL be
idempotent, report a per-path status, support a dry run that writes nothing,
preserve all content outside the managed block, and abort without writing when a
managed path conflicts with an unrelated file. It SHALL report recognised existing ADR directories and a collection directory whose Markdown is not vrdx records as onboarding hints without modifying them, and the installed skill SHALL describe how to import such sources as a baseline without inventing dates, statuses or reasons.

#### Scenario: Fresh project
- **WHEN** `init` runs in a directory without agent configuration
- **THEN** the skill file, the symlink and `AGENTS.md` with the managed block SHALL exist and be reported as created
- **AND** a second run SHALL report every path unchanged

#### Scenario: Existing AGENTS.md
- **WHEN** `init` runs where `AGENTS.md` already has content or an older managed block
- **THEN** only the text between the markers SHALL change and every other byte SHALL be preserved

#### Scenario: Skill triggers at the right time
- **WHEN** an agent with the installed skill faces a consequential technical choice
- **THEN** the skill SHALL direct it to consult existing records first and to record the agreed choice with `new --from-json`, defaulting to proposed
- **AND** the skill SHALL exclude routine edits, bug fixes and progress notes

#### Scenario: Existing ADR folder
- **WHEN** `init` runs in a project with `docs/adr` or Markdown in the collection directory that is not a vrdx record
- **THEN** the output SHALL name those sources as onboarding candidates and SHALL leave them unchanged

