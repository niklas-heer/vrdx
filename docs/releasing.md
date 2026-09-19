# Releasing vrdx

Releases contain four native archives (macOS and Linux, ARM64 and x86-64), the
README, MIT license and `SHA256SUMS`. The executable includes its dashboard and
AI guide. We publish through GitHub Releases and `niklas-heer/tap/vrdx`.

1. Update `Cargo.toml` and `Cargo.lock` together. Review the README and AI guide.
2. Run `mise run ci-native` and `mise run ci`. Open a PR and wait for CI plus all
   four release-package checks before merging.
3. From the merged revision, create and push a matching annotated tag, for example
   `git tag -a v0.3.0 -m 'Release v0.3.0'` followed by `git push origin v0.3.0`.
4. The tag workflow reruns the quality gates, builds and tests each archive outside
   the checkout, checks the version and archive count, then publishes checksums
   and release notes. Inspect the workflow and release before announcing it.
5. The existing Homebrew tap checks hourly for new stable releases. Its
   `Update vrdx` workflow can also be dispatched manually. It generates a formula
   from release checksums, installs/tests it on all four supported platforms,
   then commits only that formula using the tap's own `GITHUB_TOKEN`.

No cross-repository token is needed. A failed release check prevents publication;
a failed Homebrew check leaves the previous formula available. Rerun a failed
workflow after fixing the underlying problem. Never replace a published tag or
archive: fix the issue in a new version. If publication itself was interrupted,
inspect the release and its assets before retrying.

Check installation with `brew install niklas-heer/tap/vrdx`, `vrdx --version`,
`vrdx guide --json`, and a temporary create/validate/fmt journey. Existing users
can run `brew update` and `brew upgrade niklas-heer/tap/vrdx`.
