## Why

The CLI needs a browsable view of the current decision graph and a self-describing AI interface. Niklas requested a local web dashboard, a logo, a polished README, writing guidance and discovery of potentially related decisions, with a PR targeting main.

## What Changes

- Add `dashboard`, a read-only localhost web application backed by fresh Markdown graph rebuilds, with search, lifecycle/tag filtering, graph exploration and readable record details.
- Add `guide` for collection-independent human/JSON onboarding, schema, examples and writing style.
- Add `suggest ID` for deterministic, explained relationship candidates; no automatic edits.
- Introduce a small SVG visual identity and reorganize user documentation.

## Impact

Affected: decision-collections, CLI, dashboard module/assets, AI guidance module, tests, packaging and Dagger compile-time inputs. Existing Markdown and JSON interfaces remain compatible. One small synchronous HTTP dependency serves fixed routes on loopback; no Node runtime, database, remote service or web editor. The user's explicit request authorizes implementation and the pull request; unrelated AGENTS.md work remains excluded.
