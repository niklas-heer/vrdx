## 1. Specification
- [ ] 1.1 Review existing UI layout specs (if any) and determine whether `openspec/specs/ui-layout/spec.md` needs to be created or updated.
- [ ] 1.2 Author spec deltas under `openspec/changes/update-pane-focus-order/specs/ui-layout/spec.md` describing file prioritisation, pane numbering, selection rules, styling, and shortcut updates, with at least one scenario per requirement.
- [ ] 1.3 Run `openspec validate update-pane-focus-order --strict` and resolve any validation issues. *(Status: not yet run)*

## 2. Implementation
- [ ] 2.1 Update file discovery/loading so Markdown files containing decision marker blocks are sorted to the top and the first such file is auto-selected (e.g. adjust logic in `vrdx/ui/app.py` and related state management).
- [ ] 2.2 Add styling logic that visually grays out Markdown files without marker blocks (update widget rendering and `styles.tcss` as needed).
- [ ] 2.3 Reorder pane hints and bindings so Files is pane 2, Editor 3, Preview 4; reflect the new order in on-screen hints and keyboard bindings.
- [ ] 2.4 Ensure help/status messaging reflects updated shortcuts and pane order.

## 3. Quality Assurance
- [ ] 3.1 Extend or add automated tests covering file prioritisation, default selection, styling states, and keyboard bindings (unit or Textual-focused tests when feasible).
- [ ] 3.2 Run `uv run pytest -v` to confirm tests pass.
- [ ] 3.3 Run `uv run ruff check .` to ensure linting passes.
- [ ] 3.4 Perform a manual TUI session to confirm the files pane behaviour, visual styling, and shortcuts work as expected.
<!-- vrdx start -->

<!-- vrdx end -->
