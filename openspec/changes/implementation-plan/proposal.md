## Why
The detailed implementation roadmap for vrdx still lives in the legacy `docs/IMPLEMENTATION.md`, outside of OpenSpec. Migrating this guidance into a dedicated change proposal ensures the staged milestones, module layout, and dependency expectations are tracked in the canonical specification archive. This lets future contributors understand historical implementation intent and reference the original plan even after the legacy doc is retired.

## What Changes
- Capture the original implementation milestones—package structure, dependency strategy, state/command wiring, Textual UI phases, and polish tasks—as an OpenSpec change.
- Document the recommended tooling setup (pytest, ruff, mypy, uv workflows) and quality gates inside the spec so they can inform future changes.
- Preserve risks and mitigations identified in the legacy document, including parsing ambiguity, Textual API drift, distribution concerns, and cross-link integrity.
- Prepare supporting spec deltas (milestones, risks, testing strategy, tooling) that can be archived for long-term reference.

## Impact
- Affected specs: adds a new capability covering the implementation roadmap (initial addition).
- Affected code: none immediately—this migration codifies historical plan details to guide upcoming proposals and implementations.
- Follow-up: later changes can refine or supersede individual milestones by referencing this archived specification.