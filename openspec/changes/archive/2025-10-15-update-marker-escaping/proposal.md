## Why
Documentation authors frequently need to mention the literal marker strings `<!-- vrdx start -->` and `<!-- vrdx end -->` when explaining how vrdx works. Today, the discovery and parsing layers interpret those strings as real marker boundaries even when they are wrapped in Markdown inline code fences, which breaks indexing and forces contributors to escape the markers manually. We need a first-class rule that treats inline-code markers as inert so documentation stays readable without harming decision parsing.

## What Changes
- Update the marker detection logic so that `<!-- vrdx start -->` and `<!-- vrdx end -->` enclosed by single backticks are ignored during discovery and parsing.
- Add explicit test coverage verifying that inline-code markers are skipped while genuine marker blocks remain discoverable.
- Document the new guidance in the parser and distribution docs so authors know inline-code markers are safe.
- Ensure persistence continues to serialize canonical markers without altering inline-code examples.

## Impact
- Affected specs: decision parsing, discovery, and documentation authoring guidelines (new or modified requirements to capture inline-code handling).
- Affected code: `vrdx/parser/markers.py`, related parsing utilities, plus accompanying tests under `tests/unit/test_markers.py` (and any discovery tests that inspect marker detection).
<!-- vrdx start -->

<!-- vrdx end -->
