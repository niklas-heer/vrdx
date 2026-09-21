+++
schema_version = 1
id = "01M31CVYPCQ87F6S3C7EKQ1RNY"
title = "Ship the agent skill inside the binary"
date = "2026-09-21"
status = "accepted"
tags = ["agents", "distribution", "workflow"]
+++

## Decision

vrdx embeds one Agent Skills compatible skill and installs it with `vrdx init` into `.agents/skills/vrdx/`, a `.claude/skills/vrdx` symlink and a managed `AGENTS.md` block.

## Why

Agents only consult and record decisions when something tells them to at the right moment. A skill under `.agents/skills/` is read by Codex, Cursor and others, the symlink covers Claude Code, and the AGENTS.md block is the portable fallback. Embedding the skill ties its text to the CLI version, so a release updates both together; a registry such as skills.sh would let them drift and hand ownership to another tool, while a wizard would add interaction without covering more agents.

## Consequences

- No network, Node or prompt is needed; `init` is idempotent, reports per-path status and only touches four paths.
- Skill changes require a vrdx release and a rerun of `init` in each project.
- Skill triggering is validated by manual agent runs, not by CI; the description carries the trigger and an explicit exclusion of routine work.
- Existing ADR folders are reported by `init` and imported by the agent following the bundled onboarding reference.
