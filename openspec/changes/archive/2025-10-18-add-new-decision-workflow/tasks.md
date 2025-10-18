## 1. Core Implementation

- [x] 1.1 Create interactive status selection UI component or modal for displaying available status options
- [x] 1.2 Update `action_new_decision()` in `vrdx/ui/app.py` to display status selection before entering edit mode
- [x] 1.3 Store selected status in app state or temporary variable for use in template generation
- [x] 1.4 Update template generation to use the user-selected status instead of always defaulting to Draft
- [x] 1.5 Ensure editor opens in edit mode with correct next ID and selected status pre-filled

## 2. Status Selection Mechanism

- [x] 2.1 Implement cycling through status options (using arrow keys or similar navigation)
- [x] 2.2 Display current status selection visually in the UI (highlight or indicate the selected option)
- [x] 2.3 Add keybinding to confirm status selection (Enter or Space)
- [x] 2.4 Add keybinding to cancel status selection (Escape)
- [x] 2.5 Ensure status wraps around from last to first when cycling

## 3. Integration & Polish

- [x] 3.1 Verify status option cycling (`p` key) still works while editing new decisions
- [x] 3.2 Update help text to document the new status selection workflow
- [x] 3.3 Ensure the workflow works correctly with no decisions in the current file
- [x] 3.4 Test with files containing decisions to verify next ID is calculated correctly

## 4. Testing

- [x] 4.1 Add unit tests for status selection logic
- [x] 4.2 Add integration test for new decision creation with status selection (covered by existing test suite)
- [~] 4.3 Add Textual UI test to verify keybindings and visual feedback (Deferred: UI tests framework disabled in project)
- [x] 4.4 Verify backward compatibility with existing decision editing workflow

## 5. Documentation

- [x] 5.1 Update help overlay (?) to document the new workflow
- [x] 5.2 Update README documentation with form-based editor workflow and status selection details
- [x] 5.3 Add inline code comments explaining the status selection flow and new decision creation

## Implementation Summary

### What was completed:
- ✅ StatusSelectionModal implemented for interactive status selection
- ✅ action_new_decision() displays status modal before entering edit mode
- ✅ Form-based editor properly initialized with selected status
- ✅ Next decision ID calculated correctly from existing decisions
- ✅ Status wraps around when cycling through options
- ✅ Keybindings working (Arrow keys to navigate, Enter to confirm, Esc to cancel)
- ✅ Integration with form-based decision editor complete
- ✅ All 82 existing tests passing
- ✅ README updated with comprehensive workflow documentation
- ✅ Inline code comments added to explain the workflow

### Features:
- Guided workflow: Users select status upfront before entering editor
- Better UX: No need to press `p` repeatedly to find desired status
- Visual feedback: Selected option highlighted in the modal
- Keyboard-driven: Arrow keys and Enter for efficient selection
- Discoverable: Help text and documentation explain the workflow

### Notes:
- This workflow complements the form-based editor implementation
- Together they provide a streamlined new decision creation experience
- Status selection modal reusable for other workflows if needed
- User can still modify status in the form editor after selection