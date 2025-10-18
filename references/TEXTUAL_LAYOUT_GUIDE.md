# Posting App: Textual Layout & Pane Headers Guide

A comprehensive technical guide on how the Posting HTTP client achieves its layout using Textual and implements dynamic pane headers with collection/request names.

## Table of Contents

1. [Architecture Overview](#architecture-overview)
2. [Main Layout Structure](#main-layout-structure)
3. [Dynamic Pane Headers](#dynamic-pane-headers)
4. [Styling System (SCSS)](#styling-system-scss)
5. [Layout Switching](#layout-switching)
6. [Key Components & Implementation](#key-components--implementation)
7. [Code Examples](#code-examples)
8. [Best Practices](#best-practices)

---

## Architecture Overview

Posting's UI is built using **Textual**, a framework for building Python TUI (Terminal User Interface) applications. The app consists of:

- **AppHeader**: Top bar showing title, version, and user/host info
- **UrlBar**: URL input and method selector
- **AppBody**: Main content area containing three panes
  - **CollectionBrowser**: Left sidebar with request tree
  - **RequestEditor**: Middle pane with request details (headers, body, auth, etc.)
  - **ResponseArea**: Right pane showing HTTP responses

The layout can be switched between **vertical** and **horizontal** orientations using reactive layout binding.

---

## Main Layout Structure

### 1. Component Hierarchy

```
Posting (App)
├── AppHeader (Horizontal)
├── UrlBar (Container)
├── AppBody (Vertical or Horizontal)
│   ├── CollectionBrowser (Vertical with border)
│   │   ├── Tree (collection/request tree)
│   │   └── RequestPreview (bottom dock)
│   ├── RequestEditor (Vertical with border and tabs)
│   │   └── TabbedContent (Headers, Body, Path, Query, Auth, Info, Scripts, Options)
│   └── ResponseArea (Vertical with border and tabs)
│       └── TabbedContent (Body, Headers, Cookies, Scripts, Trace)
└── Footer

```

### 2. HTML/Textual Composition (Python)

The main screen's compose method in `app.py` (MainScreen class, line 255):

```python
def compose(self) -> ComposeResult:
    yield AppHeader()
    yield UrlBar()
    with AppBody():
        collection_browser = CollectionBrowser(collection=self.collection)
        collection_browser.display = (
            self.settings.collection_browser.show_on_startup
        )
        yield collection_browser
        yield RequestEditor()
        yield ResponseArea()

    footer = Footer(show_command_palette=False)
    footer.compact = self.posting.spacing == "compact"
    yield footer
```

---

## Dynamic Pane Headers

### Understanding Textual Borders

In Textual, the `border_title` and `border_subtitle` properties are used to add text to widget borders:

- **`border_title`**: Main text shown in the border (typically on the left or configurable via `border-title-align`)
- **`border_subtitle`**: Secondary text shown in the border (typically on the right)

### Example: CollectionBrowser Pane Header

**File**: `src/posting/widgets/collection/browser.py` (line 580-597)

```python
def compose(self) -> ComposeResult:
    self.styles.dock = SETTINGS.get().collection_browser.position
    self.border_title = "Collection"  # Set main title
    self.add_class("section")
    collection = self.collection

    # ... yield tree and preview ...
    
    self.border_subtitle = collection.name  # Set collection name on the right
```

**Result**: Displays as:
```
┌─ Collection ─────────────────────── My API Collection ─┐
│  ▼ My First Sub-Collection                            │
│    ▶ GET  List Users                                  │
│    ▶ POST Create User                                 │
└──────────────────────────────────────────────────────┘
```

### Example: ResponseArea Pane Header

**File**: `src/posting/widgets/response/response_area.py` (line 36-38, 113-117)

The response area dynamically updates its border based on the HTTP response:

```python
def on_mount(self) -> None:
    self.border_title = "Response"
    self._latest_response: httpx.Response | None = None
    self.add_class("section")

def watch_response(self, response: httpx.Response | None) -> None:
    # ... handling code ...
    self.border_title = self._make_border_title(response)
    
    settings = SETTINGS.get()
    if settings.response.show_size_and_time:
        self.border_subtitle = f"{human_readable_size(len(response.content))} in {response.elapsed.total_seconds() * 1000:.2f}[dim]ms[/]"

def _make_border_title(self, response: httpx.Response) -> str:
    style = self.get_component_rich_style("border-title-status")
    return f"Response [{style}] {response.status_code} {response.reason_phrase} [/]"
```

**Result**: Dynamically displays as:
```
┌─ Response [200 OK] ──────────────────────── 2.5KB in 234.56ms ─┐
│ {                                                              │
│   "id": 1,                                                     │
│   "name": "John Doe"                                           │
│ }                                                              │
└──────────────────────────────────────────────────────────────┘
```

### Setting Border Title with Request Name

When a request is selected in the collection tree, the request name should be shown in the RequestEditor pane header. Here's how to implement this:

**Option 1: Watch Pattern (Recommended)**

```python
from textual.reactive import reactive

class RequestEditor(Vertical):
    current_request_name: Reactive[str] = reactive("", init=False)
    
    def watch_current_request_name(self, name: str) -> None:
        """Update the border title when the request name changes."""
        self.border_title = f"Request: {name}" if name else "Request"
    
    def on_mount(self):
        self.border_title = "Request"
        self.add_class("section")
```

**Option 2: Direct Property Update**

```python
# From the app when a request is selected
@on(CollectionTree.RequestSelected)
def on_request_selected(self, event: CollectionTree.RequestSelected) -> None:
    request = event.request
    self.request_editor.border_title = f"Request: {request.name}" if request.name else "Request"
    self.load_request_model(request)
```

---

## Styling System (SCSS)

### SCSS File Location

**File**: `src/posting/posting.scss`

Textual supports SCSS (Sass) for styling, which is compiled to TCSS (Textual CSS).

### Key Styling Concepts

#### 1. Border Styling for Sections

**File**: `posting.scss` (lines 244-255)

```scss
.section {
  border: round $accent 40%;
  border-title-color: $text-accent 50%;
  border-title-align: right;  // Align title to the right

  &:focus-within {
    border: round $accent 100%;
    border-title-color: $foreground;
    border-title-style: b;  // Bold when focused
  }
}
```

**Features**:
- `border: round` - Rounded corners
- `$accent 40%` - Accent color at 40% opacity
- `border-title-color` - Color of the border text
- `border-title-align: right` - Position of the title
- `:focus-within` - Styles applied when widget has focus

#### 2. Status-Based Styling (Response Area)

**File**: `posting.scss` (lines 178-195)

```scss
ResponseArea {
  border-subtitle-color: $text-muted;
  
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

This CSS uses component classes that are dynamically added/removed based on HTTP status:

```python
# From response_area.py watch_response method
self.remove_class("success", "warning", "error")
if response.status_code < 300:
    self.add_class("success")
elif response.status_code < 400:
    self.add_class("warning")
else:
    self.add_class("error")
```

#### 3. Layout Switching

**File**: `posting.scss` (lines 283-310)

```scss
AppBody {
  padding: 0 2;

  &.layout-horizontal {
    layout: horizontal;  // Side-by-side layout
    
    & KeyValueInput {
      dock: top;
      & #key-value-inputs {
        layout: vertical;
        height: 3;
      }
    }
  }
  
  &.layout-vertical {
    layout: vertical;  // Stacked layout
    
    & KeyValueInput #key-value-inputs {
      layout: horizontal;
    }
  }
}
```

#### 4. Docking and Positioning

**File**: `posting.scss` (various locations)

```scss
CollectionBrowser {
  height: 1fr;  // Take full height
  dock: left;   // Position on left side
  width: auto;
  max-width: 33%;  // Maximum 1/3 of screen width
}

RequestPreview {
  dock: bottom;  // Position at bottom of parent
  height: auto;
  max-height: 50%;
  width: 100%;
}

Footer {
  padding-left: 2;
  dock: bottom;  // Automatically positions at bottom
}
```

---

## Layout Switching

### Reactive Layout System

**File**: `src/posting/app.py` (line ~190)

```python
current_layout: Reactive[PostingLayout] = reactive("vertical", init=False)
"""The current layout of the app."""

def watch_current_layout(self, layout: Literal["horizontal", "vertical"]) -> None:
    """Update the current layout of the app to be horizontal or vertical."""
    classes = {"horizontal", "vertical"}
    app_body = self.query_one(AppBody)
    
    # Remove both classes and add the new one
    app_body.remove_class(*classes)
    app_body.add_class(f"layout-{layout}")
```

### Triggering Layout Changes

**File**: `src/posting/app.py` (command handler)

```python
def command_layout(self, layout: Literal["vertical", "horizontal"]) -> None:
    self.main_screen.current_layout = layout
```

**How it works**:
1. Command is triggered (e.g., from command palette: "Toggle Layout")
2. `current_layout` reactive attribute is set to new value
3. `watch_current_layout` method is called automatically
4. CSS class on AppBody changes (e.g., `layout-vertical` → `layout-horizontal`)
5. SCSS rules for `.layout-horizontal` or `.layout-vertical` apply
6. `layout: horizontal/vertical` property changes the flexbox direction

---

## Key Components & Implementation

### 1. PostingTabbedContent (Custom Tabs)

**File**: `src/posting/widgets/tabbed_content.py`

```python
class PostingTabbedContent(TabbedContent):
    BINDINGS = [
        Binding("l", "next_tab", "Next tab", show=False),
        Binding("h", "previous_tab", "Previous tab", show=False),
        Binding("down,j", "app.focus_next", "Focus next", show=False),
        Binding("up,k", "app.focus_previous", "Focus previous", show=False),
    ]

    def action_next_tab(self) -> None:
        tabs = self.query_one(Tabs)
        if tabs.has_focus:
            tabs.action_next_tab()

    def action_previous_tab(self) -> None:
        tabs = self.query_one(Tabs)
        if tabs.has_focus:
            tabs.action_previous_tab()
```

**Features**:
- Extends Textual's TabbedContent
- Adds vim-like keybindings (h/l for tab navigation)
- Only activates tab switching when tabs have focus

### 2. RequestEditor with Tabs

**File**: `src/posting/widgets/request/request_editor.py` (line 38-58)

```python
class RequestEditor(Vertical):
    def compose(self) -> ComposeResult:
        app = cast("Posting", self.app)
        with Vertical() as vertical:
            vertical.border_title = "Request"
            with RequestEditorTabbedContent():
                with TabPane("Headers", id="headers-pane"):
                    yield HeaderEditor()
                with TabPane("Body", id="body-pane"):
                    yield Lazy(RequestBodyEditor())
                with TabPane("Path", id="path-pane"):
                    yield Lazy(PathEditor())
                with TabPane("Query", id="query-pane"):
                    yield Lazy(QueryStringEditor())
                with TabPane("Auth", id="auth-pane"):
                    yield Lazy(RequestAuth())
                with TabPane("Info", id="info-pane"):
                    yield Lazy(RequestMetadata())
                with TabPane("Scripts", id="scripts-pane"):
                    yield Lazy(RequestScripts(collection_root=app.collection.path))
                with TabPane("Options", id="options-pane"):
                    yield Lazy(RequestOptions())

    def on_mount(self):
        self.border_title = "Request"
        self.add_class("section")
```

**Features**:
- Tabs are placed inside a `Vertical` with border
- `Lazy()` widget wraps expensive-to-render components (only render when tab is active)
- Border title set both in compose and on_mount for consistency

### 3. ResponseArea with Dynamic Headers

**File**: `src/posting/widgets/response/response_area.py` (full implementation)

```python
class ResponseArea(Vertical):
    response: Reactive[httpx.Response | None] = reactive(None)

    def on_mount(self) -> None:
        self.border_title = "Response"
        self._latest_response: httpx.Response | None = None
        self.add_class("section")
        self.app.theme_changed_signal.subscribe(self, self.on_theme_change)

    def watch_response(self, response: httpx.Response | None) -> None:
        if response is None:
            return

        self.query_one(ResponseTabbedContent).disabled = False
        self.add_class("response-ready")
        
        # Update border with status code
        self.border_title = self._make_border_title(response)
        
        # Set size and time info in subtitle
        settings = SETTINGS.get()
        if settings.response.show_size_and_time:
            self.border_subtitle = f"{human_readable_size(len(response.content))} in {response.elapsed.total_seconds() * 1000:.2f}[dim]ms[/]"
        
        # Update status-based styling
        self.remove_class("success", "warning", "error")
        if response.status_code < 300:
            self.add_class("success")
        elif response.status_code < 400:
            self.add_class("warning")
        else:
            self.add_class("error")

    def _make_border_title(self, response: httpx.Response) -> str:
        style = self.get_component_rich_style("border-title-status")
        return f"Response [{style}] {response.status_code} {response.reason_phrase} [/]"
```

**Features**:
- `Reactive` variable watches for response changes
- When response updates, watchers automatically trigger
- Border title is dynamically constructed with status code and Rich formatting
- Status-based CSS classes are added/removed for color coding

---

## Code Examples

### Example 1: Creating a Custom Pane with Dynamic Header

```python
from textual.app import ComposeResult
from textual.containers import Vertical
from textual.reactive import reactive

class CustomPane(Vertical):
    """A pane with a dynamic header showing current selection."""
    
    current_item_name: Reactive[str] = reactive("Untitled", init=False)
    
    def watch_current_item_name(self, name: str) -> None:
        """Update border when item name changes."""
        self.border_title = f"Items: {name}"
    
    def on_mount(self) -> None:
        self.border_title = "Items: Untitled"
        self.add_class("section")
    
    def compose(self) -> ComposeResult:
        yield SomeContent()
        yield MoreContent()
    
    def update_selected_item(self, name: str) -> None:
        """Called when an item is selected."""
        self.current_item_name = name  # Triggers watch_current_item_name
```

### Example 2: Status-Based Border Styling

```python
from textual.reactive import reactive

class DataPane(Vertical):
    """A pane with status-based styling."""
    
    status: Reactive[str] = reactive("idle", init=False)
    
    def watch_status(self, status: str) -> None:
        """Update classes when status changes."""
        # Remove all status classes
        self.remove_class("loading", "success", "error")
        # Add the new status class
        self.add_class(status)
        
        # Update border subtitle
        if status == "loading":
            self.border_subtitle = "Loading..."
        elif status == "success":
            self.border_subtitle = "✓ Success"
        elif status == "error":
            self.border_subtitle = "✗ Error"
    
    def on_mount(self) -> None:
        self.border_title = "Data"
        self.add_class("section")
```

### Example 3: Collection and Request Names in Headers

```python
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    from posting.collection import Collection, RequestModel

class BrowserPane(Vertical):
    """Pane showing collection and current request."""
    
    current_collection_name: Reactive[str] = reactive("", init=False)
    current_request_name: Reactive[str] = reactive("", init=False)
    
    def watch_current_collection_name(self, name: str) -> None:
        self._update_header()
    
    def watch_current_request_name(self, name: str) -> None:
        self._update_header()
    
    def _update_header(self) -> None:
        """Update the pane header with collection and request info."""
        parts = []
        if self.current_collection_name:
            parts.append(f"[bold]{self.current_collection_name}[/bold]")
        if self.current_request_name:
            parts.append(f"({self.current_request_name})")
        
        # Combine for display
        header_text = " ".join(parts) or "Browser"
        self.border_title = header_text
    
    def load_collection(self, collection: "Collection") -> None:
        """Load a collection into the pane."""
        self.current_collection_name = collection.name
        self.current_request_name = ""
    
    def load_request(self, request: "RequestModel") -> None:
        """Load a request into the pane."""
        self.current_request_name = request.name or "Untitled"
```

### Example 4: SCSS for Dynamic Border Headers

```scss
// Define base pane styling
.pane {
  border: solid $accent 40%;
  border-title-color: $text-accent 50%;
  border-title-align: right;
  height: 1fr;
  width: 1fr;

  &:focus-within {
    border: solid $accent 100%;
    border-title-color: $foreground;
    border-title-style: b;
  }
}

// Collection pane
CollectionPane {
  @extend .pane;
  dock: left;
  width: auto;
  max-width: 33%;
  
  &.has-requests {
    border-subtitle-color: $text-muted;
  }
}

// Request pane with dynamic status
RequestPane {
  @extend .pane;
  
  &.loading {
    border-title-color: $text-warning;
    border-subtitle-color: $text-warning;
  }
  
  &.success {
    border-title-color: $text-success;
    border-subtitle-color: $text-success;
  }
  
  &.error {
    border-title-color: $text-error;
    border-subtitle-color: $text-error;
  }
}

// Response pane with status-based colors
ResponsePane {
  @extend .pane;
  
  &.status-2xx {
    border: solid $success 40%;
    border-title-color: $text-success 50%;
  }
  
  &.status-3xx {
    border: solid $warning 40%;
    border-title-color: $text-warning 50%;
  }
  
  &.status-4xx,
  &.status-5xx {
    border: solid $error 40%;
    border-title-color: $text-error 50%;
  }
}
```

---

## Best Practices

### 1. Use Reactive Properties for Dynamic Updates

```python
# ✅ Good: Reactive property with watcher
class Pane(Vertical):
    selection: Reactive[str] = reactive("", init=False)
    
    def watch_selection(self, value: str) -> None:
        self.border_title = f"Selected: {value}"

# ❌ Avoid: Manual updates scattered throughout code
def on_item_selected(self, event):
    self.pane.border_title = f"Selected: {event.item}"
```

### 2. Separate Layout Concerns

```python
# ✅ Good: Layout controlled through CSS and reactive layout property
# Python:
current_layout: Reactive[PostingLayout] = reactive("vertical", init=False)

# SCSS:
AppBody.layout-vertical {
  layout: vertical;
}
AppBody.layout-horizontal {
  layout: horizontal;
}

# ❌ Avoid: Hardcoding layout changes in Python
for widget in container.children:
    if layout == "horizontal":
        widget.styles.width = "50%"
```

### 3. Use Lazy Loading for Expensive Components

```python
# ✅ Good: Lazy-load tabs that are expensive to render
from textual.lazy import Lazy

with TabPane("Scripts", id="scripts-pane"):
    yield Lazy(ComplexScriptEditor())

# ❌ Avoid: Rendering all tabs immediately
with TabPane("Scripts", id="scripts-pane"):
    yield ComplexScriptEditor()  # Renders even if tab isn't active
```

### 4. Compose Headers in on_mount for Consistency

```python
# ✅ Good: Set border_title in both compose and on_mount
def compose(self) -> ComposeResult:
    vertical.border_title = "Request"
    with vertical:
        yield SomeWidget()

def on_mount(self) -> None:
    self.border_title = "Request"  # Ensure it's set
    self.add_class("section")

# ℹ️ Why: Ensures the border is always visible, even if compose is bypassed
```

### 5. Use Markup for Rich Text in Headers

```python
# ✅ Good: Use Rich markup for styled headers
self.border_title = f"[bold]{method}[/bold] {path}"
self.border_subtitle = f"[green]{status_code}[/green] in [dim]{elapsed}ms[/dim]"

# This displays as:
# ┌─ GET /api/users ────────────────────── 200 in 234ms ─┐
```

### 6. Theme-Aware Colors

```python
# ✅ Good: Use app theme variables
style = self.get_component_rich_style("border-title-status")
self.border_title = f"Response [{style}] {status_code} {reason}[/]"

# ❌ Avoid: Hardcoded colors
self.border_title = f"Response [green]{status_code}[/green] {reason}"
```

### 7. Component Classes for Styling

```python
# ✅ Good: Use component classes for consistency
COMPONENT_CLASSES = {
    "border-title-status",
    "node-selected",
}

# Then style in SCSS:
ResponseArea {
    & .border-title-status {
        color: $text-success;
        background: $success-muted;
    }
}

# ❌ Avoid: Inline styles for everything
self.border_title = "[bold #00ff00 on #001100]..."
```

---

## Key Takeaways

1. **Border Titles & Subtitles**: Use `border_title` and `border_subtitle` to display collection/request names
2. **Reactive Properties**: Use `@reactive` for state that should update the UI automatically
3. **Watchers**: Implement `watch_<property>()` methods to handle reactive updates
4. **CSS Classes**: Apply/remove classes to trigger SCSS-based styling
5. **Layout System**: Use reactive layout properties with SCSS media-query-like rules
6. **Lazy Loading**: Use `Lazy()` for expensive components in tabs
7. **Markup & Rich Text**: Leverage Rich library for styled text in headers
8. **SCSS Variables**: Use theme variables for consistent colors across the app

---

## References

- **Textual Documentation**: https://textual.textualize.io/
- **Rich Documentation**: https://rich.readthedocs.io/
- **Posting Source**: https://github.com/posting-sh/posting
- **Key Files**:
  - `src/posting/app.py` - Main app structure
  - `src/posting/posting.scss` - Styling
  - `src/posting/widgets/collection/browser.py` - Collection pane
  - `src/posting/widgets/request/request_editor.py` - Request pane
  - `src/posting/widgets/response/response_area.py` - Response pane

