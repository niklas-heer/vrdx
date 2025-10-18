# Implementation Tasks

## 1. Color Scheme and Palette Update

- [ ] 1.1 Define CSS color variables in `vrdx/ui/styles.tcss`
  - Define primary background color (#0f172a)
  - Define surface color (#1e293b)
  - Define panel color (#0f172a)
  - Define primary accent color (#3b82f6)
  - Define highlight accent color (#06b6d4)
  - Define text color (#f1f5f9)
  - Define text-muted color (#94a3b8)
  - Define success color (#10b981)
  - Define warning color (#f59e0b)
  - Define error color (#ef4444)
  - Define boost color (#1e293b for status bar)

- [ ] 1.2 Update Screen element styling
  - Apply background color variable
  - Apply text color variable
  - Ensure base layout properties are correct

- [ ] 1.3 Update left column styling (#left-column)
  - Apply updated border color from palette
  - Adjust padding if needed
  - Ensure visual separation is clear

- [ ] 1.4 Update decision list and file list styling
  - Apply background color from surface palette
  - Update border colors to primary accent
  - Update hover state background to highlight accent
  - Update focused selection state styling

- [ ] 1.5 Update editor and preview pane styling
  - Apply panel background color
  - Update border colors to primary accent
  - Adjust padding and margins for consistency
  - Improve visual hierarchy through color

- [ ] 1.6 Update status bar styling
  - Apply boost color for background
  - Ensure text contrast is sufficient
  - Update border color

- [ ] 1.7 Add focus state styling
  - Create `.focused` or similar class for focused elements
  - Apply accent color border
  - Add subtle background highlight for focused fields

## 2. Dynamic Textarea Sizing Implementation

- [ ] 2.1 Create utility function for height calculation
  - Implement `calculate_textarea_height()` function in `vrdx/ui/forms.py`
  - Function should accept TextArea widget and content
  - Return calculated height based on line count
  - Define MIN_HEIGHT = 3 lines, MAX_HEIGHT = 15 lines

- [ ] 2.2 Add height tracking to FormBasedDecisionEditor
  - Add reactive attributes for each textarea field height
  - Initialize with MIN_HEIGHT
  - Create watch methods to update heights

- [ ] 2.3 Implement dynamic height for Title field
  - Calculate height based on title content
  - Set min-height to 3 lines, max-height to 5 lines
  - Watch for content changes and update

- [ ] 2.4 Implement dynamic height for Decision field
  - Calculate height based on decision content
  - Set min-height to 3 lines, max-height to 15 lines
  - Watch for content changes and update

- [ ] 2.5 Implement dynamic height for Context field
  - Calculate height based on context content
  - Set min-height to 3 lines, max-height to 15 lines
  - Watch for content changes and update

- [ ] 2.6 Implement dynamic height for Consequences field
  - Calculate height based on consequences content
  - Set min-height to 3 lines, max-height to 15 lines
  - Watch for content changes and update

- [ ] 2.7 Handle Status field (fixed height)
  - Ensure Status dropdown maintains consistent fixed height
  - No dynamic sizing needed for this field

- [ ] 2.8 Test dynamic sizing with various content
  - Test with short content (1-2 lines)
  - Test with medium content (5-10 lines)
  - Test with long content (20+ lines)
  - Test rapid typing to ensure smooth updates
  - Test delete/backspace to ensure shrinking works

## 3. Form Layout and Structure Refinement

- [ ] 3.1 Reorganize FormBasedDecisionEditor layout
  - Ensure fields are arranged vertically
  - Order: Title → Status → Decision → Context → Consequences → Buttons
  - Add section separators if needed

- [ ] 3.2 Add clear field labels
  - Create Label widgets for each form field
  - Style labels with bold font and consistent appearance
  - Position labels above their input fields

- [ ] 3.3 Improve spacing between form sections
  - Define consistent spacing (2-3 units between sections)
  - Add padding around form container
  - Ensure visual separation is clear

- [ ] 3.4 Style the Status dropdown field
  - Ensure dropdown has consistent appearance with other fields
  - Add label above dropdown
  - Style selected state clearly
  - Add focus indicator

- [ ] 3.5 Style and position Save/Cancel buttons
  - Ensure buttons are clearly visible at form bottom
  - Apply consistent styling to both buttons
  - Add focus indicators for keyboard navigation
  - Test keyboard accessibility (Tab navigation)

- [ ] 3.6 Update CSS for form elements in styles.tcss
  - Style `.form-section` containers
  - Style `.form-label` elements
  - Style button styling rules
  - Ensure all form elements use palette colors

- [ ] 3.7 Improve form container scrolling
  - Test scrolling behavior on small terminals
  - Ensure form remains usable when content exceeds viewport
  - Test keyboard navigation with scrolling

## 4. Pane Styling and Visual Hierarchy

- [ ] 4.1 Update decision list pane styling
  - Ensure border and background use palette colors
  - Apply consistent padding
  - Update list item hover/focus states
  - Test visual hierarchy

- [ ] 4.2 Update file list pane styling
  - Ensure border and background use palette colors
  - Apply consistent padding
  - Update file item styling
  - De-emphasize files without markers visually

- [ ] 4.3 Update editor pane styling
  - Apply panel background color
  - Ensure editor is visually prominent
  - Update border styling
  - Test form visibility and usability

- [ ] 4.4 Update preview pane styling
  - Apply panel background color with slightly different tone
  - Update border styling
  - Ensure preview is readable but secondary to editor
  - Test markdown rendering with new colors

- [ ] 4.5 Update pane borders and separators
  - Ensure vertical separator between left and center columns is clear
  - Ensure horizontal separators use consistent styling
  - Apply accent color to all borders consistently

- [ ] 4.6 Test visual hierarchy across different content
  - Test with single decision
  - Test with multiple decisions
  - Test with long text in editor
  - Verify that editor remains focal point

## 5. Enhancement and Polish

- [ ] 5.1 Add hover state styling to list items
  - Implement consistent hover backgrounds
  - Ensure hover state is not distracting
  - Test across mouse and keyboard navigation

- [ ] 5.2 Improve focus indicators
  - Ensure all interactive elements show clear focus
  - Use consistent accent color for focus states
  - Test keyboard navigation through all panes

- [ ] 5.3 Update header/title styling
  - Apply bold styling to pane titles
  - Apply accent color where appropriate
  - Ensure titles are clearly distinguishable

- [ ] 5.4 Refine typography throughout UI
  - Verify text-style properties (bold, italic, etc.)
  - Ensure consistent font weight usage
  - Check text color contrast

- [ ] 5.5 Polish status bar styling
  - Ensure all status text is readable
  - Update mode indicator styling
  - Verify help text styling

## 6. Testing and Validation

- [ ] 6.1 Test on 80×24 terminal
  - Verify layout displays correctly
  - Ensure no content is cut off
  - Test form scrolling if needed
  - Verify readability on small screen

- [ ] 6.2 Test on larger terminals (120×40, etc.)
  - Verify layout scales gracefully
  - Ensure spacing is proportional
  - Test pane resizing behavior

- [ ] 6.3 Test color scheme across terminal emulators
  - Test on iTerm2 (macOS)
  - Test on Terminal.app (macOS)
  - Test on common Linux terminals (Alacritty, GNOME Terminal, etc.)
  - Verify color accuracy and contrast

- [ ] 6.4 Test dynamic textarea sizing
  - Test with short, medium, and long content
  - Verify heights stay within min/max bounds
  - Test rapid typing and deletion
  - Verify no layout thrashing

- [ ] 6.5 Test form usability
  - Test creating new decision with form
  - Test editing existing decision
  - Test keyboard navigation (Tab, Shift+Tab)
  - Test Ctrl+S save and Esc cancel
  - Verify focus management

- [ ] 6.6 Test list interactions
  - Test navigating decisions list with j/k and arrows
  - Test navigating files list
  - Test hover and focus states
  - Verify highlight colors are clear

- [ ] 6.7 Test preview rendering
  - Verify markdown renders correctly with new colors
  - Test with various decision content
  - Ensure readability is maintained

- [ ] 6.8 Verify no regressions
  - Test all existing functionality still works
  - Test mode switching (NORMAL, EDIT, INSERT)
  - Test help overlay with new colors
  - Test status indicators

## 7. Documentation and Cleanup

- [ ] 7.1 Add inline code comments
  - Document color palette definitions
  - Explain dynamic sizing logic
  - Add comments to complex CSS rules

- [ ] 7.2 Update any relevant documentation
  - Check if TEXTUAL_REFERENCE.md needs updates
  - Update any internal design docs if needed

- [ ] 7.3 Verify code style and formatting
  - Run ruff formatting on modified Python files
  - Check CSS formatting consistency
  - Ensure no linting errors

- [ ] 7.4 Remove any debug code
  - Clean up any temporary test files
  - Remove debugging statements
  - Verify clean implementation

- [ ] 7.5 Final visual inspection
  - Take screenshots of improved layout
  - Verify color scheme is cohesive
  - Ensure overall appearance is polished
  - Compare against design mockups

## 8. Headers Positioned Inside Panes

- [ ] 8.1 Reorganize pane structure to include headers inside containers
  - Refactor Decisions pane to display header inside border
  - Refactor Files pane to display header inside border
  - Refactor Editor pane to display header inside border
  - Refactor Preview pane to display header inside border

- [ ] 8.2 Style pane headers with neon colors
  - Use magenta (#FF00FF) for primary pane headers
  - Use cyan (#00FFFF) for secondary pane headers
  - Apply bold text styling

- [ ] 8.3 Add separator between header and content
  - Add subtle line separator after header
  - Or use spacing to separate header from content
  - Ensure visual distinction is clear

- [ ] 8.4 Test header positioning on various terminal sizes
  - Verify headers display correctly on 80×24 terminal
  - Test on larger terminals (120×40, etc.)
  - Ensure headers remain properly positioned

- [ ] 8.5 Verify headers integrate with pane content
  - Ensure content flows naturally below headers
  - Verify headers are visually part of pane
  - Check overall integrated appearance

## 9. Rounded Corners on Panes and Elements

- [ ] 9.1 Add rounded corner styling to CSS framework
  - Determine appropriate corner radius (1-2 char width)
  - Define CSS rules for rounded corners
  - Ensure compatibility with terminal rendering

- [ ] 9.2 Apply rounded corners to pane borders
  - Add rounded corners to Decisions pane border
  - Add rounded corners to Files pane border
  - Add rounded corners to Editor pane border
  - Add rounded corners to Preview pane border

- [ ] 9.3 Apply rounded corners to form field borders
  - Add rounded corners to Title field border
  - Add rounded corners to Decision field border
  - Add rounded corners to Context field border
  - Add rounded corners to Consequences field border
  - Add rounded corners to Status dropdown border

- [ ] 9.4 Apply rounded corners to buttons
  - Add rounded corners to Save button
  - Add rounded corners to Cancel button
  - Add rounded corners to Change Status button
  - Add rounded corners to other action buttons

- [ ] 9.5 Apply rounded corners to list items
  - Add rounded corners to decision list items
  - Add rounded corners to file list items
  - Ensure consistency across all list elements

- [ ] 9.6 Test rounded corners maintain clarity
  - Verify rounded corners don't reduce readability
  - Test on various terminal emulators
  - Ensure corners remain distinct and clear

- [ ] 9.7 Test rounded corners with interactive states
  - Verify rounded corners with focus state
  - Verify rounded corners with hover state
  - Verify rounded corners with selected state
  - Ensure consistency across all states

- [ ] 9.8 Verify corner consistency across elements
  - Ensure all pane borders use same corner radius
  - Ensure all form fields use same corner radius
  - Ensure all buttons use same corner radius
  - Maintain visual consistency throughout UI

## 10. Final Polish and Validation

- [ ] 10.1 Verify headers-inside-panes appearance
  - Review pane layouts with integrated headers
  - Confirm headers are properly styled with neon colors
  - Check separator between header and content

- [ ] 10.2 Verify rounded corners appearance
  - Review all rounded corner implementations
  - Confirm corners are subtle and polished
  - Check consistency across all elements

- [ ] 10.3 Combined visual inspection
  - Headers + Dynamic sizing + Neon colors + Rounded corners
  - Verify all elements work together cohesively
  - Ensure overall design matches Posting aesthetic

- [ ] 10.4 Test on all target terminals
  - iTerm2 (macOS)
  - Terminal.app (macOS)
  - Alacritty
  - Linux terminals
  - Verify rounded corners and headers render correctly