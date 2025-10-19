# Layout Improvement Proposal: Neon/Cyberpunk Design System

## Why

The current vrdx TUI layout is functional but lacks visual polish and distinctive character. The editing pane has fixed-size text areas regardless of content, which wastes space or forces unnecessary scrolling. The color scheme feels basic and lacks personality. There's an opportunity to create a bold, modern, visually striking interface inspired by cyberpunk/neon aesthetics—combining the dark, eye-friendly backgrounds of contemporary tools with vibrant, high-contrast neon colors that make the UI feel cutting-edge and energetic while remaining highly readable.

Inspired by modern dark IDE themes and tools like the Neon colorscheme found in Postman, we can create a distinctive visual identity that sets vrdx apart while improving usability through better spacing, typography, and dynamic content sizing.

## What Changes

- **Dynamic text box sizing**: Text areas in the editing pane now scale to fit their content, making efficient use of space and eliminating unnecessary scrolling
- **Neon/Cyberpunk color scheme**: Introduce a bold, high-contrast palette with bright magenta, cyan, lime, and purple accents against a very dark navy background—creating a striking, modern aesthetic
- **Improved spacing and padding**: Better visual separation between sections, panes, and form fields with refined proportions
- **Enhanced typography**: Better font weights, text styles, and alignment to guide user attention through strategic color coding
- **Polished borders and separators**: More sophisticated visual boundaries between UI components using neon accent colors
- **Better visual hierarchy**: Clearer distinction between primary (editor), secondary (decisions/files lists), and tertiary (preview) panes through color and brightness
- **Form field improvements**: Labeled form sections with glowing focus indicators and clear visual structure using neon accents

## Impact

- **Affected specs**: UI rendering and styling (`ui-layout` capability)
- **Affected code**: 
  - `vrdx/ui/styles.tcss` - Complete redesign with neon palette and glowing effects
  - `vrdx/ui/forms.py` - Dynamic sizing logic for form fields
  - `vrdx/ui/app.py` - Layout adjustments for improved spacing
  - `vrdx/ui/panes.py` - Enhanced pane structure with neon styling

## Color Palette

### Core Colors
- **Background**: `#0a0e27` (Very dark navy/black) - Main screen and pane backgrounds
- **Text**: `#E0E0E0` (Light gray/white) - Primary readable text
- **Primary Accent**: `#FF00FF` (Bright magenta/pink) - Main interactive focus, borders, highlights
- **Secondary Accent**: `#00FFFF` (Cyan/turquoise) - Alternative focus, hover states
- **Tertiary Accent**: `#00FF00` (Lime green) - Success states, important indicators
- **Highlight**: `#9945FF` (Purple/violet) - Special emphasis, status indicators
- **Keywords GET**: `#B084CC` (Purple) - HTTP method styling
- **Keywords POST/DEL**: `#00FF88` / `#FF6B35` (Cyan/Orange) - HTTP method styling

## Inspiration

This design draws from:
- **Postman's Neon Colorscheme** - High-contrast cyberpunk aesthetic
- **Modern Dark IDE Themes** - Like Dracula, Nord, and other cyberpunk-inspired themes
- **Cyberpunk Aesthetic** - Vibrant neon against dark backgrounds, visually striking yet comfortable
- **Posting HTTP Client** - Professional Textual TUI application with polished design

The neon palette provides:
- ✨ Distinctive, memorable visual identity
- 👁️ High contrast for excellent readability
- ⚡ Modern, energetic aesthetic
- 🎮 Cyberpunk/hacker tool vibe
- 💜 Professional yet striking appearance