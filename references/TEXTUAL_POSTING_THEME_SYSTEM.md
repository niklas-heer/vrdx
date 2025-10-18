# Posting's Complete Color Theme System

A detailed technical guide to Posting's SCSS color system, theme variables, and how they're applied throughout the application.

## Table of Contents

1. [Theme Color Hierarchy](#theme-color-hierarchy)
2. [SCSS Color Variables Reference](#scss-color-variables-reference)
3. [Color Application Patterns](#color-application-patterns)
4. [Component-Specific Styling](#component-specific-styling)
5. [Status & State Colors](#status--state-colors)
6. [Compact Mode Theming](#compact-mode-theming)
7. [Complete Implementation Map](#complete-implementation-map)
8. [Creating Themes for Posting](#creating-themes-for-posting)

---

## Theme Color Hierarchy

### Base Theme Colors (from themes.py)

```
PRIMARY COLORS:
├── primary         - Main accent color (CTA buttons, selection)
├── secondary       - Secondary accent (alternative interactions)
└── accent          - Emphasis color (borders, highlights)

BACKGROUND COLORS:
├── background      - Main app background
├── surface         - Widget/panel backgrounds
│   ├── surface-darken-1   - Darker variant
│   └── surface-lighten-1  - Lighter variant
└── panel           - Alternative panel color

STATUS COLORS:
├── success         - Positive/complete (green)
├── warning         - Caution/pending (yellow/orange)
├── error           - Negative/failure (red)
└── *-muted         - Muted variants (soft backgrounds)

TEXT COLORS:
├── text            - Primary text
├── text-primary    - Primary text variant
├── text-secondary  - Secondary text
├── text-accent     - Accent-colored text
├── text-success    - Success text
├── text-warning    - Warning text
├── text-error      - Error text
├── text-muted      - Muted/dim text
└── text-disabled   - Disabled state text

FOREGROUND COLORS:
├── foreground      - Foreground color
└── foreground-muted - Muted foreground

SPECIAL COLORS:
├── block-cursor-background   - Selection/cursor background
├── block-cursor-foreground   - Selection/cursor text
└── block-cursor-text-style   - Selection/cursor text style
```

---

## SCSS Color Variables Reference

**File**: `src/posting/posting.scss` (899 lines)

### Complete Color Variable Usage Map

```scss
// PRIMARY & ACCENT
$primary              // Lines: 523, 654
$primary-muted        // Lines: 409, 654
$primary-darken-1     // Line: 532

$secondary            // HTTP method colors

$accent               // Lines: 4, 65, 123, 137, 198, 245, 319, 391, 417, 
                      // 471, 660, 764, 780, 823, 838, 866
$accent-muted         // Lines: 208, 471, 498, 704, 823, 838

// BACKGROUND & SURFACE
$background           // Lines: 213, 233
$surface              // Lines: 63, 75, 100, 199, 327, 438, 451, 653, 667, 
                      // 806
$surface-darken-1     // Lines: 81, 85, 90, 94, 407
$surface-lighten-1    // Lines: 238, 551, 599, 736

// TEXT COLORS
$text                 // Lines: 239, 274, 330, 449, 451, 455, 459, 463, 504, 
                      // 507, 524, 550, 693
$text-primary         // Lines: 274, 411
$text-secondary       // (HTTP method colors)
$text-accent          // Lines: 207, 472, 489, 703, 823
$text-success         // Lines: 184, 325, 337, 422
$text-warning         // Lines: 188, 426, 460, 711
$text-error           // Lines: 191, 430, 456, 731
$text-muted           // Lines: 179, 202, 280, 360, 447, 464, 550, 573, 620, 
                      // 636, 860

// STATUS COLORS
$success              // Lines: 452
$success-muted        // Lines: 185, 423
$warning              // Lines: 460
$warning-muted        // Lines: 189, 427, 712
$error                // Lines: 430, 456, 731
$error-muted          // Lines: 192, 431, 712

// FOREGROUND
$foreground           // Lines: 251, 564, 568
$foreground-muted     // Line: 851

// SPECIAL
$block-cursor-background  // Lines: 330, 684, 792, 795
$block-cursor-foreground  // Lines: 50, 792
$block-cursor-text-style  // Line: 49
```

---

## Color Application Patterns

### 1. Borders with Color & Opacity

```scss
/* Standard section border */
.section {
  border: round $accent 40%;           // Rounded border, 40% opacity
  border-title-color: $text-accent 50%;
  border-title-align: right;
  
  &:focus-within {
    border: round $accent 100%;        // Full opacity when focused
    border-title-color: $foreground;
    border-title-style: b;             // Bold title
  }
}

/* Specific borders */
border-left: vkey $surface-darken-1;   // Vertical key border
border-left: outer $surface-lighten-1; // Outer border
border-left: wide $accent;             // Wide left border
border: solid $accent 40%;             // Solid border with opacity
```

### 2. Background with Opacity

```scss
/* Surface backgrounds at different opacities */
background: $surface 75%;              // 75% opacity
background: $surface 50%;              // 50% opacity
background: $surface 25%;              // 25% opacity
background: $surface 10%;              // 10% opacity
background: transparent;               // Transparent

/* Muted backgrounds for status */
background: $success-muted;            // Success background
background: $warning-muted;            // Warning background
background: $error-muted;              // Error background
background: $accent-muted;             // Accent background
```

### 3. Text Colors

```scss
/* Primary text */
color: $text;                          // Main text
color: $text-primary;                  // Primary variant
color: $text-accent;                   // Accent text

/* Status text */
color: $text-success;                  // Success message
color: $text-warning;                  // Warning message
color: $text-error;                    // Error message
color: $text-muted;                    // Disabled/muted

/* Opacity variants */
color: $text 50%;                      // Semi-transparent
color: $foreground 80%;                // Foreground 80% opacity
color: $text-muted 30%;                // Very dim
```

### 4. Interactive Elements (Hover, Focus)

```scss
/* Hover effects */
&:hover {
  background-tint: $text-accent 10%;   // Tint with text-accent
  text-style: b;                       // Make bold
}

/* Focus effects */
&:focus {
  border: solid $accent;               // Add border
  background: $block-cursor-background;
  color: $block-cursor-foreground;
}

/* Disabled effects */
&:disabled {
  opacity: 40%;                        // Fade out
  color: $text-muted;                  // Use muted text
}
```

---

## Component-Specific Styling

### AppHeader

**File**: `src/posting/posting.scss` (lines 273-281)

```scss
AppHeader {
  color: $text-primary;                // Primary text color
  padding: 1 3;
  height: auto;
  & > #app-user-host {
    dock: right;
    color: $text-muted;                // Muted for secondary info
  }
}
```

### SendRequestButton (Primary Action)

**File**: `src/posting/posting.scss` (lines 469-477)

```scss
SendRequestButton {
  min-width: 8;
  background: $accent-muted;           // Muted accent for prominence
  color: $text-accent;                 // Accent text
  text-style: b;                       // Bold
  &:hover {
    background-tint: $text-accent 10%; // Tint on hover
  }
}
```

**Design Decision**: Uses muted accent instead of full accent to be distinct but not aggressive.

### KeyValueEditor

**File**: `src/posting/posting.scss` (lines 479-555)

```scss
KeyValueEditor {
  KeyValueInput {
    & Button {
      background: $primary;            // Primary color
      color: $text;                    // Main text
      
      &:hover {
        background: $primary-darken-1; // Darker on hover
        text-style: b;
      }
    }
    
    #add-button {
      background: $accent;             // Accent for add
    }
    
    &.edit-mode {
      background: $accent-muted;       // Muted accent when editing
      Input {
        background: $accent 10%;       // Very faint accent
      }
    }
  }
}
```

### ResponseArea

**File**: `src/posting/posting.scss` (lines 178-195)

```scss
ResponseArea {
  border-subtitle-color: $text-muted;
  
  &.success .border-title-status {
    color: $text-success;              // Success green text
    background: $success-muted;        // Success green background
  }
  
  &.warning .border-title-status {
    color: $text-warning;              // Warning orange text
    background: $warning-muted;        // Warning orange background
  }
  
  &.error .border-title-status {
    color: $text-error;                // Error red text
    background: $error-muted;          // Error red background
  }
}
```

### UrlBar Status Display

**File**: `src/posting/posting.scss` (lines 416-466)

```scss
& #response-status-code {
  width: 5;
  height: 1;
  display: none;
  
  &.-success {
    color: $text-success;
    background: $success-muted;
  }
  
  &.-warning {
    color: $text-warning;
    background: $warning-muted;
  }
  
  &.-error {
    color: $text-error;
    background: $error-muted;
  }
}

/* Status markers */
& .complete-marker {
  color: $success;                     // Green for complete
  background: $surface;
}

& .failed-marker {
  color: $error;                       // Red for failed
  background: $surface;
}

& .started-marker {
  color: $warning;                     // Orange for started
  background: $surface;
}

& .not-started-marker {
  color: $text-muted 30%;              // Very dim when not started
  background: $surface;
}
```

### DataTable Styling

**File**: `src/posting/posting.scss` (lines 323-342)

```scss
PostingDataTable {
  & > .datatable--header {
    color: $text-success;              // Green headers
    background: $surface;
  }
  
  & > .datatable--header-cursor {
    color: $text;
    background: $block-cursor-background;
  }
  
  &:blur {
    & > .datatable--cursor {
      background: transparent;         // Hide cursor when not focused
    }
    & > .datatable--header-cursor {
      color: $text-success;            // Still show header in green
      background: $surface;
    }
  }
}
```

### Input Fields

**File**: `src/posting/posting.scss` (lines 375-385, 725-741)

```scss
Input {
  border: none;
  width: 1fr;
  
  &:focus {
    border: none;
    padding: 0 1;
    border-left: outer $surface-lighten-1;  // Light border when focused
  }
  
  &.-invalid {
    border-left: outer $error;         // Red border for errors
  }
}
```

### CommandPalette

**File**: `src/posting/posting.scss` (lines 753-784)

```scss
CommandPalette {
  background: black 33%;               // Dark semi-transparent
  
  & #--input {
    border: none;
    border-left: wide $accent;         // Accent left border
  }
}
```

### TextAreaFooter (Mode & Status Labels)

**File**: `src/posting/posting.scss` (lines 697-722)

```scss
#mode-label {
  &.visual-mode {
    color: $text-accent;               // Accent text
    background: $accent-muted;         // Muted accent background
  }
}

#rw-label {
  color: $text-warning;                // Warning text
  background: $warning-muted;          // Warning background
  
  &:disabled {
    opacity: 30%;                      // Very faint when disabled
  }
}
```

### Jump Mode UI

**File**: `src/posting/posting.scss` (lines 820-852)

```scss
.textual-jump-label {
  color: $text-accent;                 // Accent text
  background: $accent-muted;           // Muted accent background
  text-style: bold;
}

#textual-jump-info {
  background: $accent-muted;
  color: $text-accent;
  hatch: right $accent 30%;            // 30% opacity hatch pattern
}
```

### CollectionBrowser

**File**: `src/posting/posting.scss` (lines 557-577, 854-871)

```scss
CollectionBrowser {
  height: 1fr;
  dock: left;
  width: auto;
  
  & Tree {
    color: $foreground 80%;            // 80% opacity foreground
    background: transparent;
    
    &:focus {
      color: $foreground;              // Full opacity when focused
    }
  }
  
  & RequestPreview {
    color: $text-muted;                // Muted text for preview
    border-top: solid $accent 40%;     // Accent border
  }
}
```

---

## Status & State Colors

### Complete Status Color System

```scss
/* Success States */
$text-success          // Green text
$success               // Green status marker
$success-muted         // Light green background

/* Warning States */
$text-warning          // Orange/yellow text
$warning               // Orange/yellow status marker
$warning-muted         // Light orange/yellow background

/* Error States */
$text-error            // Red text
$error                 // Red status marker
$error-muted           // Light red background

/* Muted/Inactive States */
$text-muted            // Dim gray text
$text-disabled         // Disabled state text
$foreground-muted      // Muted foreground
```

### HTTP Response Status Styling

```scss
&.-success {              // 2xx responses
  color: $text-success;
  background: $success-muted;
}

&.-warning {              // 3xx responses
  color: $text-warning;
  background: $warning-muted;
}

&.-error {                // 4xx/5xx responses
  color: $text-error;
  background: $error-muted;
}
```

---

## Compact Mode Theming

**File**: `src/posting/posting.scss` (lines 17-174)

Compact mode reduces visual clutter while maintaining color scheme:

```scss
Posting {
  &.-compact {
    /* Remove borders */
    & .section {
      border: none;                    // No borders in compact mode
    }
    
    /* Reduce padding */
    & AppHeader {
      padding: 0 1;                    // Minimal padding
    }
    
    /* Compact tabs */
    & Tabs {
      height: 1;                       // Single line height
      & Underline {
        display: none;
      }
    }
    
    /* Background opacity adjustments */
    & CollectionBrowser {
      background: $surface 50%;        // 50% opacity in compact
    }
    
    & RequestEditor {
      background: $surface 25%;        // 25% opacity in compact
    }
    
    & ResponseArea {
      background: $surface 10%;        // 10% opacity in compact
    }
    
    /* Maintain color scheme */
    & CommandPalette {
      border-left: wide $accent;       // Still use accent color
    }
  }
}
```

**Key Design**: Compact mode removes visual elements but maintains color hierarchy.

---

## Complete Implementation Map

### By Color Purpose

**Primary Actions**
- SendRequestButton: `background: $accent-muted`
- Method selector: `background: $primary-muted`
- Add buttons: `background: $primary`

**Secondary Actions**
- Cancel buttons: `background: $surface`
- Toggle buttons: `background: $surface`

**Status Indicators**
- Success: `color: $text-success`, `background: $success-muted`
- Warning: `color: $text-warning`, `background: $warning-muted`
- Error: `color: $text-error`, `background: $error-muted`

**Borders**
- Main panes: `border: round $accent 40%`
- Focus state: `border: round $accent 100%`
- Error input: `border-left: outer $error`

**Text**
- Headers: `color: $text-primary`
- Body: `color: $text`
- Muted: `color: $text-muted`
- Disabled: `opacity: 40%`

### By Component

| Component | Background | Text | Border | Status |
|-----------|-----------|------|--------|--------|
| SendRequestButton | $accent-muted | $text-accent | none | $text-accent hover |
| ResponseArea | transparent | $text-muted | $accent 40% | $success/$warning/$error |
| Input | transparent | $text | $surface-lighten-1 (focus) | $error (invalid) |
| DataTable | $surface | $text-success (header) | none | $block-cursor-background |
| CollectionBrowser | transparent | $foreground 80% | none | $accent 40% |
| CommandPalette | black 33% | $text | $accent | none |

---

## Creating Themes for Posting

### YAML Theme with Color Mapping

```yaml
name: my_custom_theme
author: Your Name
description: A custom Posting theme

# Base Colors
primary: '#007AFF'          # iOS blue
secondary: '#5AC8FA'        # Light blue
accent: '#FF2D55'           # Rose red

# Backgrounds
background: '#F2F2F7'       # Light gray
surface: '#FFFFFF'          # White
panel: '#F9F9FB'            # Lighter gray

# Status Colors
success: '#34C759'          # Green
warning: '#FF9500'          # Orange
error: '#FF3B30'            # Red

# Text Colors (optional - will be auto-computed)
text: '#000000'             # Black
dark: false                 # Light theme

# HTTP Method Colors
method:
  get: '#007AFF'            # Blue
  post: '#34C759'           # Green
  put: '#FF9500'            # Orange
  delete: '#FF3B30'         # Red
  patch: '#17C0EB'          # Cyan
  options: '#9B59B6'        # Purple
  head: '#E74C3C'           # Dark red
```

### How the Color System Works

1. **Theme Definition** (themes.py)
   - User defines colors in YAML
   - Pydantic validates colors
   - Theme object created

2. **Textual Conversion** (to_textual_theme())
   - Converts to Textual Theme format
   - Creates theme variables
   - Builds muted variants automatically

3. **SCSS Application** (posting.scss)
   - SCSS references theme variables
   - Applies colors consistently
   - Respects opacity for hierarchy

4. **Runtime Application**
   - Theme applied at app startup
   - Can be switched via command palette
   - All colors update automatically

### Default Theme Variables

```scss
// Primary palette
$primary             -> User's primary color
$primary-muted       -> Auto-generated muted variant
$primary-darken-1    -> Auto-generated dark variant

// Accent palette
$accent              -> User's accent color
$accent-muted        -> Auto-generated muted variant

// Text palette
$text                -> User's text color (auto-computed from background)
$text-primary        -> Primary text variant
$text-accent         -> Accent-colored text (auto-computed)
$text-success        -> Success text (from $success)
$text-warning        -> Warning text (from $warning)
$text-error          -> Error text (from $error)
$text-muted          -> Muted text (auto-computed)

// Status palette
$success-muted       -> Auto-generated from $success
$warning-muted       -> Auto-generated from $warning
$error-muted         -> Auto-generated from $error
```

---

## Practical Usage Example

### Creating a Cohesive Color Scheme

**Light Professional Theme**:
```yaml
name: light_professional
primary: '#2C3E50'          # Dark blue-gray
accent: '#E74C3C'           # Red for attention
background: '#ECF0F1'       # Light gray
surface: '#FFFFFF'          # Pure white
success: '#27AE60'          # Green
warning: '#F39C12'          # Orange
error: '#E74C3C'            # Red
text: '#2C3E50'             # Dark
dark: false
```

Result:
- Professional, muted palette
- High contrast for readability
- Red accent for important actions
- Green/orange/red for status

**Dark Creative Theme**:
```yaml
name: dark_creative
primary: '#64FFDA'          # Cyan
accent: '#FF006E'           # Hot pink
background: '#0A0E27'       # Very dark blue
surface: '#16213E'          # Dark blue
success: '#00F5A0'          # Bright green
warning: '#FFB703'          # Bright orange
error: '#FB5607'            # Bright red
text: '#FFFFFF'             # White
dark: true
```

Result:
- Modern, vibrant palette
- Creative color choices
- Dark background reduces eye strain
- Bright status colors for visibility

---

## Color Accessibility Considerations

### Contrast Requirements

```scss
/* WCAG AA Compliance (4.5:1 minimum) */
Good:
  dark text (#2C3E50) on light background (#FFFFFF)
  light text (#FFFFFF) on dark background (#0A0E27)

Problem:
  light gray text (#AAAAAA) on light background (#FFFFFF)
  yellow text (#FFFF00) on white background
```

### Color-Blind Friendly Palettes

Avoid:
- Red-only status indicators (use pattern + color)
- Green-red combinations without text labels
- Low saturation differences

Prefer:
- Distinct hues (red, yellow, blue, green)
- Text labels with colors
- Icons with colors

---

## Summary: Theme Color System

The Posting theme system achieves cohesive coloring through:

1. **Base Colors** - Primary, secondary, accent, background, surface
2. **Status Colors** - Success, warning, error with muted variants
3. **Text Colors** - Hierarchy from primary to muted
4. **Opacity Modifiers** - Creates depth and hierarchy
5. **Automatic Variants** - Textual generates complementary colors
6. **Consistent Application** - SCSS variables used throughout
7. **Component-Specific** - Each component styled appropriately
8. **Accessibility** - Contrast and color-blind considerations
9. **User Customizable** - YAML theme files
10. **Runtime Switchable** - Themes changed without restart

This system ensures visual consistency while allowing customization and accessibility.

