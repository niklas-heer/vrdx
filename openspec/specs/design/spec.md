# design Specification

## Purpose
Current requirements for design, including the approved Rust migration.
## Requirements
### Requirement: CLI TUI Decision Management
The system SHALL provide a standalone terminal application that discovers Markdown decision records, parses structured decision content, and enables users to browse and edit those decisions without relying on external services.

#### Scenario: Launching the standalone TUI
- **GIVEN** a repository that contains Markdown files
- **WHEN** the user starts the application from the command line
- **THEN** the application SHALL scan the working directory for decision records and present them in the TUI

#### Scenario: Markdown-focused workflow
- **GIVEN** a Markdown file that contains decision entries between vrdx markers
- **WHEN** the user navigates to that file within the TUI
- **THEN** the application SHALL expose the decisions for viewing and editing inside the terminal

### Requirement: Layered Architecture Boundaries
The system SHALL separate Rust CLI/terminal startup, Ratatui rendering, repository discovery, decision parsing, draft state, and persistence. Premise SHALL provide reusable named-record and keyed-lookup primitives in the domain core.

#### Scenario: CLI startup initializes app state
- **WHEN** the Rust entrypoint resolves a target directory
- **THEN** it SHALL construct application state and launch the Ratatui interface

#### Scenario: Parser and persistence isolation
- **WHEN** source parsing or writing is required
- **THEN** dedicated document operations SHALL perform it independently of UI widgets

### Requirement: Pane-Oriented TUI Layout
The Ratatui interface SHALL provide Decisions, Files, Editor, and Preview panes within an 80×24 friendly layout. Normal layout SHALL place Decisions and Files in the left column and Editor above Preview in the right column. Compact layout MAY show Editor and Preview separately to keep editing usable, while numeric shortcuts retain access to every pane.

#### Scenario: Pane layout at launch
- **GIVEN** the TUI has started at a size sufficient for normal layout
- **WHEN** the interface renders
- **THEN** Decisions and Files SHALL appear in the left column and Editor above Preview in the right column

#### Scenario: Numeric pane focus
- **WHEN** the user invokes a pane shortcut outside text input
- **THEN** `1` SHALL expose and focus Decisions, `2` Files, `3` Editor, and `4` Preview
- **AND** compact layout SHALL reveal the requested pane without discarding the active draft

### Requirement: Interaction Model Parity
The system SHALL support lazygit-inspired keyboard interactions including `j`/`k` navigation, `space` to edit the selected decision, `n` to start a new decision, `r` to refresh discovery, and `?` for help outside text inputs. Editing SHALL use Ctrl+S to save and Escape to cancel. Plain letters typed into form fields SHALL remain text.

#### Scenario: Navigating with j and k
- **GIVEN** the decisions list pane is focused
- **WHEN** the user presses `j` or `k`
- **THEN** the selection SHALL move to the next or previous decision subject to the unsaved-draft guard

#### Scenario: Editing and saving a decision
- **GIVEN** a decision is selected
- **WHEN** the user presses `space`, edits it, and presses Ctrl+S
- **THEN** the editor SHALL accept changes and save through the persistence layer using the same validation and outcome handling as the Save button

#### Scenario: Ordinary text entry
- **GIVEN** an editable field has focus
- **WHEN** the user types letters used by application shortcuts
- **THEN** those characters SHALL be inserted without triggering navigation, status changes, saving, or quitting

### Requirement: Marker Block Canonicalization
The system SHALL treat `<!-- vrdx start -->` and `<!-- vrdx end -->` as the canonical decision block delimiters, validating their ordering and ignoring examples in Markdown inline code and fenced code blocks. Adding a marker block to a previously uninitialized file SHALL require user confirmation and SHALL be committed together with the first successfully saved decision.

#### Scenario: Detecting canonical markers
- **GIVEN** a Markdown file with a properly ordered marker block outside code examples
- **WHEN** the application scans the file
- **THEN** it SHALL recognize a single decision block bounded by the canonical delimiters

#### Scenario: Inserting scaffold when markers absent
- **GIVEN** a Markdown file without markers
- **WHEN** the user confirms initialization and successfully saves the first decision
- **THEN** the system SHALL append the canonical marker block containing that decision using the file's newline convention
- **AND** cancellation or failure before committing the first save SHALL leave the original file unchanged

#### Scenario: Ignoring inline examples
- **GIVEN** documentation that references markers inside Markdown inline-code spans or fenced code
- **WHEN** the parser processes the file
- **THEN** those examples SHALL NOT create marker blocks or disrupt discovery

