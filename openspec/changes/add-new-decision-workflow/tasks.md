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
- [ ] 4.3 Add Textual UI test to verify keybindings and visual feedback (UI tests framework disabled)
- [x] 4.4 Verify backward compatibility with existing decision editing workflow

## 5. Documentation

- [x] 5.1 Update help overlay (?) to document the new workflow
- [ ] 5.2 Update any relevant README or documentation files
- [ ] 5.3 Add inline code comments explaining the status selection flow