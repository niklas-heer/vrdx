## 1. Specification
- [x] 1.1 Determine whether additional capability folders are needed beyond `implementation`.
- [x] 1.2 Translate the milestones, package layout, dependency strategy, testing plan, and risk mitigation details from `docs/IMPLEMENTATION.md` into the spec delta.
- [x] 1.3 Run `openspec validate implementation-plan --strict` and resolve any validation issues.

## 2. Documentation Migration
- [x] 2.1 Verify every milestone (1–7) from the legacy implementation document is accurately reflected in the spec.
- [x] 2.2 Confirm tooling notes (uv workflows, justfile commands, linting, typing, watch tools) are captured.
- [x] 2.3 Record the original risk/mitigation bullets to ensure they remain referenceable.

## 3. Review and Archive Preparation
- [x] 3.1 Circulate the proposal for review to confirm the migrated spec matches historical intent.
- [x] 3.2 Retire the legacy `docs/IMPLEMENTATION.md` file and direct contributors to the OpenSpec archive.

## 4. Follow-Up (Post-Approval)
- [ ] 4.1 After approval, use the OpenSpec CLI to archive the change (e.g., `openspec archive implementation-plan --strict`) to preserve the roadmap.