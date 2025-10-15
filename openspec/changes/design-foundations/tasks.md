## 1. Specification
- [ ] 1.1 Confirm the scope of the “design” capability and whether additional capability directories are required.
- [ ] 1.2 Draft the specification delta capturing goals, architecture layers, interaction model, marker rules, and persistence expectations.
- [ ] 1.3 Validate the change with `openspec validate design-foundations --strict` and resolve any reported issues.

## 2. Documentation Migration
- [ ] 2.1 Ensure all relevant content from `docs/DESIGN.md` is represented in the new spec (goals/non-goals, architecture, UI layout, interaction model, marker management, persistence expectations, roadmap).
- [ ] 2.2 Cross-reference any remaining topics that should move into follow-up proposals (e.g., UI polish, state management details).

## 3. Review and Archive Preparation
- [ ] 3.1 Share the proposal for review to confirm the archived spec mirrors the historical design intent.
- [ ] 3.2 Update the legacy `docs/DESIGN.md` with a pointer to the archived OpenSpec once migration is approved.

## 4. Follow-Up (Post-Approval)
- [ ] 4.1 After approval and implementation, use the OpenSpec tooling to archive the change (e.g., `openspec archive design-foundations --strict` when appropriate).