## Why
The current text-based editor for creating and editing decisions requires users to manually structure markdown, which is error-prone and unintuitive. After selecting a status via the modal, users are dropped into a raw text editor with placeholder text, making the workflow feel incomplete and the save path unclear. A form-based editor with distinct sections for each decision field would provide:
- Clear visual structure matching the decision data model
- Explicit field validation and formatting
- Obvious affordances for saving (Save/Cancel buttons)
- Better discoverability for new users
- Seamless integration with the existing status selection modal

## What Changes
- Replace the current TextArea-based editor with a structured form-based editor
- Implement separate input sections for: Title, Status (with Change button), Decision, Context, and Consequences
- Add markdown support to the Decision, Context, and Consequences fields
- Display explicit Save and Cancel buttons at the bottom of the form
- Add field validation (e.g., ensure title is not empty or just placeholder text)
- Allow users to return to status selection modal via a [Change] button
- Maintain the same editing modes (edit-new, edit-existing)
- Display the next decision ID prominently in the form header

## Impact
- Affected specs: `ui-layout` (decision editor redesign)
- Affected code:
  - `vrdx/ui/app.py` - Replace EditorPane widget or update its behavior
  - `vrdx/ui/panes.py` - Update EditorPane or create new form-based widget
  - `vrdx/ui/modals.py` - Integrate status change affordance into form
  - `vrdx/app/commands.py` - May need adjustments for form data extraction
- Breaking changes: None (internal UI change, same save behavior)
- User experience: Significantly improved workflow clarity and discoverability