# Textual Layout Visual Examples

Visual reference for common Textual patterns used in Posting.

## 1. Pane Border with Title and Subtitle

### Visual Output

```
┌─ Collection ─────────────────────────────── My API Collection ─┐
│ ▼ Users                                                        │
│   ▶ GET    List Users                                         │
│   ▶ POST   Create User                                        │
│   ▶ PUT    Update User                                        │
│   ▶ DELETE Delete User                                        │
│ ▼ Posts                                                        │
│   ▶ GET    List Posts                                         │
└──────────────────────────────────────────────────────────────┘
```

### Code

```python
class CollectionPane(Vertical):
    def on_mount(self):
        self.border_title = "Collection"
        self.border_subtitle = "My API Collection"
        self.add_class("section")
```

### SCSS

```scss
.section {
  border: round $accent 40%;
  border-title-color: $text-accent 50%;
  border-title-align: right;  # Right-aligns title, subtitle on far right
}
```

---

## 2. Dynamic Response Status Header

### Visual Output (Success)

```
┌─ Response [200 OK] ───────────────────────────── 2.5KB in 234.56ms ─┐
│ {                                                                  │
│   "id": 1,                                                         │
│   "name": "John Doe",                                              │
│   "email": "john@example.com"                                      │
│ }                                                                  │
└─────────────────────────────────────────────────────────────────────┘
```

### Visual Output (Error)

```
┌─ Response [500 Internal Server Error] ─── 1.2KB in 1234.56ms ─┐
│ {                                                            │
│   "error": "Database connection failed",                     │
│   "code": "DB_ERROR"                                         │
│ }                                                            │
└────────────────────────────────────────────────────────────┘
```

### Code

```python
class ResponsePane(Vertical):
    response: Reactive[httpx.Response | None] = reactive(None)
    
    def watch_response(self, response: httpx.Response | None):
        if response is None:
            return
        
        # Update title with status
        self.border_title = f"Response [{response.status_code} {response.reason_phrase}]"
        
        # Update subtitle with metrics
        size = len(response.content)
        elapsed = response.elapsed.total_seconds() * 1000
        self.border_subtitle = f"{size}B in {elapsed:.2f}ms"
        
        # Add status-based class
        self.remove_class("success", "warning", "error")
        if response.status_code < 300:
            self.add_class("success")
        elif response.status_code < 400:
            self.add_class("warning")
        else:
            self.add_class("error")
```

### SCSS

```scss
ResponseArea {
  border-subtitle-color: $text-muted;
  
  &.success .border-title-status {
    color: $text-success;
    background: $success-muted;
  }
  
  &.error .border-title-status {
    color: $text-error;
    background: $error-muted;
  }
}
```

---

## 3. Multi-Pane Layout - Vertical

### Visual Output

```
┌─ URL Bar ────────────────────────────────────────────┐
│ GET  https://api.example.com/users            Send   │
└──────────────────────────────────────────────────────┘

┌─ Request ────────────────────────────────────────────┐
│ Headers │ Body │ Query │ Auth │ Info │ Scripts       │
├──────────────────────────────────────────────────────┤
│ Content-Type: application/json                       │
│ Authorization: Bearer token123                       │
│                                                      │
└──────────────────────────────────────────────────────┘

┌─ Response [200 OK] ──────────────── 2.5KB in 234ms ─┐
│ Body │ Headers │ Cookies │ Scripts │ Trace           │
├──────────────────────────────────────────────────────┤
│ [                                                    │
│   {"id": 1, "name": "Alice"},                        │
│   {"id": 2, "name": "Bob"}                           │
│ ]                                                    │
└──────────────────────────────────────────────────────┘
```

### Python Code

```python
class MainScreen(Screen):
    def compose(self) -> ComposeResult:
        yield AppHeader()
        yield UrlBar()
        with AppBody():
            yield CollectionBrowser()
            yield RequestEditor()
            yield ResponseArea()
        yield Footer()
```

### SCSS

```scss
AppBody {
  &.layout-vertical {
    layout: vertical;  # Stack panes vertically
    padding: 0 2;
  }
  
  & CollectionBrowser {
    height: 1fr;
    dock: left;
    width: auto;
    max-width: 33%;
  }
  
  & RequestEditor {
    height: 1fr;
  }
  
  & ResponseArea {
    height: 1fr;
  }
}
```

---

