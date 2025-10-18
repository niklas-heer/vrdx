# Complete Textual TUI Framework Guide

A comprehensive guide to building professional TUI applications with Textual, based on design patterns from the Posting HTTP client.

## Table of Contents

1. [Introduction](#introduction)
2. [Architecture & Layout](#architecture--layout)
3. [Dynamic Pane Headers](#dynamic-pane-headers)
4. [Color Theming System](#color-theming-system)
5. [Button Styling & Implementation](#button-styling--implementation)
6. [Practical Examples](#practical-examples)
7. [Best Practices](#best-practices)

---

## Introduction

### What is Textual?

Textual is a Python framework for building sophisticated Terminal User Interface (TUI) applications. Unlike traditional CLI tools, Textual applications support:

- Multiple panes and widgets with independent styling
- Color themes and CSS-like styling (TCSS)
- Mouse support in terminals
- Reactive programming with automatic UI updates
- Complex layouts with responsive design

### Why This Guide?

This guide teaches you how to build professional TUI applications by examining real-world patterns from Posting, an HTTP client built with Textual. You'll learn:

- How to structure multi-pane applications
- Creating dynamic headers that reflect application state
- Implementing comprehensive color themes
- Designing accessible buttons and controls
- Building responsive layouts

You don't need the Posting source code—all examples are self-contained and generic.

---

## Architecture & Layout

### Understanding the Layout Structure

A Textual application is composed of widgets arranged in a hierarchy. Think of it like HTML:

```
App (Container)
├── Header (Horizontal)
│   ├── Title
│   └── Status
├── Body (Vertical or Horizontal)
│   ├── Sidebar (Vertical)
│   │   ├── Tree/List
│   │   └── Preview
│   ├── MainContent (Vertical)
│   │   ├── Tabs/Panes
│   │   └── Content
│   └── ResultsPanel (Vertical)
│       └── Output/Results
└── Footer (Horizontal)
```

### Creating a Basic Multi-Pane Application

```python
from textual.app import ComposeResult, App
from textual.containers import Container, Horizontal, Vertical
from textual.widgets import Header, Footer, Static
from textual.reactive import reactive

class Sidebar(Vertical):
    """Left sidebar for navigation."""
    
    def compose(self) -> ComposeResult:
        yield Static("Navigation", id="nav-title")
        yield Static("Items list here", id="nav-items")

class MainPane(Vertical):
    """Center pane for main content."""
    
    def compose(self) -> ComposeResult:
        yield Static("Main Content", id="main-header")
        yield Static("Content goes here", id="main-content")

class ResultPane(Vertical):
    """Right pane for results/output."""
    
    def compose(self) -> ComposeResult:
        yield Static("Results", id="results-header")
        yield Static("Results go here", id="results-content")

class MyApp(App):
    """A three-pane TUI application."""
    
    def compose(self) -> ComposeResult:
        yield Header()
        with Container(id="app-body"):
            yield Sidebar(id="sidebar")
            yield MainPane(id="main")
            yield ResultPane(id="results")
        yield Footer()
```

### Styling the Layout

Textual uses TCSS (Textual CSS Stylesheet) for styling. Create a `styles.tcss` file:

```tcss
Screen {
    layout: vertical;
    background: $surface;
}

#app-body {
    layout: horizontal;
    height: 1fr;
}

#sidebar {
    width: 25%;
    border: solid $accent 40%;
    border-title-color: $text-accent 50%;
}

#main {
    width: 50%;
    border: solid $accent 40%;
}

#results {
    width: 25%;
    border: solid $accent 40%;
}
```

### Switching Between Layouts

Make your layout responsive by switching between vertical and horizontal:

```python
from textual.reactive import reactive

class ResponsiveApp(App):
    current_layout: Reactive[str] = reactive("horizontal", init=False)
    
    def watch_current_layout(self, layout: str):
        """Update the layout when the reactive property changes."""
        body = self.query_one("#app-body")
        body.remove_class("layout-vertical", "layout-horizontal")
        body.add_class(f"layout-{layout}")
    
    def action_toggle_layout(self):
        """Switch between vertical and horizontal layout."""
        new_layout = "vertical" if self.current_layout == "horizontal" else "horizontal"
        self.current_layout = new_layout

# In TCSS:
# #app-body.layout-horizontal { layout: horizontal; }
# #app-body.layout-vertical { layout: vertical; }
```

---

## Dynamic Pane Headers

### Using Border Titles and Subtitles

Textual widgets support border titles and subtitles that can be dynamically updated. This is perfect for showing the current selection or state:

```python
from textual.containers import Vertical
from textual.widgets import Static

class SelectablePane(Vertical):
    """A pane that shows the current selection in its border."""
    
    def on_mount(self):
        self.border_title = "Collections"
        self.border_subtitle = "My API"
```

### Dynamic Headers with Reactive Properties

To update the header when state changes, use reactive properties:

```python
from textual.reactive import reactive

class DynamicPane(Vertical):
    """Pane with dynamic header based on state."""
    
    current_item_name: Reactive[str] = reactive("", init=False)
    
    def watch_current_item_name(self, name: str):
        """Called automatically when current_item_name changes."""
        self.border_title = f"Item: {name}"
    
    def on_mount(self):
        self.border_title = "Item: None"
    
    def select_item(self, name: str):
        """Select an item and update the header."""
        self.current_item_name = name  # This triggers watch_current_item_name
```

### Advanced: Status-Based Headers

Display contextual information in headers:

```python
class RequestPane(Vertical):
    """Pane showing HTTP request details."""
    
    current_request: Reactive[dict] = reactive(None, init=False)
    
    def watch_current_request(self, request: dict):
        """Update the header when request changes."""
        if request:
            method = request.get("method", "GET")
            url = request.get("url", "")
            # Set the title with the request info
            self.border_title = f"{method} {url}"
        else:
            self.border_title = "Request"
    
    def load_request(self, method: str, url: str):
        """Load a new request."""
        self.current_request = {"method": method, "url": url}
```

### Response Headers with Dynamic Status

Show the HTTP response status in the border:

```python
class ResponsePane(Vertical):
    """Pane showing HTTP response with status indicator."""
    
    response_data: Reactive[dict] = reactive(None, init=False)
    
    def watch_response_data(self, response: dict):
        """Update the pane when response changes."""
        if response:
            status_code = response.get("status", 0)
            status_text = response.get("reason", "Unknown")
            size = response.get("size", 0)
            elapsed = response.get("elapsed", 0)
            
            # Update title with status
            self.border_title = f"Response [{status_code} {status_text}]"
            # Update subtitle with metrics
            self.border_subtitle = f"{size}B in {elapsed}ms"
            
            # Add status-based styling
            self.remove_class("success", "warning", "error")
            if status_code < 300:
                self.add_class("success")
            elif status_code < 400:
                self.add_class("warning")
            else:
                self.add_class("error")
    
    def on_mount(self):
        self.border_title = "Response"
    
    def receive_response(self, status: int, reason: str, content_size: int, elapsed_ms: float):
        """Called when a response is received."""
        self.response_data = {
            "status": status,
            "reason": reason,
            "size": content_size,
            "elapsed": elapsed_ms
        }

# TCSS styling:
# ResponsePane {
#   &.success .border-title {
#     color: $text-success;
#   }
#   &.warning .border-title {
#     color: $text-warning;
#   }
#   &.error .border-title {
#     color: $text-error;
#   }
# }
```

---

## Color Theming System

### Understanding the Color Hierarchy

A professional TUI application needs a consistent color system:

```
Base Colors (defined in theme):
├── Primary      - Main accent color
├── Secondary    - Alternative accent
└── Accent       - Emphasis color

Backgrounds:
├── Background   - Main app background
├── Surface      - Widget backgrounds
└── Panel        - Alternative panels

Text Colors:
├── Text         - Primary text
├── Text-Primary - Primary variant
├── Text-Accent  - Accent text
├── Text-Muted   - Disabled/dim text
└── Text-Success/Warning/Error

Status Colors:
├── Success      - Positive feedback
├── Warning      - Caution/pending
└── Error        - Negative feedback
```

### Creating a Theme System

Define your color scheme using Pydantic models:

```python
from pydantic import BaseModel
from typing import Optional

class Theme(BaseModel):
    """Application theme definition."""
    name: str
    
    # Primary colors
    primary: str                    # e.g., "#007AFF"
    secondary: Optional[str] = None
    accent: str                     # e.g., "#FF2D55"
    
    # Backgrounds
    background: str                 # e.g., "#F2F2F7"
    surface: str                    # e.g., "#FFFFFF"
    panel: Optional[str] = None
    
    # Status colors
    success: str                    # e.g., "#34C759"
    warning: str                    # e.g., "#FF9500"
    error: str                      # e.g., "#FF3B30"
    
    # Text colors (can be auto-computed)
    text: str                       # e.g., "#000000"
    text_accent: Optional[str] = None
    text_muted: Optional[str] = None

# Create theme instances
LIGHT_THEME = Theme(
    name="light",
    primary="#007AFF",
    accent="#FF2D55",
    background="#F2F2F7",
    surface="#FFFFFF",
    success="#34C759",
    warning="#FF9500",
    error="#FF3B30",
    text="#000000"
)

DARK_THEME = Theme(
    name="dark",
    primary="#64FFDA",
    accent="#FF006E",
    background="#0A0E27",
    surface="#16213E",
    success="#00F5A0",
    warning="#FFB703",
    error="#FB5607",
    text="#FFFFFF"
)
```

### Applying Colors via TCSS

Define color variables in your theme and use them in TCSS:

```tcss
/* Define color variables in Textual's CSS variables */
Screen {
    /* Using Textual's built-in colors */
    background: $background;
}

.section {
    /* Borders with opacity for hierarchy */
    border: round $accent 40%;              /* 40% opacity */
    border-title-color: $text-accent 50%;
    
    &:focus-within {
        border: round $accent 100%;         /* Full opacity when focused */
        border-title-color: $foreground;
        border-title-style: b;              /* Bold */
    }
}

/* Status-based colors */
.pane {
    &.success {
        border-title-color: $text-success;
        background: $success 10%;           /* 10% opacity background */
    }
    
    &.warning {
        border-title-color: $text-warning;
        background: $warning 10%;
    }
    
    &.error {
        border-title-color: $text-error;
        background: $error 10%;
    }
}

/* Text hierarchy */
#primary-text {
    color: $text;
}

#secondary-text {
    color: $text-muted;
}

#accent-text {
    color: $text-accent;
}
```

### Loading Themes from YAML

Allow users to customize themes:

```yaml
# themes/professional.yaml
name: professional
primary: '#2C3E50'
secondary: '#34495E'
accent: '#E74C3C'
background: '#ECF0F1'
surface: '#FFFFFF'
success: '#27AE60'
warning: '#F39C12'
error: '#E74C3C'
text: '#2C3E50'
```

Load and apply themes:

```python
import yaml
from pathlib import Path

class ThemeManager:
    """Manages application themes."""
    
    def __init__(self):
        self.themes = {}
        self.current_theme = None
    
    def load_themes(self, theme_dir: Path):
        """Load all themes from a directory."""
        for theme_file in theme_dir.glob("*.yaml"):
            with open(theme_file) as f:
                data = yaml.safe_load(f)
                theme = Theme(**data)
                self.themes[theme.name] = theme
    
    def apply_theme(self, theme_name: str, app):
        """Apply a theme to the Textual app."""
        if theme_name in self.themes:
            theme = self.themes[theme_name]
            self.current_theme = theme
            # Update app.theme with the theme's colors
            # (Textual handles the color variable binding)
```

---

## Button Styling & Implementation

### Basic Button Styling

Textual provides a Button widget with built-in styling:

```python
from textual.widgets import Button
from textual.containers import Horizontal

class MyApp(App):
    def compose(self) -> ComposeResult:
        with Horizontal():
            yield Button("Save", id="save-btn")
            yield Button("Cancel", id="cancel-btn")
            yield Button("Delete", id="delete-btn")
```

Style buttons in TCSS:

```tcss
Button {
    padding: 0 1;
    height: 1;
    border: none;
    background: $primary;
    color: $text;
    
    &:hover {
        background-tint: $text 10%;     /* Tint on hover */
        text-style: b;                  /* Bold on hover */
    }
    
    &:focus {
        border: solid $accent;
        background-tint: $accent 5%;
    }
    
    &:disabled {
        opacity: 40%;
        color: $text-muted;
    }
}
```

### Creating Custom Button Classes

Design specialized buttons for different actions:

```python
from textual.widgets import Button

class PrimaryButton(Button):
    """Primary action button (Save, Confirm, etc.)."""
    DEFAULT_CSS = """
    PrimaryButton {
        background: $accent;
        color: $text-accent;
        text-style: b;
        width: 1fr;
        margin: 1 0;
    }
    
    PrimaryButton:hover {
        background-tint: white 10%;
    }
    """

class SecondaryButton(Button):
    """Secondary action button (Cancel, Reset, etc.)."""
    DEFAULT_CSS = """
    SecondaryButton {
        background: $surface;
        border: solid $accent;
        color: $text;
        width: 1fr;
        margin: 1 0;
    }
    
    SecondaryButton:hover {
        background: $accent-muted;
    }
    """

class DangerButton(Button):
    """Destructive action button (Delete, Remove, etc.)."""
    DEFAULT_CSS = """
    DangerButton {
        background: $error;
        color: white;
        text-style: b;
        width: 1fr;
        margin: 1 0;
    }
    
    DangerButton:hover {
        background-tint: white 15%;
    }
    """

# Usage
with Horizontal():
    yield PrimaryButton("Confirm")
    yield SecondaryButton("Cancel")
    yield DangerButton("Delete")
```

### Button State Management

Manage button states reactively:

```python
from textual.reactive import reactive

class StateButton(Button):
    """Button that changes based on state."""
    
    status: Reactive[str] = reactive("idle")
    
    STATUS_LABELS = {
        "idle": "Send",
        "loading": "Sending...",
        "success": "✓ Sent",
        "error": "✗ Error"
    }
    
    def watch_status(self, status: str):
        """Update button when status changes."""
        self.label = self.STATUS_LABELS.get(status, "Send")
        self.disabled = (status == "loading")
        
        # Update classes for styling
        self.remove_class("idle", "loading", "success", "error")
        self.add_class(status)
    
    def on_mount(self):
        self.status = "idle"

# TCSS styling:
# StateButton {
#   &.idle { background: $surface; }
#   &.loading { background: $warning-muted; text-style: b; }
#   &.success { background: $success-muted; }
#   &.error { background: $error-muted; }
# }
```

### Button Groups

Group related buttons with consistent styling:

```python
class ConfirmationButtons(Static):
    """Group of confirmation buttons."""
    
    def compose(self) -> ComposeResult:
        with Horizontal(id="buttons"):
            yield PrimaryButton("Confirm", id="confirm")
            yield SecondaryButton("Cancel", id="cancel")

# TCSS styling:
# #buttons {
#     layout: horizontal;
#     height: 1;
#     margin-top: 1;
# }
#
# #buttons Button {
#     width: 1fr;
#     margin-right: 1;
# }
```

---

## Practical Examples

### Example 1: Complete Three-Pane Application

A complete, runnable example of a three-pane TUI application:

```python
from textual.app import ComposeResult, App
from textual.containers import Container, Horizontal, Vertical
from textual.widgets import Header, Footer, Static, Button, Input
from textual.reactive import reactive

class CollectionBrowser(Vertical):
    """Left sidebar showing available items."""
    
    selected_item: Reactive[str] = reactive("", init=False)
    
    def compose(self) -> ComposeResult:
        yield Static("Collections", id="browser-title")
        yield Static("Item 1\nItem 2\nItem 3", id="browser-list")
    
    def on_mount(self):
        self.border_title = "Collections"
        self.selected_item = "Item 1"
    
    def watch_selected_item(self, name: str):
        """Update when selection changes."""
        self.border_subtitle = name

class EditorPane(Vertical):
    """Center pane for editing."""
    
    current_item: Reactive[str] = reactive("", init=False)
    
    def compose(self) -> ComposeResult:
        yield Static("Editor", id="editor-title")
        yield Input(id="editor-input")
        with Horizontal(id="editor-buttons"):
            yield Button("Save", id="save")
            yield Button("Reset", id="reset")
    
    def on_mount(self):
        self.border_title = "Editor"
        self.current_item = "Item 1"
    
    def watch_current_item(self, name: str):
        """Update editor when item changes."""
        self.border_subtitle = name

class ResultsPane(Vertical):
    """Right pane showing results."""
    
    result_status: Reactive[str] = reactive("idle", init=False)
    
    def compose(self) -> ComposeResult:
        yield Static("Results", id="results-title")
        yield Static("Results will appear here", id="results-content")
    
    def on_mount(self):
        self.border_title = "Results"
        self.result_status = "idle"
    
    def watch_result_status(self, status: str):
        """Update when results change."""
        status_text = {
            "idle": "Ready",
            "processing": "Processing...",
            "success": "✓ Success",
            "error": "✗ Error"
        }
        self.border_subtitle = status_text.get(status, "Ready")

class ThreePaneApp(App):
    """A three-pane TUI application."""
    
    CSS = """
    Screen {
        layout: vertical;
    }
    
    #app-body {
        layout: horizontal;
        height: 1fr;
    }
    
    CollectionBrowser {
        width: 25%;
        border: solid $accent 40%;
    }
    
    EditorPane {
        width: 50%;
        border: solid $accent 40%;
    }
    
    ResultsPane {
        width: 25%;
        border: solid $accent 40%;
    }
    
    .section {
        border-title-color: $text-accent 50%;
        
        &:focus-within {
            border: solid $accent 100%;
            border-title-color: $foreground;
        }
    }
    """
    
    def compose(self) -> ComposeResult:
        yield Header()
        with Container(id="app-body"):
            yield CollectionBrowser(classes="section")
            yield EditorPane(classes="section")
            yield ResultsPane(classes="section")
        yield Footer()

if __name__ == "__main__":
    ThreePaneApp().run()
```

### Example 2: Tabs and Dynamic Content

Implement tabbed content with dynamic headers:

```python
from textual.widgets import TabbedContent, TabPane, Static
from textual.containers import Vertical

class TabbedEditor(Vertical):
    """Editor with multiple tabs."""
    
    current_tab: Reactive[str] = reactive("editor", init=False)
    
    def compose(self) -> ComposeResult:
        with TabbedContent():
            with TabPane("Editor", id="editor"):
                yield Static("Editor content", id="editor-content")
            with TabPane("Preview", id="preview"):
                yield Static("Preview content", id="preview-content")
            with TabPane("Settings", id="settings"):
                yield Static("Settings content", id="settings-content")
    
    def on_mount(self):
        self.border_title = "Editor"
    
    def watch_current_tab(self, tab: str):
        """Update header based on current tab."""
        tab_names = {
            "editor": "Editor",
            "preview": "Preview",
            "settings": "Settings"
        }
        self.border_title = tab_names.get(tab, "Editor")
```

### Example 3: Status Colors in Response Display

Show HTTP response with color-coded status:

```python
class ResponseDisplay(Vertical):
    """Display HTTP response with status indicators."""
    
    response: Reactive[dict | None] = reactive(None, init=False)
    
    def compose(self) -> ComposeResult:
        yield Static("Response", id="response-content")
    
    def on_mount(self):
        self.border_title = "Response"
    
    def watch_response(self, response: dict | None):
        """Update display when response arrives."""
        if response is None:
            return
        
        status = response.get("status", 0)
        reason = response.get("reason", "Unknown")
        size = response.get("size", 0)
        elapsed = response.get("elapsed", 0)
        
        # Update titles
        self.border_title = f"Response [{status} {reason}]"
        self.border_subtitle = f"{size}B in {elapsed}ms"
        
        # Update content
        content = self.query_one("#response-content")
        content.update(f"Status: {status}\nSize: {size}\nTime: {elapsed}ms")
        
        # Add status-based classes
        self.remove_class("success", "warning", "error")
        if status < 300:
            self.add_class("success")
        elif status < 400:
            self.add_class("warning")
        else:
            self.add_class("error")

# CSS for status colors:
# ResponseDisplay {
#   &.success {
#     border-title-color: $text-success;
#   }
#   &.warning {
#     border-title-color: $text-warning;
#   }
#   &.error {
#     border-title-color: $text-error;
#   }
# }
```

---

## Best Practices

### 1. Use Reactive Properties for State

✅ **Good**: Automatic UI updates when state changes

```python
class MyPane(Vertical):
    selected_item: Reactive[str] = reactive("", init=False)
    
    def watch_selected_item(self, name: str):
        self.border_subtitle = name
```

❌ **Avoid**: Manual updates scattered throughout code

```python
def on_item_selected(self, event):
    self.pane.border_subtitle = event.item  # Updates manually
```

### 2. Use TCSS for Styling

✅ **Good**: Separate styling from Python code

```python
# In TCSS:
Button {
    background: $primary;
    color: $text;
}
```

❌ **Avoid**: Hardcoded colors in Python

```python
button.styles.background = "#007AFF"
button.styles.color = "#FFFFFF"
```

### 3. Create Reusable Components

✅ **Good**: Compose complex UIs from simple components

```python
class PrimaryButton(Button):
    """Reusable primary button."""
    pass

class SecondaryButton(Button):
    """Reusable secondary button."""
    pass

# Usage:
with Horizontal():
    yield PrimaryButton("Save")
    yield SecondaryButton("Cancel")
```

❌ **Avoid**: Recreating the same component repeatedly

```python
button1 = Button("Save")
button1.styles.background = "$primary"

button2 = Button("Cancel")
button2.styles.background = "$surface"
```

### 4. Use Container Layout Classes

✅ **Good**: Use layout containers for structure

```python
with Horizontal():
    yield Sidebar()
    yield MainContent()
    yield Results()

with Vertical():
    yield Header()
    yield Body()
    yield Footer()
```

❌ **Avoid**: Manual positioning

```python
sidebar.styles.dock = "left"
main_content.styles.width = "50%"
results.styles.dock = "right"
```

### 5. Keep State and UI Synchronized

✅ **Good**: Single source of truth

```python
class DataPane(Vertical):
    items: Reactive[list] = reactive([])
    
    def watch_items(self, new_items: list):
        # Update UI whenever items change
        content = self.query_one("#content")
        content.update("\n".join(new_items))
    
    def add_item(self, item: str):
        """Add an item (this automatically updates UI)."""
        self.items = [*self.items, item]
```

❌ **Avoid**: Separate state and UI

```python
self.my_items = []  # Internal state
content.update(...)  # UI state
# Now they can get out of sync!
```

### 6. Handle Component-Specific Styling

✅ **Good**: Component classes for context-specific styling

```python
class ResponsePane(Vertical):
    COMPONENT_CLASSES = {"status-indicator"}
    
    def on_mount(self):
        self.add_class("success")  # Triggers CSS rules

# CSS:
# ResponsePane.success { border-title-color: $text-success; }
# ResponsePane.error { border-title-color: $text-error; }
```

❌ **Avoid**: Generic styling

```python
self.border_title_color = "$text-success"
# Not themable or flexible
```

### 7. Accessibility

✅ **Good**: Clear focus indicators and disabled states

```tcss
Button:focus {
    border: solid $accent 2;
    text-style: b;
    background-tint: $accent 20%;
}

Button:disabled {
    opacity: 40%;
    color: $text-muted;
}
```

❌ **Avoid**: Making buttons indistinguishable

```tcss
Button:disabled {
    opacity: 10%;  /* Too subtle */
}
```

### 8. Color Hierarchy with Opacity

✅ **Good**: Use opacity to create visual hierarchy

```tcss
.section {
    border: round $accent 40%;        /* 40% opacity - subtle */
    
    &:focus-within {
        border: round $accent 100%;   /* 100% opacity - prominent */
    }
}

#background {
    background: $surface 10%;         /* Very subtle background */
}

#foreground {
    background: $surface 75%;         /* More prominent background */
}
```

❌ **Avoid**: Multiple shades of the same color

```tcss
.section {
    border-color: #999999;            /* Hard to maintain */
    background-color: #DDDDDD;        /* Not theme-aware */
}
```

### 9. Responsive Design

✅ **Good**: Adapt layout to available space

```python
current_layout: Reactive[str] = reactive("horizontal")

def watch_current_layout(self, layout: str):
    """Switch layouts dynamically."""
    body = self.query_one("#body")
    body.remove_class("vertical", "horizontal")
    body.add_class(layout)
```

### 10. Performance

✅ **Good**: Use Lazy loading for expensive components

```python
from textual.lazy import Lazy

with TabPane("Heavy", id="heavy"):
    yield Lazy(ExpensiveComponent())  # Only renders when active
```

---

## Summary

### Key Concepts

1. **Layout**: Use `Horizontal` and `Vertical` containers to structure panes
2. **Headers**: Use `border_title` and `border_subtitle` for context-aware headers
3. **Reactivity**: Use `Reactive` properties and watchers for automatic UI updates
4. **Theming**: Define colors in TCSS using variables for consistency
5. **Buttons**: Create custom button classes for different actions
6. **State**: Keep reactive properties as the single source of truth

### The Complete Pattern

```python
# 1. Define the component with reactive state
class MyPane(Vertical):
    selection: Reactive[str] = reactive("", init=False)
    
    # 2. Watch for changes
    def watch_selection(self, value: str):
        self.border_subtitle = value
    
    # 3. Compose the UI
    def compose(self) -> ComposeResult:
        yield Static("Content")
    
    # 4. Initialize on mount
    def on_mount(self):
        self.border_title = "My Pane"
    
    # 5. Update state to trigger UI changes
    def select(self, item: str):
        self.selection = item  # Triggers watch_selection
```

### TCSS Template

```tcss
/* Global variables */
Screen {
    background: $background;
}

/* Components */
.pane {
    border: solid $accent 40%;
    border-title-color: $text-accent 50%;
    
    &:focus-within {
        border: solid $accent 100%;
        border-title-color: $foreground;
        border-title-style: b;
    }
}

/* States */
.pane.success { border-title-color: $text-success; }
.pane.warning { border-title-color: $text-warning; }
.pane.error { border-title-color: $text-error; }

/* Buttons */
Button {
    background: $primary;
    color: $text;
}

Button:hover {
    background-tint: $text 10%;
    text-style: b;
}

Button:disabled {
    opacity: 40%;
    color: $text-muted;
}
```

This guide provides everything you need to build professional TUI applications with Textual. The patterns shown here can be adapted to create anything from simple utilities to complex multi-pane applications with dynamic layouts and sophisticated theming.

