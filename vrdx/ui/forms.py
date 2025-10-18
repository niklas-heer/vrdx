"""Form-based decision editor widget for the vrdx TUI.

This module provides a structured form interface for creating and editing decisions,
with dedicated sections for Title, Status, Decision, Context, and Consequences fields.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Optional

from textual.app import ComposeResult
from textual.binding import Binding
from textual.containers import Container, Horizontal, Vertical
from textual.widgets import Button, Input, Label, Static, TextArea
from textual.message import Message

from vrdx.parser import DecisionRecord


@dataclass
class FormData:
    """Data class for form field values."""

    title: str
    status: str
    decision: str
    context: str
    consequences: str


class FormBasedDecisionEditor(Static):
    """A structured form-based editor for creating and editing decisions.

    This widget displays a form with dedicated sections for each decision field:
    Title, Status, Decision, Context, and Consequences. It supports both creating
    new decisions and editing existing ones.
    """

    CSS = """
    FormBasedDecisionEditor {
        width: 100%;
        height: 100%;
        layout: vertical;
        background: $surface;
    }

    #editor-header {
        width: 100%;
        height: auto;
        padding: 1;
        background: $panel;
        text-style: bold;
        border-bottom: solid $accent;
    }

    #form-scroll {
        width: 100%;
        height: 1fr;
        overflow: auto;
    }

    .form-section {
        width: 100%;
        height: auto;
        padding: 1;
        border-bottom: solid $boost;
    }

    .form-section-title {
        text-style: bold;
        color: $text-muted;
        padding-bottom: 0;
        height: auto;
    }

    #title-input {
        width: 100%;
        height: auto;
        margin-bottom: 2;
    }

    .status-section {
        width: 100%;
        height: auto;
        layout: horizontal;
        align-horizontal: left;
        align-vertical: middle;
    }

    .status-label {
        width: 1fr;
        height: auto;
        padding-right: 1;
    }

    #change-status-btn {
        width: auto;
        height: auto;
        margin-left: 1;
    }

    #decision-area {
        width: 100%;
        height: 10;
        margin-bottom: 1;
    }

    #context-area {
        width: 100%;
        height: 10;
        margin-bottom: 1;
    }

    #consequences-area {
        width: 100%;
        height: 10;
        margin-bottom: 1;
    }

    #button-row {
        width: 100%;
        height: auto;
        layout: horizontal;
        align-horizontal: left;
    }

    #save-btn {
        margin-right: 2;
    }

    .validation-error {
        color: $error;
        text-style: bold;
    }
    """

    BINDINGS = [
        Binding("ctrl+s", "save", show=False),
        Binding("escape", "cancel", show=False),
    ]

    class Saved(Message):
        """Posted when the form is saved with valid data."""

        def __init__(self, data: FormData) -> None:
            super().__init__()
            self.data = data

    class Cancelled(Message):
        """Posted when the form is cancelled."""

        pass

    class StatusChangeRequested(Message):
        """Posted when the user clicks the Change Status button."""

        pass

    def __init__(
        self,
        decision_id: int,
        current_status: str,
        *,
        id: str | None = None,
        classes: str | None = None,
    ) -> None:
        """Initialize the form-based decision editor.

        Args:
            decision_id: The ID of the decision being edited (or next ID for new).
            current_status: The current/default status for the decision.
            id: Optional ID for the widget.
            classes: Optional CSS classes for the widget.
        """
        super().__init__(id=id, classes=classes)
        self.decision_id = decision_id
        self.current_status = current_status
        self.is_new_decision = True

        # Form widgets (will be set in compose)
        self._title_input: Optional[Input] = None
        self._decision_area: Optional[TextArea] = None
        self._context_area: Optional[TextArea] = None
        self._consequences_area: Optional[TextArea] = None
        self._status_label: Optional[Label] = None
        self._error_label: Optional[Label] = None

    def compose(self) -> ComposeResult:
        """Compose the form layout."""
        # Header with decision ID
        yield Label(
            f"Create New Decision #{self.decision_id}",
            id="editor-header",
        )

        # Scrollable form content
        with Vertical(id="form-scroll"):
            # Title section
            with Vertical(classes="form-section"):
                yield Label("Title", classes="form-section-title")
                self._title_input = Input(
                    placeholder="Decision title (e.g., 'Use PostgreSQL for data storage')",
                    id="title-input",
                )
                yield self._title_input

            # Status section
            with Vertical(classes="form-section"):
                yield Label("Status", classes="form-section-title")
                with Horizontal(classes="status-section"):
                    self._status_label = Label(
                        self.current_status,
                        classes="status-label",
                    )
                    yield self._status_label
                    yield Button(
                        "[Change]",
                        id="change-status-btn",
                        variant="default",
                    )

            # Decision section
            with Vertical(classes="form-section"):
                yield Label("Decision", classes="form-section-title")
                self._decision_area = TextArea(
                    id="decision-area",
                    read_only=False,
                )
                yield self._decision_area

            # Context section
            with Vertical(classes="form-section"):
                yield Label("Context", classes="form-section-title")
                self._context_area = TextArea(
                    id="context-area",
                    read_only=False,
                )
                yield self._context_area

            # Consequences section
            with Vertical(classes="form-section"):
                yield Label("Consequences", classes="form-section-title")
                self._consequences_area = TextArea(
                    id="consequences-area",
                    read_only=False,
                )
                yield self._consequences_area

            # Error message (hidden by default)
            self._error_label = Label("", classes="validation-error")
            yield self._error_label

        # Button row
        with Horizontal(id="button-row"):
            yield Button("Save", id="save-btn", variant="primary")
            yield Button("Cancel", id="cancel-btn", variant="default")

    def on_mount(self) -> None:
        """Set up the form when mounted."""
        if self._title_input:
            self._title_input.focus()

    def on_button_pressed(self, event: Button.Pressed) -> None:
        """Handle button presses."""
        if event.button.id == "save-btn":
            self.action_save()
        elif event.button.id == "cancel-btn":
            self.action_cancel()
        elif event.button.id == "change-status-btn":
            self.post_message(self.StatusChangeRequested())

    def action_save(self) -> None:
        """Validate and save the form data."""
        # Get form data
        title = (self._title_input.value if self._title_input else "").strip()
        decision = (self._decision_area.text if self._decision_area else "").strip()
        context = (self._context_area.text if self._context_area else "").strip()
        consequences = (
            self._consequences_area.text if self._consequences_area else ""
        ).strip()

        # Validate title
        if not title or "Decision Title" in title or "decision title" in title.lower():
            self._show_error("Title cannot be empty or just placeholder text")
            return

        # Clear error
        if self._error_label:
            self._error_label.update("")

        # Create form data and post message
        form_data = FormData(
            title=title,
            status=self.current_status,
            decision=decision,
            context=context,
            consequences=consequences,
        )
        self.post_message(self.Saved(form_data))

    def action_cancel(self) -> None:
        """Cancel editing and post cancellation message."""
        self.post_message(self.Cancelled())

    def reset_for_new_decision(self, decision_id: int, status: str) -> None:
        """Reset the form for creating a new decision.

        Args:
            decision_id: The ID for the new decision.
            status: The status for the new decision.
        """
        self.decision_id = decision_id
        self.current_status = status
        self.is_new_decision = True

        # Update header to show "Create New" with correct ID
        header = self.query_one("#editor-header", Label)
        header.update(f"Create New Decision #{decision_id}")

        # Clear form fields
        if self._title_input:
            self._title_input.value = ""
        if self._decision_area:
            self._decision_area.text = ""
        if self._context_area:
            self._context_area.text = ""
        if self._consequences_area:
            self._consequences_area.text = ""
        if self._status_label:
            self._status_label.update(status)
        if self._error_label:
            self._error_label.update("")

    def set_existing_decision_data(
        self,
        record: DecisionRecord,
    ) -> None:
        """Populate the form with existing decision data.

        Args:
            record: The DecisionRecord to populate the form from.
        """
        self.is_new_decision = False
        if self._title_input:
            self._title_input.value = record.title
        if self._decision_area:
            self._decision_area.text = record.decision
        if self._context_area:
            self._context_area.text = record.context
        if self._consequences_area:
            self._consequences_area.text = record.consequences

        # Update header to show "Edit" instead of "Create New"
        header = self.query_one("#editor-header", Label)
        header.update(f"Edit Decision #{self.decision_id}")

    def set_status(self, status: str) -> None:
        """Update the status display.

        Args:
            status: The new status value.
        """
        self.current_status = status
        if self._status_label:
            self._status_label.update(status)

    def get_form_data(self) -> FormData:
        """Get the current form data.

        Returns:
            FormData object with current field values.
        """
        return FormData(
            title=(self._title_input.value if self._title_input else "").strip(),
            status=self.current_status,
            decision=(self._decision_area.text if self._decision_area else "").strip(),
            context=(self._context_area.text if self._context_area else "").strip(),
            consequences=(
                self._consequences_area.text if self._consequences_area else ""
            ).strip(),
        )

    def _show_error(self, message: str) -> None:
        """Display an error message in the form.

        Args:
            message: The error message to display.
        """
        if self._error_label:
            self._error_label.update(message)

    def clear_error(self) -> None:
        """Clear any error messages."""
        if self._error_label:
            self._error_label.update("")
