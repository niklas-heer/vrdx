## MODIFIED Requirements

### Requirement: Form-Based Decision Editor Interface
The system SHALL provide a structured form-based editor for creating and editing decisions with distinct Title, Status, Decision, Context, and Consequences fields. The Title SHALL be a single-line input and the Status SHALL be a dropdown containing the supported status options.

#### Scenario: Form displays all required fields
- **GIVEN** a user initiates new decision creation or selects an existing decision for editing
- **WHEN** the editor becomes active
- **THEN** the form MUST display a single-line Title input, a Status dropdown, and editable Decision, Context, and Consequences areas
- **AND** the decision ID MUST be displayed prominently
- **AND** Save and Cancel buttons MUST be available at the bottom of the form

#### Scenario: Form pre-populates data for existing decisions
- **GIVEN** a user selects an existing decision to edit
- **WHEN** the form opens
- **THEN** all five fields MUST contain the existing decision's values
- **AND** loading those values MUST establish an unchanged draft baseline

#### Scenario: Form receives status from selection modal
- **GIVEN** the user confirms a status in the new-decision status selection modal
- **WHEN** the form opens
- **THEN** the Status dropdown MUST display the selected status
- **AND** focus MUST be in the Title input

### Requirement: Field Validation and Feedback
The system SHALL validate form fields before persistence, provide actionable validation feedback, and derive the unsaved-changes indicator from the active draft's current values and baseline. Validation failure MUST preserve the draft.

#### Scenario: Title field validation prevents empty submission
- **GIVEN** the title is empty or contains only whitespace
- **WHEN** the user activates Save
- **THEN** the system MUST NOT persist the decision
- **AND** an error explaining that the title is required MUST be displayed
- **AND** focus MUST return to the Title input

#### Scenario: Title placeholder text is not accepted as valid
- **GIVEN** the title equals a placeholder supplied by the application
- **WHEN** the user activates Save
- **THEN** the system MUST reject that placeholder and request a real title
- **AND** a legitimate title MUST NOT be rejected merely because it contains the words "decision title"

#### Scenario: Invalid field values preserve the draft
- **GIVEN** a draft contains a multiline title or a status that cannot be persisted in the supported decision format
- **WHEN** the user activates Save
- **THEN** the system MUST explain the invalid field before writing
- **AND** all draft field values MUST remain available for correction

#### Scenario: Unsaved changes indicator
- **GIVEN** a user changes a text field or the selected status
- **WHEN** the draft differs from its baseline
- **THEN** a visible indicator MUST show unsaved changes immediately
- **AND** an attempt to leave the draft MUST follow the draft navigation guard

#### Scenario: Reverting changes restores the unchanged indicator
- **GIVEN** an existing decision has unsaved changes
- **WHEN** the user restores every field to its baseline value
- **THEN** the unsaved-changes indicator MUST clear
- **AND** loading or previewing an unchanged decision MUST NOT mark it modified

### Requirement: Explicit Save and Cancel Actions
The system SHALL provide Save and Cancel buttons and keyboard equivalents with the same behavior. Successful persistence or explicit cancellation SHALL return focus to the decision list and leave any visible editor fields read-only until editing is explicitly entered again.

#### Scenario: Save button creates or updates decision
- **GIVEN** a user completes a valid draft
- **WHEN** the user activates Save by mouse, button keyboard activation, or Ctrl+S
- **THEN** the system MUST validate and persist the new or updated decision without an interaction error
- **AND** only after persistence succeeds MUST it clear the dirty state and close the edit session
- **AND** a success message MUST remain visible after the view refreshes
- **AND** focus MUST return to the decision list with the saved decision selected
- **AND** the saved decision MUST appear in the list and preview

#### Scenario: Save failure retains an editable draft
- **GIVEN** a draft cannot be persisted because of a missing file, invalid markers, a write failure, or a detected external change
- **WHEN** the user activates Save
- **THEN** the system MUST retain the draft values and editing identity
- **AND** it MUST display the failure without replacing it with success or generic navigation hints
- **AND** it MUST allow correction or retry without requiring the user to re-enter the draft
- **AND** it MUST NOT clear the dirty state or publish the draft as a successfully saved record

