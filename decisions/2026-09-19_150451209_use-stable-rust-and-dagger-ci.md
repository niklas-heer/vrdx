+++
schema_version = 1
id = "01M2X35QR9ERE0JFE9TN01XPQX"
title = "Use stable Rust and Dagger CI"
date = "2026-09-19"
status = "accepted"
tags = ["tooling", "ci"]
supersedes = []
superseded_by = []
depends_on = []
related_to = ["01M2X21HW4QXRHR3EPWP4TV7YS"]
+++

## Decision

Use stable Rust 1.97.1 for the CLI, with mise for tools/tasks, cargo-nextest for tests, and Dagger 0.21.9 with the Dang SDK for Linux CI. Preserve a native macOS job. Keep runtime application logic in Rust and container orchestration in a small Dang module.

## Context

On 2026-09-19 Niklas asked to align vrdx with his development preferences, remove remaining Python, and explicitly confirmed Dagger + Dang, mise and nextest. The CLI does not need nightly Rust; compilation and strict Clippy pass on the selected stable release. This replaces the earlier nightly toolchain choice while retaining the accepted CLI architecture.

## Consequences

Rust pins agree across Cargo.toml, rust-toolchain.toml and mise.toml. Cargo and mise lockfiles remain checked in. CI uses a digest-pinned image, filtered source inputs, project-scoped caches and only the required Rust/nextest tools. Dagger CLI and engine pins agree. No Cloud credentials are needed. Colima with its Docker runtime was used for local Linux verification; native macOS checks remain necessary.

The local Python virtual environment, caches and Textual checkout were removed. The historical Python vrdx tool was uninstalled and replaced by the native Cargo binary. Historical OpenSpec and read-only reference documents remain as records, not active build inputs. No Python package manager or interpreter is needed to build or run the CLI.

Optional Bacon, watchexec, cargo-binstall, cargo-generate and cargo-seek remain developer tools. No new application crate, lint framework, release/deployment stage or speculative infrastructure was added.

## Verification

Both native macOS arm64 and Dagger Linux arm64 passed formatting, compilation, strict Clippy, all 12 tests, the doctest command and 10 installed-binary CLI journeys. A malformed src/main.rs in an isolated copy made the same Dagger pipeline fail with exit 1. actionlint validated the workflow; the hub tooling audit reported no findings. GitHub-hosted x86 Linux and Apple container-engine execution were not exercised locally.

## References

- [Project setup and commands](../README.md)
- [Dagger/Dang pipeline](../.dagger/main.dang)
- [CI workflow](../.github/workflows/ci.yml)
