# ui-layout Specification

## Purpose
TBD - created by archiving change update-pane-focus-order. Update Purpose after archive.
## Requirements
### Requirement: Prioritize Files with Decision Blocks
The files pane SHALL list Markdown files containing `<!-- vrdx start -->` … `<!-- vrdx end -->` marker blocks before any other Markdown files and automatically select the first such file on application startup.

#### Scenario: Files with marker blocks are sorted and focused
- **GIVEN** the repository contains multiple Markdown files, some with decision marker blocks and some without
- **WHEN** the TUI loads the files pane
- **THEN** all files that contain decision marker blocks MUST appear before files without marker blocks
- **AND** the first file in the list that contains a marker block MUST be selected by default

### Requirement: Visualize Files Without Decision Blocks
Markdown files that do not contain decision marker blocks SHALL remain focusable but MUST present a visually muted (grayed out) style to distinguish them from files containing decisions.

#### Scenario: Files without markers render in muted style
- **GIVEN** a Markdown file lacking decision marker blocks appears in the files pane
- **WHEN** the files pane renders the list
- **THEN** the entry representing that file MUST use the designated muted styling token(s)
- **AND** the entry MUST remain selectable so users can insert a new decision block if desired

### Requirement: Update Pane Numbering and Shortcuts
Pane numbering, on-screen hints, and keyboard shortcuts SHALL map as: `1` Decisions, `2` Files, `3` Editor, `4` Preview. The Editor pane SHALL now display a form-based interface for decision creation and editing instead of a raw text editor.

#### Scenario: Editor pane displays form-based interface
- **GIVEN** a user focuses the Editor pane (press `3`)
- **WHEN** editing a decision or creating a new one
- **THEN** the Editor pane MUST display the form-based interface with structured fields
- **AND** the existing pane numbering and shortcuts MUST remain unchanged
- **AND** keyboard navigation between form fields MUST work intuitively

### Requirement: Form-Based Decision Editor Interface
The system SHALL provide a structured form-based editor for creating and editing decisions that displays distinct input sections for Title, Status, Decision, Context, and Consequences fields, replacing the raw text editor.

#### Scenario: Form displays all required fields
- **GIVEN** a user initiates new decision creation or selects an existing decision for editing
- **WHEN** the editor pane becomes active
- **THEN** the form MUST display all five required sections:
  - Title field (single-line input)
  - Status display with [Change] button
  - Decision section (editable area)
  - Context section (editable area)
  - Consequences section (editable area)
- **AND** the decision ID MUST be displayed prominently (e.g., "Create New Decision #3")
- **AND** Save and Cancel buttons MUST appear at the bottom

#### Scenario: Form pre-populates data for existing decisions
- **GIVEN** a user selects an existing decision to edit
- **WHEN** the form-based editor opens
- **THEN** all fields MUST be pre-populated with the existing decision's data
- **AND** the decision title MUST appear in the Title field
- **AND** the current status MUST display in the Status section
- **AND** the decision text MUST appear in the Decision section
- **AND** the context MUST appear in the Context section
- **AND** the consequences MUST appear in the Consequences section

#### Scenario: Form receives status from selection modal
- **GIVEN** the user confirms a status selection from the status selection modal
- **WHEN** the form-based editor opens
- **THEN** the selected status MUST be displayed in the Status section
- **AND** the form MUST be ready for user input in the Title field

### Requirement: Field Validation and Feedback
The system SHALL validate form fields and provide clear feedback when validation fails, preventing save of invalid data.

#### Scenario: Title field validation prevents empty submission
- **GIVEN** the user attempts to save a decision with an empty title
- **WHEN** the user presses the Save button
- **THEN** the form MUST NOT submit
- **AND** a validation error MUST be displayed (e.g., "Title cannot be empty")
- **AND** focus MUST return to the Title field

#### Scenario: Title placeholder text is not accepted as valid
- **GIVEN** the user leaves the title as the placeholder text (e.g., "Decision Title (e.g., ...)")
- **WHEN** the user attempts to save
- **THEN** the form MUST NOT submit
- **AND** an error message MUST indicate the user must enter a real title

#### Scenario: Unsaved changes indicator
- **GIVEN** a user makes changes to any form field
- **WHEN** the field is modified from its last saved state
- **THEN** a visual indicator MUST show unsaved changes (e.g., asterisk or highlight)
- **AND** if the user attempts to navigate away, a confirmation MUST prompt them

