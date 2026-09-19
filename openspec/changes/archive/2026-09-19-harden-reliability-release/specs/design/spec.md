## MODIFIED Requirements

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

### Requirement: Layered Architecture Boundaries
The system SHALL separate Rust CLI/terminal startup, Ratatui rendering, repository discovery, decision parsing, draft state, and persistence. Premise SHALL provide reusable named-record and keyed-lookup primitives in the domain core.

#### Scenario: CLI startup initializes app state
- **WHEN** the Rust entrypoint resolves a target directory
- **THEN** it SHALL construct application state and launch the Ratatui interface

#### Scenario: Parser and persistence isolation
- **WHEN** source parsing or writing is required
- **THEN** dedicated document operations SHALL perform it independently of UI widgets
