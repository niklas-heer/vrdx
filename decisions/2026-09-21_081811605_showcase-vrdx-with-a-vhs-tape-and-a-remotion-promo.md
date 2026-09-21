+++
schema_version = 1
id = "01M31GPJ0NNFB1PCJZQCRMFZVR"
title = "Showcase vrdx with a VHS tape and a Remotion promo"
date = "2026-09-21"
status = "proposed"
tags = ["documentation", "tooling", "demo"]
+++

## Decision

Keep demo sources in demo/: a small fictional decision collection, a VHS tape that renders the README recording, and a self-contained Remotion project for a promotional video.

## Why

A terminal recording shows developers the real CLI in under a minute, and VHS keeps it reproducible as code instead of a hand-recorded screen capture. Remotion lets the promo reuse the dashboard palette and real command output as React scenes. Both stay optional host tooling so the Rust CLI, tests and CI remain free of Node.

## Consequences

- assets/demo.gif is committed and embedded in the README; rerender it with mise run demo when shown output changes.
- VHS, ttyd, ffmpeg, bat and jq (for the tape) and Node.js (for the promo) are host prerequisites, not project-pinned tools.
- Rendered promo videos are not committed; demo/promo has its own npm lockfile and .gitignore.
- The demo collection uses fixed IDs so the tape can reference prefixes deterministically.
