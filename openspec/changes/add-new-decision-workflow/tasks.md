## 1. Core Implementation

- [ ] 1.1 Create interactive status selection UI component or modal for displaying available status options
- [ ] 1.2 Update `action_new_decision()` in `vrdx/ui/app.py` to display status selection before entering edit mode
- [ ] 1.3 Store selected status in app state or temporary variable for use in template generation
- [ ] 1.4 Update template generation to use the user-selected status instead of always defaulting to Draft
- [ ] 1.5 Ensure editor opens in edit mode with correct next ID and selected status pre-filled

## 2. Status Selection Mechanism

- [ ] 2.1 Implement cycling through status options (using arrow keys or similar navigation)
- [ ] 2.2 Display current status selection visually in the UI (highlight or indicate the selected option)
- [ ] 2.3 Add keybinding to confirm status selection (Enter or Space)
- [ ] 2.4 Add keybinding to cancel status selection (Escape)
- [ ] 2.5 Ensure status wraps around from last to first when cycling

## 3. Integration & Polish

- [ ] 3.1 Verify status option cycling (`p` key) still works while editing new decisions
- [ ] 3.2 Update help text to document the new status selection workflow
- [ ] 3.3 Ensure the workflow works correctly with no decisions in the current file
- [ ] 3.4 Test with files containing decisions to verify next ID is calculated correctly

## 4. Testing

- [ ] 4.1 Add unit tests for status selection logic
- [ ] 4.2 Add integration test for new decision creation with status selection
- [ ] 4.3 Add Textual UI test to verify keybindings and visual feedback
- [ ] 4.4 Verify backward compatibility with existing decision editing workflow

## 5. Documentation

- [ ] 5.1 Update help overlay (?) to document the new workflow
- [ ] 5.2 Update any relevant README or documentation files
- [ ] 5.3 Add inline code comments explaining the status selection flow