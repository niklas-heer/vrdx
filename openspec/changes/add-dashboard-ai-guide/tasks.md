## Implementation

- [x] Add local HTTP dashboard backend and packaged browser assets.
- [x] Add AI guide, decision writing guidance and explained related-decision suggestions.
- [x] Create SVG identity and improve README/user documentation.
- [x] Verify CLI/HTTP workflows, browser interactions, native/container checks and packaged binary.
- [ ] Preserve accepted choices in decisions and open a PR targeting main.

## Verification

- Native macOS and Dagger/Dang Linux: formatting, all-target compilation, strict Clippy, 18 nextest tests, doctests and 16 installed-binary CLI/AI/HTTP journeys passed.
- Cargo package verification and cargo audit passed. The package reports the existing missing-license metadata warning; no registry release was requested.
- Native browser review covered desktop and narrow layouts, records/map/details, current-data polling, and suggestions. HTTP tests cover live edits, renames, malformed records, bundled routes, and local-only read access.
- OpenSpec strict validation and Markdown collection validation passed.
