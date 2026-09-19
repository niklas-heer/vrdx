+++
schema_version = 1
id = "01M2XE18DTT6BDKCVVPZ9R2SC7"
title = "Keep decision writing small"
date = "2026-09-19"
status = "accepted"
tags = ["authoring", "ai", "tooling"]
related_to = ["01M2X21HW4QXRHR3EPWP4TV7YS", "01M2X45M37XSCG0YP4396R3TQS"]
+++

## Decision

Keep new records brief, with a memorable title, the choice, its main reason and concrete benefits and costs.

## Why

Niklas requested less ceremony and developer-friendly human and AI authoring on 2026-09-19. Under that delegated design scope, use JSON creation, explicit editor drafting, copyable prompts and actionable validation hints.

## Consequences

- Guide authors toward 3–7 word titles and under 150 words; these are defaults, not validation gates.
- Generate IDs, dates and formatted metadata so authors focus on the decision.
- Keep guidance and the dashboard inside one native executable; no AI service is required.
- Explicit formatting preserves comments, metadata values and exact Markdown bodies. Existing narrative history remains intact.
