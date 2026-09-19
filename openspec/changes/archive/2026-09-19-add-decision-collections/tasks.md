## Implementation

- [x] Implement metadata parsing, creation, stable identifiers and deterministic graph validation.
- [x] Implement human/JSON CLI queries, relationship traversal and AI context.
- [x] Add subprocess workflows and malformed/cyclic/rename fixtures.
- [x] Document format, commands, output contract and limitations.
- [x] Run OpenSpec validation, formatting, compilation, Clippy, nextest, doctests, installed checks and hub audit.

OpenSpec cannot archive complete capability removal (it rejects empty rebuilt specs). Applied the validated deltas to current specs manually, then archived with `--skip-specs`; current specs are validated separately.
