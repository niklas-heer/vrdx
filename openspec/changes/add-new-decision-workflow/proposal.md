## Why
Currently, when creating a new decision, users must enter edit mode, then press `p` to cycle through status options to choose one. This requires two separate interactions and the status cycling is not discoverable for new users. Users would benefit from a streamlined workflow where they can select the decision status upfront before entering the editor, reducing cognitive load and making the new decision creation process more intuitive and guided.

## What Changes
- When pressing `n` (new decision), prompt the user to select a status from the available options before entering edit mode
- Present status options in an interactive menu (dropdown-style selection interface)
- Auto-populate the editor with the decision template including the selected status
- Keep the existing `p` key for in-editor status cycling for power users

## Impact
- Affected specs: ui-layout (new interaction pattern for decision creation workflow)
- Affected code: vrdx/ui/app.py (action_new_decision), vrdx/app/commands.py (template generation)
- UI flow becomes more guided and discoverable for new users
- No breaking changes to existing functionality