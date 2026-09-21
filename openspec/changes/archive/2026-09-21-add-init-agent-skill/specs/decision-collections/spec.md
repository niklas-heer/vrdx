## ADDED Requirements

### Requirement: Agent skill installation
The CLI SHALL provide an `init` command that installs a bundled Agent Skills
`SKILL.md` for consulting and recording decisions at `.agents/skills/vrdx/`,
links it for Claude Code at `.claude/skills/vrdx`, and maintains one marked block
in `AGENTS.md` that names the collection directory. The command SHALL be
idempotent, report a per-path status, support a dry run that writes nothing,
preserve all content outside the managed block, and abort without writing when a
managed path conflicts with an unrelated file. It SHALL report recognised existing ADR directories and a collection directory whose Markdown is not vrdx records as onboarding hints without modifying them, and the installed skill SHALL describe how to import such sources as a baseline without inventing dates, statuses or reasons.

#### Scenario: Fresh project
- **WHEN** `init` runs in a directory without agent configuration
- **THEN** the skill file, the symlink and `AGENTS.md` with the managed block SHALL exist and be reported as created
- **AND** a second run SHALL report every path unchanged

#### Scenario: Existing AGENTS.md
- **WHEN** `init` runs where `AGENTS.md` already has content or an older managed block
- **THEN** only the text between the markers SHALL change and every other byte SHALL be preserved

#### Scenario: Skill triggers at the right time
- **WHEN** an agent with the installed skill faces a consequential technical choice
- **THEN** the skill SHALL direct it to consult existing records first and to record the agreed choice with `new --from-json`, defaulting to proposed
- **AND** the skill SHALL exclude routine edits, bug fixes and progress notes

#### Scenario: Existing ADR folder
- **WHEN** `init` runs in a project with `docs/adr` or Markdown in the collection directory that is not a vrdx record
- **THEN** the output SHALL name those sources as onboarding candidates and SHALL leave them unchanged
