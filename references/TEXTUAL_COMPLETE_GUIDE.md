# Complete Textual TUI Framework Guide

A comprehensive guide to building professional TUI applications with Textual, based on design patterns from the Posting HTTP client.

## Table of Contents

1. [Introduction](#introduction)
2. [Architecture & Layout](#architecture--layout)
3. [Widgets & Components](#widgets--components)
4. [Dynamic Pane Headers](#dynamic-pane-headers)
5. [Color Theming System](#color-theming-system)
6. [Button Styling & Implementation](#button-styling--implementation)
7. [Practical Examples](#practical-examples)
8. [Best Practices](#best-practices)

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

## Widgets & Components

### Understanding Textual Widgets

Widgets are the building blocks of Textual applications. They handle rendering, events, and user interaction. Textual provides both built-in widgets and allows you to create custom ones.

#### Widget Hierarchy

```
Widget (Base class)
├── Static              - Display static or dynamic content
├── Input               - Text input field
├── Button              - Clickable button
├── Select              - Dropdown selection
├── Checkbox            - Boolean toggle
├── Tree                - Hierarchical navigation
├── DataTable           - Tabular data display
├── RichLog             - Scrollable text output
├── TextArea            - Multi-line code/text editor
├── TabbedContent       - Tab interface
└── Custom Widgets      - Extend any widget
```

### Essential Built-in Widgets

#### Static

The `Static` widget displays content and is the base for custom components:

```python
from textual.widgets import Static
from textual.app import ComposeResult

class ContentDisplay(Static):
    """Display content with optional border and styling."""
    
    def render(self) -> str:
        """Render the content."""
        return "Hello, TUI World!"

# Usage:
yield Static("Simple text content")
yield Static("Content", id="my-content", classes="panel")
```

Update content dynamically:

```python
class DynamicDisplay(Static):
    def update_content(self, content: str) -> None:
        """Update displayed content."""
        self.update(content)

# From another component:
display = self.query_one("#content", DynamicDisplay)
display.update_content("New content here")
```

#### Input

Text input for user input:

```python
from textual.widgets import Input
from textual import on

class FormSection(Static):
    """Form with text inputs."""
    
    def compose(self) -> ComposeResult:
        yield Input(placeholder="Enter name", id="name-input")
        yield Input(placeholder="Enter email", id="email-input")
    
    @on(Input.Changed)
    def on_input_changed(self, event: Input.Changed) -> None:
        """Handle input changes."""
        if event.input.id == "name-input":
            # Handle name change
            pass
    
    @on(Input.Submitted)
    def on_input_submitted(self, event: Input.Submitted) -> None:
        """Handle when user presses Enter."""
        pass

# Access input values:
name_input = self.query_one("#name-input", Input)
name_value = name_input.value
```

Create a custom input with specific behavior:

```python
class PostingInput(Input):
    """Custom input with theme-aware cursor."""
    
    def on_mount(self) -> None:
        # Customize based on settings
        self.cursor_blink = True
        self.cursor_style = "block"
```

#### DataTable

Display tabular data:

```python
from textual.widgets import DataTable

class TableDisplay(Static):
    """Display data in table format."""
    
    def compose(self) -> ComposeResult:
        table = DataTable()
        table.add_columns("Name", "Email", "Status")
        table.add_row("Alice", "alice@example.com", "Active")
        table.add_row("Bob", "bob@example.com", "Inactive")
        yield table
    
    def on_mount(self) -> None:
        table = self.query_one(DataTable)
        table.focus()
```

Custom DataTable with checkboxes:

```python
from textual.widgets import DataTable

class CheckableDataTable(DataTable):
    """DataTable with checkbox support."""
    
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.checked_rows = set()
    
    def on_key(self, event) -> None:
        """Handle space bar to toggle checkbox."""
        if event.key == "space":
            row_key = self.cursor_row
            self.checked_rows.toggle(row_key)
            self.refresh()
```

#### Tree

Hierarchical navigation:

```python
from textual.widgets import Tree, Static

class DirectoryBrowser(Static):
    """Browse directory structure."""
    
    def compose(self) -> ComposeResult:
        tree = Tree("root")
        root = tree.root
        root.expand()
        
        projects = root.add("projects/")
        projects.add("project1/")
        projects.add("project2/")
        
        files = root.add("files/")
        files.add("README.md")
        files.add("LICENSE")
        
        yield tree
```

Custom Tree with keyboard shortcuts:

```python
from textual.widgets import Tree
from textual.binding import Binding

class PostingTree(Tree):
    """Tree with vim-like keybindings."""
    
    BINDINGS = [
        Binding("k", "cursor_up", "Up"),
        Binding("j", "cursor_down", "Down"),
        Binding("h", "collapse", "Collapse"),
        Binding("l", "expand", "Expand"),
    ]
```

#### Select/Dropdown

Selection dropdown:

```python
from textual.widgets import Select

class MethodSelector(Static):
    """Select HTTP method."""
    
    def compose(self) -> ComposeResult:
        yield Select([
            ("GET", "GET"),
            ("POST", "POST"),
            ("PUT", "PUT"),
            ("DELETE", "DELETE"),
            ("PATCH", "PATCH"),
        ], id="method-select")
    
    def on_mount(self) -> None:
        select = self.query_one("#method-select", Select)
        select.value = "GET"
```

#### TextArea

Multi-line code editor:

```python
from textual.widgets import TextArea

class CodeEditor(Static):
    """Edit code with syntax highlighting."""
    
    def compose(self) -> ComposeResult:
        editor = TextArea(language="json")
        yield editor
    
    def on_mount(self) -> None:
        editor = self.query_one(TextArea)
        editor.text = '{"key": "value"}'
```

#### RichLog

Scrollable log output:

```python
from textual.widgets import RichLog

class OutputLog(Static):
    """Display execution output."""
    
    def compose(self) -> ComposeResult:
        log = RichLog(markup=True)
        yield log
    
    def on_mount(self) -> None:
        log = self.query_one(RichLog)
        log.write("[green]✓[/green] Operation successful")
        log.write("[red]✗[/red] Operation failed")
```

#### TabbedContent

Tab interface:

```python
from textual.widgets import TabbedContent, TabPane, Static

class RequestEditor(Static):
    """Request editor with multiple tabs."""
    
    def compose(self) -> ComposeResult:
        with TabbedContent():
            with TabPane("Body", id="body-tab"):
                yield Static("Request body goes here")
            with TabPane("Headers", id="headers-tab"):
                yield Static("Headers go here")
            with TabPane("Params", id="params-tab"):
                yield Static("Query parameters go here")
```

### Creating Custom Widgets

Extend widgets to create specialized components:

```python
class KeyValuePair(Static):
    """A key-value input pair."""
    
    DEFAULT_CSS = """
    KeyValuePair {
        height: 1;
        layout: horizontal;
    }
    
    KeyValuePair #key {
        width: 20%;
    }
    
    KeyValuePair #value {
        width: 1fr;
    }
    """
    
    def __init__(self, key: str = "", value: str = "", *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.key = key
        self.value = value
    
    def compose(self) -> ComposeResult:
        from textual.widgets import Input
        yield Input(value=self.key, id="key")
        yield Input(value=self.value, id="value")
    
    def get_pair(self) -> tuple[str, str]:
        """Get current key and value."""
        key_input = self.query_one("#key", Input)
        value_input = self.query_one("#value", Input)
        return (key_input.value, value_input.value)
```

Message-based communication:

```python
from textual.message import Message

class KeyValueInput(Static):
    """Key-value input that sends messages."""
    
    class Submitted(Message):
        """Posted when the user submits the pair."""
        def __init__(self, key: str, value: str):
            super().__init__()
            self.key = key
            self.value = value
    
    def compose(self) -> ComposeResult:
        from textual.widgets import Input, Button
        yield Input(id="key")
        yield Input(id="value")
        yield Button("Add", id="add-btn")
    
    @on(Button.Pressed, "#add-btn")
    def submit(self) -> None:
        key_input = self.query_one("#key", Input)
        value_input = self.query_one("#value", Input)
        self.post_message(self.Submitted(key_input.value, value_input.value))
```

### Widget State Management

Use reactive properties for widget state:

```python
from textual.reactive import reactive

class FilterableList(Static):
    """List with filter capability."""
    
    items: reactive[list] = reactive([])
    filter_text: reactive[str] = reactive("")
    
    def watch_items(self, items: list) -> None:
        """Update display when items change."""
        self.refresh()
    
    def watch_filter_text(self, text: str) -> None:
        """Update display when filter changes."""
        self.refresh()
    
    def render(self) -> str:
        """Render filtered items."""
        filtered = [
            item for item in self.items
            if self.filter_text.lower() in item.lower()
        ]
        return "\n".join(filtered)
```

### Widget Composition Patterns

Compose complex UIs from simpler widgets:

```python
class HeaderWithIcon(Static):
    """Header with icon and title."""
    
    def __init__(self, icon: str, title: str, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.icon = icon
        self.title = title
    
    def render(self) -> str:
        return f"{self.icon} {self.title}"

class RequestForm(Static):
    """Complete form for HTTP request."""
    
    def compose(self) -> ComposeResult:
        from textual.widgets import Input, Select, Button
        from textual.containers import Vertical, Horizontal
        
        with Vertical():
            yield HeaderWithIcon("📝", "Request Details")
            
            with Horizontal():
                yield Select([("GET", "GET"), ("POST", "POST")], id="method")
                yield Input(placeholder="URL", id="url")
            
            yield Input(placeholder="Body", id="body")
            
            with Horizontal():
                yield Button("Send", id="send")
                yield Button("Clear", id="clear")
```

### Modal Dialogs

Create modal dialogs with custom widgets:

```python
from textual.screen import ModalScreen
from textual.containers import Vertical, Horizontal
from textual.widgets import Button, Static, Input

class ConfirmDialog(ModalScreen[bool]):
    """Confirmation dialog widget."""
    
    DEFAULT_CSS = """
    ConfirmDialog {
        align: center middle;
    }
    
    ConfirmDialog #content {
        border: solid $accent;
        width: 50;
        height: 10;
    }
    """
    
    def __init__(self, message: str, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.message = message
    
    def compose(self) -> ComposeResult:
        with Vertical(id="content"):
            yield Static(self.message)
            with Horizontal():
                yield Button("Yes", id="yes")
                yield Button("No", id="no")
    
    @on(Button.Pressed, "#yes")
    def confirm(self) -> None:
        self.dismiss(True)
    
    @on(Button.Pressed, "#no")
    def cancel(self) -> None:
        self.dismiss(False)
```

Use the modal:

```python
async def show_confirmation(self, message: str) -> bool:
    """Show confirmation dialog and wait for result."""
    result = await self.app.push_screen_wait(ConfirmDialog(message))
    return result
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

## Widget Patterns from Posting

### Pattern 1: Custom DataTable with Extended Functionality

Posting extends DataTable with checkboxes and custom keybindings:

```python
from textual.widgets import DataTable

class PostingDataTable(DataTable):
    """DataTable with checkbox and vim keybindings."""
    
    BINDINGS = [
        Binding("k", "cursor_up", "Up", show=False),
        Binding("j", "cursor_down", "Down", show=False),
        Binding("h", "cursor_left", "Left", show=False),
        Binding("l", "cursor_right", "Right", show=False),
        Binding("f", "toggle_fixed_columns", "Toggle Fixed", show=False),
    ]
    
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.cursor_vertical_escape = True
        self.row_disable = False

class Checkbox:
    """Checkbox for table rows."""
    checked: bool = True
    
    def toggle(self) -> bool:
        self.checked = not self.checked
        return self.checked
```

### Pattern 2: Custom Input with Theme Awareness

Posting creates a themed input field:

```python
class PostingInput(Input):
    """Input with theme-aware styling."""
    
    def on_mount(self) -> None:
        self.cursor_blink = SETTINGS.get().text_input.blinking_cursor
        self.app.theme_changed_signal.subscribe(self, self.on_theme_change)
    
    @property
    def cursor_style(self) -> Style:
        return self.get_component_rich_style("input--cursor")
    
    def on_theme_change(self, theme) -> None:
        self.refresh()
```

### Pattern 3: Key-Value Input Pairs

Posting uses specialized key-value widgets for headers/query params:

```python
class KeyValueInput(Horizontal):
    """Input pair for key-value data."""
    
    class Change(Message):
        key: str
        value: str
    
    edit_mode: Reactive[bool] = reactive(False)
    
    def compose(self) -> ComposeResult:
        yield Input(id="key-input")
        yield Input(id="value-input")
        yield Button("Add", id="add-button")
    
    @property
    def submit_allowed(self) -> bool:
        key_input = self.query_one("#key-input", Input)
        value_input = self.query_one("#value-input", Input)
        return bool(key_input.value and value_input.value)
```

### Pattern 4: TextArea with Custom Footer

Posting adds a footer to TextArea for language and options:

```python
class TextAreaFooter(Horizontal):
    """Footer for TextArea with language selection."""
    
    class LanguageChanged(Message):
        language: str | None
    
    language: Reactive[str | None] = reactive("json")
    soft_wrap: Reactive[bool] = reactive(True)
    
    def compose(self) -> ComposeResult:
        yield Select([
            ("JSON", "json"),
            ("Python", "python"),
            ("XML", "xml"),
        ], id="language-select")
        yield Checkbox("Soft Wrap", id="soft-wrap")
    
    def watch_language(self, language: str) -> None:
        self.post_message(self.LanguageChanged(language))
```

### Pattern 5: Tree with Vim Keybindings

Posting extends Tree with vim-like navigation:

```python
class PostingTree(Tree):
    """Tree with vim-style keybindings."""
    
    BINDINGS = [
        Binding("k", "cursor_up", "Up", show=False),
        Binding("j", "cursor_down", "Down", show=False),
        Binding("K", "cursor_up_parent", "Parent", show=False),
        Binding("J", "cursor_down_parent", "Child", show=False),
        Binding("g", "scroll_home", "Home", show=False),
        Binding("G", "scroll_end", "End", show=False),
        Binding("space,r", "toggle_node", "Toggle", show=False),
    ]
    
    def action_cursor_up_parent(self) -> None:
        """Move to previous collapsible node."""
        for line in range(self.cursor_line - 1, -1, -1):
            node = self.get_node_at_line(line)
            if node and node.allow_expand:
                self.cursor_line = line
                return
```

### Pattern 6: RichLog for Styled Output

Posting uses RichLog to display script output:

```python
class RichLogIO(StringIO):
    """Redirect stdout/stderr to RichLog."""
    
    def write(self, s: str) -> int:
        lines = s.splitlines(True)
        for line in lines:
            if line.endswith("\n"):
                self._flush_line(line.rstrip("\n"))
        return len(s)
    
    def _flush_line(self, line: str) -> None:
        if self.stream_type == "stdout":
            self.rich_log.write(f" [green]out[/green] {line}")
        else:
            self.rich_log.write(f" [red]err[/red] {line}")
```

### Pattern 7: Select with Vim Keybindings

Posting customizes Select widget:

```python
class PostingSelect(Select):
    """Select with vim keybindings."""
    
    BINDINGS = [
        Binding("enter,space,l", "show_overlay", "Show", show=False),
        Binding("up,k", "cursor_up", "Up", show=False),
        Binding("down,j", "cursor_down", "Down", show=False),
    ]
    
    def action_cursor_up(self):
        if self.expanded:
            self.select_overlay.action_cursor_up()
        else:
            self.screen.focus_previous()
    
    def action_cursor_down(self):
        if self.expanded:
            self.select_overlay.action_cursor_down()
        else:
            self.screen.focus_next()
```

### Pattern 8: Modal Confirmation Dialog

Posting uses modal screens for confirmations:

```python
class ConfirmationModal(ModalScreen[bool]):
    """Confirmation dialog."""
    
    def __init__(
        self,
        message: str,
        confirm_text: str = "Yes [y]",
        cancel_text: str = "No [n]",
        auto_focus: Literal["confirm", "cancel"] = "confirm",
    ):
        super().__init__()
        self.message = message
        self.confirm_text = confirm_text
        self.cancel_text = cancel_text
        self.auto_focus = auto_focus
    
    def on_mount(self) -> None:
        self._bindings.bind("y", "screen.dismiss(True)")
        self._bindings.bind("n", "screen.dismiss(False)")
        self._bindings.bind("escape", "screen.dismiss(False)")
    
    def compose(self) -> ComposeResult:
        with Vertical():
            yield Static(self.message)
            with Horizontal():
                yield Button(self.confirm_text, id="confirm-button")
                yield Button(self.cancel_text, id="cancel-button")
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

### 11. Widget-Specific Tips

#### DataTable Tips

✅ **Good**: Extend DataTable for specialized behavior

```python
class CustomTable(DataTable):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.cursor_vertical_escape = True  # Allow up-arrow to focus previous widget
```

#### Input Tips

✅ **Good**: Handle Input events for validation

```python
@on(Input.Changed)
def validate_input(self, event: Input.Changed) -> None:
    if not is_valid(event.value):
        event.input.add_class("error")
    else:
        event.input.remove_class("error")
```

#### TreeView Tips

✅ **Good**: Extend Tree for custom navigation

```python
class NavigationTree(Tree):
    BINDINGS = [
        Binding("k", "cursor_up"),
        Binding("j", "cursor_down"),
        Binding("enter,l", "select_cursor"),
        Binding("space,r", "toggle_node"),
    ]
```

#### TextArea Tips

✅ **Good**: Provide language selection for proper highlighting

```python
text_area = TextArea(language="json")
text_area.language = "python"  # Change language dynamically
```

#### Modal Dialog Tips

✅ **Good**: Use ModalScreen for focused interactions

```python
async def confirm_action(self, message: str) -> bool:
    result = await self.app.push_screen_wait(
        ConfirmDialog(message)
    )
    return result
```

#### Custom Widget Tips

✅ **Good**: Use Message classes for inter-widget communication

```python
class CustomWidget(Static):
    class Submitted(Message):
        def __init__(self, value: str):
            super().__init__()
            self.value = value
    
    def submit(self, value: str) -> None:
        self.post_message(self.Submitted(value))
```

✅ **Good**: Define DEFAULT_CSS for widget styling

```python
class MyWidget(Static):
    DEFAULT_CSS = """
    MyWidget {
        width: 100%;
        border: solid $accent;
        padding: 1 2;
    }
    """
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

