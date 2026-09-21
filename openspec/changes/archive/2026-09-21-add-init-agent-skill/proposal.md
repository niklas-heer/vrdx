# Install an agent skill with `vrdx init`

## Why
Agents only consult and record decisions when something tells them to. Niklas
asked on 2026-09-21 for an `init` command that installs a skill so any coding
agent checks existing decisions before a consequential choice and records the
outcome with vrdx, compatible with the different agents without a wizard.

## What Changes
- Add `vrdx init`: install the bundled skill at `.agents/skills/vrdx/SKILL.md`,
  link it for Claude Code at `.claude/skills/vrdx`, and upsert a short managed
  block in `AGENTS.md` naming the collection directory. Idempotent; reports
  created, updated or unchanged per path; `--dry-run` writes nothing.
- Ship the skill inside the executable. Its source is the checked-in
  `.agents/skills/vrdx/SKILL.md`, so the vrdx repository dogfoods it and
  `npx skills add niklas-heer/vrdx` finds the same file without a registry.
- Keep the skill short: consult with `context`/`show` before deciding, record
  with `new --from-json` after agreement, defer rules to `vrdx guide`.
- Give the skill an onboarding reference for importing existing ADRs or decision
  logs as a baseline, and have `init` report recognised ADR directories and a
  collection directory whose files are not vrdx records.
- Document `init` in the README, AI guide and guide JSON; add a decision record.
- Verify triggering with external agents (Cursor CLI with Grok 4.6, Claude Code,
  Codex) in disposable projects: positive, negative and consult scenarios.

## Authorization and boundaries
Niklas approved the consult-and-record scope, the embedded skill and release-based
updates on 2026-09-21. No interactive wizard, no network access, no Node
dependency, no edits outside the three managed paths, and no changes to existing
records. `init` never runs an agent. Files outside the managed AGENTS.md block are
preserved byte for byte. Agent evaluations run outside CI and never require credits.

## Research
- https://agentskills.io/specification — SKILL.md frontmatter, naming and
  progressive-disclosure rules (name matches directory, description ≤ 1024 chars).
- Codex, Cursor, OpenCode, Zed and Cline read project skills from `.agents/skills/`;
  Claude Code reads `.claude/skills/` and follows symlinked skill directories
  (code.claude.com/docs/en/skills, learn.chatgpt.com/docs/build-skills, checked
  2026-09-21).
- https://github.com/vercel-labs/skills — installer over git that scans
  `.agents/skills/` in any repository; no registration required.
