# Textual Button Styling & Implementation Guide

Complete guide on creating, styling, and managing buttons in Textual applications using Posting as reference.

## Table of Contents

1. [Button Basics](#button-basics)
2. [Basic Button Styling](#basic-button-styling)
3. [Custom Button Types](#custom-button-types)
4. [Button States & Interactions](#button-states--interactions)
5. [Advanced Button Patterns](#advanced-button-patterns)
6. [Button Accessibility](#button-accessibility)
7. [Implementation Examples](#implementation-examples)
8. [Common Patterns](#common-patterns)

---

## Button Basics

### 1. Textual Button Widget

Textual provides a built-in `Button` widget that can be customized through SCSS or Python.

```python
from textual.widgets import Button
from textual.containers import Horizontal

# Basic button
yield Button("Click me", id="my-button")

# Styled button with variant
yield Button.success("Save", id="save-button")
yield Button.warning("Cancel", id="cancel-button")
yield Button.error("Delete", id="delete-button")
```

### 2. Button Properties

```python
button = Button("Send")

# Configuration
button.can_focus = False           # Prevent tab focus
button.disabled = False            # Enable/disable
button.label = "New Label"         # Change text
button.id = "my-button"           # Set identifier
button.classes = "primary"        # Add CSS classes
```

### 3. Button Variants (Built-in)

Textual provides factory methods for common button types:

```python
Button("Normal")                   # Standard button
Button.success("Confirm")          # Green success button
Button.warning("Caution")          # Yellow/orange warning button
Button.error("Delete")             # Red danger button
Button.primary("Action")           # Primary action button
```

---

## Basic Button Styling

### SCSS Button Styling (File: `src/posting/posting.scss`)

**Basic Button Style** (lines 743-751):

```scss
Button {
  padding: 0 1;           # Horizontal padding
  height: 1;              # Single line height
  border: none;           # No border by default
  
  &:disabled {
    opacity: 40%;         # Faded when disabled
  }
}
```

### Theme Variable Application

```scss
Button {
  background: $primary;              # Use theme primary color
  color: $text;                      # Use theme text color
  
  # On hover
  &:hover {
    background-tint: $text 10%;      # Tint with 10% text color
    text-style: b;                   # Make bold on hover
  }
  
  # When focused
  &:focus {
    border: solid $accent;           # Accent border on focus
  }
  
  # When disabled
  &:disabled {
    opacity: 40%;                    # 40% opacity when disabled
    color: $text-muted;              # Muted text color
  }
}
```

---

## Custom Button Types

### 1. SendRequestButton (Primary Action)

**File**: `src/posting/posting.scss` (lines 469-477)

```scss
SendRequestButton {
  min-width: 8;                      # Minimum width in cells
  background: $accent-muted;         # Muted accent background
  color: $text-accent;               # Accent text color
  text-style: b;                     # Bold
  
  &:hover {
    background-tint: $text-accent 10%;  # Tint on hover
  }
}
```

**Python Class**:

```python
class SendRequestButton(Button, can_focus=False):
    """The button for sending the request."""
    pass
```

Key design:
- `can_focus=False` - Not reachable by tabbing
- `min-width: 8` - Large enough to see label
- Muted accent colors - Distinct but not aggressive
- Bold text - Draws attention without bright colors

### 2. Key-Value Editor Add Button

**File**: `src/posting/posting.scss` (lines 522-534)

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
        background: $primary-darken-1;  # Darker on hover
      }
    }
  }
  
  #add-button {
    color: $text;
    background: $accent;               # Special add button styling
  }
}
```

### 3. Creating Custom Button Classes

```python
from textual.widgets import Button

class PrimaryButton(Button):
    """Primary action button."""
    DEFAULT_CSS = """
    PrimaryButton {
        background: $accent;
        color: $text-accent;
        text-style: b;
        width: 100%;
        margin: 1 0;
        padding: 0 2;
    }
    
    PrimaryButton:hover {
        background-tint: white 15%;
    }
    
    PrimaryButton:disabled {
        opacity: 40%;
        text-style: none;
    }
    """

class SecondaryButton(Button):
    """Secondary action button."""
    DEFAULT_CSS = """
    SecondaryButton {
        background: $surface;
        border: solid $accent;
        color: $text;
        width: 100%;
        margin: 1 0;
        padding: 0 2;
    }
    
    SecondaryButton:hover {
        background: $accent-muted;
        text-style: b;
    }
    """

class DangerButton(Button):
    """Destructive action button."""
    DEFAULT_CSS = """
    DangerButton {
        background: $error;
        color: white;
        text-style: b;
        width: 100%;
        margin: 1 0;
        padding: 0 2;
    }
    
    DangerButton:hover {
        background-tint: white 15%;
    }
    """
```

---

## Button States & Interactions

### 1. Button State Types

```scss
Button {
  # Normal/idle state
  background: $primary;
  color: $text;
  
  # Hover state - user moved mouse over
  &:hover {
    background-tint: $text 10%;
    text-style: b;
  }
  
  # Focus state - keyboard focused
  &:focus {
    border: solid $accent;
  }
  
  # Active state - currently pressed
  &:active {
    background-tint: $text 20%;
  }
  
  # Disabled state - not interactive
  &:disabled {
    opacity: 40%;
    color: $text-muted;
  }
}
```

### 2. Button Press Handling

```python
from textual import on
from textual.widgets import Button

class MyScreen(Screen):
    def on_mount(self):
        button = self.query_one("#send-button", Button)
    
    def on_button_pressed(self, event: Button.Pressed) -> None:
        """Called when any button is pressed."""
        button = event.button
        if button.id == "send-button":
            self.send_request()
        elif button.id == "cancel-button":
            self.dismiss()
    
    # Or use decorator
    @on(Button.Pressed, "#send-button")
    def handle_send(self, event: Button.Pressed) -> None:
        self.send_request()
```

### 3. Dynamic Button State Changes

```python
from textual.reactive import reactive

class ToggleButton(Button):
    """Button that tracks state."""
    
    is_active: Reactive[bool] = reactive(False)
    
    def watch_is_active(self, is_active: bool):
        """Update button appearance when state changes."""
        self.remove_class("active", "inactive")
        if is_active:
            self.add_class("active")
            self.label = "✓ Active"
        else:
            self.add_class("inactive")
            self.label = "○ Inactive"

# SCSS:
ToggleButton {
  &.active {
    background: $success-muted;
    color: $text-success;
  }
  
  &.inactive {
    background: $surface;
    color: $text-muted;
  }
}
```

---

## Advanced Button Patterns

### 1. Status-Based Button

```python
class StatusButton(Button):
    """Button that changes based on operation status."""
    
    status: Reactive[str] = reactive("idle")
    
    STATUS_CONFIGS = {
        "idle": {
            "label": "Send",
            "class": "idle",
            "disabled": False,
        },
        "loading": {
            "label": "Sending...",
            "class": "loading",
            "disabled": True,
        },
        "success": {
            "label": "✓ Sent",
            "class": "success",
            "disabled": False,
        },
        "error": {
            "label": "✗ Failed",
            "class": "error",
            "disabled": False,
        },
    }
    
    def watch_status(self, status: str):
        """Update button when status changes."""
        config = self.STATUS_CONFIGS.get(status, self.STATUS_CONFIGS["idle"])
        
        # Update appearance
        self.label = config["label"]
        self.disabled = config["disabled"]
        
        # Update classes
        self.remove_class("idle", "loading", "success", "error")
        self.add_class(config["class"])

# SCSS:
StatusButton {
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

### 2. Button Group (Related Buttons)

```python
class ButtonGroup(Static):
    """Group of related buttons."""
    
    def compose(self) -> ComposeResult:
        with Horizontal():
            yield Button("Save", id="save")
            yield Button("Cancel", id="cancel")
            yield Button("Delete", id="delete")

# SCSS:
ButtonGroup {
  Horizontal {
    layout: horizontal;
    height: auto;
    
    & Button {
      width: 1fr;
      margin-right: 1;
      
      &#save {
        background: $success;
      }
      
      &#cancel {
        background: $surface;
      }
      
      &#delete {
        background: $error;
      }
    }
  }
}
```

### 3. Confirmation Button Pair

```python
class ConfirmationButtons(Static):
    """Confirm/Cancel button pair."""
    
    def compose(self) -> ComposeResult:
        with Horizontal():
            yield Button.success("Confirm", id="confirm")
            yield Button.warning("Cancel", id="cancel")

# SCSS:
ConfirmationButtons {
  Horizontal {
    layout: horizontal;
    height: 1;
    margin-top: 1;
    
    & Button {
      width: 1fr;
      margin-right: 1;
      
      &#confirm {
        background: $success;
        color: white;
      }
      
      &#cancel {
        background: $warning;
        color: $text;
      }
    }
  }
}
```

---

## Button Accessibility

### 1. Keyboard Navigation

```scss
Button {
  # Visible focus indicator
  &:focus {
    border: solid $accent 2;
    text-style: b;
    background-tint: $accent 20%;
  }
}
```

### 2. Disabled State Clarity

```scss
Button {
  &:disabled {
    opacity: 40%;              # Visually distinct
    color: $text-muted;        # Desaturated text
    # Importantly: still visible and identified
  }
}
```

### 3. High Contrast Mode

```python
class AccessibleButton(Button):
    """Button with accessibility support."""
    
    DEFAULT_CSS = """
    AccessibleButton {
        background: $primary;
        color: $text;
        border: none;
    }
    
    AccessibleButton:focus {
        border: solid $accent 2;      # Clear focus indicator
        text-style: b;
        background-tint: $accent 25%;
    }
    
    AccessibleButton:disabled {
        opacity: 50%;                 # Clearly disabled
        border: dashed $text-muted;   # Additional visual cue
    }
    """
```

---

## Implementation Examples

### Example 1: Complete Form with Buttons

```python
from textual.app import ComposeResult
from textual.containers import Horizontal, Vertical
from textual.widgets import Button, Input, Label

class FormWithButtons(Vertical):
    """Form showing button usage patterns."""
    
    def compose(self) -> ComposeResult:
        with Vertical():
            yield Label("User Details")
            yield Input(id="name-input", placeholder="Name")
            yield Input(id="email-input", placeholder="Email")
            
            with Horizontal(id="button-group"):
                yield Button.success("Save", id="save-btn")
                yield Button.warning("Reset", id="reset-btn")
                yield Button.error("Cancel", id="cancel-btn")
    
    @on(Button.Pressed, "#save-btn")
    def handle_save(self):
        name = self.query_one("#name-input", Input).value
        email = self.query_one("#email-input", Input).value
        # Process save
    
    @on(Button.Pressed, "#reset-btn")
    def handle_reset(self):
        self.query_one("#name-input", Input).value = ""
        self.query_one("#email-input", Input).value = ""
    
    @on(Button.Pressed, "#cancel-btn")
    def handle_cancel(self):
        self.app.pop_screen()

# SCSS:
FormWithButtons {
  Vertical {
    padding: 1 2;
  }
  
  #button-group {
    layout: horizontal;
    height: 1;
    margin-top: 2;
    
    & Button {
      width: 1fr;
      margin-right: 1;
    }
  }
}
```

### Example 2: Loading Button with Spinner

```python
from textual.widgets import Button, Static
from textual.reactive import reactive
import asyncio

class LoadingButton(Button):
    """Button that shows loading state with spinner."""
    
    status: Reactive[str] = reactive("ready")
    _original_label: str = ""
    
    SPINNER_FRAMES = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]
    
    def __init__(self, label: str, **kwargs):
        super().__init__(label, **kwargs)
        self._original_label = label
        self._spinner_index = 0
    
    async def perform_action(self, action_coro):
        """Perform async action with loading state."""
        self.status = "loading"
        try:
            await action_coro
            self.status = "success"
            await asyncio.sleep(1)
        except Exception as e:
            self.status = "error"
            await asyncio.sleep(2)
        finally:
            self.status = "ready"
    
    def watch_status(self, status: str):
        """Update button display based on status."""
        if status == "ready":
            self.label = self._original_label
            self.disabled = False
        elif status == "loading":
            self.label = f"{self.SPINNER_FRAMES[self._spinner_index]} Loading..."
            self.disabled = True
            self._animate_spinner()
        elif status == "success":
            self.label = "✓ Complete"
            self.disabled = False
        elif status == "error":
            self.label = "✗ Error"
            self.disabled = False
    
    def _animate_spinner(self):
        """Animate spinner in loading state."""
        if self.status == "loading":
            self._spinner_index = (self._spinner_index + 1) % len(self.SPINNER_FRAMES)
            self.set_timer(0.1, self._animate_spinner)

# Usage:
async def on_button_pressed(self, event: Button.Pressed):
    button = event.button
    if isinstance(button, LoadingButton):
        await button.perform_action(self.send_request())
```

### Example 3: Dynamic Button Management

```python
class DynamicButtonPanel(Static):
    """Panel managing button state based on context."""
    
    selected_item: Reactive[str | None] = reactive(None)
    
    def watch_selected_item(self, item: str | None):
        """Enable/disable buttons based on selection."""
        edit_btn = self.query_one("#edit-btn", Button)
        delete_btn = self.query_one("#delete-btn", Button)
        
        if item is None:
            edit_btn.disabled = True
            delete_btn.disabled = True
        else:
            edit_btn.disabled = False
            delete_btn.disabled = False
    
    def compose(self) -> ComposeResult:
        with Vertical():
            with Horizontal():
                yield Button("New", id="new-btn")
                yield Button("Edit", id="edit-btn", disabled=True)
                yield Button("Delete", id="delete-btn", disabled=True)
```

---

## Common Patterns

### Pattern 1: Submit Button

```python
class SubmitButton(Button):
    """Standard submit button."""
    
    def __init__(self, label: str = "Submit", **kwargs):
        super().__init__(label, **kwargs)
        self.can_focus = False

# SCSS:
SubmitButton {
  background: $success;
  color: white;
  text-style: b;
  min-width: 12;
}
```

### Pattern 2: Cancel Button

```python
class CancelButton(Button):
    """Standard cancel button."""
    
    def __init__(self, label: str = "Cancel", **kwargs):
        super().__init__(label, **kwargs)

# SCSS:
CancelButton {
  background: $surface;
  border: solid $text-muted;
  color: $text;
}
```

### Pattern 3: Danger Button

```python
class DangerButton(Button):
    """Dangerous/destructive button."""
    
    def __init__(self, label: str = "Delete", **kwargs):
        super().__init__(label, **kwargs)

# SCSS:
DangerButton {
  background: $error;
  color: white;
  text-style: b;
}

DangerButton:hover {
  background-tint: white 15%;
}
```

### Pattern 4: Disabled State

```python
# Disable button programmatically
button.disabled = True

# Re-enable button
button.disabled = False

# SCSS handles disabled styling
Button {
  &:disabled {
    opacity: 40%;
    color: $text-muted;
  }
}
```

---

## Summary Checklist

- [ ] Created base Button styles in SCSS
- [ ] Defined button states (normal, hover, focus, disabled)
- [ ] Created custom button classes for different actions
- [ ] Applied theme colors to buttons
- [ ] Added focus indicators for accessibility
- [ ] Implemented button press handlers
- [ ] Used reactive properties for state changes
- [ ] Added loading states if needed
- [ ] Ensured sufficient color contrast
- [ ] Tested with keyboard navigation
- [ ] Tested hover and focus states
- [ ] Tested disabled state clarity

