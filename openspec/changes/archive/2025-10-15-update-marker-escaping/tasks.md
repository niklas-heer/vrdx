## 1. Specification
- [x] 1.1 Confirm whether an existing capability covers marker parsing; if not, identify or create the appropriate spec scope.
- [x] 1.2 Author spec delta(s) documenting that inline-code wrapped marker strings are ignored, including at least one scenario for parser behavior.
- [ ] 1.3 Run strict validation for the change (`openspec validate update-marker-escaping --strict`) and resolve any findings.

## 2. Implementation
- [x] 2.1 Update marker detection to skip `<!-- vrdx start -->` / `<!-- vrdx end -->` sequences that appear inside single backtick inline code spans.
- [x] 2.2 Ensure persistence logic continues to emit canonical markers and does not modify inline-code examples.
- [x] 2.3 Extend unit tests (e.g., `tests/unit/test_markers.py`, discovery coverage) to cover the new inline-code ignoring behavior and guard against regressions.
- [x] 2.4 Document the inline-marker guidance in relevant docs so authors know backticked markers are safe.

## 3. Quality Assurance
- [x] 3.1 Run the full unit test suite (`uv run pytest -v`) and ensure all tests pass.
- [x] 3.2 Execute lint and formatting checks (`uv run ruff check .`) to verify code quality.
- [x] 3.3 Perform a manual run of vrdx against documentation containing inline-code markers to confirm decision indexing remains stable.
<!-- vrdx start -->

<!-- vrdx end -->
