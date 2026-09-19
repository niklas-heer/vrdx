## ADDED Requirements

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
