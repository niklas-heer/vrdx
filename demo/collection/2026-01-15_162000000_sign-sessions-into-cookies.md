+++
schema_version = 1
id = "01KF179SW0Q7ZH4NDM8XK2TR6V"
title = "Sign sessions into cookies"
date = "2026-01-15"
status = "accepted"
tags = ["sessions", "security"]
supersedes = ["01JNGADM503XR7Q2WKH9TB5FMV"]
+++

## Decision

Carry the session in a signed, HttpOnly cookie and drop the Redis session store.

## Why

Two outages traced back to Redis evicting sessions under memory pressure. The session only holds a user ID and expiry, which fits in a cookie and needs no server-side state.

## Consequences

- Login no longer depends on Redis; the API serves requests during a cache outage.
- Revoking one session early requires a short denylist checked on sensitive routes.
- The signing key must rotate without logging everyone out.
