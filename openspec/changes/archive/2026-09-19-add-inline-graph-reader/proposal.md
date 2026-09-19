# Read decision content alongside connections

## Why
Niklas requested visible decision contents after trying the graph-first dashboard.
Requiring another click into a modal separates the reasoning from its connections.

## What Changes
- Selecting a graph node immediately shows its full Markdown reasoning in an adjacent reading pane, with lifecycle, date, source path and named relationships.
- Keep the graph visible and interactive; selecting a neighbor updates both views.
- Stack the reading pane below a compact graph on narrower screens. Retain the expanded record drawer for metadata and suggestions.
- Reuse the existing safe Markdown renderer and theme tokens; no dependencies or API changes.

## Authorization
The user's follow-up explicitly requests visible content and authorizes this scoped implementation on the existing dashboard feature branch.
