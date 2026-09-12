# Complete decision workflows and agent interface

## Why
The Rust migration omitted original delete/reorder requirements and left the stated agent-facing vision incomplete. The user explicitly approved completing all identified gaps, with no deferrals.

## What Changes
- Restore source-preserving deletion and reordering in the TUI and CLI.
- Add deterministic headless list/show/search/create/update/delete/move/validate commands with JSON output and concurrency protection.
- Add reusable repository templates, persisted decision relationships, Git history inspection, and conservative automatic three-way merging of independent edits.
- Expose search, templates, relationships, history, and conflict merge in the TUI.
- Complete enforceable Premise contracts and exercise each delivered capability through meaningful integration/terminal tests.

## Impact
Affected specifications: design, implementation, ui-layout. Existing Markdown records remain compatible. This is an approved continuation of the Rust migration on pinned nightly/mise, preserving the user's AGENTS.md edits and read-only references.

## Approval
The user's instruction to complete the missing parity and vision capabilities authorizes implementation of this proposal.