## 4. Multi-Pane Layout - Horizontal

### Visual Output

```
┌─────────────────────────────────────────────────────────────────┐
│ ┌─ Collection ──────┐ ┌─ Request ─────┐ ┌─ Response ────────┐  │
│ │ ▼ Users          │ │ Headers │Body │ │ Body │ Headers    │  │
│ │  ▶ GET List      │ │─────────────────│ │──────────────────│  │
│ │  ▶ POST Create   │ │Content-Type: /j│ │ [                │  │
│ │  ▶ PUT Update    │ │                │ │  {"id": 1}       │  │
│ │  ▶ DELETE Delete │ │                │ │ ]                │  │
│ │ ▼ Posts         │ │                │ │                  │  │
│ │  ▶ GET List      │ │                │ │                  │  │
│ └──────────────────┘ └─────────────────┘ └──────────────────┘  │
└─────────────────────────────────────────────────────────────────┘
```

### SCSS

```scss
AppBody {
  &.layout-horizontal {
    layout: horizontal;  # Stack panes horizontally
    padding: 0 2;
  }
  
  & CollectionBrowser {
    height: 1fr;
    width: auto;
    max-width: 33%;
  }
  
  & RequestEditor {
    height: 1fr;
    width: 1fr;
  }
  
  & ResponseArea {
    height: 1fr;
    width: 1fr;
  }
}
```

---

## 5. Tabbed Pane Headers

### Visual Output

```
┌─ Request ────────────────────────────────────────────┐
│ Headers │ Body │ Query │ Auth │ Info │ Scripts       │
├──────────────────────────────────────────────────────┤
│ Name      │ Value                                    │
├───────────┼────────────────────────────────────────┤
│ Content-T │ application/json                        │
│ Auth      │ Bearer token123                         │
│ X-API-Key │ secret-key-here                         │
│           │                                         │
└──────────────────────────────────────────────────────┘
```

### Code

```python
class RequestEditor(Vertical):
    def compose(self) -> ComposeResult:
        with Vertical() as vertical:
            vertical.border_title = "Request"
            with TabbedContent():
                with TabPane("Headers", id="headers-pane"):
                    yield HeaderEditor()
                with TabPane("Body", id="body-pane"):
                    yield Lazy(BodyEditor())
                with TabPane("Query", id="query-pane"):
                    yield Lazy(QueryEditor())
                with TabPane("Auth", id="auth-pane"):
                    yield Lazy(AuthEditor())
    
    def on_mount(self):
        self.border_title = "Request"
        self.add_class("section")
```

---

## 6. Docked Widgets

### Visual Output

```
┌────────────────────────────────────────────────────────┐
│ Collection │                                           │
│ Panel      │     Main Content Area                    │
│ (Docked    │                                           │
│  Left)     │                                           │
├────────────┤                                           │
│ Preview    │                                           │
│ (Docked    │                                           │
│  Bottom)   │                                           │
└────────────┴───────────────────────────────────────────┘

┌────────────────────────────────────────────────────────┐
│                                                         │
│                  Main Content Area                      │
│                                                         │
│                                                         │
├────────────────────────────────────────────────────────┤
│ Footer (Docked Bottom)                                 │
└────────────────────────────────────────────────────────┘
```

### SCSS

```scss
CollectionBrowser {
  dock: left;        # Position on left side
  width: auto;
  max-width: 33%;
  height: 1fr;
}

RequestPreview {
  dock: bottom;      # Position at bottom within parent
  height: auto;
  max-height: 50%;
  width: 100%;
}

Footer {
  dock: bottom;      # Position at bottom of screen
}
```

---

## 7. Status-Based Styling Chain

### Visual Output (Idle)

```
┌─ Request ────────────────────────────────────────────┐
│ Normal border, muted title color                     │
└──────────────────────────────────────────────────────┘
```

### Visual Output (Loading)

```
┌─ Request ────────────────────────────────────────────┐
│ Yellow border, warning title color                   │
│ Status: Loading...                                   │
└──────────────────────────────────────────────────────┘
```

### Visual Output (Success)

```
┌─ Request ────────────────────────────────────────────┐
│ Green border, success title color                    │
│ Status: ✓ Success                                    │
└──────────────────────────────────────────────────────┘
```

### Code