### Requirement: Decision Template Structure
New decisions SHALL use the template heading `### <ID> <Title>` followed by bullet-prefixed Status, Decision, Context, and Consequences fields, with default status set to `📝 Draft`. The same validation contract SHALL apply to form submission and persisted source: title and status are nonempty, IDs are nonnegative and unique within a file, and each canonical field label occurs exactly once. Decision, Context, and Consequences values MAY be empty for any status.

#### Scenario: Creating a new decision
- **GIVEN** the user presses `n`
- **WHEN** the editor opens with a template
- **THEN** it SHALL pre-populate the next ID, default status, and placeholders for all required fields
- **AND** placeholder text SHALL NOT be persisted as entered content

#### Scenario: Maintaining descending ID order
- **GIVEN** multiple decisions exist in a block
- **WHEN** a new decision is saved
- **THEN** the persistence layer SHALL insert it at the top of the block so the highest ID appears first

#### Scenario: Title-only record reopens
- **GIVEN** a decision has a valid title and status with empty narrative fields
- **WHEN** it is saved and loaded in a new application session
- **THEN** the record SHALL remain discoverable with identical field values

#### Scenario: Ambiguous record source
- **GIVEN** a document contains duplicate decision IDs, duplicate canonical labels, or missing canonical labels
- **WHEN** the document is loaded or a proposed edit is validated
- **THEN** the system SHALL report the affected file and line and SHALL prevent rewriting the ambiguous document
- **AND** it SHALL NOT silently choose a duplicate or discard source

#### Scenario: Paragraphs and code examples survive editing
- **GIVEN** a narrative field contains paragraph breaks, indentation, or fenced code
- **WHEN** a valid edit is saved and reopened
- **THEN** the field's meaningful whitespace and contents SHALL survive
- **AND** apparent decision headings or labels inside fenced code SHALL NOT be interpreted as record structure

#### Scenario: Submission would change structure
- **GIVEN** entered text would be reinterpreted as a record heading, marker, or field boundary after rendering
- **WHEN** generated source fails validation or fails to reproduce the submitted field values
- **THEN** the system SHALL refuse the save with a validation error and retain the draft

### Requirement: Persistence Integrity
The system SHALL preserve non-marker content, newline styles, and unedited decision source when rewriting marker blocks. Saves SHALL validate generated records, detect observed changes to the loaded file baseline, and replace existing files atomically. A failed save before replacement MUST preserve both the original file and the active draft.

#### Scenario: Saving with unchanged surroundings
- **GIVEN** a user edits one decision in an existing document
- **WHEN** the changes are saved
- **THEN** content outside the edited decision span SHALL remain byte-for-byte unchanged, including other decisions, surrounding prose, original line endings, and any UTF-8 BOM
- **AND** newly rendered structural lines SHALL use the document's existing newline convention

#### Scenario: No-op save
- **GIVEN** the draft values equal the saved baseline
- **WHEN** the user saves and the file baseline is still current
- **THEN** the system SHALL perform no file write and SHALL report a successful unchanged save

#### Scenario: External edit detected before save
- **GIVEN** a file changed after loading, including a change outside its marker block
- **WHEN** either baseline comparison in the save transaction observes that change
- **THEN** the save SHALL be refused as a conflict without overwriting the current file
- **AND** the draft SHALL remain available with an actionable conflict message

#### Scenario: Failure before atomic replacement
- **GIVEN** a validated edit to an existing document
- **WHEN** temporary writing, flushing, synchronization, or replacement fails before the destination is replaced
- **THEN** the existing file SHALL remain unchanged and the draft SHALL remain unsaved
- **AND** unused temporary files SHALL be cleaned up

#### Scenario: Successful atomic replacement
- **WHEN** a validated edit passes baseline checks and replacement succeeds
- **THEN** the complete document SHALL be published through a temporary sibling and atomic replacement, preserving the original permission bits
- **AND** the in-memory saved records and baseline SHALL be updated only after replacement succeeds

#### Scenario: Invalid or missing destination
- **GIVEN** an existing draft's file disappears, becomes unreadable, or no longer has a valid marker block
- **WHEN** Save is requested
- **THEN** the system SHALL retain the draft and report the relevant error without recreating or silently repairing the file

### Requirement: Future Iteration Roadmap Reference
The specification SHALL document outstanding design milestones (status picker enhancements, Markdown preview rendering, in-app reordering polish, logging improvements, tooling updates, and CI workflows) so future proposals can reference and refine them.

#### Scenario: Referencing planned enhancements
- **GIVEN** a future proposal evaluates improvements to the TUI or tooling
- **WHEN** it reviews the archived design foundations
- **THEN** it SHALL find the enumerated milestones to guide scope decisions

#### Scenario: Maintaining authoritative baseline
- **GIVEN** maintainers need to confirm historical design intent
- **WHEN** they consult the archived spec
- **THEN** it SHALL reflect the original goals and architecture described in the legacy design document

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
