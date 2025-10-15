# decision-markers Specification

## Purpose
TBD - created by archiving change update-marker-escaping. Update Purpose after archive.
## Requirements
### Requirement: Ignore Inline Marker Examples
The marker detection subsystem SHALL treat the literal strings `<!-- vrdx start -->` and `<!-- vrdx end -->` as plain text whenever they are surrounded by a single backtick inline-code span, ensuring they do not open or close a decision block.

#### Scenario: Inline-code start marker is skipped
- **GIVEN** a Markdown document containing the text `` `<!-- vrdx start -->` ``
- **WHEN** the discovery and parsing workflow scans the document for decision markers
- **THEN** the inline-code sequence MUST NOT be treated as the beginning of a marker block
- **AND** the parser MUST continue searching for the next non-inline marker boundary

#### Scenario: Inline-code end marker is skipped
- **GIVEN** a Markdown document containing the text `` `<!-- vrdx end -->` ``
- **WHEN** the discovery and parsing workflow scans the document for decision markers
- **THEN** the inline-code sequence MUST NOT be treated as the end of a marker block
- **AND** any active decision block MUST remain open until a non-inline closing marker is encountered

#### Scenario: Real marker blocks remain honored
- **GIVEN** a Markdown document that includes both inline-code examples of the markers and a genuine block with `<!-- vrdx start -->` and `<!-- vrdx end -->` on their own lines
- **WHEN** the parser processes the file
- **THEN** it MUST ignore the inline-code examples
- **AND** it MUST still extract the decisions contained within the genuine marker block
<!-- vrdx start -->

<!-- vrdx end -->
