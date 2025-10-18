# Textual Color Scheme & Button Styling Guide

A comprehensive guide on how Posting implements its color theming system and creates custom buttons.

## Table of Contents

1. [Color Scheme Architecture](#color-scheme-architecture)
2. [Theme System Implementation](#theme-system-implementation)
3. [Color Variables & Palette](#color-variables--palette)
4. [Using Colors in SCSS](#using-colors-in-scss)
5. [Button Styling](#button-styling)
6. [Custom Button Components](#custom-button-components)
7. [Theme Configuration Files](#theme-configuration-files)
8. [Code Examples](#code-examples)
9. [Best Practices](#best-practices)

---

## Color Scheme Architecture

### Overview

Posting uses a sophisticated theming system that combines:

1. **Textual's built-in theme system** - Base colors and styles
2. **Custom theme variables** - Application-specific colors
3. **SCSS styling** - Color application across UI
4. **Rich library integration** - Text markup with colors

### Color Hierarchy

```
Theme (Base Definition)
├── Primary Colors
│   ├── primary
│   ├── secondary
│   └── accent
├── Background Colors
│   ├── background
│   ├── surface
│   └── panel
├── Status Colors
│   ├── success
│   ├── warning
│   └── error
├── Semantic Colors
│   ├── text-* variants
│   ├── border-* variants
│   └── background-* variants
└── Application-Specific Variables
    ├── Method colors (GET, POST, etc.)
    ├── URL component colors
    ├── Variable indicator colors
    └── Syntax highlighting colors
```

---

## Theme System Implementation

### 1. Theme Class Structure

**File**: `src/posting/themes.py` (line 106+)

```python
from pydantic import BaseModel, Field
from textual.theme import Theme as TextualTheme

class Theme(BaseModel):
    name: str
    primary: str                    # Main accent color
    secondary: str | None = None    # Secondary accent
    background: str | None = None   # Main background
    surface: str | None = None      # Panel/widget background
    panel: str | None = None        # Additional panel color
    warning: str | None = None      # Warning color
    error: str | None = None        # Error/danger color
    success: str | None = None      # Success/positive color
    accent: str | None = None       # Accent color
    dark: bool = True               # Dark mode flag
    
    # Sub-themes for specialized components
    text_area: PostingTextAreaTheme = Field(default_factory=PostingTextAreaTheme)
    syntax: str | SyntaxTheme = Field(default="posting")
    url: UrlStyles | None = Field(default_factory=UrlStyles)
    variable: VariableStyles | None = Field(default_factory=VariableStyles)
    method: MethodStyles | None = Field(default_factory=MethodStyles)
    
    # Metadata
    author: str | None = None
    description: str | None = None
    homepage: str | None = None
```

### 2. HTTP Method Color Styles

**File**: `src/posting/themes.py` (line 94-103)

```python
class MethodStyles(BaseModel):
    """The style to apply to HTTP methods in the sidebar."""
    
    get: str | None = Field(default="#0ea5e9")      # Cyan/Sky Blue
    post: str | None = Field(default="#22c55e")     # Green
    put: str | None = Field(default="#f59e0b")      # Amber/Orange
    delete: str | None = Field(default="#ef4444")   # Red
    patch: str | None = Field(default="#14b8a6")    # Teal
    options: str | None = Field(default="#8b5cf6")  # Purple
    head: str | None = Field(default="#d946ef")     # Magenta
```

These colors automatically appear in the collection browser to visually distinguish request types.

### 3. Converting to Textual Theme

**File**: `src/posting/themes.py` (line 144-242)

The `to_textual_theme()` method converts the Posting theme to Textual format:

```python
def to_textual_theme(self) -> TextualTheme:
    """Convert this theme to a Textual Theme."""
    theme_data = {
        "name": self.name,
        "dark": self.dark,
    }
    
    colors = {
        "primary": self.primary,
        "secondary": self.secondary,
        "background": self.background,
        "surface": self.surface,
        "panel": self.panel,
        "warning": self.warning,
        "error": self.error,
        "success": self.success,
        "accent": self.accent,
    }
    
    # Validate colors
    for color in colors.values():
        if color is not None:
            Color.parse(color)  # Ensure valid color format
    
    # Merge colors into theme data
    theme_data = {**colors, **theme_data}
    
    # Build variables dictionary
    variables = {}
    
    # Add URL styling variables
    if self.url:
        url_styles = self.url.fill_with_defaults(self)
        variables.update({
            "url-base": url_styles.base,
            "url-protocol": url_styles.protocol,
            "url-separator": url_styles.separator,
        })
    
    # Add variable indicator styles
    if self.variable:
        var_styles = self.variable.fill_with_defaults(self)
        variables.update({
            "variable-resolved": var_styles.resolved,
            "variable-unresolved": var_styles.unresolved,
        })
    
    # Add HTTP method colors
    if self.method:
        variables.update({
            "method-get": self.method.get,
            "method-post": self.method.post,
            "method-put": self.method.put,
            "method-delete": self.method.delete,
            "method-patch": self.method.patch,
            "method-options": self.method.options,
            "method-head": self.method.head,
        })
    
    # Add text area theme variables
    if self.text_area:
        if self.text_area.gutter:
            variables["text-area-gutter"] = self.text_area.gutter
        if self.text_area.cursor:
            variables["text-area-cursor"] = self.text_area.cursor
        # ... more text area variables
    
    # Add syntax highlighting theme
    if isinstance(self.syntax, SyntaxTheme):
        if self.syntax.json_key:
            variables["syntax-json-key"] = self.syntax.json_key
        # ... more syntax variables
    
    # Filter out None values and create Textual theme
    theme_data = {k: v for k, v in theme_data.items() if v is not None}
    theme_data["variables"] = {k: v for k, v in variables.items() if v is not None}
    
    return TextualTheme(**theme_data)
```

---

## Color Variables & Palette

### Base Colors Used in SCSS

**File**: `src/posting/posting.scss`

Posting uses SCSS variables that reference Textual theme colors:

```scss
$primary           // Main accent color
$secondary         // Secondary accent
$background        // Main background
$surface           // Widget/panel background
$surface-darken-1  // Darkened surface
$surface-lighten-1 // Lightened surface
$accent            // Emphasis color
$accent-muted      // Muted accent
$text               // Main text color
$text-primary      // Primary text
$text-secondary    // Secondary text
$text-accent       // Accent text
$text-muted        // Muted text
$text-success      // Success text
$text-warning      // Warning text
$text-error        // Error text
$foreground        // Default foreground
$foreground-muted  // Muted foreground
$success           // Success color
$success-muted     // Muted success
$warning           // Warning color
$warning-muted     // Muted warning
$error             // Error color
$error-muted       // Muted error
$block-cursor-*    // Cursor styling colors
```

### Color Functions in SCSS

```scss
// Opacity modifier - use color with percentage opacity
border: round $accent 40%;      // Accent at 40% opacity
color: $text-accent 50%;        // Text at 50% opacity

// Darken/lighten variants
border: round $surface-darken-1 100%;
border: round $surface-lighten-1 70%;
```

---

## Using Colors in SCSS

### 1. Border Colors

```scss
.section {
  border: round $accent 40%;           # Rounded border with 40% opacity
  border-title-color: $text-accent 50%;
  border-subtitle-color: $text-muted;
  
  &:focus-within {
    border: round $accent 100%;        # Full opacity when focused
    border-title-color: $foreground;
    border-title-style: b;
  }
}
```

### 2. Text Colors

```scss
& Label {
  color: $text;
  
  &.-success {
    color: $text-success;
  }
  
  &.-error {
    color: $text-error;
  }
}
```

### 3. Background Colors

```scss
ResponseArea {
  background: $surface 75%;     # Surface color at 75% opacity
  
  &.success {
    background: $success-muted; # Muted success background
  }
  
  &.error {
    background: $error-muted;   # Muted error background
  }
}
```

### 4. Status-Based Color Changes

```scss
ResponseArea {
  &.success .border-title-status {
    color: $text-success;
    background: $success-muted;
  }
  
  &.warning .border-title-status {
    color: $text-warning;
    background: $warning-muted;
  }
  
  &.error .border-title-status {
    color: $text-error;
    background: $error-muted;
  }
}
```

### 5. Interactive Element Colors

```scss
Button {
  padding: 0 1;
  background: $primary;
  color: $text;
  
  &:hover {
    background: $primary-darken-1;
  }
  
  &:disabled {
    opacity: 40%;
  }
}

Input {
  border: none;
  
  &:focus {
    border-left: outer $surface-lighten-1;
  }
  
  &.-invalid {
    border-left: outer $error;
  }
}
```

---

## Button Styling

### 1. Basic Button Implementation

**File**: `src/posting/posting.scss` (line 743-751)

```scss
Button {
  padding: 0 1;
  height: 1;
  border: none;
  
  &:disabled {
    opacity: 40%;
  }
}
```

### 2. Custom SendRequestButton

**File**: `src/posting/posting.scss` (line 469-477)

```scss
SendRequestButton {
  min-width: 8;                    # Minimum width for label
  background: $accent-muted;       # Muted accent background
  color: $text-accent;             # Accent text color
  text-style: b;                   # Bold text
  
  &:hover {
    background-tint: $text-accent 10%;  # 10% tint on hover
  }
}
```

**Python Implementation**:

**File**: `src/posting/widgets/request/url_bar.py` (line 137-140)

```python
class SendRequestButton(Button, can_focus=False):
    """
    The button for sending the request.
    """
```

Key features:
- `can_focus=False` - Not focusable via tab (intentional design)
- Uses custom SCSS styling for visual distinction
- Styled as the primary action button

### 3. Key-Value Editor Add Button

**File**: `src/posting/posting.scss` (line 522-534)

```scss
KeyValueEditor {
  KeyValueInput {
    & Button {
      background: $primary;
      color: $text;
      text-style: none;
      min-width: 0;
      width: auto;
      margin: 0 1;
      
      &:hover {
        text-style: b;
        border: none;
        background: $primary-darken-1;
      }
    }
  }
  
  #add-button {
    color: $text;
    background: $accent;
  }
}
```

### 4. Button State Styling

```scss
Button {
  # Normal state
  background: $primary;
  color: $text;
  
  # Hover state
  &:hover {
    background-tint: $text 10%;
    text-style: b;
  }
  
  # Focus state
  &:focus {
    border: solid $accent;
  }
  
  # Disabled state
  &:disabled {
    opacity: 40%;
    color: $text-muted;
  }
}
```

---

## Custom Button Components

### 1. Creating Custom Button Classes

```python
from textual.widgets import Button

class PrimaryButton(Button):
    """A primary action button."""
    DEFAULT_CSS = """
    PrimaryButton {
        background: $accent;
        color: $text-accent;
        text-style: b;
        width: 100%;
        margin: 1 0;
    }
    
    PrimaryButton:hover {
        background-tint: white 10%;
    }
    """

class SecondaryButton(Button):
    """A secondary action button."""
    DEFAULT_CSS = """
    SecondaryButton {
        background: $surface;
        border: solid $accent;
        color: $text;
        width: 100%;
        margin: 1 0;
    }
    
    SecondaryButton:hover {
        background: $accent-muted;
    }
    """

class DangerButton(Button):
    """A destructive action button."""
    DEFAULT_CSS = """
    DangerButton {
        background: $error;
        color: white;
        text-style: b;
        width: 100%;
        margin: 1 0;
    }
    
    DangerButton:hover {
        background-tint: white 10%;
    }
    """
```

### 2. Success Button (Textual Built-in)

Textual provides built-in button variants:

```python
# From new_request_modal.py
yield Button.success("Create request", id="create-button")
# Renders as green button with checkmark-like styling

yield Button.warning("Cancel", id="cancel-button")
# Renders as yellow/orange button

yield Button.error("Delete", id="delete-button")
# Renders as red button
```

### 3. Button with Reactive Color Changes

```python
from textual.widgets import Button
from textual.reactive import reactive

class StatusButton(Button):
    """Button that changes color based on status."""
    
    status: Reactive[str] = reactive("idle")
    
    def watch_status(self, status: str):
        """Update button styling when status changes."""
        self.remove_class("success", "warning", "error", "loading")
        self.add_class(status)
        
        # Update button label
        if status == "loading":
            self.label = "Loading..."
        elif status == "success":
            self.label = "✓ Success"
        elif status == "error":
            self.label = "✗ Error"
```

### 4. Button Styling in SCSS

```scss
StatusButton {
  background: $surface;
  color: $text;
  border: solid $accent;
  
  &.loading {
    background: $warning-muted;
    color: $text-warning;
    text-style: b;
  }
  
  &.success {
    background: $success-muted;
    color: $text-success;
  }
  
  &.error {
    background: $error-muted;
    color: $text-error;
  }
}
```

---

## Theme Configuration Files

### 1. YAML Theme Format

**File**: `tests/sample-themes/serene_ocean.yaml`

```yaml
name: serene_ocean
primary: '#1E88E5'      # Ocean Blue - main accent
secondary: '#00ACC1'    # Teal - secondary accent
accent: '#D32F2F'       # Crimson Red - emphasis color
background: '#E3F2FD'   # Light Sky Blue - main background
surface: '#FFFFFF'      # White - widget backgrounds
error: '#D32F2F'        # Crimson Red - errors
success: '#43A047'      # Forest Green - success
warning: '#FFA726'      # Orange - warnings
text: '#212121'         # Almost Black - main text
dark: false             # Light theme indicator

# Optional advanced configuration
author: "Your Name"
description: "A serene ocean-themed color scheme"
homepage: "https://example.com"

# Optional: URL component colors
url:
  base: '#00ACC1'       # Base URL color
  protocol: '#1E88E5'   # Protocol (http://) color
  separator: 'dim'      # Separator (/) style

# Optional: Variable indicator colors
variable:
  resolved: '#43A047'   # Color for resolved variables
  unresolved: '#D32F2F' # Color for unresolved variables

# Optional: HTTP method colors
method:
  get: '#0ea5e9'        # GET - Cyan
  post: '#22c55e'       # POST - Green
  put: '#f59e0b'        # PUT - Orange
  delete: '#ef4444'     # DELETE - Red
  patch: '#14b8a6'      # PATCH - Teal
  options: '#8b5cf6'    # OPTIONS - Purple
  head: '#d946ef'       # HEAD - Magenta
```

### 2. Loading and Using Themes

**File**: `src/posting/themes.py`

```python
def load_user_theme(path: Path) -> Theme:
    """Load a theme from a YAML file."""
    with open(path) as f:
        data = yaml.safe_load(f)
    return Theme(**data)

def load_user_themes(theme_dir: Path) -> dict[str, Theme]:
    """Load all themes from a directory."""
    themes = {}
    for theme_file in theme_dir.glob("*.yaml"):
        try:
            theme = load_user_theme(theme_file)
            themes[theme.name] = theme
        except Exception as e:
            log.warning(f"Failed to load theme {theme_file}: {e}")
    return themes
```

### 3. Applying Themes at Runtime

```python
from textual.app import App

class Posting(App):
    def on_mount(self):
        # Load and apply theme
        theme = load_user_theme(Path("themes/my_theme.yaml"))
        textual_theme = theme.to_textual_theme()
        self.theme = textual_theme.name
```

---

## Code Examples

### Example 1: Creating a Complete Theme System

```python
from pathlib import Path
from typing import Dict
from posting.themes import Theme, load_user_themes
from textual.app import App
from textual.signal import Signal

class MyApp(App):
    """App with custom theme system."""
    
    available_themes: Dict[str, Theme] = {}
    current_theme_name: str = "default"
    theme_changed_signal = Signal()
    
    def on_mount(self):
        # Load all available themes
        themes_dir = Path("~/.config/myapp/themes").expanduser()
        self.available_themes = load_user_themes(themes_dir)
        
        # Apply default theme
        if "default" in self.available_themes:
            self.apply_theme("default")
    
    def apply_theme(self, theme_name: str):
        """Apply a theme by name."""
        if theme_name not in self.available_themes:
            return
        
        theme = self.available_themes[theme_name]
        textual_theme = theme.to_textual_theme()
        self.theme = textual_theme.name
        self.current_theme_name = theme_name
        
        # Notify listeners
        self.theme_changed_signal.emit()
    
    def command_switch_theme(self, theme_name: str):
        """Command to switch themes."""
        self.apply_theme(theme_name)
```

### Example 2: Color-Coded Request Methods

```python
from textual.widgets import Label
from rich.text import Text

class MethodLabel(Label):
    """Display HTTP method with color coding."""
    
    METHOD_COLORS = {
        "GET": "#0ea5e9",      # Cyan
        "POST": "#22c55e",     # Green
        "PUT": "#f59e0b",      # Orange
        "DELETE": "#ef4444",   # Red
        "PATCH": "#14b8a6",    # Teal
        "OPTIONS": "#8b5cf6",  # Purple
        "HEAD": "#d946ef",     # Magenta
    }
    
    def render_method(self, method: str) -> Text:
        """Render HTTP method with appropriate color."""
        color = self.METHOD_COLORS.get(method, "#888888")
        return Text(method, style=f"bold {color}")
```

### Example 3: Status-Based Button Styling

```python
from textual.widgets import Button
from textual.reactive import reactive

class StatusButton(Button):
    """Button with status-based color changes."""
    
    status: Reactive[str] = reactive("idle", init=False)
    
    STATUS_COLORS = {
        "idle": "$surface",
        "loading": "$warning-muted",
        "success": "$success-muted",
        "error": "$error-muted",
    }
    
    STATUS_TEXT_COLORS = {
        "idle": "$text",
        "loading": "$text-warning",
        "success": "$text-success",
        "error": "$text-error",
    }
    
    def watch_status(self, status: str):
        """Update button when status changes."""
        self.remove_class("idle", "loading", "success", "error")
        self.add_class(status)
        
        # Update label based on status
        labels = {
            "idle": self.label,
            "loading": "Processing...",
            "success": "✓ Complete",
            "error": "✗ Failed",
        }
        self.label = labels.get(status, self.label)

# SCSS:
StatusButton {
  background: $surface;
  color: $text;
  border: solid $accent;
  
  &.idle {
    background: $surface;
    color: $text;
  }
  
  &.loading {
    background: $warning-muted;
    color: $text-warning;
    text-style: b;
  }
  
  &.success {
    background: $success-muted;
    color: $text-success;
  }
  
  &.error {
    background: $error-muted;
    color: $text-error;
  }
}
```

### Example 4: URL Syntax Highlighting with Colors

```python
from rich.text import Text
from posting.themes import UrlStyles

class UrlHighlighter:
    """Highlight different parts of URL with distinct colors."""
    
    def __init__(self, url_styles: UrlStyles):
        self.styles = url_styles
    
    def highlight_url(self, url: str) -> Text:
        """Return URL with syntax highlighting."""
        # Example URL: https://api.example.com/users/123
        text = Text()
        
        # Protocol part
        if "://" in url:
            protocol, rest = url.split("://", 1)
            text.append(protocol, style=self.styles.protocol)
            text.append("://", style=self.styles.separator)
            
            # Base URL part
            if "/" in rest:
                base, path = rest.split("/", 1)
                text.append(base, style=self.styles.base)
                text.append("/", style=self.styles.separator)
                text.append(path, style=self.styles.base)
            else:
                text.append(rest, style=self.styles.base)
        
        return text
```

---

## Best Practices

### 1. Use Theme Variables Instead of Hardcoded Colors

```scss
# ✅ Good: Use theme variables
Button {
  background: $primary;
  color: $text;
}

# ❌ Bad: Hardcoded colors
Button {
  background: #007AFF;
  color: #FFFFFF;
}
```

### 2. Respect Dark/Light Mode

```python
# ✅ Good: Use theme's dark property
class Theme(BaseModel):
    dark: bool = True  # Indicates dark mode
    
    def to_textual_theme(self) -> TextualTheme:
        theme_data["dark"] = self.dark

# ❌ Bad: Assume dark mode
colors = {
    "background": "#000000",  # Always black
}
```

### 3. Provide Semantic Color Names

```python
# ✅ Good: Semantic names
class MethodStyles(BaseModel):
    get: str = "#0ea5e9"       # What it is (GET method)
    success: str = "#22c55e"   # What it means (success)
    error: str = "#ef4444"     # What it means (error)

# ❌ Bad: Generic names
colors = {
    "color1": "#0ea5e9",
    "color2": "#22c55e",
    "color3": "#ef4444",
}
```

### 4. Use Opacity for Visual Hierarchy

```scss
# ✅ Good: Opacity creates hierarchy
.primary {
  color: $accent;        # 100% opacity
}

.secondary {
  color: $accent 50%;    # 50% opacity
}

.tertiary {
  color: $accent 30%;    # 30% opacity
}

# ❌ Bad: Different colors without relationship
.primary { color: $accent; }
.secondary { color: $surface; }
.tertiary { color: $text-muted; }
```

### 5. Create Consistent Button Variants

```python
# ✅ Good: Consistent system
class PrimaryButton(Button): pass
class SecondaryButton(Button): pass
class DangerButton(Button): pass

# ❌ Bad: Inconsistent styling
if action == "save":
    button.background = "#00AA00"
elif action == "delete":
    button.background = "#FF0000"
```

### 6. Handle Color Accessibility

```python
# ✅ Good: Sufficient contrast
success_text = "#22c55e"       # Good contrast on light background
success_text = "#006633"       # Even better contrast

# ❌ Bad: Low contrast
success_text = "#88FF88"       # Light green on light background = poor contrast
error_text = "#FF8888"         # Light red on light background = poor contrast
```

### 7. Test Themes with Different Backgrounds

```python
# ✅ Good: Works in both dark and light modes
def fill_with_defaults(self, theme: "Theme") -> "UrlStyles":
    return UrlStyles(
        base=self.base or theme.secondary,
        protocol=self.protocol or theme.accent,
        separator=self.separator or "dim",
    )

# ❌ Bad: Assumes dark mode
url_styles = UrlStyles(
    base="#AAAAAA",  # Gray that only works on dark background
    protocol="#0099FF",
)
```

### 8. Use Component Classes for Consistency

```scss
# ✅ Good: Component class for all status indicators
ResponseArea {
  COMPONENT_CLASSES = {
    "border-title-status"
  }
  
  & .border-title-status {
    color: $text-success;
    background: $success-muted;
  }
}

# ❌ Bad: Inline styles scattered
ResponseArea {
  &.success {
    border-title-color: #22c55e;
    border-title-background: #dcfce7;
  }
}
```

---

## Summary

### Key Takeaways

1. **Theme System**: Use Pydantic models for theme configuration
2. **Color Variables**: Define semantic color names (not color1, color2)
3. **SCSS Application**: Use theme variables in all SCSS rules
4. **Button Styling**: Create custom button classes for consistency
5. **Status Colors**: Use add/remove classes to change colors dynamically
6. **Accessibility**: Ensure sufficient contrast in your color schemes
7. **Themes as YAML**: Allow users to create custom themes
8. **Dynamic Switching**: Support theme switching at runtime

### Color Scheme Checklist

- [ ] Primary, secondary, and accent colors defined
- [ ] Background, surface, and panel colors defined
- [ ] Success, warning, and error colors defined
- [ ] Text color variants (primary, secondary, muted)
- [ ] HTTP method colors defined
- [ ] URL component colors defined
- [ ] Variable indicator colors defined
- [ ] Sufficient contrast for accessibility
- [ ] Consistent in both dark and light modes
- [ ] YAML theme file created
- [ ] Theme can be loaded at runtime
- [ ] Colors used in SCSS variables

