## Verification and fixes

- [x] Independently review the current CLI, graph, AI interface and dashboard; resolve concrete findings.
- [x] Add and run reproducible realistic collection simulations, including faults and recovery.
- [x] Exercise the dashboard with a larger collection in a real browser and fix demonstrated usability failures.

## Distribution

- [x] Add MIT licensing and include it in source and binary packages.
- [x] Build and test native release archives with checksums through project tasks and CI.
- [x] Document installation, platform support, compatibility and simulation scope.

## Delivery

- [x] Archive completed historical proposals without reintroducing retired editor specifications.
- [x] Run native/container quality gates, package and release checks; record measured results.
- [x] Push updates to PR #17 and supervise current-revision checks and review.
- [x] Merge PR #17 and reconcile superseded PR #16.
- [x] Publish the tested release and verify downloaded assets.

## Verification

- Independent review found and fixed a misleading human context no-match message;
  a subprocess regression covers both matching and empty results.
- Native macOS ARM64 and Dagger Linux ARM64 passed formatting, compilation,
  strict Clippy, 21 tests, doctests and 19 installed-binary journeys. Cargo package
  verification passed with the license included and no missing-license warning.
- Simulation exercises 221 records across 12 background topics and a concrete
  cache decision history; three seeded runs add 144 state transitions. Local
  standalone simulation tests took 1.47 seconds; timing is machine/workload specific.
- Real browser checks exercised 221-record browsing, topic filtering, replacement
  navigation, reasoning, explained suggestions, malformed-edit diagnostics and recovery.
- Extracted macOS ARM64 and Dagger Linux ARM64 archives passed behavioral tests.
- PR #17 revision a95dab4 passed hosted Linux/macOS CI (run 35453041325) and all
  four native archive jobs (run 35453041420). PR runs correctly skipped publication.
- PR #17 merged as 0f8ab68 on 2026-09-19. GitHub automatically marked PR #16
  merged because its historical commits are ancestors of this change; the final
  main tree retains the current CLI architecture. There is no open editor PR.
- Post-merge Linux/macOS CI passed (run 35453160959). The annotated v0.2.0 tag
  points at 0f8ab68. Release run 35453261486 passed all four platforms and published
  four archives plus SHA256SUMS at https://github.com/niklas-heer/vrdx/releases/tag/v0.2.0.
- All four downloaded archive checksums matched. The downloaded macOS ARM64
  executable reported vrdx 0.2.0, validated the repository's four decision records,
  and passed all 19 CLI/AI/HTTP/simulation subprocess journeys in 0.955 seconds.
- No real-world adoption, semantic relevance guarantee, signing or notarization
  is claimed by these checks.