### Requirement: Explicit Save and Cancel Actions
The system SHALL provide clear, easily accessible Save and Cancel buttons with obvious keyboard shortcuts.

#### Scenario: Save button creates or updates decision
- **GIVEN** a user completes filling out the form
- **WHEN** the user presses the Save button (or Ctrl+S)
- **THEN** the system MUST validate all fields
- **AND** if valid, create a new decision or update the existing one
- **AND** a success message MUST appear (e.g., "Decision saved")
- **AND** the editor MUST close and return to the decision list
- **AND** the newly created/updated decision MUST appear in the decisions pane

#### Scenario: Cancel button aborts editing
- **GIVEN** a user is editing in the form-based editor
- **WHEN** the user presses the Cancel button (or Escape)
- **THEN** any unsaved changes MUST be discarded
- **AND** the editor MUST close
- **AND** the user MUST return to the decision list view

#### Scenario: Keyboard shortcuts for save and cancel
- **GIVEN** the form-based editor is active
- **WHEN** the user presses Ctrl+S
- **THEN** the Save action MUST be triggered (same as Save button)
- **AND** when the user presses Escape
- **THEN** the Cancel action MUST be triggered (same as Cancel button)

### Requirement: Status Change Integration
The system SHALL allow users to change the selected status from within the form via a [Change] button, maintaining form data while returning to status selection.

#### Scenario: Change button re-opens status modal
- **GIVEN** a user is editing a decision in the form-based editor
- **WHEN** the user clicks the [Change] button next to the Status display
- **THEN** the status selection modal MUST appear
- **AND** all form data MUST be preserved (Title, Decision, Context, Consequences)
- **AND** the current status MUST be highlighted in the modal

#### Scenario: Updated status reflects in form
- **GIVEN** the status selection modal is open from the form editor
- **WHEN** the user selects a new status and confirms
- **THEN** the modal MUST close
- **AND** the Status section in the form MUST update to show the new status
- **AND** the cursor/focus MUST return to the form
- **AND** the form MUST be ready for further editing or saving

### Requirement: Status Selection in New Decision Workflow
When creating a new decision by pressing `N`, the system SHALL present the user with an interactive status selection interface before opening the editor, allowing the user to choose from the curated set of status options (📝 Draft, ✅ Accepted, ❌ Rejected, ⛔ Deprecated by …, ⬆️ Supersedes …).

#### Scenario: User creates new decision with status selection
- **WHEN** the user presses `N` with an active file containing a marker block
- **THEN** a status selection menu MUST appear showing all available status options
- **AND** the first status option (📝 Draft) MUST be highlighted or marked as the current selection
- **AND** the user MUST be able to navigate between options using arrow keys or j/k
- **AND** pressing Enter MUST confirm the selection
- **AND** pressing Escape MUST cancel the operation and return to the previous pane

#### Scenario: Editor opens with selected status pre-filled
- **GIVEN** the user has selected a status from the status selection menu
- **WHEN** the user confirms the selection by pressing Enter
- **THEN** the editor MUST open in edit mode with a new decision template
- **AND** the template MUST include the correct next decision ID calculated from the file
- **AND** the Status field MUST be pre-filled with the user-selected status option
- **AND** all other fields (Title, Decision, Context, Consequences) MUST contain appropriate placeholder text

#### Scenario: Status selection wraps around
- **GIVEN** the status selection menu is open and the last status option is highlighted
- **WHEN** the user presses the down arrow or `j` key
- **THEN** the selection MUST wrap to the first status option (📝 Draft)
- **AND** pressing the up arrow or `k` key from the first option MUST wrap to the last status option

### Requirement: Accessible Status Cycling During New Decision Editing
The system SHALL maintain the ability to cycle through status options using the `P` key while editing a new decision in the editor pane, in addition to the upfront status selection during workflow initiation.

#### Scenario: Status cycling in editor during new decision creation
- **GIVEN** the user is editing a new decision in the editor pane (after confirming status selection)
- **WHEN** the user presses `P`
- **THEN** the Status line MUST advance to the next status option in the predefined list
- **AND** the status MUST wrap from the last option back to the first
- **AND** the cursor position and focus MUST remain in the editor pane

