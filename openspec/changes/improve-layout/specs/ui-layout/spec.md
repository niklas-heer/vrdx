## ADDED Requirements

### Requirement: Dynamic Text Area Sizing
The system SHALL automatically adjust the height of text area fields in the decision editor based on their content, providing a responsive and natural editing experience.

#### Scenario: Textarea grows with content
- **WHEN** a user types content into a decision field (Decision, Context, or Consequences)
- **THEN** the textarea height increases to accommodate the content
- **AND** the height remains within a reasonable maximum (approximately 15 lines)
- **AND** scrolling is enabled for content exceeding the maximum height

#### Scenario: Textarea shrinks when content is removed
- **WHEN** a user deletes content from a textarea
- **THEN** the textarea height decreases to fit the remaining content
- **AND** the height never goes below a minimum of 3 lines

#### Scenario: Dynamic sizing applies to all form fields
- **WHEN** editing a decision in the form editor
- **THEN** Title, Decision, Context, and Consequences fields all support dynamic sizing
- **AND** Status field maintains fixed height (dropdown widget)

### Requirement: Neon/Cyberpunk Color Palette
The UI SHALL implement a distinctive neon/cyberpunk color scheme with a very dark navy background (#0a0e27) and vibrant bright neon accents (magenta #FF00FF, cyan #00FFFF, lime #00FF00, purple #9945FF) for a striking, modern aesthetic.

#### Scenario: Neon colors render correctly across terminals
- **WHEN** the application starts on various terminal emulators
- **THEN** the background displays as very dark navy (#0a0e27)
- **AND** neon accent colors (magenta #FF00FF, cyan #00FFFF, lime #00FF00) display with full brightness and saturation
- **AND** text (#E0E0E0) displays with sufficient contrast against the dark background
- **AND** the palette works consistently on iTerm2, Terminal.app, Alacritty, and Linux terminals

#### Scenario: Visual hierarchy through neon color coding
- **WHEN** viewing the 4-pane layout
- **THEN** the editor pane shows magenta (#FF00FF) borders for primary visual emphasis
- **AND** the decisions/files lists show cyan (#00FFFF) accents for secondary importance
- **AND** the preview pane shows subtle neon borders
- **AND** visual hierarchy is clear through color prominence

#### Scenario: Focus states glow with magenta neon
- **WHEN** a user navigates to a form field or interactive element
- **THEN** the focused element shows a bright magenta (#FF00FF) border
- **AND** optionally displays a subtle purple (#9945FF) background glow
- **AND** hover states show cyan (#00FFFF) highlighting
- **AND** focus indicators are unmistakable and energetic

### Requirement: Polished Form Layout with Neon Styling
The form editor SHALL present decision fields with clear neon-colored labels, consistent spacing, and striking visual organization.

#### Scenario: Form fields have neon-colored labels
- **WHEN** the user enters edit mode for a decision
- **THEN** each form field (Title, Status, Decision, Context, Consequences) has a bold cyan (#00FFFF) label
- **AND** labels are positioned above their corresponding input fields
- **AND** labels stand out clearly against the dark background

#### Scenario: Neon focus indicators guide form navigation
- **WHEN** the user focuses on a form field
- **THEN** the field displays a bright magenta (#FF00FF) border
- **AND** optional subtle purple (#9945FF) background indicates focus state
- **AND** unfocused fields show minimal border styling
- **AND** the neon indicator is unmistakable and energetic

#### Scenario: Form buttons use semantic neon colors
- **WHEN** the user completes editing a decision
- **THEN** the Save button displays lime green (#00FF00) text or border (success color)
- **AND** the Cancel button displays orange (#FF6B35) text or border (alternative action)
- **AND** buttons show cyan (#00FFFF) highlighting on hover
- **AND** buttons are easily accessible via keyboard navigation

### Requirement: Neon Visual Feedback for All Interactions
The UI SHALL provide striking neon visual feedback for hover states, focus indicators, and selection states using bright cyan (#00FFFF), magenta (#FF00FF), and lime (#00FF00) colors.

#### Scenario: Hover states glow with neon cyan
- **WHEN** a user hovers over an interactive element (list item, button)
- **THEN** the element shows a cyan (#00FFFF) border or background change
- **AND** the change is consistent across all interactive elements
- **AND** the change is striking but not overwhelming

#### Scenario: Selection states highlight with magenta
- **WHEN** a user selects a decision or file from the list
- **THEN** the selected item displays a magenta (#FF00FF) background or border
- **AND** the highlight is distinct from the hover state (cyan)
- **AND** the highlight persists until another item is selected
- **AND** the neon magenta clearly indicates active selection

### Requirement: Refined Spacing with Neon Borders
The UI SHALL use consistent spacing and neon-colored borders to improve readability and visual balance.

#### Scenario: Neon borders define pane boundaries
- **WHEN** the application renders the main layout
- **THEN** each pane displays neon-colored borders (magenta #FF00FF for editor, cyan #00FFFF for lists)
- **AND** borders are crisp and clearly delineate pane areas
- **AND** consistent padding inside each pane maintains balance
- **AND** content is neither cramped nor excessively spaced

#### Scenario: Form sections are separated with neon styling
- **WHEN** the user views the decision form
- **THEN** each form section is separated by consistent vertical spacing
- **AND** neon accent colors (cyan labels, magenta focus) guide the eye through fields
- **AND** the overall form feels striking and professionally organized

### Requirement: Headers Positioned Inside Pane Containers
The system SHALL position pane headers and titles inside the pane content area rather than outside, creating an integrated, unified visual appearance.

#### Scenario: Headers are part of pane content
- **WHEN** viewing each pane (Decisions, Editor, Preview, Files)
- **THEN** the pane title/header is displayed inside the pane container
- **AND** the header appears at the top of the pane content
- **AND** the header is displayed in neon accent color (magenta #FF00FF for primary, cyan #00FFFF for secondary)
- **AND** the header is followed by a subtle separator line or spacing
- **AND** content flows below the internal header

#### Scenario: Integrated header appearance
- **WHEN** viewing the UI
- **THEN** headers are visually part of their pane (not floating above)
- **AND** the combined header + content creates a unified, polished look
- **AND** pane borders encompass both header and content
- **AND** this creates a more modern, integrated aesthetic

### Requirement: Rounded Corners on Panes and Interactive Elements
The system SHALL apply subtle rounded corners to all pane borders, form fields, buttons, and interactive elements for a polished, modern appearance.

#### Scenario: Pane borders have rounded corners
- **WHEN** viewing the main layout with panes
- **THEN** each pane border (Decisions, Editor, Preview, Files) displays rounded corners
- **AND** the rounded corners are subtle (approximately 1-2 character radius)
- **AND** corners remain sharp enough to maintain clarity and readability
- **AND** rounded corners match the neon aesthetic

#### Scenario: Form fields and buttons have rounded corners
- **WHEN** viewing the decision editor form
- **THEN** all form field borders (Title, Decision, Context, Consequences) have rounded corners
- **AND** Status dropdown border has rounded corners
- **AND** Save and Cancel buttons have rounded corners
- **AND** input field borders maintain rounded corners even in focus/hover states

#### Scenario: Rounded corners maintain visual consistency
- **WHEN** viewing any interactive element (list items, form fields, buttons, containers)
- **THEN** all elements consistently use the same rounded corner style
- **AND** corners are proportional to element size
- **AND** rounded corners do not compromise readability or visual clarity
- **AND** the soft edges create a modern, polished appearance

## MODIFIED Requirements

### Requirement: Form-Based Decision Editor with Neon Styling
The form-based editor for creating and editing decisions SHALL support dynamic textarea sizing, neon-colored labels and focus indicators, and striking visual styling.

#### Scenario: Editor displays all fields with neon labels, dynamic sizing, and rounded corners
- **WHEN** opening the form editor
- **THEN** Title, Status, Decision, Context, and Consequences fields are all visible with cyan (#00FFFF) labels
- **AND** textarea fields (Title, Decision, Context, Consequences) expand/contract based on content (min 3, max 15 lines)
- **AND** all form field borders have rounded corners for polished appearance
- **AND** Status dropdown maintains fixed height with rounded corners
- **AND** the neon label color clearly identifies each field

#### Scenario: Form shows glowing magenta focus state
- **WHEN** user navigates to a form field
- **THEN** the focused field displays a bright magenta (#FF00FF) border
- **AND** optional purple (#9945FF) background provides subtle glow effect
- **AND** the neon focus state is unmistakable and energetic
- **AND** unfocused fields show minimal styling

#### Scenario: Form buttons use semantic neon colors with rounded corners
- **WHEN** user views form action buttons
- **THEN** Save button uses lime green (#00FF00) or magenta (#FF00FF) color (positive action) with rounded corners
- **AND** Cancel button uses orange (#FF6B35) or cyan (#00FFFF) color (alternative action) with rounded corners
- **AND** buttons show clear hover states with contrasting neon colors
- **AND** rounded button corners maintain a polished, modern appearance
- **AND** neon coloring makes button purpose immediately clear

#### Scenario: Form remains functional with neon styling on small terminals
- **WHEN** terminal is sized to 80×24 or smaller
- **THEN** form remains readable with neon colors maintaining clarity
- **AND** fields are appropriately sized for the available space
- **AND** scrolling is available if content exceeds viewport
- **AND** all neon colors render correctly even on limited terminal color support

### Requirement: Neon/Cyberpunk UI Layout and Styling
The UI layout and styling SHALL use a striking neon/cyberpunk color palette with dark background, vibrant neon accents, and clear visual hierarchy.

#### Scenario: Neon palette creates striking visual impact
- **WHEN** viewing any part of the UI
- **THEN** the background displays as very dark navy (#0a0e27)
- **AND** accent colors use bright neon (magenta #FF00FF, cyan #00FFFF, lime #00FF00, purple #9945FF)
- **AND** primary text (#E0E0E0) maintains excellent contrast for readability
- **AND** neon colors create a distinctive, memorable aesthetic

#### Scenario: 2-column layout with vertical editor/preview stacking
- **WHEN** opening the application
- **THEN** left column (20-25% width) displays Decisions and Files lists with cyan (#00FFFF) borders
- **AND** right column (75-80% width) displays Editor pane (top) with magenta (#FF00FF) borders
- **AND** Preview pane displays below Editor pane (bottom) with cyan (#00FFFF) borders
- **AND** Editor and Preview panes are stacked vertically in the right column
- **AND** status bar displays purple (#9945FF) accents for distinction
- **AND** all rounded corners are consistent across all panes
- **AND** horizontal space for form fields is maximized

#### Scenario: Lists have neon hover and selection states with rounded corners
- **WHEN** viewing decision or file lists in left column
- **THEN** list items show cyan (#00FFFF) hover highlighting with rounded corners
- **AND** selected items show magenta (#FF00FF) background or border highlighting with rounded corners
- **AND** file items without markers are visually de-emphasized with muted text (#999999)
- **AND** left column pane borders use cyan (#00FFFF) for secondary prominence with rounded corners

#### Scenario: Editor pane prominence with vertical preview below
- **WHEN** viewing the right column with Editor and Preview panes
- **THEN** Editor pane is positioned at top with magenta (#FF00FF) borders
- **AND** Preview pane is positioned below Editor with cyan (#00FFFF) borders
- **AND** Editor pane receives more vertical space than Preview pane
- **AND** Preview scrolls independently if content exceeds viewport

#### Scenario: Typography with neon color coding
- **WHEN** viewing any pane
- **THEN** Editor pane header is bold magenta (#FF00FF) and positioned inside the pane
- **AND** Preview pane header is bold cyan (#00FFFF) and positioned inside the pane
- **AND** field labels are bold with cyan (#00FFFF) color
- **AND** content text is regular weight with light gray (#E0E0E0) color
- **AND** hints and muted information use dark gray (#999999) color
- **AND** semantic colors (green #00FF00 for success, orange #FF6B35 for warning) convey meaning

#### Scenario: Buttons are smaller and refined
- **WHEN** viewing form action buttons in Editor pane
- **THEN** Save button uses lime green (#00FF00) with minimal padding and rounded corners
- **AND** Cancel button uses orange (#FF6B35) with minimal padding and rounded corners
- **AND** buttons are compact and do not dominate the form visually
- **AND** button sizing is consistent across all action buttons

### Requirement: Smaller Button Sizing
The system SHALL implement smaller button dimensions for a refined, polished appearance consistent with modern TUI design.

#### Scenario: Buttons use minimal dimensions
- **WHEN** viewing form action buttons (Save, Cancel, Change Status)
- **THEN** buttons use minimal padding (0-1 unit horizontal)
- **AND** button height matches single-line content (minimal vertical padding)
- **AND** text remains readable but compact
- **AND** buttons maintain rounded corners consistent with design
- **AND** all buttons use the same compact sizing

#### Scenario: Buttons are inline or closely spaced
- **WHEN** viewing the form layout
- **THEN** buttons are positioned inline or with minimal spacing between them
- **AND** buttons do not dominate the form visually
- **AND** buttons remain easily accessible and clickable
- **AND** smaller sizing reduces visual clutter while maintaining functionality