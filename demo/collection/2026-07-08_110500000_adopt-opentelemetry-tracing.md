+++
schema_version = 1
id = "01KX0PE2V02RJM7WCX5KNQ9H3T"
title = "Adopt OpenTelemetry tracing"
date = "2026-07-08"
status = "proposed"
tags = ["observability", "orders"]
related_to = ["01JY1NHEY08PZC4TQN6HWJ2RKY"]
+++

## Decision

Emit OpenTelemetry traces from the API and the order database layer.

## Why

Slow checkouts are debugged today by reading logs from three services side by side. A trace shows the same request across all of them, including each Postgres query.

## Consequences

- Every incoming request carries a trace ID into logs and database calls.
- A collector runs beside the API; it is one more process to operate.
- Sampling must stay low in production to keep storage costs flat.
