# Ship the verified decision CLI

## Why

The CLI, dashboard and AI interface exist on PR #17 but have not reached main or a downloadable release. Real-world adoption cannot be obtained during this task; reproducible simulated workflows can test the remaining practical uncertainties without claiming production experience.

## What Changes

- Exercise a substantial mixed-topic collection, lifecycle transitions, renames, invalid edits and recovery through the actual CLI, with reproducible action sequences and explicit retrieval expectations.
- Fix concrete failures found by independent review, simulation or browser checks while retaining Markdown editing and deterministic local retrieval.
- Add the MIT license following Niklas's established preference, package metadata, and tested downloadable native binaries with checksums and installation instructions.
- Reconcile completed historical OpenSpec proposals and the superseded editor PR, then merge PR #17 after checks pass and publish the verified release.

## Scope

No hosted service, semantic model, mutation API, database or automatic legacy migration. Simulation establishes behavior only for modeled workloads. Native release artifacts target supported Linux and macOS architectures; they are exercised before publication.

## Approval

On 2026-09-19 Niklas explicitly requested implementation of all remaining assessment items, authorized subagents, requested simulated usage, and instructed continued supervision until the PR is merged. This authorizes this release-readiness work and delivery. MIT follows his existing licensing preference. The proposal is reviewed against the current accepted CLI scope before implementation.
