## Implementation
- [x] Add the bundled skill source (SKILL.md and references/onboarding.md) and `init` module with embedded content, path planning, onboarding hints, dry run and JSON/human output.
- [x] Wire `init` into the CLI, guide JSON and packaging (Cargo include, Dang allowlist, verify-install test list).
- [x] Add subprocess tests for install, idempotence, AGENTS.md handling, dry run, conflicts and frontmatter rules; add the repository dogfood check.
- [x] Run `vrdx init` on this repository; update README, AI guide and add the decision record.
- [x] Evaluate triggering with Cursor CLI (Grok 4.6), Claude Code and Codex in disposable projects; tune the description and body from the evidence.
- [x] Run `mise run ci-native` and `mise run ci`; record results.
