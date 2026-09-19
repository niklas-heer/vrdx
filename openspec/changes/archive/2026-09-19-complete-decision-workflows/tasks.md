## Completion
- [x] Implement conflict-aware source-preserving delete and reorder operations.
- [x] Implement safe three-way merge of independent record-field and surrounding-source edits.
- [x] Implement headless JSON commands and subprocess integration tests.
- [x] Implement templates, durable relationship references, and Git history inspection.
- [x] Expose new workflows in the TUI and test real keystrokes and persisted outcomes.
- [x] Implement and verify applicable Premise contracts and checks.
- [x] Restore/synchronize specifications and document every shipped command and limitation.
- [x] Run formatting, compilation, strict Clippy, tests, doctests, package and installed-binary checks, and remote CI.
- [x] Commit and push the complete implementation for review without deferred feature tasks.

## Verification

Implementation commit e19f8cc passed the complete local mise CI task: formatting, all-target/all-feature compilation, strict Clippy, 84 nextest tests, one doctest, and 26 installed-binary checks (17 terminal and 9 headless CLI). All 85 tests also passed from the committed 20-file Cargo package. Criterion measured about 4.4 ms for 1,000 records after eliminating duplicate marker/metadata scanning. Strict OpenSpec validation passed. Linux and macOS CI both passed in run 34662843925. PR 16 contains the implementation; no feature tasks remain deferred.
