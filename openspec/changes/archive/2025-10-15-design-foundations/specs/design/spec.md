## ADDED Requirements
### Requirement: CLI TUI Decision Management
The system SHALL provide a standalone terminal application that discovers Markdown decision records, parses structured decision content, and enables users to browse and edit those decisions without relying on external services.

#### Scenario: Launching the standalone TUI
- **GIVEN** a repository that contains Markdown files
- **WHEN** the user starts the application from the command line
- **THEN** the application SHALL scan the working directory for decision records and present them in the TUI

#### Scenario: Markdown-focused workflow
- **GIVEN** a Markdown file that contains decision entries between vrdx markers
- **WHEN** the user navigates to that file within the TUI
- **THEN** the application SHALL expose the decisions for viewing and editing inside the terminal

### Requirement: Layered Architecture Boundaries
The system SHALL maintain a layered architecture that separates CLI startup, Textual app orchestration, file discovery, decision parsing, state management, and persistence responsibilities.

#### Scenario: CLI startup initializes app state
- **GIVEN** the user invokes the CLI with a target directory
- **WHEN** the entrypoint resolves arguments and logging
- **THEN** it SHALL construct an application state object and launch the Textual interface

#### Scenario: Parser and persistence isolation
- **GIVEN** the application is parsing or rewriting decision content
- **WHEN** marker detection or serialization is required
- **THEN** those responsibilities SHALL be handled by dedicated parser and persistence modules, not by UI widgets

### Requirement: Pane-Oriented TUI Layout
The Textual interface SHALL render four primary panes within an 80×24 friendly layout: decisions list, decision editor, preview, and file list, with numeric shortcuts for focus.

#### Scenario: Pane layout at launch
- **GIVEN** the TUI has started
- **WHEN** the interface renders
- **THEN** it SHALL display the decisions list and file list in the left column, the editor in the center, and the preview on the right

#### Scenario: Numeric pane focus
- **GIVEN** the user presses a numeric key between 1 and 4
- **WHEN** the corresponding pane exists
- **THEN** the application SHALL shift focus to that pane and update the visual highlight

### Requirement: Interaction Model Parity
The system SHALL support lazygit-inspired keyboard interactions including `j`/`k` navigation, `space` to activate the focused item, `n` to start a new decision, `s` to save, `r` to refresh discovery, and `?` for help.

#### Scenario: Navigating with j and k
- **GIVEN** the decisions list pane is focused
- **WHEN** the user presses `j` or `k`
- **THEN** the selection SHALL move to the next or previous decision respectively

#### Scenario: Editing and saving a decision
- **GIVEN** a decision is selected
- **WHEN** the user presses `space` to edit and `s` to save
- **THEN** the editor SHALL toggle into edit mode, accept changes, and write the updated decision back through the persistence layer

### Requirement: Marker Block Canonicalization
The system SHALL treat `<!-- vrdx start -->` and `<!-- vrdx end -->` as the canonical decision block delimiters, inserting them when missing, validating their ordering, and ignoring inline-code examples wrapped in single backticks.

#### Scenario: Detecting canonical markers
- **GIVEN** a Markdown file with a properly ordered marker block
- **WHEN** the application scans the file
- **THEN** it SHALL recognize a single decision block bounded by the canonical delimiters

#### Scenario: Inserting scaffold when markers absent
- **GIVEN** a Markdown file without markers
- **WHEN** the user agrees to add decision management
- **THEN** the system SHALL append the canonical start marker, a blank line, and the end marker using the file’s newline convention

#### Scenario: Ignoring inline examples
- **GIVEN** documentation that references the markers using inline backticks
- **WHEN** the parser processes the file
- **THEN** the inline examples SHALL NOT create marker blocks or disrupt discovery

### Requirement: Decision Template Structure
New decisions SHALL use the template heading `### <ID> <Title>` followed by bullet-prefixed Status, Decision, Context, and Consequences fields, with default status set to `📝 Draft`.

#### Scenario: Creating a new decision
- **GIVEN** the user presses `n`
- **WHEN** the editor opens with a template
- **THEN** it SHALL pre-populate the heading, default status, and placeholder bullets for all required fields

#### Scenario: Maintaining descending ID order
- **GIVEN** multiple decisions exist in a block
- **WHEN** a new decision is saved
- **THEN** the persistence layer SHALL insert it at the top of the block so the highest ID appears first

### Requirement: Persistence Integrity
The system SHALL preserve non-marker content, newline styles, and existing decisions when rewriting marker blocks, while supporting reorder and delete operations initiated from the TUI.

#### Scenario: Saving with unchanged surroundings
- **GIVEN** a user edits a decision body
- **WHEN** the changes are saved
- **THEN** content outside the marker block SHALL remain untouched and the original newline convention SHALL be preserved

#### Scenario: Reordering decisions
- **GIVEN** the user moves a decision up or down
- **WHEN** the operation completes
- **THEN** the block serialization SHALL reflect the new order without duplicating or dropping decisions

### Requirement: Future Iteration Roadmap Reference
The specification SHALL document outstanding design milestones (status picker enhancements, Markdown preview rendering, in-app reordering polish, logging improvements, tooling updates, and CI workflows) so future proposals can reference and refine them.

#### Scenario: Referencing planned enhancements
- **GIVEN** a future proposal evaluates improvements to the TUI or tooling
- **WHEN** it reviews the archived design foundations
- **THEN** it SHALL find the enumerated milestones to guide scope decisions

#### Scenario: Maintaining authoritative baseline
- **GIVEN** maintainers need to confirm historical design intent
- **WHEN** they consult the archived spec
- **THEN** it SHALL reflect the original goals and architecture described in the legacy design document
