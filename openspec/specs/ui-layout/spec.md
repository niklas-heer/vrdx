# ui-layout Specification

## Purpose
TBD - created by archiving change update-pane-focus-order. Update Purpose after archive.
## Requirements
### Requirement: Prioritize Files with Decision Blocks
The files pane SHALL list Markdown files containing `<!-- vrdx start -->` … `<!-- vrdx end -->` marker blocks before any other Markdown files and automatically select the first such file on application startup.

#### Scenario: Files with marker blocks are sorted and focused
- **GIVEN** the repository contains multiple Markdown files, some with decision marker blocks and some without
- **WHEN** the TUI loads the files pane
- **THEN** all files that contain decision marker blocks MUST appear before files without marker blocks
- **AND** the first file in the list that contains a marker block MUST be selected by default

### Requirement: Visualize Files Without Decision Blocks
Markdown files that do not contain decision marker blocks SHALL remain focusable but MUST present a visually muted (grayed out) style to distinguish them from files containing decisions.

#### Scenario: Files without markers render in muted style
- **GIVEN** a Markdown file lacking decision marker blocks appears in the files pane
- **WHEN** the files pane renders the list
- **THEN** the entry representing that file MUST use the designated muted styling token(s)
- **AND** the entry MUST remain selectable so users can insert a new decision block if desired

### Requirement: Update Pane Numbering and Shortcuts
Pane numbering, on-screen hints, and keyboard shortcuts SHALL map as: `1` Decisions, `2` Files, `3` Editor, `4` Preview, and all bindings MUST focus the corresponding panes.

#### Scenario: Pane hints and bindings match new order
- **GIVEN** a user views the pane hint display or invokes the numeric shortcuts
- **WHEN** the user presses `2`, `3`, or `4`
- **THEN** the Files, Editor, and Preview panes respectively MUST receive focus
- **AND** the pane hint text MUST advertise the same numeric-to-pane mapping
