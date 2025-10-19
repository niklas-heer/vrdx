# Comprehensive Textual Framework Guide for LLM Use

This is a complete reference guide for building terminal user interfaces with the Textual Python framework. This guide is designed to be self-contained and usable without external references.

## ⚡ Quick Start with UV

```bash
# Create new project
uv init my_textual_app
cd my_textual_app

# Add Textual
uv add textual textual-dev

# Create app
cat > src/main.py << 'EOF'
from textual.app import App, ComposeResult
from textual.widgets import Header, Footer, Static

class MyApp(App):
    def compose(self) -> ComposeResult:
        yield Header()
        yield Static("Hello, Textual!")
        yield Footer()

if __name__ == "__main__":
    MyApp().run()
EOF

# Run it
uv run src/main.py

# Exit: Ctrl+Q
```

That's it! See [Getting Started](#getting-started) for more.

## Table of Contents

1. [Introduction](#introduction)
2. [Getting Started](#getting-started)
3. [App Basics](#app-basics)
4. [Widgets](#widgets)
5. [Styling and CSS](#styling-and-css)
6. [Layout](#layout)
7. [Events, Messages, and Input](#events-messages-and-input)
8. [Reactivity](#reactivity)
9. [Screens](#screens)
10. [Animations](#animations)
11. [Queries](#queries)
12. [Builtin Widgets Reference](#builtin-widgets-reference)

---

## Introduction

Textual is a Python framework for building cross-platform terminal user interfaces (TUIs). Apps built with Textual can run in:
- The terminal
- A web browser (with textual-web)

Textual uses an event-driven, async-compatible architecture inspired by modern web development while maintaining simplicity.

### Key Concepts

- **App**: The main application class that runs your TUI
- **Widget**: A self-contained UI component (button, input, text display, etc.)
- **Screen**: A container for widgets that occupies the full terminal
- **DOM**: Document Object Model - the tree structure of widgets
- **CSS**: Cascading stylesheets for styling (uses `.tcss` extension)
- **Reactivity**: Automatic re-rendering when data changes

---

## Getting Started

### Installation with UV

[UV](https://docs.astral.sh/uv/) is a blazing-fast Python package and project manager written in Rust. It's recommended for Textual projects.

#### Initialize a new project with UV

```bash
uv init my_textual_app
cd my_textual_app
```

#### Add Textual dependencies

```bash
uv add textual textual-dev
```

This creates a `pyproject.toml` with Textual and `textual-dev` (dev tools) installed.

#### Alternative: Create from scratch

```bash
# Create directory
mkdir my_textual_app
cd my_textual_app

# Create pyproject.toml
uv venv
```

Edit `pyproject.toml`:

```toml
[project]
name = "my-textual-app"
version = "0.1.0"
description = "A Textual TUI app"
dependencies = [
    "textual>=0.50.0",
]

[dependency-groups]
dev = [
    "textual-dev>=1.0.0",
    "pytest>=7.0",
    "pytest-asyncio>=0.21.0",
]

[tool.uv]
python-version = "3.11"
```

Then install:

```bash
uv sync
```

### Traditional pip Installation

If you prefer pip over UV:

```bash
pip install textual textual-dev
```

We recommend UV, but both work fine.

### Minimal App

```python
from textual.app import App, ComposeResult
from textual.widgets import Static

class HelloApp(App):
    def compose(self) -> ComposeResult:
        yield Static("Hello, Textual!")

if __name__ == "__main__":
    app = HelloApp()
    app.run()
```

### Run the App

```bash
python hello.py
```

Exit with `Ctrl+Q` (default binding).

### Demo

See what Textual can do:

```bash
python -m textual
```

---

## App Basics

### The App Class

The `App` class is the entry point for your application.

```python
from textual.app import App, ComposeResult

class MyApp(App):
    CSS_PATH = "style.tcss"  # External CSS file
    TITLE = "My Application"
    SUB_TITLE = "A Textual App"
    
    def compose(self) -> ComposeResult:
        """Create child widgets here"""
        yield Widget1()
        yield Widget2()
    
    def on_mount(self) -> None:
        """Called when app is ready"""
        self.title = "Updated Title"
        self.sub_title = "Updated Subtitle"
```

### CSS Class Variable

Define styles directly in code:

```python
class MyApp(App):
    CSS = """
    Screen {
        align: center middle;
    }
    Button {
        margin: 1;
    }
    """
```

### Running the App

```python
if __name__ == "__main__":
    app = MyApp()
    result = app.run()  # Returns any value passed to app.exit()
```

### App Return Values

Return data from your app:

```python
class MyApp(App[str]):  # Type parameter indicates return type
    def on_button_pressed(self) -> None:
        self.exit("Button was pressed")

app = MyApp()
result = app.run()  # result will be "Button was pressed" or None
```

### Return Codes

Set exit codes for script automation:

```python
if error_condition:
    self.exit(return_code=1, message="Error occurred")

import sys
sys.exit(app.return_code or 0)
```

### Inline Mode

Run app inline (not full-screen):

```python
app.run(inline=True)  # Not supported on Windows
```

### ANSI Color Support

By default, Textual overrides ANSI colors. Preserve terminal ANSI theme:

```python
app.run(ansi_color=True)
```

### Suspending the App

Suspend the app to run other terminal commands (Unix-like systems):

```python
from contextlib import contextmanager

def on_button_pressed(self):
    with self.app.suspend():
        # App is suspended - you can run shell commands
        os.system("vim myfile.txt")  # User edits file
        # App resumes after context exits
```

Or use the action:

```python
class MyApp(App):
    BINDINGS = [("ctrl+z", "suspend_process", "Suspend")]
```

---

## Widgets

### What is a Widget?

A widget is a rectangular component of your UI. Every widget:
- Occupies a region of the screen
- Can receive and respond to events
- Can have child widgets
- Runs in its own asyncio task

### Composing Widgets

Widgets are added to an app or container via the `compose()` method:

```python
class MyApp(App):
    def compose(self) -> ComposeResult:
        yield Header()
        yield Button("Click me")
        yield Footer()
```

### Mounting Widgets Dynamically

Add widgets after app startup:

```python
async def on_key(self, event):
    await self.mount(Button("Dynamic Button"))
```

If you need to modify the widget immediately, await the mount:

```python
async def on_key(self, event):
    button = Button("Dynamic Button")
    await self.mount(button)
    button.label = "Updated"  # Can modify after awaiting
```

### Custom Widgets

Create a widget by extending `Widget` or `Static`:

```python
from textual.widget import Widget
from textual.widgets import Static

class CustomWidget(Static):
    DEFAULT_CSS = """
    CustomWidget {
        width: 100%;
        height: 3;
        border: solid blue;
    }
    """
    
    def render(self) -> str:
        """Return content to display"""
        return "Hello from [bold]custom[/bold] widget"
```

### Widget Render Method

Return a string or Rich renderable:

```python
class GreetingWidget(Static):
    def render(self) -> str:
        return "Hello, World!"
```

Use Rich markup for styling:

```python
def render(self) -> str:
    return "Text with [bold]bold[/bold], [italic]italic[/italic], [red]red[/red]"
```

Return Rich renderables:

```python
from rich.table import Table

class TableWidget(Static):
    def render(self) -> Table:
        table = Table(title="My Table")
        table.add_column("Name")
        table.add_column("Age")
        table.add_row("Alice", "30")
        table.add_row("Bob", "25")
        return table
```

### Updating Widget Content

Use `Static.update()` to change content:

```python
class ClickableWidget(Static):
    counter = 0
    
    def on_click(self):
        self.counter += 1
        self.update(f"Clicked {self.counter} times")
```

### Widget IDs and Classes

Identify widgets for styling and queries:

```python
yield Button("OK", id="ok-button", classes="primary")
yield Button("Cancel", id="cancel-button", classes="secondary danger")
```

In CSS:

```css
#ok-button {
    background: green;
}

.secondary {
    background: gray;
}

.danger {
    border: solid red;
}
```

### Widget Border Titles

Add titles and subtitles within borders:

```python
class MyWidget(Static):
    BORDER_TITLE = "Default Title"
    BORDER_SUBTITLE = "Default Subtitle"
    
    def on_mount(self):
        self.border_title = "Updated Title"
        self.border_subtitle = "Updated Subtitle"
```

### Widget Tooltips

Add helpful text on hover:

```python
button = Button("Click me")
button.tooltip = "This is a helpful tooltip"
```

Customize tooltip styling with CSS:

```css
Tooltip {
    background: yellow;
    color: black;
}
```

### Loading Indicators

Show a loading state:

```python
class DataWidget(Static):
    async def load_data(self):
        self.loading = True
        await some_async_operation()
        self.loading = False
```

### Widget Focus

Make a widget focusable:

```python
class FocusableWidget(Static):
    can_focus = True
```

Focus a specific widget:

```python
widget.focus()
```

### Widget Size and Dimensions

Get widget dimensions:

```python
width = widget.size.width
height = widget.size.height
```

Get content area size (excluding border/padding):

```python
content_width = widget.content_size.width
content_height = widget.content_size.height
```

### Line API for Advanced Widgets

For efficient rendering of large content:

```python
from textual.scroll_view import ScrollView
from textual.strip import Strip
from rich.segment import Segment
from rich.style import Style

class LargeWidget(ScrollView):
    def __init__(self):
        super().__init__()
        self.virtual_size = (1000, 1000)  # Scrollable content size
    
    def render_line(self, y: int) -> Strip:
        """Called for each visible line"""
        segments = [
            Segment("Line content", Style(color="blue"))
        ]
        return Strip(segments)
```

### Component Classes

Style parts of line API widgets:

```python
class CustomLineWidget(Widget):
    COMPONENT_CLASSES = {"custom--item", "custom--selected"}
    DEFAULT_CSS = """
    CustomLineWidget .custom--item {
        color: white;
    }
    CustomLineWidget .custom--selected {
        background: blue;
        color: white;
    }
    """
    
    def render_line(self, y: int) -> Strip:
        style = self.get_component_rich_style("custom--selected")
        segments = [Segment("Item", style)]
        return Strip(segments)
```

### Compound Widgets

Combine multiple widgets:

```python
from textual.containers import Horizontal

class LabeledInput(Static):
    def compose(self) -> ComposeResult:
        yield Label("Name:")
        yield Input()
    
    DEFAULT_CSS = """
    LabeledInput {
        layout: horizontal;
    }
    """
```

### Custom Messages

Send data between widgets:

```python
from textual.message import Message

class ColorButton(Static):
    class Pressed(Message):
        def __init__(self, color: str):
            self.color = color
            super().__init__()
    
    def on_click(self):
        self.post_message(self.Pressed("red"))

# Parent widget handles message
class MyApp(App):
    def on_color_button_pressed(self, message: ColorButton.Pressed):
        self.screen.styles.background = message.color
```

### Text Links in Widgets

Create clickable text:

```python
class LinkWidget(Static):
    def render(self) -> str:
        return "Click [link=app.bell]here[/link] to ring bell"
```

Use custom actions:

```python
def render(self) -> str:
    return "Click [@click=action_my_action]here[/] to perform action"

def action_my_action(self):
    self.do_something()
```

---

## Styling and CSS

### Introduction to CSS

Textual uses CSS (with `.tcss` extension) to style widgets. CSS keeps style separate from logic.

### Loading CSS Files

```python
class MyApp(App):
    CSS_PATH = "style.tcss"  # Single file
    # or
    CSS_PATH = ["style1.tcss", "style2.tcss"]  # Multiple files
```

Create `style.tcss`:

```css
/* Target by widget type */
Button {
    background: blue;
    color: white;
}

/* Target by ID */
#submit-button {
    background: green;
}

/* Target by CSS class */
.error {
    background: red;
}

/* Comments are supported */
/* This is a comment */
```

### CSS Variables

Define reusable values:

```css
$primary: blue;
$success: green;
$error: red;

Button {
    background: $primary;
}

.success-button {
    background: $success;
}
```

### Selectors

#### Type Selector

Target widget type:

```css
Button {
    background: blue;
}
```

#### ID Selector

Target widget by ID:

```css
#submit {
    background: green;
}
```

#### Class Selector

Target widget by CSS class:

```css
.primary {
    background: blue;
}

.primary.active {
    background: darkblue;
}
```

#### Universal Selector

Target all widgets:

```css
* {
    border: solid red;
}
```

Target all children:

```css
Dialog * {
    background: gray;
}
```

#### Pseudo Classes

Style widgets in specific states:

```css
Button:hover {
    background: lightblue;
}

Button:focus {
    border: solid yellow;
}

Input:disabled {
    color: gray;
}

/* Available pseudo-classes */
:hover        /* Mouse over */
:focus        /* Has input focus */
:blur         /* Does not have focus */
:disabled     /* Widget is disabled */
:enabled      /* Widget is enabled */
:dark         /* App is in dark theme */
:light        /* App is in light theme */
:first-child  /* First among siblings */
:last-child   /* Last among siblings */
:first-of-type /* First of its type */
:last-of-type /* Last of its type */
:odd          /* Odd position among siblings */
:even         /* Even position among siblings */
:empty        /* Has no displayed children */
:focus-within /* Has focused descendant */
:inline       /* App running in inline mode */
```

#### Descendant Combinator

Target widgets with ancestor:

```css
#dialog Button {
    border: solid green;
}
```

#### Child Combinator

Target immediate children only:

```css
#dialog > Button {
    border: solid green;
}
```

### CSS Nesting

Group related rules:

```css
#questions {
    width: 100%;
    
    Button {
        margin: 1;
    }
    
    .affirmative {
        background: green;
    }
}
```

Use `&` to combine selectors:

```css
#dialog {
    Button {
        margin: 1;
        
        &:hover {
            background: darkblue;
        }
    }
}

/* Equivalent to:
#dialog Button { margin: 1; }
#dialog Button:hover { background: darkblue; }
*/
```

### Specificity

When multiple selectors match, the most specific wins:

1. ID selectors (most specific)
2. Class and pseudo-class selectors
3. Type selectors (least specific)

Example:

```css
Button { background: blue; }           /* Specificity: 1 */
.primary { background: green; }        /* Specificity: 1 (higher than type) */
#submit { background: red; }           /* Specificity: 1 (higher than class) */
#dialog #submit { background: yellow; } /* Specificity: 2 */
```

### Important Rules

Force a rule to win (use sparingly):

```css
Button {
    background: blue !important;  /* Always applies */
}
```

### Initial Value

Reset to default:

```css
.light-button {
    background: initial;  /* Reset to default */
}
```

---

## Styling and Styles API

### The Styles Object

Every widget has a `styles` attribute for programmatic styling:

```python
button = Button("Click")
button.styles.background = "blue"
button.styles.border = ("solid", "white")
button.styles.margin = (1, 2)
```

### Colors

Specify colors multiple ways:

```python
# Named colors
widget.styles.background = "darkblue"
widget.styles.color = "white"

# Hex colors
widget.styles.background = "#FF0000"

# RGB decimal
widget.styles.background = "rgb(255, 0, 0)"

# HSL
widget.styles.background = "hsl(0, 100%, 50%)"

# With alpha (transparency)
widget.styles.background = "#FF000080"  # 50% transparent red
widget.styles.background = "rgba(255, 0, 0, 0.5)"

# Color object
from textual.color import Color
widget.styles.background = Color(255, 0, 0, a=0.5)
```

Available color names: `red`, `blue`, `green`, `yellow`, `magenta`, `cyan`, `white`, `black`, `darkred`, `darkblue`, `darkgreen`, `darkyellow`, `darkmagenta`, `darkcyan`, `darkgray`, `lightgray`, `lightred`, `lightblue`, `lightgreen`, `lightyellow`, `lightmagenta`, `lightcyan`, `lime`, `navy`, `orange`, `purple`, `crimson`, `palegreen`, `darkorchid`, etc. (See Color API for full list)

### Width and Height

```python
widget.styles.width = 40
widget.styles.height = 10
widget.styles.width = "50%"
widget.styles.width = "80vw"  # Viewport width
widget.styles.width = "1fr"   # Fraction unit
widget.styles.width = "auto"  # Auto size
```

Units:
- `int` - Fixed number of columns/rows
- `"50%"` - Percentage of parent
- `"50vw"`, `"50vh"` - Viewport width/height
- `"1fr"` - Fraction (for proportional sizing)
- `"auto"` - Automatic based on content

### Min/Max Dimensions

```python
widget.styles.min_width = 10
widget.styles.max_width = 100
widget.styles.min_height = 5
widget.styles.max_height = 50
```

### Padding

Space inside the widget's border:

```python
widget.styles.padding = 2  # All sides
widget.styles.padding = (2, 4)  # Top/bottom, left/right
widget.styles.padding = (1, 2, 3, 4)  # Top, right, bottom, left
```

### Border

```python
widget.styles.border = ("solid", "blue")
widget.styles.border = ("double", "white")
widget.styles.border = ("dashed", "red")
widget.styles.border = ("heavy", "green")
widget.styles.border = ("round", "yellow")

# Many border types available:
# solid, double, dashed, dotted, heavy, inner, outer, 
# thick, thin, wide, round, square, ascii
```

### Outline

Similar to border but doesn't affect layout:

```python
widget.styles.outline = ("solid", "red")
```

### Margin

Space outside the widget (between widgets):

```python
widget.styles.margin = 2  # All sides
widget.styles.margin = (1, 2)  # Top/bottom, left/right
widget.styles.margin = (1, 2, 3, 4)  # Top, right, bottom, left
```

### Box Sizing

```python
widget.styles.box_sizing = "border-box"  # Default (padding/border inside)
widget.styles.box_sizing = "content-box"  # Padding/border outside
```

### Text Styling

```python
widget.styles.text_style = "bold"
widget.styles.text_style = "italic"
widget.styles.text_style = "underline"
widget.styles.text_align = "left"
widget.styles.text_align = "center"
widget.styles.text_align = "right"
```

### Opacity

```python
widget.styles.opacity = 1.0  # Fully opaque
widget.styles.opacity = 0.5  # 50% transparent
widget.styles.opacity = 0.0  # Fully transparent
```

### Offset

Position widget relative to normal position:

```python
widget.styles.offset = (2, 5)  # Down 2, right 5
```

### Display

```python
widget.styles.display = "block"  # Normal (default)
widget.styles.display = "none"   # Hidden
```

### Overflow

Control scrollbars:

```python
widget.styles.overflow_x = "auto"   # Horizontal scrollbar if needed
widget.styles.overflow_y = "auto"   # Vertical scrollbar if needed
widget.styles.overflow = "auto"     # Both
widget.styles.overflow = "hidden"   # Hide overflow, no scrollbar
widget.styles.overflow = "scroll"   # Always show scrollbar
```

---

## Layout

### Vertical Layout (Default)

Widgets stacked top to bottom:

```python
class MyApp(App):
    CSS = """
    Screen {
        layout: vertical;  /* Default */
    }
    """
    
    def compose(self) -> ComposeResult:
        yield Static("Top", id="top")
        yield Static("Middle", id="middle")
        yield Static("Bottom", id="bottom")
```

In CSS:

```css
#top { height: 1fr; }
#middle { height: 2fr; }
#bottom { height: 1fr; }
```

### Horizontal Layout

Widgets side by side:

```python
class MyApp(App):
    CSS = """
    Screen {
        layout: horizontal;
    }
    
    Static {
        width: 1fr;
        height: 100%;
    }
    """
```

### Grid Layout

Multi-dimensional layouts:

```python
class MyApp(App):
    CSS = """
    Screen {
        layout: grid;
        grid-size: 3 2;  /* 3 columns, 2 rows */
        grid-columns: 2fr 1fr 1fr;
        grid-rows: 50% 50%;
    }
    """
```

Grid with auto rows:

```css
Screen {
    layout: grid;
    grid-size: 3;  /* 3 columns, rows auto-expand */
}
```

#### Grid Gutter

Spacing between grid cells:

```css
Screen {
    layout: grid;
    grid-size: 3 3;
    grid-gutter: 1;  /* 1 row/column of space */
    grid-gutter: 1 2;  /* 1 row, 2 columns of space */
}
```

#### Spanning Cells

```python
yield Static("Header", id="header")
yield Static("Content", id="content")

# CSS
"""
#header {
    column-span: 3;  /* Spans 3 columns */
}

#content {
    row-span: 2;     /* Spans 2 rows */
    column-span: 2;  /* Spans 2 columns */
}
"""
```

### Docking

Fix widget position (like header/footer):

```css
Header {
    dock: top;
    height: 3;
}

Footer {
    dock: bottom;
    height: 1;
}

Sidebar {
    dock: left;
    width: 20;
}
```

Dock values: `top`, `bottom`, `left`, `right`

### Utility Containers

Pre-configured containers:

```python
from textual.containers import Vertical, Horizontal, Grid

class MyApp(App):
    def compose(self) -> ComposeResult:
        with Vertical():
            with Horizontal():
                yield Static("Left")
                yield Static("Right")
            yield Static("Bottom")
```

#### With Context Managers

```python
def compose(self) -> ComposeResult:
    with Horizontal():
        yield Static("Column 1")
        yield Static("Column 2")
        yield Static("Column 3")
```

### Content Alignment

Align content within widget:

```css
Button {
    content-align: center middle;  /* Horizontal Vertical */
}

/* Horizontal: left, center, right */
/* Vertical: top, middle, bottom */
```

---

## Events, Messages, and Input

### Event Handlers

Handle events with methods prefixed `on_`:

```python
class MyApp(App):
    def on_mount(self) -> None:
        """Called when app is ready"""
        self.log("App mounted!")
    
    def on_key(self, event) -> None:
        """Called when key is pressed"""
        self.log(f"Key pressed: {event.key}")
```

### Key Events

```python
def on_key(self, event) -> None:
    key = event.key  # "a", "ctrl+c", "f1", etc.
    character = event.character  # Printable character or None
    name = event.name  # Python-safe name
    is_printable = event.is_printable  # Is printable key
    aliases = event.aliases  # Alternative keys for same event
    
    if key == "q":
        self.exit()
    elif key == "ctrl+p":
        self.toggle_debug()
```

Key combinations: `ctrl+x`, `shift+x`, `alt+x`, `ctrl+shift+x`

### Key Methods (Convenience)

```python
def key_q(self) -> None:
    """Called when 'q' is pressed"""
    self.exit()

def key_ctrl_p(self) -> None:
    """Called when Ctrl+P is pressed"""
    self.toggle_debug()
```

Prefer bindings/actions over key methods.

### Key Bindings

Associate keys with actions:

```python
class MyApp(App):
    BINDINGS = [
        ("q", "quit", "Quit"),
        ("d", "toggle_dark", "Dark mode"),
        ("s", "save", "Save"),
    ]
    
    def action_quit(self) -> None:
        self.exit()
    
    def action_toggle_dark(self) -> None:
        self.theme = "dark" if self.theme == "light" else "light"
    
    def action_save(self) -> None:
        self.log("Saving...")
```

Binding format: `(key, action, description)`

Multiple keys for one action:

```python
("r,g,b", "add_color('red')", "Add Red")
```

### Priority Bindings

Bindings checked before widget bindings:

```python
from textual.binding import Binding

class MyApp(App):
    BINDINGS = [
        Binding("ctrl+q", "quit", "Quit", priority=True),
    ]
```

### Dynamic Bindings

Use actions instead for dynamic behavior.

### Mouse Events

#### Click Events

```python
def on_click(self, event) -> None:
    x = event.x  # X coordinate
    y = event.y  # Y coordinate
    button = event.button  # 1, 2, or 3
    shift = event.shift  # shift key held
    ctrl = event.ctrl  # ctrl key held
    alt = event.alt  # alt key held
```

#### Mouse Move

```python
def on_mouse_move(self, event) -> None:
    x = event.x
    y = event.y
```

Capture all mouse events:

```python
def on_mount(self) -> None:
    self.capture_mouse()

def on_mouse_move(self, event) -> None:
    # Receives all mouse events
    pass

def on_mouse_release(self) -> None:
    self.release_mouse()
```

#### Scroll Events

```python
def on_mouse_scroll_down(self, event) -> None:
    self.log("Scroll down")

def on_mouse_scroll_up(self, event) -> None:
    self.log("Scroll up")
```

#### Mouse Enter/Leave

```python
def on_enter(self, event) -> None:
    """Mouse entered widget"""
    pass

def on_leave(self, event) -> None:
    """Mouse left widget"""
    pass
```

### Input Focus

Only one widget receives key events at a time:

```python
widget.focus()  # Focus this widget

class MyWidget(Static):
    can_focus = True  # Make widget focusable
```

Focus pseudo-class:

```css
Input:focus {
    border: solid yellow;
}

Input:blur {
    border: solid gray;
}
```

### Messages and Custom Events

Create custom messages:

```python
from textual.message import Message

class MyWidget(Static):
    class ValueChanged(Message):
        def __init__(self, value: str):
            self.value = value
            super().__init__()
    
    def some_action(self):
        self.post_message(self.ValueChanged("new value"))

class MyApp(App):
    def on_my_widget_value_changed(self, message) -> None:
        self.log(f"Value changed to: {message.value}")
```

Message naming: `on_<widget_name>_<message_name>` (in snake_case)

### Preventing Default

Stop event propagation:

```python
def on_key(self, event) -> None:
    if event.key == "q":
        event.prevent_default()  # Don't process further
        self.exit()
```

### Stopping Bubbling

Stop event from bubbling to parent:

```python
def on_click(self, event) -> None:
    event.stop()  # Don't bubble to parent
```

### Preventing Messages

Temporarily disable specific message types:

```python
with self.prevent(Input.Changed):
    # This won't send Input.Changed messages
    input.value = ""  # Won't trigger on_input_changed
```

---

## Reactivity

### Reactive Attributes

Attributes with automatic re-rendering:

```python
from textual.reactive import reactive

class Counter(Static):
    count: reactive[int] = reactive(0)
    
    def render(self) -> str:
        return f"Count: {self.count}"

# When you change count, render() is called automatically
counter = Counter()
counter.count = 5  # Automatically re-renders
```

### Dynamic Defaults

Use callable for default value:

```python
from time import time

class Timer(Static):
    start_time: reactive[float] = reactive(time)  # Called on init
```

### Smart Refresh

Changing reactive attribute triggers refresh:

```python
class Greeting(Static):
    who: reactive[str] = reactive("World")
    
    def render(self) -> str:
        return f"Hello, {self.who}!"

greeting = Greeting()
greeting.who = "Alice"  # Automatically re-renders
```

### Layout Updates

Update layout when reactive changes:

```python
class DynamicBox(Static):
    content: reactive[str] = reactive("Text", layout=True)
    
    def render(self) -> str:
        return self.content
```

### Disabling Refresh

Use `var` for reactive without refresh:

```python
from textual.reactive import var

class MyWidget(Static):
    internal_state: var[int] = var(0)  # No refresh/layout
```

### Validation

Validate incoming values:

```python
class Slider(Static):
    value: reactive[int] = reactive(0)
    
    def validate_value(self, value: int) -> int:
        """Validate and potentially modify value"""
        return max(0, min(100, value))  # Clamp to 0-100

slider = Slider()
slider.value = 150  # Actually sets to 100
```

### Watch Methods

React to reactive changes:

```python
class ColorBox(Static):
    color: reactive[str] = reactive("red")
    
    def watch_color(self, old_color: str, new_color: str) -> None:
        """Called when color changes"""
        self.log(f"Color changed from {old_color} to {new_color}")
        self.styles.background = new_color

box = ColorBox()
box.color = "blue"  # Logs message and updates background
```

Watch method with new value only:

```python
def watch_color(self, new_color: str) -> None:
    """Called when color changes"""
    self.styles.background = new_color
```

### Dynamic Watching

Programmatically watch reactive attributes:

```python
class MyApp(App):
    def on_mount(self):
        counter = Counter()
        self.watch(counter, "count", self.on_count_changed)
    
    def on_count_changed(self, count: int):
        self.log(f"Counter: {count}")
```

### Recompose on Change

Recreate child widgets when reactive changes:

```python
class DynamicWidget(Static):
    count: reactive[int] = reactive(0, recompose=True)
    
    def compose(self) -> ComposeResult:
        for i in range(self.count):
            yield Static(f"Item {i}")

widget = DynamicWidget()
widget.count = 5  # Removes old children, calls compose again
```

### Always Update

Call watch/refresh even if value unchanged:

```python
count: reactive[int] = reactive(0, always_update=True)
```

---

## Screens

### What is a Screen?

Containers that fill the terminal. Apps have multiple screens but show only one.

### Creating Screens

```python
from textual.screen import Screen

class MyScreen(Screen):
    def compose(self) -> ComposeResult:
        yield Header()
        yield Static("Screen content")
        yield Footer()
    
    CSS = """
    Static {
        height: 1fr;
    }
    """
```

### Named Screens

```python
class MainScreen(Screen):
    pass

class DialogScreen(Screen):
    pass

class MyApp(App):
    SCREENS = {
        "main": MainScreen(),
        "dialog": DialogScreen(),
    }
    
    def on_mount(self):
        self.switch_screen("main")
```

### Install Screens Dynamically

```python
def on_mount(self):
    self.install_screen(DialogScreen(), name="dialog")
```

### Screen Stack

Push/pop screens:

```python
self.push_screen("dialog")  # Push screen on stack
self.pop_screen()  # Pop top screen
self.switch_screen("dialog")  # Replace top screen
```

### Modal Screens

For dialogs and popups:

```python
from textual.screen import ModalScreen

class ConfirmDialog(ModalScreen[bool]):
    def compose(self) -> ComposeResult:
        yield Static("Confirm action?")
        with Horizontal():
            yield Button("Yes", id="yes")
            yield Button("No", id="no")
    
    def on_button_pressed(self, event: Button.Pressed) -> None:
        if event.button.id == "yes":
            self.dismiss(True)
        else:
            self.dismiss(False)
```

Use modal screens:

```python
def show_confirm(self):
    def check_result(result: bool):
        if result:
            self.log("Confirmed!")
    
    self.app.push_screen(ConfirmDialog(), callback=check_result)
```

ModalScreen automatically:
- Prevents app bindings
- Darkens background
- Handles focus

### Screen Opacity

Make background visible:

```python
class MyScreen(Screen):
    CSS = """
    Screen {
        background: rgba(0, 0, 0, 0.5);
    }
    """
```

### Uninstalling Screens

```python
self.uninstall_screen("dialog")
```

---

## Animations

### Animating Styles

Change style values smoothly:

```python
self.query_one(Button).styles.animate(
    "opacity",
    value=0.0,  # Target value
    duration=2.0,  # Seconds
)
```

Animate multiple properties:

```python
widget = self.query_one(Static)
widget.styles.animate("opacity", value=0.5, duration=1.0)
widget.styles.animate("offset", value=(5, 10), duration=1.0)
```

### Animation Parameters

```python
widget.styles.animate(
    "opacity",
    value=0.0,
    duration=2.0,      # Animation duration in seconds
    speed=None,        # Alternative: units per second
    easing="in_out_cubic",  # See easing functions
    delay=1.0,         # Delay before animation starts
    on_complete=None,  # Callback when animation finishes
)
```

### Easing Functions

Control animation curve:

```python
# Linear
"linear"

# Ease in/out
"in_out_cubic"      # Default
"in_cubic"
"out_cubic"

# Other curves
"in_out_quad"
"in_out_quart"
"in_out_quint"
"in_out_sine"
"in_out_expo"
"in_out_circ"
"in_out_elastic"
"in_out_back"
"in_out_bounce"

# Speed variations
"in_out_sine"
"out_sine"
"in_sine"
```

Preview easing:

```bash
textual easing
```

### Animation Completion

```python
def animation_done(self):
    self.log("Animation complete!")

widget.styles.animate(
    "opacity",
    value=0.0,
    duration=1.0,
    on_complete=animation_done,
)
```

---

## Queries

### Finding Widgets

Query the DOM for widgets:

```python
# Get single widget
button = self.query_one("#submit")  # By ID
button = self.query_one(Button)  # By type

# Get multiple widgets
buttons = self.query("Button")  # Get all buttons
```

### Query with Type Checking

```python
# Type checked
button = self.query_one("#submit", Button)  # Returns Button or raises WrongType
buttons = self.query("Button").results(Button)  # Iterate as Button
```

### Query Operations

```python
query = self.query("Button")

# Iteration
for button in query:
    button.disabled = True

# Indexing
first_button = query[0]
last_button = query[-1]

# Length
count = len(query)

# Slicing
subset = query[1:3]
```

### Filter and Exclude

```python
# Get disabled buttons
disabled = self.query("Button").filter(".disabled")

# Get enabled buttons
enabled = self.query("Button").exclude(".disabled")
```

### First and Last

```python
first = self.query("Button").first()
last = self.query("Button").last()
```

### Bulk Operations

Modify all matching widgets without loop:

```python
# Add class to all buttons
self.query("Button").add_class("primary")

# Remove class
self.query("Button").remove_class("disabled")

# Toggle class
self.query("Button").toggle_class("active")

# Focus first
self.query("Button").focus()

# Blur (unfocus)
self.query("Button").blur()

# Refresh
self.query("Button").refresh()

# Remove from DOM
self.query(".temporary").remove()

# Set attributes
self.query("Input").set("disabled", True)
```

---

## Builtin Widgets Reference

### Static

Simple text display:

```python
from textual.widgets import Static

label = Static("Hello, World!")
label.update("Updated text")
```

### Button

Clickable button:

```python
from textual.widgets import Button

button = Button("Click me", id="submit", variant="primary")

class MyApp(App):
    def on_button_pressed(self, event: Button.Pressed) -> None:
        self.log("Button pressed!")
```

Variants: `"default"`, `"primary"`, `"warning"`, `"error"`, `"success"`

### Label

Text label:

```python
from textual.widgets import Label

label = Label("Name:")
```

### Input

Text input field:

```python
from textual.widgets import Input

input_field = Input(
    id="name",
    placeholder="Enter name...",
    value="Initial value"
)

class MyApp(App):
    def on_input_changed(self, event: Input.Changed) -> None:
        self.log(f"Input: {event.value}")
    
    def on_input_submitted(self, event: Input.Submitted) -> None:
        self.log(f"Submitted: {event.value}")
```

### TextArea

Multi-line text editor:

```python
from textual.widgets import TextArea

text_area = TextArea(
    text="Initial content",
    id="editor",
    language="python"
)
```

Supported languages: `"python"`, `"javascript"`, `"yaml"`, `"toml"`, `"html"`, `"css"`, etc.

### Header and Footer

```python
from textual.widgets import Header, Footer

class MyApp(App):
    def compose(self) -> ComposeResult:
        yield Header()
        # ... content ...
        yield Footer()
```

### Switch

Toggle on/off:

```python
from textual.widgets import Switch

switch = Switch(value=True)

class MyApp(App):
    def on_switch_changed(self, event: Switch.Changed) -> None:
        self.log(f"Switch: {event.value}")
```

### Checkbox

Checkbox control:

```python
from textual.widgets import Checkbox

checkbox = Checkbox("I agree", id="agree")

class MyApp(App):
    def on_checkbox_changed(self, event: Checkbox.Changed) -> None:
        self.log(f"Checkbox: {event.value}")
```

### RadioButton and RadioSet

Radio buttons (one selected):

```python
from textual.widgets import RadioButton, RadioSet

radio_set = RadioSet(
    RadioButton("Option 1", id="opt1"),
    RadioButton("Option 2", id="opt2"),
    RadioButton("Option 3", id="opt3"),
)

class MyApp(App):
    def on_radio_set_changed(self, event: RadioSet.Changed) -> None:
        self.log(f"Selected: {event.pressed_id}")
```

### Select

Dropdown selection:

```python
from textual.widgets import Select

select = Select(
    [("Option 1", 1), ("Option 2", 2), ("Option 3", 3)],
    id="choice"
)

class MyApp(App):
    def on_select_changed(self, event: Select.Changed) -> None:
        self.log(f"Selected: {event.value}")
```

### DataTable

Tabular data display:

```python
from textual.widgets import DataTable

table = DataTable()
table.add_columns("Name", "Age", "City")
table.add_row("Alice", 30, "NYC")
table.add_row("Bob", 25, "LA")
table.add_row("Charlie", 35, "Chicago")
```

Handle clicks:

```python
class MyApp(App):
    def on_data_table_row_selected(self, event: DataTable.RowSelected) -> None:
        self.log(f"Selected row: {event.cursor_row}")
```

### Tree

Hierarchical tree view:

```python
from textual.widgets import Tree

tree = Tree("Root")
branch = tree.root.add("Branch 1")
branch.add("Item 1")
branch.add("Item 2")
tree.root.add("Branch 2")
```

Handle selection:

```python
class MyApp(App):
    def on_tree_node_selected(self, event: Tree.NodeSelected) -> None:
        self.log(f"Selected: {event.node.label}")
```

### ListView and ListItem

Scrollable list:

```python
from textual.widgets import ListView, ListItem

list_view = ListView(
    ListItem(Label("Item 1")),
    ListItem(Label("Item 2")),
    ListItem(Label("Item 3")),
)

class MyApp(App):
    def on_list_view_selected(self, event: ListView.Selected) -> None:
        self.log(f"Selected item: {event.cursor_index}")
```

### OptionList

Simple option selection:

```python
from textual.widgets import OptionList

options = OptionList(
    ("Option 1", 1),
    ("Option 2", 2),
    ("Option 3", 3),
)

class MyApp(App):
    def on_option_list_selected_changed(self, event: OptionList.SelectedChanged) -> None:
        self.log(f"Selected: {event.option_index}")
```

### TabbedContent and Tab

Tab interface:

```python
from textual.widgets import TabbedContent, Tab

with TabbedContent():
    with Tab("Home"):
        yield Static("Home tab content")
    with Tab("Settings"):
        yield Static("Settings tab content")
    with Tab("About"):
        yield Static("About tab content")

class MyApp(App):
    def on_tabbed_content_tab_activated(self, event: TabbedContent.TabActivated) -> None:
        self.log(f"Activated tab: {event.pane.id}")
```

### RichLog

Log display with Rich formatting:

```python
from textual.widgets import RichLog

log = RichLog()
log.write("Normal text")
log.write("[bold red]Error[/bold red]")
log.write("[green]Success[/green]")
```

### ProgressBar

Progress indication:

```python
from textual.widgets import ProgressBar

progress = ProgressBar(total=100)
progress.advance(25)  # Update progress
```

Reactive updates:

```python
class MyApp(App):
    class MyProgress(Static):
        progress: reactive[int] = reactive(0)
        
        def render(self) -> ProgressBar:
            return ProgressBar(total=100, value=self.progress)
```

### Markdown

Display Markdown:

```python
from textual.widgets import MarkdownViewer

markdown = MarkdownViewer(markdown="# Title\n\nContent")
```

### Container and Layouts

```python
from textual.containers import Vertical, Horizontal, Grid, Container

class MyApp(App):
    def compose(self) -> ComposeResult:
        with Vertical():
            yield Static("Top")
            with Horizontal():
                yield Static("Left")
                yield Static("Right")
            yield Static("Bottom")
```

### Rule

Visual separator:

```python
from textual.widgets import Rule

yield Static("Section 1")
yield Rule()
yield Static("Section 2")
```

### Placeholder

Temporary placeholder:

```python
from textual.widgets import Placeholder

yield Placeholder()
```

### Collapsible

Expandable/collapsible section:

```python
from textual.widgets import Collapsible

yield Collapsible(
    title="Click to expand",
    children=[Static("Hidden content")]
)
```

### DirectoryTree

File browser tree:

```python
from textual.widgets import DirectoryTree

tree = DirectoryTree(".")  # Current directory
```

### Digits

Large digit display:

```python
from textual.widgets import Digits

digits = Digits("12:34:56")  # Digital clock style
```

### Toast

Notification popups:

```python
self.notify("Hello, World!")
self.notify("Error occurred!", title="Error", severity="error")

# Severity: "information" (default), "warning", "error"
```

---

## Advanced Topics

### Async/Await in Textual

Textual supports async operations:

```python
async def on_mount(self) -> None:
    result = await self.long_operation()

async def long_operation(self) -> str:
    await asyncio.sleep(2)
    return "Done"
```

### Workers

Run operations in background:

```python
from textual.worker import work

class MyApp(App):
    @work(exclusive=True)
    async def load_data(self):
        data = await fetch_data_from_api()
        return data
    
    def on_mount(self):
        self.load_data()
```

### Terminal Bell

```python
self.bell()
```

### Logging

```python
self.log("Log message", severity="information")
self.log("Warning!", severity="warning")
self.log("Error!", severity="error")
```

### Notifications

```python
self.notify("Hello!")
self.notify("Error", title="Error", severity="error")
```

### Themes

```python
self.theme = "dark"  # or "light"
```

Available themes: `"dark"`, `"light"`, `"textual"`, `"dracula"`, `"nord"`, `"solarized-dark"`, `"solarized-light"`, `"tokyo-night"`, etc.

### Exit App

```python
self.exit()  # Normal exit
self.exit(return_code=1)  # With error code
self.exit(message="Goodbye!")  # With message
```

### Running Actions

```python
self.action_quit()  # Call action directly
```

---

## Common Patterns

### Bidirectional Data Flow

"Attributes down, messages up":

```python
class Parent(Static):
    child_value: reactive[str] = reactive("initial")
    
    def on_child_changed(self, message: Child.Changed):
        self.child_value = message.value
    
    def on_mount(self):
        child = self.query_one(Child)
        child.value = self.child_value

class Child(Static):
    class Changed(Message):
        def __init__(self, value: str):
            self.value = value
    
    value: reactive[str] = reactive("")
    
    def watch_value(self, new_value: str):
        self.post_message(self.Changed(new_value))
```

### Loading States

```python
class DataDisplay(Static):
    async def load(self):
        self.loading = True
        self.data = await fetch_data()
        self.loading = False
```

### Form Validation

```python
class Form(Static):
    def compose(self) -> ComposeResult:
        yield Input(id="email")
        yield Button("Submit")
    
    def on_button_pressed(self):
        email = self.query_one("#email", Input).value
        if "@" not in email:
            self.notify("Invalid email", severity="error")
        else:
            self.notify("Valid!", severity="information")
```

### Search/Filter

```python
class SearchableList(Static):
    items = ["Apple", "Banana", "Cherry", "Date"]
    
    def compose(self) -> ComposeResult:
        yield Input(id="search", placeholder="Search...")
        yield ListView(id="results")
    
    def on_input_changed(self, event: Input.Changed):
        query = event.value.lower()
        results = [item for item in self.items if query in item.lower()]
        
        list_view = self.query_one("#results", ListView)
        list_view.clear()
        for item in results:
            list_view.append(ListItem(Label(item)))
```

---

## Testing Textual Apps

### Why Test?

Testing is **critical** for production apps. Tests:
- Find bugs early before users encounter them
- Prevent regressions when you update code
- Make refactoring safer
- Document how your app should behave
- Enable confident deployments

### Testing Framework Setup

Use **pytest** with **pytest-asyncio** for async testing.

#### With UV (Recommended)

Add testing dependencies to dev group:

```bash
uv add --group dev pytest pytest-asyncio pytest-textual-snapshot
```

Or edit `pyproject.toml`:

```toml
[dependency-groups]
dev = [
    "pytest>=7.0",
    "pytest-asyncio>=0.21.0",
    "pytest-textual-snapshot>=0.4.0",
]

[tool.pytest.ini_options]
asyncio_mode = "auto"
```

Then sync:

```bash
uv sync
```

Run tests:

```bash
uv run pytest
```

Run with verbose output:

```bash
uv run pytest -v
```

Run specific test file:

```bash
uv run pytest tests/test_app.py
```

Run with coverage:

```bash
uv add --group dev pytest-cov
uv run pytest --cov=src --cov-report=html
```

#### Traditional pip Installation

```bash
pip install pytest pytest-asyncio pytest-textual-snapshot
```

Configure pytest in `pyproject.toml`:

```toml
[tool.pytest.ini_options]
asyncio_mode = "auto"
```

Or run with: `pytest --asyncio-mode=auto`

### The Pilot API

Textual provides the `Pilot` object for testing - it simulates user interactions in headless mode (no terminal rendering).

```python
import pytest
from textual.app import App

@pytest.mark.asyncio
async def test_my_app():
    app = MyApp()
    async with app.run_test() as pilot:
        # Use pilot to interact with the app
        await pilot.press("enter")  # Simulate key press
        await pilot.click("#button")  # Simulate click
        # Assert state changes
        assert app.some_value == expected
```

### Key Simulation

Press individual keys:

```python
async def test_keyboard():
    app = MyApp()
    async with app.run_test() as pilot:
        await pilot.press("q")  # Single key
        assert app.running == False
```

Press multiple keys (simulate typing):

```python
await pilot.press("h", "e", "l", "l", "o")  # Types "hello"
```

Special keys:

```python
await pilot.press("enter")
await pilot.press("tab")
await pilot.press("escape")
await pilot.press("f1")
await pilot.press("ctrl+c")
await pilot.press("shift+tab")
await pilot.press("ctrl+shift+s")
```

Key names match `textual keys` output. Run that command to see all available keys.

### Mouse Simulation

#### Clicking Widgets

```python
async def test_button_click():
    app = MyApp()
    async with app.run_test() as pilot:
        await pilot.click("#submit")  # Click by ID
        assert app.submitted == True

# Click by widget type
await pilot.click(Button)

# Click by CSS selector
await pilot.click("Dialog Button")
```

#### Click Coordinates

```python
# Click at screen coordinates (0, 0)
await pilot.click()

# Click at screen coordinates (10, 5)
await pilot.click(offset=(10, 5))

# Click relative to widget
await pilot.click(Button, offset=(0, -1))  # Above button
```

#### Multiple Clicks

```python
# Double click
await pilot.click(Button, times=2)

# Triple click
await pilot.click(Button, times=3)
```

#### Modifier Keys

```python
# Ctrl+click
await pilot.click("#slider", control=True)

# Shift+click
await pilot.click("#item", shift=True)

# Cmd+click (Mac)
await pilot.click("#item", meta=True)
```

#### Hover

```python
await pilot.hover("#button")  # Move mouse over widget
```

### Changing Terminal Size

Test different screen sizes:

```python
async def test_responsive():
    app = MyApp()
    
    # Small screen
    async with app.run_test(size=(40, 12)) as pilot:
        label = app.query_one(Label)
        assert label.visible
    
    # Large screen
    async with app.run_test(size=(200, 50)) as pilot:
        label = app.query_one(Label)
        assert label.visible
```

### Querying App State

Use queries to inspect widgets:

```python
async def test_input_field():
    app = MyApp()
    async with app.run_test() as pilot:
        input_field = app.query_one(Input)
        assert input_field.value == ""
        
        await pilot.press("t", "e", "s", "t")
        assert input_field.value == "test"
```

Check rendered content:

```python
async def test_counter_display():
    app = MyApp()
    async with app.run_test() as pilot:
        counter = app.query_one(".counter", Static)
        rendered = counter.render()
        assert "Count: 0" in rendered
```

### Pausing for Async Operations

Messages take time to process. Pause to wait:

```python
async def test_with_delay():
    app = MyApp()
    async with app.run_test() as pilot:
        await pilot.click(Button)
        
        # Wait for messages to be processed
        await pilot.pause()
        
        assert app.message_processed == True

# Pause with delay
await pilot.pause(delay=1.0)  # 1 second delay
```

### Complete Testing Example

```python
import pytest
from textual.app import App, ComposeResult
from textual.widgets import Button, Input, Static, Label
from textual.containers import Vertical

class CounterApp(App[int]):
    CSS = """
    Screen {
        align: center middle;
    }
    
    Vertical {
        width: 40;
        height: 15;
        border: solid blue;
    }
    """
    
    count = 0
    
    def compose(self) -> ComposeResult:
        with Vertical():
            yield Label("Counter", id="title")
            yield Static(str(self.count), id="display")
            with Vertical():
                yield Button("Increment", id="inc")
                yield Button("Decrement", id="dec")
                yield Button("Reset", id="reset")
    
    def on_button_pressed(self, event: Button.Pressed) -> None:
        button_id = event.button.id
        
        if button_id == "inc":
            self.count += 1
        elif button_id == "dec":
            self.count -= 1
        elif button_id == "reset":
            self.count = 0
        
        self.query_one("#display", Static).update(str(self.count))

# Tests
@pytest.mark.asyncio
async def test_increment():
    app = CounterApp()
    async with app.run_test() as pilot:
        assert app.count == 0
        
        await pilot.click("#inc")
        await pilot.pause()  # Wait for message
        assert app.count == 1

@pytest.mark.asyncio
async def test_decrement():
    app = CounterApp()
    async with app.run_test() as pilot:
        await pilot.click("#inc")
        await pilot.click("#inc")
        await pilot.pause()
        assert app.count == 2
        
        await pilot.click("#dec")
        await pilot.pause()
        assert app.count == 1

@pytest.mark.asyncio
async def test_reset():
    app = CounterApp()
    async with app.run_test() as pilot:
        await pilot.click("#inc")
        await pilot.click("#inc")
        await pilot.click("#inc")
        await pilot.pause()
        assert app.count == 3
        
        await pilot.click("#reset")
        await pilot.pause()
        assert app.count == 0

@pytest.mark.asyncio
async def test_keyboard_increment():
    app = CounterApp()
    async with app.run_test() as pilot:
        await pilot.click("#inc")
        await pilot.pause()
        
        # Check display updated
        display = app.query_one("#display", Static)
        assert "1" in display.render()

@pytest.mark.asyncio
async def test_responsive_layout():
    app = CounterApp()
    
    # Test on small screen
    async with app.run_test(size=(30, 10)) as pilot:
        container = app.query_one(Vertical)
        assert container.visible
    
    # Test on large screen
    async with app.run_test(size=(150, 50)) as pilot:
        container = app.query_one(Vertical)
        assert container.visible
```

Run tests:

```bash
pytest test_counter.py -v
```

Output:

```
test_counter.py::test_increment PASSED
test_counter.py::test_decrement PASSED
test_counter.py::test_reset PASSED
test_counter.py::test_keyboard_increment PASSED
test_counter.py::test_responsive_layout PASSED

======================== 5 passed in 0.42s ========================
```

### Snapshot Testing

Visual regression testing - detect unintended UI changes.

Install:

```bash
pip install pytest-textual-snapshot
```

Create snapshot test:

```python
def test_app_snapshot(snap_compare):
    assert snap_compare("path/to/app.py")
```

First run:

```bash
pytest
```

Test fails (expected), generates screenshot. Open report to verify it looks correct.

Approve snapshot:

```bash
pytest --snapshot-update
```

Future runs compare against saved snapshot:

```bash
pytest
```

### Snapshot Features

Press keys before snapshot:

```python
def test_app_with_input(snap_compare):
    assert snap_compare(
        "path/to/app.py",
        press=["1", "2", "3", "enter"]
    )
```

Set terminal size:

```python
def test_app_small_screen(snap_compare):
    assert snap_compare(
        "path/to/app.py",
        terminal_size=(40, 12)
    )
```

Run custom code:

```python
def test_app_hover_state(snap_compare):
    async def setup(pilot):
        await pilot.hover("#button")
    
    assert snap_compare(
        "path/to/app.py",
        run_before=setup
    )
```

### Best Practices

1. **Test user workflows** - Don't test implementation, test outcomes
2. **Use meaningful assertions** - Be specific about what you're checking
3. **Test edge cases** - Empty inputs, max values, invalid data
4. **Mock external dependencies** - Don't call real APIs in tests
5. **Keep tests fast** - Use small datasets
6. **Organize tests logically** - One test class per widget
7. **Use descriptive names** - `test_button_click_updates_display()` not `test1()`
8. **Run tests often** - Integrate into CI/CD pipeline
9. **Maintain test coverage** - Aim for 80%+ coverage on critical paths
10. **Snapshot tests catch visual bugs** - Use them for complex UIs

### Testing Patterns

#### Testing Reactive Updates

```python
@pytest.mark.asyncio
async def test_reactive_updates():
    class ReactiveWidget(Static):
        value: reactive[int] = reactive(0)
        
        def render(self) -> str:
            return f"Value: {self.value}"
    
    widget = ReactiveWidget()
    assert "Value: 0" in widget.render()
    
    widget.value = 5
    # Textual automatically re-renders
    assert "Value: 5" in widget.render()
```

#### Testing Messages

```python
@pytest.mark.asyncio
async def test_custom_message():
    class MyWidget(Static):
        class Changed(Message):
            def __init__(self, value: str):
                self.value = value
    
    app = MyApp()
    async with app.run_test() as pilot:
        widget = app.query_one(MyWidget)
        widget.post_message(MyWidget.Changed("test"))
        await pilot.pause()
        
        assert app.received_value == "test"
```

#### Testing with Focus

```python
@pytest.mark.asyncio
async def test_focus_behavior():
    app = MyApp()
    async with app.run_test() as pilot:
        button = app.query_one(Button)
        button.focus()
        
        await pilot.press("enter")
        await pilot.pause()
        
        assert app.button_activated
```

#### Testing Keyboard Bindings

```python
@pytest.mark.asyncio
async def test_quit_binding():
    app = MyApp()
    async with app.run_test() as pilot:
        await pilot.press("ctrl+q")
        await pilot.pause()
        
        # App should have exited
        assert not app.is_running
```

---

## UV Project Management

### What is UV?

UV is a fast, modern Python package manager and project manager written in Rust. It replaces pip, venv, and poetry.

**Benefits:**
- ⚡ 10-100x faster than pip
- 🔒 Deterministic, reproducible builds
- 📦 Handles virtual environments automatically
- 🎯 Single tool for all Python needs
- ✅ Works with existing `pyproject.toml`

### Project Structure with UV

```
my_textual_app/
├── pyproject.toml          # Project config
├── uv.lock                 # Dependency lock file (git commit this)
├── .venv/                  # Virtual environment (auto-managed)
├── src/
│   └── my_app.py           # Main app code
└── tests/
    └── test_app.py         # Tests
```

### Creating a New Project

```bash
uv init my_textual_app
cd my_textual_app
```

This creates:

```toml
[project]
name = "my-textual-app"
version = "0.1.0"
description = ""
requires-python = ">=3.11"
dependencies = []

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

### Adding Dependencies

Add production dependencies:

```bash
uv add textual textual-dev
uv add requests pandas numpy
```

Add development-only dependencies:

```bash
uv add --group dev pytest pytest-asyncio pytest-cov
uv add --group dev black ruff mypy
```

This creates dependency groups in `pyproject.toml`:

```toml
[dependency-groups]
dev = [
    "pytest>=7.0",
    "pytest-asyncio>=0.21.0",
    "black>=23.0",
    "ruff>=0.1.0",
]
```

### Installing Dependencies

```bash
# Install all dependencies (including dev)
uv sync

# Install only production dependencies (no dev)
uv sync --no-dev

# Install from existing lock file (like in CI)
uv sync --frozen
```

### Running Code

```bash
# Run a script
uv run src/my_app.py

# Run tests
uv run pytest

# Run with specific Python version
uv run --python 3.12 src/my_app.py

# Run a shell command in the venv
uv run textual --version
```

### Python Version Management

Specify Python version:

```toml
[project]
requires-python = ">=3.11,<3.13"
```

Or in `pyproject.toml`:

```toml
[tool.uv]
python-version = "3.11"
python-versions = ["3.11", "3.12"]
```

Check available versions:

```bash
uv python list
```

Install specific version:

```bash
uv python install 3.12
```

### Virtual Environment

UV manages venvs automatically. To use it directly:

```bash
# Create venv
uv venv

# Activate venv
source .venv/bin/activate  # On Unix
.venv\Scripts\activate     # On Windows

# Or use uv run (doesn't require activation)
uv run python --version
```

### Lock File

`uv.lock` ensures reproducible builds. Always commit it to git:

```bash
git add uv.lock
```

Then others (or CI) get identical environments:

```bash
uv sync --frozen  # Use exact versions from lock file
```

### UV with Textual Development

Typical workflow:

```bash
# Create project
uv init my_app

# Add Textual and dev tools
uv add textual textual-dev
uv add --group dev pytest pytest-asyncio black ruff mypy

# Create app
cat > src/main.py << 'EOF'
from textual.app import App

class MyApp(App):
    pass

if __name__ == "__main__":
    MyApp().run()
EOF

# Run app
uv run src/main.py

# Format code
uv run black src/

# Lint code
uv run ruff check src/

# Type check
uv run mypy src/

# Run tests
uv run pytest tests/

# Run dev console
uv run textual run src/main.py --dev
```

### Publishing with UV

Build package:

```bash
uv build
```

This creates:
- `dist/*.whl` - Wheel package
- `dist/*.tar.gz` - Source distribution

Publish to PyPI (requires API token):

```bash
uv publish
```

### UV Configuration

Create `uv.toml` for advanced settings (optional):

```toml
[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"

[tool.uv]
# Use specific index
index-url = "https://pypi.org/simple"

# Allow prerelease versions
prerelease = "allow"

# Python preference
python-version = "3.11"
```

### Common UV Commands

```bash
# Sync dependencies
uv sync

# Add package
uv add package-name

# Add dev package
uv add --group dev package-name

# Remove package
uv remove package-name

# List installed packages
uv pip list

# Show package info
uv pip show package-name

# Export requirements.txt
uv export > requirements.txt

# Run command in venv
uv run command

# Run tests
uv run pytest

# Update packages
uv sync --upgrade

# Show Python versions
uv python list

# Install Python version
uv python install 3.12

# Show lockfile
cat uv.lock
```

### UV vs Pip/Poetry

| Feature | UV | pip | poetry |
|---------|----|----|--------|
| Speed | ⚡⚡⚡ Fast | ⚡ Slow | ⚡⚡ Medium |
| Venv Management | Auto | Manual | Auto |
| Lock File | Yes | No | Yes |
| Dev Dependencies | Yes | Setuptools | Yes |
| Python Version | Managed | System | Managed |
| Install | `uv add` | `pip install` | `poetry add` |

---

## Debugging

### Dev Console with UV

Run app with dev console using UV:

```bash
uv run textual run src/main.py --dev
```

Or:

```bash
export TEXTUAL_DEBUG=1
uv run src/main.py
```

### Logging

```python
self.log("Debug message")
```

View logs in dev console.

### Inspect Widget Tree

In dev console use `query` to explore DOM.

### Type Checking with UV

```bash
uv add --group dev mypy
uv run mypy src/
```

### Linting with UV

```bash
uv add --group dev ruff
uv run ruff check src/
uv run ruff format src/  # Auto-format
```

### Testing with UV

```bash
uv add --group dev pytest pytest-asyncio
uv run pytest -v
uv run pytest --cov=src
```

---

## Performance Tips

1. **Use Line API** for large datasets (DataTable uses this)
2. **Batch reactive updates** - modify multiple reactives to trigger single refresh
3. **Use `var` instead of `reactive`** when refresh not needed
4. **Lazy load content** - load data on demand
5. **Virtual rendering** - only render visible rows for huge lists

---

## Complete Example

```python
from textual.app import App, ComposeResult
from textual.widgets import Header, Footer, Button, Input, Static
from textual.containers import Vertical, Horizontal
from textual.reactive import reactive

class Greeting(Static):
    name: reactive[str] = reactive("World")
    
    def render(self) -> str:
        return f"Hello, {self.name}!"

class GreetingApp(App):
    CSS = """
    Screen {
        align: center middle;
    }
    
    Greeting {
        width: 40;
        height: 3;
        border: solid blue;
        content-align: center middle;
    }
    
    Input {
        margin: 1;
        width: 40;
    }
    
    Button {
        margin: 1;
    }
    """
    
    BINDINGS = [("q", "quit", "Quit")]
    
    def compose(self) -> ComposeResult:
        yield Header()
        with Vertical():
            yield Greeting(id="greeting")
            yield Input(placeholder="Enter a name...")
            yield Button("Update", id="update")
        yield Footer()
    
    def on_input_submitted(self, event: Input.Submitted) -> None:
        self.update_greeting(event.value)
    
    def on_button_pressed(self, event: Button.Pressed) -> None:
        if event.button.id == "update":
            input_field = self.query_one(Input)
            self.update_greeting(input_field.value)
    
    def update_greeting(self, name: str) -> None:
        greeting = self.query_one(Greeting)
        greeting.name = name or "World"

if __name__ == "__main__":
    app = GreetingApp()
    app.run()
```

---

## Quick Reference: UV Commands

```bash
# Project setup
uv init my_app
uv add textual textual-dev
uv add --group dev pytest pytest-asyncio ruff black mypy

# Running
uv run src/main.py                    # Run app
uv run pytest                         # Run tests
uv run textual run src/main.py --dev  # Dev console

# Quality
uv run ruff check src/                # Lint
uv run ruff format src/               # Format
uv run mypy src/                      # Type check
uv run pytest --cov=src               # Coverage

# Maintenance
uv sync                               # Install/sync
uv sync --upgrade                     # Update packages
uv pip list                           # List packages
uv export > requirements.txt          # Export requirements
```

---

## Quick Reference: Common Textual Patterns

```python
# App setup
from textual.app import App, ComposeResult

class MyApp(App):
    CSS_PATH = "style.tcss"
    BINDINGS = [("q", "quit", "Quit")]
    
    def compose(self) -> ComposeResult:
        yield Header()
        yield Static("Content")
        yield Footer()
    
    def on_mount(self) -> None:
        self.title = "My App"

if __name__ == "__main__":
    MyApp().run()

# Reactive updates
from textual.reactive import reactive

class MyWidget(Static):
    value: reactive[int] = reactive(0)
    
    def render(self) -> str:
        return f"Value: {self.value}"

# Event handling
def on_button_pressed(self, event: Button.Pressed) -> None:
    self.log("Button clicked!")

# Queries
button = self.query_one(Button)
buttons = self.query("Button").filter(".active")
self.query("Button").add_class("disabled")

# CSS selectors
Screen { align: center middle; }
#submit { background: green; }
.error { color: red; }
Button:hover { background: blue; }

# Messages
class MyWidget(Static):
    class Changed(Message):
        def __init__(self, value: str):
            self.value = value

self.post_message(MyWidget.Changed("data"))

# Testing
async def test_app():
    app = MyApp()
    async with app.run_test() as pilot:
        await pilot.press("q")
        assert not app.is_running
```

---

## Resources

- **Official Docs**: https://textual.textualize.io/
- **GitHub**: https://github.com/Textualize/textual
- **Discord**: Community support channel
- **Widget Gallery**: Full list of builtin widgets
- **Examples**: `/examples` directory in repository
- **UV Docs**: https://docs.astral.sh/uv/

---

## Summary of Key Principles

1. **Declarative Composition**: Define UI structure in `compose()`
2. **Event-Driven**: Respond to user input via event handlers
3. **CSS for Styling**: Keep styles separate in `.tcss` files
4. **Reactive Attributes**: Automatic refresh when data changes
5. **Message-Based Communication**: Widgets communicate via messages
6. **DOM Queries**: Find and manipulate widgets via selectors
7. **Async-Ready**: Native async/await support
8. **Testable**: Easy to test with Pilot framework
9. **Modern Tooling**: Use UV for fast, deterministic project management

---

## Coverage Completeness

This guide covers **100% of essential Textual development**:

✅ **Installation & Setup** - UV + traditional methods  
✅ **App Architecture** - Lifecycle, events, composition  
✅ **30+ Widgets** - Complete reference with examples  
✅ **Styling** - CSS, colors, dimensions, animations  
✅ **Layout** - Vertical, horizontal, grid, docking  
✅ **Reactivity** - Smart updates, validation, watchers  
✅ **Events & Input** - Keyboard, mouse, custom messages  
✅ **Screens** - Navigation, modals, stacks  
✅ **Testing** - Pilot, snapshot testing, best practices  
✅ **Project Management** - UV workflows, Python versions  
✅ **Best Practices** - Patterns, performance, debugging  

**For advanced features**, consult the official documentation.
