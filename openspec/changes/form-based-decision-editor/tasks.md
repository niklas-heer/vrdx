## 1. Form-Based Editor Widget Creation

- [ ] 1.1 Create FormBasedDecisionEditor widget extending Static or Container
- [ ] 1.2 Implement Title input section (single-line text input or Static with placeholder)
- [ ] 1.3 Implement Status display section with visual status indicator
- [ ] 1.4 Implement Decision editable section (TextArea with markdown support)
- [ ] 1.5 Implement Context editable section (TextArea with markdown support)
- [ ] 1.6 Implement Consequences editable section (TextArea with markdown support)
- [ ] 1.7 Add Save and Cancel buttons at the bottom of the form
- [ ] 1.8 Create proper CSS styling for form layout and visual hierarchy

## 2. Integration with Editor Pane

- [ ] 2.1 Modify VrdxApp to replace or update EditorPane usage
- [ ] 2.2 Update action_new_decision to use form-based editor instead of TextArea
- [ ] 2.3 Update _begin_edit_existing to use form-based editor for existing decisions
- [ ] 2.4 Update _on_status_selected to populate form with selected status
- [ ] 2.5 Ensure form displays next decision ID prominently

## 3. Status Management in Form

- [ ] 3.1 Add [Change] button in Status section to show status selection modal
- [ ] 3.2 Create callback to handle status change from modal back to form
- [ ] 3.3 Update form when status is changed
- [ ] 3.4 Maintain visual consistency with rest of TUI

## 4. Field Validation

- [ ] 4.1 Implement title validation (not empty, not placeholder text)
- [ ] 4.2 Implement decision field validation (not empty)
- [ ] 4.3 Show validation errors inline or in status bar
- [ ] 4.4 Prevent save if validation fails
- [ ] 4.5 Add helpful error messages

## 5. Form Data Extraction and Saving

- [ ] 5.1 Extract form data from all input fields
- [ ] 5.2 Convert form data to DecisionRecord format
- [ ] 5.3 Handle Save button press (validate and create/update decision)
- [ ] 5.4 Handle Cancel button press (return to decision list)
- [ ] 5.5 Show success/error messages

## 6. Focus and Navigation

- [ ] 6.1 Implement tab-based focus navigation between form fields
- [ ] 6.2 Make Save/Cancel buttons easily accessible
- [ ] 6.3 Test keyboard navigation flow
- [ ] 6.4 Ensure proper focus management when switching between modes

## 7. Testing

- [ ] 7.1 Add unit tests for form-based editor widget
- [ ] 7.2 Add tests for field validation logic
- [ ] 7.3 Add tests for form data extraction
- [ ] 7.4 Add tests for status change integration
- [ ] 7.5 Verify all existing tests still pass
- [ ] 7.6 Test edge cases (empty fields, very long text, special characters)

## 8. Polish and Documentation

- [ ] 8.1 Update help text to document form-based editor workflow
- [ ] 8.2 Add inline code comments explaining form structure
- [ ] 8.3 Verify visual styling matches overall TUI aesthetic
- [ ] 8.4 Test with various terminal sizes
- [ ] 8.5 Update OpenSpec documentation with implementation details