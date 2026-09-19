## ADDED Requirements

### Requirement: Graph-first exploration
The dashboard SHALL open in graph view and retain a records view. Pointer hover and keyboard focus SHALL distinguish the highlighted decision, its direct neighbors and connecting edges. Selecting a decision SHALL show its immediate neighborhood with named relationship directions and access to full reasoning. Context outside current filters SHALL be labeled, and clearing selection SHALL restore the filtered overview. Isolated decisions SHALL have an explicit empty connection state.

#### Scenario: Explore a filtered decision
- **WHEN** a user selects a decision whose direct neighbor does not match the current filters
- **THEN** the neighborhood SHALL include that neighbor as context and label it as outside the filters
- **AND** the user SHALL be able to navigate the relationship, read full reasoning and return to the filtered overview

### Requirement: Dashboard appearance
The dashboard SHALL support light, dark and system appearance, use system preference initially, and remember explicit choices when browser storage is available. Controls, records, graph connections, validation findings and full record details SHALL remain readable and keyboard accessible in both themes.

#### Scenario: Remember dark appearance
- **WHEN** a user chooses dark appearance and reloads the page with browser storage available
- **THEN** the dashboard SHALL retain dark appearance
- **AND** choosing system appearance SHALL follow subsequent operating-system appearance changes
