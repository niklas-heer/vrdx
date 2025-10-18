## ADDED Requirements

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
