## Implementation
- [x] Review the proposal and measure the current release binary.
- [x] Add concise JSON creation, safe editor drafting and copyable prompt guidance.
- [x] Add conservative formatting/check mode and actionable diagnostics.
- [x] Add subprocess, round-trip, failure/recovery and standalone-artifact tests; fix measured performance issues.
- [x] Update docs and decisions, verify native/container checks and record measured limits.

## Verification

- Native macOS ARM64 and Linux ARM64 through Dagger: formatting, type checks, strict Clippy, 29 tests and installed release-binary journeys (27 tests) passed. The opt-in latency test is excluded from CI timing gates.
- Packaged macOS ARM64 archive extracted and verified outside the checkout: all 27 binary journeys passed.
- Eight authoring tests cover guide-driven JSON creation, invalid and bounded input, quoted editor arguments, failed draft recovery, formatting idempotence and byte preservation, repairable diagnostics and relocation without runtime tools. Existing seeded simulation and HTTP journeys also pass.
- Release executable: approximately 2.8 MB on macOS ARM64; only system libiconv/libSystem dynamic dependencies. Guidance and dashboard assets remain embedded.
- `mise run bench`: 1,000 records, 250-record replacement chain, warm filesystem, subprocess startup included, three warmups and 20 samples per command. Local measurements are workload examples, not universal performance guarantees.

| Command | Previous median | New median | New p95 |
| --- | --- | --- | --- |
| guide | 1.67 ms | 1.79 ms | 1.91 ms |
| list | 20.92 ms | 20.42 ms | 22.32 ms |
| validate | 18.37 ms | 19.61 ms | 33.08 ms |
| context | 33.67 ms | 33.56 ms | 35.08 ms |
| suggest | 25.38 ms | 26.61 ms | 27.44 ms |
| chain | 20.44 ms | 18.59 ms | 19.25 ms |

No material regression or demonstrated need for a cache/index emerged, so graph storage and query algorithms remain unchanged. The JSON example contract is exercised by subprocess tests; live external-agent quality is not claimed. Formatting is atomic per file, not across the collection, and intentionally does not reflow arbitrary Markdown.
