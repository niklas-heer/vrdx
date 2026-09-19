## Implementation
- [x] Review and validate the inline-reader specification.
- [x] Show safe full Markdown content alongside the selected graph neighborhood, with responsive layout.
- [x] Verify selection, connected-record navigation, themes and full content in the browser; run native checks.
- [x] Update documentation and synchronize the specification.

## Verification

- Safari verified full reasoning visible beside the graph, neighbor selection updating both panes, light and dark themes, and the stacked reader at a narrow effective viewport using browser zoom. Normal zoom and dark appearance were restored.
- JavaScript syntax, release build and native quality gates passed: formatting, compilation, strict Clippy, 21 tests, doctest invocation (none present) and 19 installed-binary journeys.
- Context7 and CORE Memory tools remain unavailable; existing renderer, repository specifications and direct browser checks were used.
