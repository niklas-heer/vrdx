## 1. Rust foundation

- [x] 1.1 Create the Cargo application/library on pinned nightly Rust with Ratatui/Crossterm and minimal dependency features.
- [x] 1.2 Integrate Premise Fielded/Keyed and rationale into the domain core; test lossless IDs and invalid field shapes.
- [x] 1.3 Configure strict Clippy, rustfmt, Rust Analyzer, mise tool pins, and local/CI tasks.

## 2. File integrity

- [x] 2.1 Implement source-aware marker/record parsing with empty narratives, paragraphs, code examples, CRLF/BOM, and ambiguity diagnostics.
- [x] 2.2 Preserve unedited source and validate generated records before saving.
- [x] 2.3 Implement atomic existing-file saves, conservative conflicts, no-op saves, and exclusive new-file publication.
- [x] 2.4 Test duplicate IDs/labels, malformed markers, round trips, source preservation, concurrent changes, and injected write failures.

## 3. Complete terminal workflow

- [x] 3.1 Implement browse/edit/new/status flows, Unicode text editing, paste, mouse buttons, focus, and truthful dirty indicators.
- [x] 3.2 Implement Save/Cancel, failed-save retention, and Save/Discard/Stay guards for navigation and quit.
- [x] 3.3 Implement confirmed first-file/marker initialization, disk refresh, selection retention, and per-file diagnostics.
- [x] 3.4 Implement compact/normal layout, resize handling, NO_COLOR, help, and clean interrupt/terminal restoration.

## 4. Real verification

- [x] 4.1 Add Ratatui TestBackend and input-event tests for rendering and workflow state.
- [x] 4.2 Add pseudo-terminal tests launching the real binary, sending UTF-8 characters, control keys, paste, and resize, then asserting screen/disk state and cleanup.
- [x] 4.3 Verify character-driven create/save/reopen/edit, draft discard, conflict handling, and Ctrl+C exit.
- [x] 4.4 Configure and run nextest, doc tests, Criterion, and installed-binary smoke/journey checks outside the checkout.
- [x] 4.5 Run formatting, all-target compilation, strict Clippy, and the full test suite; fix all failures.

## 5. Migration and delivery

- [x] 5.1 Replace Python/uv/justfile/Earthly sources and configuration with the verified Rust implementation and mise workflow.
- [x] 5.2 Update README examples, bindings, distribution decisions, and project conventions, preserving historical decisions.
- [x] 5.3 Synchronize the approved OpenSpec requirements and validate all specifications.
- [x] 5.4 Commit conventional changes, push the feature branch, and verify the committed artifact.

Archive this approved change after merge/deployment as a release follow-up; the current specifications are already synchronized.

## Verification

Pinned nightly local CI passed: rustfmt, all-target/all-feature compilation, Clippy with warnings denied, 45 nextest tests, one doctest, and nine installed-binary checks. The terminal suite sends UTF-8 characters, control keys, bracketed paste, mouse clicks, resize events, and SIGTERM through actual OS pseudo-terminals. Criterion ran against 1, 100, and 1,000 record fixtures. The committed Cargo package passed packaging and compilation with only the 16 intended source/metadata files; legacy ignored caches are excluded. The feature branch is pushed. Linux/macOS remote CI results are reported on the pull request.