```python
class RequestPane(Vertical):
    status: Reactive[str] = reactive("idle")
    
    def watch_status(self, status: str):
        # Remove all status classes
        self.remove_class("loading", "success", "error")
        # Add new status class
        self.add_class(status)
        
        # Update subtitle
        if status == "loading":
            self.border_subtitle = "Loading..."
        elif status == "success":
            self.border_subtitle = "✓ Success"
        elif status == "error":
            self.border_subtitle = "✗ Error"
```

### SCSS

```scss
RequestPane {
  border: round $accent 40%;
  border-title-color: $text-muted;
  
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
```

---

## 8. Focus States

### Visual Output (Unfocused)

```
┌─ Request ────────────────────────────────────────────┐
│ dim: $accent 40%, border-title-color: $text-accent  │
│ 50%                                                  │
└──────────────────────────────────────────────────────┘
```

### Visual Output (Focused)

```
┌─ Request ────────────────────────────────────────────┐
│ bright: $accent 100%, border-title-color: $foreground│
│ [bold], border-title-style: bold                     │
└──────────────────────────────────────────────────────┘
```

### SCSS

```scss
.section {
  border: round $accent 40%;
  border-title-color: $text-accent 50%;
  
  &:focus-within {
    border: round $accent 100%;           # Brighter accent
    border-title-color: $foreground;      # Bright text
    border-title-style: b;                # Bold
  }
}
```

---

## 9. Rich Text in Headers

### Visual Output

```
┌─ [GET] /api/users ─────────────────────────────────────┐
│ Bold method, normal path                              │
└──────────────────────────────────────────────────────────┘

┌─ Response ─────────────────────────── [200 OK in 234ms] ─┐
│ Green status, dim elapsed time                          │
└──────────────────────────────────────────────────────────┘

┌─ ✓ Success ────────────────────────────────────────────┐
│ Styled status indicator                               │
└──────────────────────────────────────────────────────────┘
```

### Code

```python
# Method and path
self.border_title = f"[bold]{method}[/bold] {path}"

# Status with color
status = response.status_code
reason = response.reason_phrase
style = self.get_component_rich_style("border-title-status")
self.border_title = f"Response [{style}]{status} {reason}[/]"

# Status with indicator
self.border_subtitle = f"[green]✓[/green] Success"
self.border_subtitle = f"[yellow]⏳[/yellow] Loading..."
self.border_subtitle = f"[red]✗[/red] Error"

# Metrics with varying styles
size = 2.5  # KB
time = 234.56  # ms
self.border_subtitle = f"{size}KB in [dim]{time}ms[/dim]"
```

---

## 10. Layout Toggle in Action

### Initial State (Vertical)

```
┌─────────────────────────────┐
│        Collection           │
├─────────────────────────────┤
│        Request              │
├─────────────────────────────┤
│        Response             │
└─────────────────────────────┘
```

### After Toggle (Horizontal)

```
┌─────────────┬──────────┬───────────┐
│ Collection  │ Request  │ Response  │
│             │          │           │
│             │          │           │
│             │          │           │
└─────────────┴──────────┴───────────┘
```

### Code

```python
class MainApp(App):
    current_layout: Reactive[str] = reactive("vertical")
    
    def watch_current_layout(self, layout: str):
        body = self.query_one(AppBody)
        body.remove_class("layout-vertical", "layout-horizontal")
        body.add_class(f"layout-{layout}")
    
    def action_toggle_layout(self):
        new = "horizontal" if self.current_layout == "vertical" else "vertical"
        self.current_layout = new
```

### SCSS

```scss
AppBody {
  &.layout-vertical {
    layout: vertical;
  }
  
  &.layout-horizontal {
    layout: horizontal;
  }
}
```

---

## Summary of Visual Patterns

| Pattern | Key Properties | Use Case |
|---------|---|---|
| **Title + Subtitle** | `border_title`, `border_subtitle` | Show collection/request context |
| **Dynamic Status** | `Reactive` + `watch_*()` | Update on data changes |
| **Color Coding** | CSS classes + SCSS | Status visualization |
| **Focus States** | `:focus-within` selector | UI feedback |
| **Docking** | `dock: left/right/top/bottom` | Fixed positioning |
| **Flexible Layout** | `layout: vertical/horizontal` | Responsive UI |
| **Lazy Loading** | `Lazy()` widget | Performance optimization |
| **Rich Markup** | `[bold]`, `[color]`, etc. | Styled text |

