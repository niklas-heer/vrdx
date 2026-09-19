## MODIFIED Requirements

### Requirement: Ignore Inline Marker Examples
The marker detection subsystem SHALL treat literal `<!-- vrdx start -->` and `<!-- vrdx end -->` strings as plain text when they appear inside Markdown inline-code spans or fenced code blocks. Code examples SHALL NOT open, close, or duplicate a decision marker block. Decision parsing SHALL likewise exclude fenced code contents when recognizing record headings and canonical field labels.

#### Scenario: Inline-code start marker is skipped
- **GIVEN** a Markdown document containing the text `` `<!-- vrdx start -->` ``
- **WHEN** discovery scans the document
- **THEN** the inline-code sequence MUST NOT be treated as the beginning of a marker block
- **AND** the parser MUST continue searching for a non-code marker boundary

#### Scenario: Inline-code end marker is skipped
- **GIVEN** a Markdown document containing the text `` `<!-- vrdx end -->` ``
- **WHEN** discovery scans the document
- **THEN** the inline-code sequence MUST NOT close an active marker block

#### Scenario: Real marker blocks remain honored
- **GIVEN** a document includes code examples and genuine start and end markers on their own lines outside code
- **WHEN** the document is parsed
- **THEN** the real block MUST be recognized and its decisions extracted
- **AND** the examples MUST remain unchanged

#### Scenario: Fenced examples are ignored
- **GIVEN** apparent markers appear inside a backtick or tilde fenced code block, including fences longer than three characters
- **WHEN** the document is scanned
- **THEN** those markers MUST NOT affect marker detection or duplicate-marker diagnostics

#### Scenario: Longer inline delimiters are respected
- **GIVEN** a marker appears inside a valid inline-code span using multiple backticks or alongside other inline-code text
- **WHEN** the document is scanned
- **THEN** it MUST be treated as example text rather than a marker boundary

#### Scenario: Invalid real markers remain errors
- **GIVEN** real markers outside code are missing a counterpart, duplicated, or out of order
- **WHEN** the file is loaded or saved
- **THEN** the system SHALL report a marker error and SHALL NOT treat the file as an uninitialized document
