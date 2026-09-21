# Contributing to vrdx

The README lists the prerequisites, the mise tasks and the Colima route for
`mise run ci`. This page covers what those tasks exercise and how to work with
the larger fixtures.

## Test suites

- `tests/cli.rs`, `tests/ai.rs`, `tests/dashboard.rs` and `tests/init.rs` run the
  installed or freshly built binary in temporary projects and check its JSON.
- `tests/authoring.rs` covers the JSON input contract, editor success and
  recovery, formatting round trips, rejected input and a relocated executable
  with no runtime tools on `PATH`.
- `tests/documentation.rs` checks the README example, the toolchain pins and
  that the checked-in skill files match the copies embedded in the binary.
  After editing `.agents/skills/vrdx/`, run `mise run run -- init` in the
  repository root so the copies come from the checkout, not an older binary.
- `mise run bench` reports warm-filesystem subprocess latency for 1,000 records
  and a 250-record replacement chain. It does not impose a CI time threshold.

`scripts/verify-install.sh` installs the binary outside the checkout and reruns
the subprocess suites against it through `VRDX_TEST_BINARY`.

## Simulation

`tests/simulation.rs` runs a 221-record, 12-topic CLI/HTTP journey and three
replayable seeds with 48 edits each. It checks renames, status and tag changes,
relationships, invalid edits, repairs, retrieval and preservation of source
files, and also runs against installed and extracted release binaries. It
measures modeled workflow behavior, not real adoption or semantic retrieval
quality.

To keep the narrative fixture for browser inspection, run

```sh
VRDX_SIMULATION_KEEP_DIR=/tmp/vrdx-example mise exec -- cargo test --test simulation mixed_project_history
vrdx --dir /tmp/vrdx-example dashboard
```

with a new or empty target directory.

## Packaging locally

Release PR checks build native archives for all four supported targets. To
package a local build:

```sh
mise exec -- cargo build --locked --release --target TARGET
mise run package-release -- TARGET
```

`TARGET` is your host's Rust target from the README table. Packaging extracts
the archive and exercises its binary outside the checkout. A matching `vVERSION`
tag publishes the verified archives and checksums only after the full quality
workflow and all four platform jobs pass; see [Releasing vrdx](releasing.md).

## Agent skill

`.agents/skills/vrdx/SKILL.md` and its `references/onboarding.md` are embedded
with `include_str!` and installed by `vrdx init`. Keep the skill short, keep the
trigger in its `description`, and defer format rules to `vrdx guide`. Changing
the skill is a release: users receive it by upgrading and rerunning `init`.
