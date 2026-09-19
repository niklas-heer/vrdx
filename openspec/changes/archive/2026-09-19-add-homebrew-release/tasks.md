## Implementation
- [x] Review release state and validate proposal.
- [x] Add tag quality gates, version bump and polished installation/release docs.
- [x] Add Homebrew formula, update automation and native installation tests.
- [x] Verify and integrate release changes; publish v0.3.0 and verify assets.
- [x] Verify Homebrew installation and publish the tap; record final outcomes.

## Verification

- Published v0.3.0 from merged PR #19 at dd1fff69090000ecf962f88cc6bc1651bc0088e7. [Tag workflow](https://github.com/niklas-heer/vrdx/actions/runs/35461887391) passed full Linux/Dagger and native macOS quality gates plus all four archive checks before publication.
- [Release](https://github.com/niklas-heer/vrdx/releases/tag/v0.3.0) contains four native archives and SHA256SUMS; release notes and README explain installation and concise authoring.
- [Homebrew PR #10](https://github.com/niklas-heer/homebrew-tap/pull/10) merged after native macOS ARM64/x86-64 and Linux ARM64/x86-64 installation tests passed. Generator tests cover incomplete assets, invalid checksums, duplicates and unpublished versions; Homebrew style passed.
- Tap automation uses its repository token, preserves mandatory PRs, and opens verified update PRs. Actions PR creation was enabled; default workflow permissions remain read-only. No approval or merge automation was added.
- An ARM Linux packaged-test race exposed an inherited writable executable handle. Serializing the cargo-test harness fixed ETXTBSY; parallel nextest isolation remains unchanged.
- Local Homebrew downloaded the verified archive but refused installation because macOS 27 requires Xcode 27 while Xcode.app is 26.6. CLT 27 is installed; Homebrew independently checks the installed app. No Xcode configuration was changed. Hosted installation checks on the documented platforms passed.
- Independently downloaded the published Apple Silicon archive, verified SHA256SUMS and ran its executable: vrdx 0.3.0.
