## Why
Niklas requested publication readiness, polished installation docs, CI and Homebrew support. v0.2.0 is already published; release the authorized CLI/dashboard improvements as v0.3.0.

## What Changes
- Keep native four-platform packaging and require the full quality workflow before tag publication.
- Add a tested binary formula to the existing niklas-heer/homebrew-tap, with scheduled/manual update PRs using its own repository token after four-platform installation tests. Preserve its PR-required main branch.
- Lead the README with Homebrew installation and document release steps and supported systems.

## Authorization
The user's request authorizes preparing, integrating and publishing this release and its Homebrew distribution. Preserve unrelated AGENTS.md and tap README changes. No new cross-repository secret or crates.io publication is needed.

## Impact
Release workflow, package version, README, release documentation and existing Homebrew tap. Runtime architecture remains unchanged.
