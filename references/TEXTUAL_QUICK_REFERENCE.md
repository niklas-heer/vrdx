# Textual Layout Quick Reference

## Dynamic Pane Headers Pattern

### Basic Pattern: Collection Name in Subtitle

```python
from textual.containers import Vertical
from textual.reactive import reactive

class MyPane(Vertical):
    collection_name: Reactive[str] = reactive("", init=False)
    
    def watch_collection_name(self, name: str) -> None:
        self.border_subtitle = name
    
    def on_mount(self) -> None:
        self.border_title = "Collection"
        self.add_class("section")
    
    def load_collection(self, name: str) -> None:
        self.collection_name = name  # Triggers watcher
```

### Response Status Pattern

```python
class MyResponsePane(Vertical):
    response: Reactive[dict | None] = reactive(None, init=False)
    
    def watch_response(self, resp: dict | None) -> None:
        if resp is None:
            return
        
        # Update title with status
        status = resp.get("status", 0)
        reason = resp.get("reason", "")
        self.border_title = f"Response [{status} {reason}]"
        
        # Update subtitle with size/time
        size = resp.get("size", 0)
        time = resp.get("elapsed", 0)
        self.border_subtitle = f"{size}B in {time}ms"
        
        # Add status-based styling
        self.remove_class("success", "warning", "error")
        if status < 300:
            self.add_class("success")
        elif status < 400:
            self.add_class("warning")
        else:
            self.add_class("error")
```

## Layout Switching Pattern

### Reactive Layout

```python
from textual.app import App
from textual.reactive import reactive

class MyApp(App):
    current_layout: Reactive[str] = reactive("vertical", init=False)
    
    def watch_current_layout(self, layout: str) -> None:
        body = self.query_one("#app-body")
        body.remove_class("layout-vertical", "layout-horizontal")
        body.add_class(f"layout-{layout}")
    
    def action_toggle_layout(self) -> None:
        new_layout = "horizontal" if self.current_layout == "vertical" else "vertical"
        self.current_layout = new_layout
```

### SCSS for Layouts

```scss
AppBody {
  &.layout-vertical {
    layout: vertical;
    
    & KeyValueInput #key-value-inputs {
      layout: horizontal;
    }
  }
  
  &.layout-horizontal {
    layout: horizontal;
    
    & KeyValueInput {
      dock: top;
      & #key-value-inputs {
        layout: vertical;
        height: 3;
      }
    }
  }
}
```

## Pane Composition Pattern

### With Tabs and Border

```python
from textual.containers import Vertical
from textual.widgets import TabPane, TabbedContent

class RequestEditor(Vertical):
    def compose(self):
        with Vertical() as vertical:
            vertical.border_title = "Request"  # Set here
            with TabbedContent():
                with TabPane("Headers", id="headers"):
                    yield HeaderWidget()
                with TabPane("Body", id="body"):
                    yield BodyWidget()
    
    def on_mount(self):
        self.border_title = "Request"  # Set here too
        self.add_class("section")
```

## SCSS Reference

### Basic Section Styling

```scss
.section {
  border: round $accent 40%;
  border-title-color: $text-accent 50%;
  border-title-align: right;
  
  &:focus-within {
    border: round $accent 100%;
    border-title-color: $foreground;
    border-title-style: b;
  }
}
```

### Docking

```scss
CollectionBrowser {
  dock: left;      // Position on left
  width: auto;
  max-width: 33%;
}

Footer {
  dock: bottom;    // Position at bottom
}

RequestPreview {
  dock: bottom;    // Within parent
  height: auto;
  max-height: 50%;
}
```

### Status-Based Classes

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

## Common Patterns

### Update Multiple Header Parts

```python
def _update_header(self) -> None:
    """Update border title and subtitle together."""
    parts = []
    if self.collection:
        parts.append(f"[bold]{self.collection.name}[/bold]")
    if self.request:
        parts.append(f"({self.request.name})")
    
    self.border_title = " ".join(parts) or "Browser"
```

### Watch Multiple Properties

```python
class Pane(Vertical):
    collection_name: Reactive[str] = reactive("", init=False)
    request_name: Reactive[str] = reactive("", init=False)
    
    def watch_collection_name(self, name: str) -> None:
        self._update_header()
    
    def watch_request_name(self, name: str) -> None:
        self._update_header()
    
    def _update_header(self) -> None:
        # Update logic here
        pass
```

### Rich Text in Headers

```python
# Use markup for styled text
self.border_title = f"[bold]{method}[/bold] {path}"
self.border_subtitle = f"[green]{status}[/green] in [dim]{elapsed}ms[/dim]"

# Get themed colors
style = self.get_component_rich_style("border-title-status")
self.border_title = f"Response [{style}]{status}[/] {reason}"
```

### Lazy Loading Tabs

```python
from textual.lazy import Lazy

with TabPane("Heavy Component", id="heavy"):
    yield Lazy(ExpensiveComponent())  # Only renders when tab is active
```

## Component Classes

### Define Custom Component Classes

```python
class MyPane(Vertical):
    COMPONENT_CLASSES = {
        "border-title-status",
        "node-selected",
    }
    
    def on_mount(self):
        # Use component class in code
        style = self.get_component_rich_style("border-title-status")
```

### Style Component Classes

```scss
MyPane {
  & .border-title-status {
    color: $text-success;
    background: $success-muted;
  }
}
```

## Common Mistakes to Avoid

❌ **DON'T**: Update border directly in multiple places
```python
def on_collection_selected(self, event):
    self.border_subtitle = event.collection.name  # Scattered logic
```

✅ **DO**: Use reactive properties with watchers
```python
def watch_collection_name(self, name: str) -> None:
    self.border_subtitle = name  # Centralized logic
```

---

❌ **DON'T**: Hardcode layout in Python
```python
if horizontal:
    self.styles.width = "50%"
```

✅ **DO**: Use CSS classes for layout
```python
self.add_class("layout-horizontal")
# Let SCSS handle: .layout-horizontal { width: 50%; }
```

---

❌ **DON'T**: Render all tabs immediately
```python
with TabPane("Scripts"):
    yield ComplexScriptEditor()
```

✅ **DO**: Use Lazy for expensive components
```python
with TabPane("Scripts"):
    yield Lazy(ComplexScriptEditor())
```

---

❌ **DON'T**: Hardcoded colors
```python
self.border_title = "[bold #00ff00 on #001100]Text[/]"
```

✅ **DO**: Use theme variables
```python
style = self.get_component_rich_style("border-title-status")
self.border_title = f"[{style}]Text[/]"
```

## Test Checklist

- [ ] Border titles update when data changes
- [ ] Border subtitles show correct information
- [ ] Layout switches between vertical/horizontal
- [ ] Focus highlighting works correctly
- [ ] Tabs lazy-load on activation
- [ ] Status-based styling applies correctly
- [ ] Responsive to theme changes
- [ ] No layout jitter during updates
