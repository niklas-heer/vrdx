## ADDED Requirements

### Requirement: Local visual decision dashboard
The CLI SHALL offer a read-only loopback web dashboard generated from the current Markdown collection, with search, lifecycle/tag filtering, relationship graph navigation and full record details. Browser content SHALL treat record data as untrusted text and expose collection findings clearly.

#### Scenario: Refresh after a Markdown change
- **WHEN** a user edits a decision in the selected directory and refreshes the dashboard
- **THEN** the dashboard SHALL show the updated graph without a database or persistent index

#### Scenario: Invalid collection
- **WHEN** a collection contains an invalid record or relationship
- **THEN** the dashboard SHALL display diagnostics and mark the graph invalid rather than implying current policy is authoritative

### Requirement: Self-describing AI workflow
The CLI SHALL provide human and versioned JSON guidance without requiring a collection, including commands, metadata, examples, lifecycle semantics and decision writing style.

#### Scenario: Agent onboarding
- **WHEN** an AI calls guide before any decisions directory exists
- **THEN** it SHALL receive sufficient instructions to discover, propose, write and validate a decision without inventing acceptance or evidence

### Requirement: Explained relationship suggestions
The CLI SHALL suggest potentially related decisions deterministically with observable reasons, lifecycle and source identity. Suggestions SHALL exclude the input record and existing direct relationships, and SHALL NOT mutate Markdown or create graph edges.

#### Scenario: Topical overlap
- **WHEN** two unlinked records share meaningful tags or words
- **THEN** suggest SHALL expose the overlap and score as an advisory candidate rather than a confirmed relationship
