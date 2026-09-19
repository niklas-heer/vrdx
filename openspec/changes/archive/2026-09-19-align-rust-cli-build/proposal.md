## Why

Niklas requested a focused CLI project aligned with his Rust and build preferences, with no Python remnants. He explicitly confirmed Dagger + Dang, mise and nextest.

## What Changes

- Pin stable Rust 1.97.1 across Cargo, rust-toolchain and mise; retain strict Rust gates and editor tools.
- Add Dagger 0.21.9 with Dang, a digest-pinned container, filtered inputs and project caches. Use the existing native gates in the container and retain native macOS CI.
- Remove local Python environments, bytecode caches and Textual remnants; remove obsolete ignore rules. Historical reference documents stay read-only.
- Document self-contained setup and contributor guidance; install the Rust CLI and retire the obsolete Python vrdx tool.

## Impact

Affected: implementation specification, tool manifests/lock, CI workflow, Dang module, documentation and setup decision. No application dependency or behavior changes. Existing uncommitted AGENTS.md content is preserved and excluded from the commit.
