# Graph-first dashboard

## Why
Niklas requested a visual default, visible connections when highlighting decisions, and dark mode on 2026-09-19. The current map hides relationships behind a record drawer and starts in the records view.

## What Changes
- Open in the graph view; retain the records view.
- Preview direct connections on pointer hover or keyboard focus. Select a decision to explore its immediate neighborhood, with labeled incoming and outgoing relationships and explicit context outside active filters.
- Provide system, light and dark appearance choices, remembering explicit preferences locally when storage is available.
- Keep the bundled, dependency-free, read-only design and safe text rendering.

## Authorization
The user's direct request authorizes this design and implementation. Claude Code is consulted for design advice only; it does not edit files.

## Impact
Browser assets, dashboard documentation and the decision-collections specification. No new service, dependency, metadata format or CLI behavior.
