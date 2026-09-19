## ADDED Requirements

### Requirement: Read reasoning alongside connections
Selecting a graph decision SHALL immediately display its full Markdown body, title, lifecycle, date and source path in a reading pane without opening a modal. The graph SHALL remain available for navigation. Selecting another decision SHALL update the reader and its neighborhood together. The reader SHALL reuse safe Markdown rendering, expose invalid-collection warnings and adapt to narrow screens without hiding content behind an additional action.

#### Scenario: Follow a connection while reading
- **WHEN** a user selects a graph decision and then one of its neighbors
- **THEN** the visible full reasoning and graph neighborhood SHALL both belong to the newly selected decision
- **AND** the user SHALL be able to read the content and continue navigating without opening or dismissing a dialog
