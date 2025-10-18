"""Modal screens for the vrdx TUI.

This module provides modal dialogs for user interactions such as status selection.
"""

from __future__ import annotations

from typing import Optional

from textual.app import ComposeResult
from textual.containers import Container, Vertical
from textual.screen import ModalScreen
from textual.widgets import Label, Static

from vrdx.parser import list_status_options


class StatusSelectionModal(ModalScreen[Optional[str]]):
    """Modal screen for selecting a decision status.

    This modal displays available status options and allows the user to navigate
    through them using arrow keys or j/k, confirm with Enter, and cancel with Escape.
    """

    CSS = """
    StatusSelectionModal {
        align: center middle;
    }

    #status-modal-container {
        width: 50;
        height: auto;
        border: solid $accent;
        background: $surface;
        padding: 1;
    }

    #status-modal-title {
        text-style: bold;
        padding-bottom: 1;
        text-align: center;
    }

    #status-options {
        width: 100%;
        height: auto;
        padding: 1;
        padding-bottom: 0;
    }

    #status-modal-help {
        padding-top: 1;
        color: $text-muted;
        width: 100%;
        text-align: center;
    }
    """

    BINDINGS = [
        ("up,k", "previous_option", "Previous"),
        ("down,j", "next_option", "Next"),
        ("enter,space", "confirm", "Confirm"),
        ("escape", "cancel", "Cancel"),
    ]

    def __init__(self) -> None:
        """Initialize the status selection modal."""
        super().__init__()
        self.status_options = list(list_status_options())
        self.selected_index = 0

    def compose(self) -> ComposeResult:
        """Compose the modal layout."""
        with Container(id="status-modal-container"):
            with Vertical():
                yield Label("Select Status for New Decision", id="status-modal-title")
                yield StatusOptionDisplay(
                    self.status_options,
                    selected_index=self.selected_index,
                    id="status-options",
                )
                yield Label(
                    "↑/k: Previous  ↓/j: Next  Enter: Confirm  Esc: Cancel",
                    id="status-modal-help",
                )

    def action_next_option(self) -> None:
        """Move to the next status option."""
        self.selected_index = (self.selected_index + 1) % len(self.status_options)
        # Refresh the display if mounted
        try:
            display = self.query_one("#status-options", StatusOptionDisplay)
            display.update_selection(self.selected_index)
        except Exception:
            # Widget not mounted yet, that's okay during unit tests
            pass

    def action_previous_option(self) -> None:
        """Move to the previous status option."""
        self.selected_index = (self.selected_index - 1) % len(self.status_options)
        # Refresh the display if mounted
        try:
            display = self.query_one("#status-options", StatusOptionDisplay)
            display.update_selection(self.selected_index)
        except Exception:
            # Widget not mounted yet, that's okay during unit tests
            pass

    def action_confirm(self) -> None:
        """Confirm the selected status and close the modal."""
        selected_status = self.status_options[self.selected_index]
        self.dismiss(selected_status)

    def action_cancel(self) -> None:
        """Cancel the selection and close the modal."""
        self.dismiss(None)


class StatusOptionDisplay(Static):
    """Widget that displays the list of status options with the current selection highlighted."""

    def __init__(
        self,
        status_options: list[str],
        selected_index: int = 0,
        **kwargs,
    ) -> None:
        super().__init__(**kwargs)
        self.status_options = status_options
        self.selected_index = selected_index

    def render(self) -> str:
        """Render the status options with the current selection highlighted."""
        lines = []
        for idx, status in enumerate(self.status_options):
            if idx == self.selected_index:
                lines.append(f"[bold cyan]→ {status}[/bold cyan]")
            else:
                lines.append(f"  {status}")
        return "\n".join(lines)

    def update_selection(self, index: int) -> None:
        """Update the selected index and refresh the display."""
        self.selected_index = index
        self.refresh()
