+++
schema_version = 1
id = "01JNGADM503XR7Q2WKH9TB5FMV"
title = "Store sessions in Redis"
date = "2025-03-04"
status = "superseded"
tags = ["sessions", "infrastructure"]
+++

## Decision

Keep login sessions in a shared Redis instance keyed by session ID.

## Why

The API runs on several replicas and sticky routing was unreliable. Redis was already deployed for rate limiting, so it added no new service.

## Consequences

- Any replica can serve any request.
- Redis becomes a hard dependency for every authenticated call.
- Session data must be expired explicitly; forgetting a TTL leaks memory.