#### Scenario: Cancel button aborts editing
- **GIVEN** a user is editing a decision
- **WHEN** the user activates Cancel by mouse, button keyboard activation, or Escape while no child popup is open
- **THEN** the system MUST discard the draft without writing to disk
- **AND** it MUST restore the saved decision's displayed values and preview, or the empty view for an unsaved new decision
- **AND** the edit session MUST close and focus MUST return to the decision list

#### Scenario: Keyboard shortcuts for save and cancel
- **GIVEN** a form field has focus and no child popup is open
- **WHEN** the user presses Ctrl+S or Escape
- **THEN** the corresponding Save or Cancel action MUST behave identically to its button
- **AND** ordinary character keys MUST remain available for text input

#### Scenario: Returning to view mode cannot accept unsavable edits
- **GIVEN** a save or cancellation has closed the edit session
- **WHEN** the editor remains visible
- **THEN** its fields MUST be read-only
- **AND** selecting a decision for editing MUST establish a new editable session with the correct record identity and baseline

### Requirement: Status Change Integration
The system SHALL allow status changes through the form's Status dropdown while preserving all other draft values. The dropdown, draft status, and preview SHALL remain synchronized.

#### Scenario: Dropdown opens with the current status
- **GIVEN** a user is editing a new or existing decision
- **WHEN** the user opens the Status dropdown by keyboard or mouse
- **THEN** the current status MUST be selected
- **AND** the Title, Decision, Context, and Consequences fields MUST remain unchanged

#### Scenario: Updated status reflects in the form and preview
- **GIVEN** the Status dropdown is open
- **WHEN** the user confirms a different supported status
- **THEN** the dropdown MUST close and update the draft and preview to that status
- **AND** the form MUST remain ready for further editing or saving
- **AND** the unsaved-changes indicator MUST reflect the updated draft

#### Scenario: Dismissing status selection preserves the draft
- **GIVEN** the Status dropdown is open
- **WHEN** the user presses Escape
- **THEN** the dropdown MUST close without changing the previously selected status
- **AND** the edit session MUST remain open with its other draft values unchanged

### Requirement: Accessible Status Cycling During New Decision Editing
The system SHALL provide keyboard access to the Status dropdown during new-decision editing and SHALL display only shortcuts implemented in the current context. Status selection SHALL use the dropdown's keyboard controls without reserving ordinary text input characters for status cycling.

#### Scenario: Keyboard status selection during new decision creation
- **GIVEN** a user is editing a new decision after confirming its initial status
- **WHEN** the user focuses the Status dropdown, opens it, and confirms an option using its keyboard controls
- **THEN** the draft status MUST update without losing other field values
- **AND** focus MUST remain within the form

#### Scenario: Shortcut hints match active actions
- **GIVEN** a text field in the form has focus
- **WHEN** shortcut hints or help are displayed
- **THEN** they MUST identify Ctrl+S for saving and Escape for cancellation when no child popup is open
- **AND** they MUST NOT advertise an unimplemented P status-cycling action or an ordinary S key as the form save shortcut
- **AND** typing P or S MUST insert the character in that field

## ADDED Requirements

### Requirement: Draft Navigation Guard
The system SHALL guard navigation that would abandon a modified draft, including selecting another decision or file, starting another decision, refreshing from disk, and quitting. The guard SHALL offer Save, Discard, and Stay; explicit Cancel SHALL retain its defined discard behavior.

#### Scenario: Stay preserves the current editing session
- **GIVEN** the active draft has unsaved changes
- **WHEN** the user requests an action that would abandon it and chooses Stay
- **THEN** the requested action MUST NOT run
- **AND** the draft values, file identity, decision identity, and editing focus MUST remain available

#### Scenario: Save proceeds only after successful persistence
- **GIVEN** a guarded navigation action is pending
- **WHEN** the user chooses Save
- **THEN** the system MUST validate and persist the draft before carrying out that action
- **AND** validation or persistence failure MUST leave the action unexecuted and the draft editable

#### Scenario: Discard proceeds without writing the draft
- **GIVEN** a guarded navigation action is pending
- **WHEN** the user chooses Discard
- **THEN** the system MUST abandon the draft without writing it and carry out the requested action

#### Scenario: Unchanged drafts do not interrupt navigation
- **GIVEN** the draft matches its baseline
- **WHEN** the user navigates away
- **THEN** the system MUST carry out the action without an unsaved-changes prompt

