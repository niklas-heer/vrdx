+++
schema_version = 1
id = "01M2X45M37XSCG0YP4396R3TQS"
title = "Browse decisions locally and explain the AI workflow"
date = "2026-09-19"
status = "accepted"
tags = ["architecture", "ai", "dashboard"]
supersedes = []
superseded_by = []
depends_on = []
related_to = ["01M2X21HW4QXRHR3EPWP4TV7YS"]
+++

## Decision

Add a read-only local web dashboard to the Rust CLI. Bundle its HTML, CSS, JavaScript and SVG identity into the executable, bind to loopback, and rebuild its data from Markdown on each API request. Provide `guide` for self-contained AI onboarding and writing conventions, and `suggest ID` for explained possible relationships based on shared tags and words.

## Context

On 2026-09-19 Niklas explicitly requested a dashboard of current decisions, an AI command explaining tool use and decision writing, and help surfacing possibly related records. The existing accepted CLI architecture keeps Markdown authoritative and allows a web consumer. The local, read-only design and deterministic lexical ranking are implementation choices made under that authorization and the requirement to avoid unnecessary infrastructure.

A separate hosted application or model-backed search would add deployment, credentials, persistent state or runtime dependencies. A small synchronous HTTP server and browser assets meet the current browsing requirement. AI tools can use the existing versioned JSON boundary without an MCP server or provider-specific integration.

## Consequences

People can explore lifecycle states, topics, explicit connections, reasoning and validation findings visually. Files remain portable; edits appear on refresh and no database is created. The dashboard cannot change records, serve arbitrary files, or accept connections from other machines. It runs only while its CLI command is active.

The guide carries a record contract, examples, writing style, evidence rules and review workflow. Suggestions expose shared tags and terms, stable ranking and historical status. They are advisory and never establish approval or write relationships. Lexical matching does not understand synonyms or intent; callers must read the full reasoning and choose links deliberately.

The browser renders a safe Markdown subset, with raw HTML displayed as text. Each refresh reads files independently, so concurrent multi-file edits may temporarily show validation findings. Larger collections may eventually justify a more scalable layout or search, but no persistent index is introduced now.

## Dashboard presentation — 2026-09-19

Niklas subsequently requested the graph as the default view, visible connections
when highlighting a decision, and dark mode. Keep Records as an alternative.
Hover and keyboard focus preview direct connections; selecting a node reveals
its neighborhood and named relationship directions without hiding the map behind
the reasoning drawer. Neighbors outside active filters are explicitly labeled as
context. Appearance follows the system by default, with locally remembered light
and dark choices. These presentation choices preserve the read-only architecture
and require no additional runtime dependency.

After trying this view, Niklas also requested visible decision contents. Selecting
a node therefore shows its full reasoning alongside the graph (below it on narrow
screens), while the expanded drawer remains available for metadata and suggestions.
