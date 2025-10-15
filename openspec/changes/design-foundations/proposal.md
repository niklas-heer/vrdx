## Why
vrdx’s initial design guidance lives in legacy `docs/DESIGN.md`, which mixes goals, architecture, and interaction patterns outside of OpenSpec. Migrating the core design principles into OpenSpec ensures future changes build on an authoritative, versioned specification while allowing the legacy doc to be archived.

## What Changes
- Capture foundational design goals and non-goals for vrdx within an OpenSpec proposal.
- Summarize the current high-level architecture (CLI entrypoint, state/command layers, Textual UI, persistence).
- Outline the UI pane layout, interaction model, and marker management rules to preserve decision record workflows.
- Enumerate longer-term milestones and considerations that should remain referenceable for future proposals.

## Impact
- Affected specs: new “design-foundations” capability (initial addition).
- Affected code: None yet—this proposal documents existing behavior to guide future specification and implementation work.
- Follow-up: Subsequent OpenSpec proposals may refine or extend individual areas (e.g., state management, UI interactions) using this document as the baseline.