# Design: `vrdx init` and the bundled skill

## Managed paths

`init` operates on the current working directory as the project root and manages
exactly four paths:

| Path | Action |
| --- | --- |
| `.agents/skills/vrdx/SKILL.md` | Write the embedded skill. Existing content that differs is replaced and reported as `updated`. |
| `.agents/skills/vrdx/references/onboarding.md` | Write the embedded onboarding reference with the same rules. |
| `.claude/skills/vrdx` | Relative symlink to `../../.agents/skills/vrdx`. An existing correct symlink is `unchanged`; an existing real directory receives `SKILL.md` inside it (copy layout); any other file or foreign symlink is a `conflict` error. |
| `AGENTS.md` | Upsert one block between `<!-- vrdx:start -->` and `<!-- vrdx:end -->`. Missing file: create with the block. No markers: append after a blank line. Markers present: replace only the enclosed text. |

The block names the collection directory from `--dir` (default `decisions`) and
points to the skill file and `vrdx guide`, so agents without skill support still
see the workflow. Non-default directories mention `--dir`.

Per-path status: `created`, `updated`, `unchanged`. `--dry-run` computes the same
statuses without writing. Human output prints one line per path plus a closing
hint to commit; JSON returns `{"dry_run":bool,"paths":[{"path","status"}]}`.
Any conflict aborts before writing anything.

## Onboarding hints

`init` additionally reports, without acting on them, `docs/adr`, `docs/decisions`,
`adr` and `doc/adr` when they exist, and whether the selected collection
directory already contains Markdown that does not parse as vrdx records. The
human output suggests asking the agent to onboard them; the JSON carries
`onboarding: {"candidates": [...], "collection_needs_import": bool}`. The skill's
`references/onboarding.md` describes the import: one record per source decision,
original dates and statuses when stated, relationships recreated afterwards,
nothing invented, sources left in place.

`init` does not create the collection directory (Git ignores empty directories;
`new` creates it), does not touch CLAUDE.md (Claude Code loads the symlinked
skill), and does not prompt.

## Skill

Source: `.agents/skills/vrdx/SKILL.md`, embedded with `include_str!` like the AI
guide. Frontmatter follows the Agent Skills specification; `metadata.vrdx-skill`
carries a small format version so a future `init` can recognise older copies.
The body stays under 60 lines and defers format and writing rules to
`vrdx guide --json`. The description carries the trigger: consequential technical
choices, before and after, and an explicit exclusion of routine work.

## Packaging

`Cargo.toml` `include` and the Dang module's source allowlist gain `.agents` so
release builds and the Dagger pipeline can embed the file. The relocated
executable scenario covers `init` with no repository assets present.

## Verification

- Subprocess tests in `tests/init.rs`: fresh project, rerun unchanged, existing
  AGENTS.md preserved with the block appended, marker replacement, custom `--dir`
  text, dry run writes nothing, symlink resolves, conflict aborts cleanly,
  frontmatter name matches the directory and description length limit.
- `tests/documentation.rs`: `vrdx init --dry-run` in the repository root reports
  the two embedded skill files unchanged, so the checked-in copies match the
  binary. The symlink and AGENTS.md are not checked because CI containers
  receive a filtered source tree.
- Manual agent evaluation in disposable projects with the installed skill:
  a consequential choice triggers consult and, after agreement, one record; a
  typo fix does not trigger; a question covered by an accepted record cites it.
  Results are summarised in the decision record and tasks; they are evidence of
  triggering, not a CI gate.
