## Why
The current TUI pane ordering surfaces the files list last and focuses the decisions pane at startup, which makes it harder to immediately inspect the Markdown file containing the active decision markers. Contributors have requested a workflow that highlights files with decision blocks up front, auto-selects the most relevant file, and updates both pane numbering and keyboard shortcuts to match the revised layout.

## What Changes
- Update the files discovery workflow so Markdown files containing `<!-- vrdx start -->` … `<!-- vrdx end -->` blocks are sorted to the top of the files pane, with the first such file auto-selected on launch.
- Visually gray out Markdown files that do not contain decision marker blocks to make them easy to distinguish at a glance.
- Reorder pane numbering so Files is pane `2`, Editor is `3`, Preview is `4`, and adjust the on-screen hints plus keyboard bindings to reflect the new order.
- Refresh status/help messaging to describe the new pane order and shortcuts.
- Add automated coverage (unit or Textual-focused tests) that verifies file prioritisation, selection, styling states, and keyboard bindings.

## Impact
- Affected specs: introduce or update the `ui-layout` capability to document the new pane ordering, focus rules, and styling requirements.
- Affected code: `vrdx/ui/app.py`, related pane widgets, keyboard binding declarations, and styling definitions.
- Affected tests: extend UI-related unit tests (or add new ones) to cover the new behaviour once Textual testing support is available.

## Open Questions
- Do we need configuration flags to restore the legacy pane ordering for existing users?
- Should grayed-out files remain selectable for creating new decision blocks, or should they be temporarily non-focusable?
<!-- vrdx start -->

<!-- vrdx end -->
