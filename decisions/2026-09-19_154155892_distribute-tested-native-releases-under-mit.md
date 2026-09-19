+++
schema_version = 1
id = "01M2X59M9M407P4BN8MZHTXXZD"
title = "Distribute tested native releases under MIT"
date = "2026-09-19"
status = "accepted"
tags = ["distribution", "testing"]
supersedes = []
superseded_by = []
depends_on = []
related_to = ["01M2X35QR9ERE0JFE9TN01XPQX", "01M2X21HW4QXRHR3EPWP4TV7YS"]
+++
## Decision

Distribute the standalone Rust CLI under the MIT license, with versioned native
archives for Linux and macOS on x86-64 and ARM64. Build and exercise artifacts
before release, include the license and README, and publish SHA-256 checksums.
Use the existing 0.2.0 package version for the first public Rust release.

## Context

On 2026-09-19 Niklas asked to complete the outstanding delivery, licensing and
distribution work, supervise the PR until merge, and simulate real usage because
real-world adoption could not be obtained in this task. MIT follows his established
default license preference. The repository had no previous tags or releases.

The existing Dagger/Dang Linux pipeline and native macOS checks remain the build
boundary. Rust source installation remains available. Native downloads remove
the Rust-toolchain prerequisite for users without adding an installer service or
publishing to a package registry.

## Consequences

Users can inspect and verify a portable archive. Checksums detect corruption;
they do not provide code signing or notarization. Platform claims are limited to
the native runners used to exercise each artifact. The CLI replacement remains
a breaking change from the historical editor, with no automatic legacy import.

Deterministic scenarios test decision history, discovery, edits and recovery at
larger collection sizes. They provide repeatable evidence about modeled behavior,
not a claim of real adoption or universal retrieval quality. Semantic search,
web editing and hosted collaboration remain outside the accepted scope.