### Requirement: Refresh Reloads Repository State
The Refresh action SHALL rediscover Markdown files and reload their contents from disk after resolving any modified draft through the navigation guard. It SHALL preserve selection by file path and decision ID when those objects still exist, and SHALL present load errors explicitly.

#### Scenario: Refresh reflects external additions and edits
- **GIVEN** a Markdown file or decision has been added or changed on disk
- **WHEN** Refresh completes
- **THEN** the file and decision lists MUST reflect the current readable repository contents
- **AND** the preview and any unchanged editor baseline MUST reflect the reloaded record
- **AND** selection MUST remain on the same file and decision when available

#### Scenario: Refresh handles removed or unreadable selections
- **GIVEN** the selected file or decision has been removed or cannot be parsed
- **WHEN** Refresh completes
- **THEN** the system MUST select an available valid entry or show an explicit empty state
- **AND** unreadable or invalid files MUST produce an actionable error rather than an apparently current editable record

### Requirement: First Decision Initialization
The system SHALL allow a first decision to be drafted when the repository contains no decision file or the selected Markdown file has no marker block. It SHALL explain and confirm the intended initialization before entering that workflow, but SHALL defer file creation or marker insertion until the first successful save. Initialization and the first decision SHALL be committed together by the persistence operation so a failed save does not leave an empty scaffold. New decision files SHALL use the fixed repository-root path `DECISIONS.md`; every initialization target SHALL be validated to remain within the repository root after path resolution.

#### Scenario: Empty repository offers a first decision file
- **GIVEN** the repository contains no Markdown files
- **WHEN** the user requests a new decision
- **THEN** the system MUST offer creation of repository-root `DECISIONS.md` and show that destination for confirmation
- **AND** after confirmation it MUST open the existing status-selection and draft workflow
- **AND** confirming initialization MUST NOT itself create a file

#### Scenario: Markerless file offers initialization
- **GIVEN** the selected Markdown file is inside the repository and has no decision markers
- **WHEN** the user requests a new decision and confirms initialization of that file
- **THEN** the system MUST allow drafting the first decision
- **AND** the first successful save MUST insert one valid marker block containing that decision while preserving all existing content outside the inserted block
- **AND** a file with invalid or ambiguous markers MUST NOT be treated as markerless

#### Scenario: Cancelling initialization has no filesystem effect
- **GIVEN** initialization has been offered or a first decision is being drafted
- **WHEN** the user declines initialization or discards the draft before its first successful save
- **THEN** the system MUST NOT create a file or insert a marker block

#### Scenario: First save cannot overwrite an existing destination
- **GIVEN** a new `DECISIONS.md` draft is ready to save
- **WHEN** the destination already exists, including when another process created it after initialization was confirmed
- **THEN** creation MUST fail without overwriting that destination
- **AND** the draft MUST remain editable with an explanation of the conflict

#### Scenario: Initialization rejects a destination outside the repository
- **GIVEN** resolving an initialization target would leave the repository root, including through a symbolic link
- **WHEN** initialization is requested or saved
- **THEN** the system MUST reject that target without writing outside the repository
- **AND** it MUST explain the invalid destination and preserve any draft

### Requirement: Compact Terminal Layout
The system SHALL remain usable at 80 columns by 24 rows and during terminal resizing. It SHALL adapt pane visibility or arrangement so that editing controls and navigation remain reachable, and SHALL preserve the current draft and selection during layout changes.

#### Scenario: Editing at the minimum supported terminal size
- **GIVEN** the terminal is 80 columns by 24 rows
- **WHEN** the user creates or edits a decision
- **THEN** every field and the Save and Cancel controls MUST be reachable by keyboard
- **AND** controls outside the current viewport MUST become visible when focused or scrolled into view
- **AND** the user MUST be able to reach the selected decision preview without increasing the terminal size

#### Scenario: Resize preserves the current work
- **GIVEN** the user has an active draft
- **WHEN** the terminal resizes between compact and larger supported sizes
- **THEN** the application MUST reflow without crashing or discarding draft values
- **AND** the selected file and decision MUST remain unchanged
- **AND** the active field MUST remain reachable

#### Scenario: Terminal below supported size
- **GIVEN** the terminal becomes smaller than 80 columns by 24 rows
- **WHEN** the application cannot display its supported layout
- **THEN** it MUST provide a clear resize message without discarding the draft
- **AND** restoring a supported size MUST restore usable controls
