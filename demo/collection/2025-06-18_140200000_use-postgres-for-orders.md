+++
schema_version = 1
id = "01JY1NHEY08PZC4TQN6HWJ2RKY"
title = "Use Postgres for orders"
date = "2025-06-18"
status = "accepted"
tags = ["storage", "orders"]
+++

## Decision

Store orders, line items and payments in one Postgres database with transactional writes.

## Why

An order and its payment must change together. We compared a document store and an event log; both pushed consistency into application code that a small team would get wrong.

## Consequences

- One transaction covers order creation and payment capture.
- Reporting queries run against read replicas, not the primary.
- Schema changes need migrations reviewed with the code that uses them.
