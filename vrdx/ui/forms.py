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
from textual.widgets import Button, Input, Label, Static, TextArea, Select
from textual.message import Message

from vrdx.parser import DecisionRecord, list_status_options


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

    This widget provides a comprehensive form interface for decision management with:
    - Title input field (required, validated)
    - Status dropdown selector with all available options
    - Decision textarea for the decision statement
    - Context textarea for decision context
    - Consequences textarea for decision consequences
    - Save and Cancel buttons with keyboard shortcuts (Ctrl+S and Esc)

    Features:
    - Form-based editing without modal disruption
    - Inline validation with error messages
    - Auto-population when editing existing decisions
    - Proper focus management between fields
    - Supports both creating new decisions and editing existing ones

    Usage:
    - For new decisions: decision_id should be the next available ID, current_status
      should be the initial status
    - For existing decisions: call set_existing_decision_data() after mounting to
      populate all fields

    Events:
    - Saved(FormData): Posted when form is successfully saved
    - Cancelled: Posted when form is cancelled
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
        padding: 1 1;
    }

    .form-section {
        width: 100%;
        height: auto;
        padding: 1 1 2 1;
        margin-bottom: 1;
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
        margin-bottom: 1;
    }

    .status-section {
        width: 100%;
        height: auto;
    }

    #status-select {
        width: 100%;
        height: auto;
    }

    #decision-area {
        width: 100%;
        height: 8;
        margin-bottom: 1;
    }

    #context-area {
        width: 100%;
        height: 8;
        margin-bottom: 1;
    }

    #consequences-area {
        width: 100%;
        height: 8;
        margin-bottom: 1;
    }

    #button-row {
        width: 100%;
        height: auto;
        layout: horizontal;
        align-horizontal: left;
        padding: 1 1;
        margin-top: 1;
        border-top: solid $boost;
    }

    #save-btn {
        margin-right: 3;
    }

    #cancel-btn {
        margin: 0;
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
        self._status_select: Optional[Select] = None
        self._error_label: Optional[Label] = None

    def compose(self) -> ComposeResult:
        """Compose the form layout.

        Creates a vertical layout with:
        1. Header showing decision ID (updated when editing)
        2. Scrollable form content with all input fields
        3. Button row with Save and Cancel actions

        The form uses a scrollable container to handle content that exceeds
        available screen height.
        """
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
                with Vertical(classes="status-section"):
                    # Create Select with status options
                    status_options = [
                        (status, status) for status in list_status_options()
                    ]
                    self._status_select = Select(
                        status_options,
                        value=self.current_status,
                        id="status-select",
                    )
                    yield self._status_select

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
        """Set up the form when mounted.

        Initializes the form by focusing on the title input field,
        making it ready for immediate user interaction.
        """
        if self._title_input:
            self._title_input.focus()

    def on_button_pressed(self, event: Button.Pressed) -> None:
        """Handle button presses.

        Routes Save and Cancel button clicks to their respective action handlers.

        Args:
            event: The button pressed event containing button information.
        """
        if event.button.id == "save-btn":
            self.action_save()
        elif event.button.id == "cancel-btn":
            self.action_cancel()

    def on_select_changed(self, event: Select.Changed) -> None:
        """Handle status selection changes.

        Updates the current_status when the user selects a different status
        from the dropdown. This keeps the form state synchronized with the
        Select widget value.

        Args:
            event: The Select widget change event with the new value.
        """
        if event.select.id == "status-select":
            self.current_status = event.value

    def action_save(self) -> None:
        """Validate and save the form data.

        Performs the following steps:
        1. Extracts and strips whitespace from all form fields
        2. Validates the title field (required, not placeholder text)
        3. Displays error message if validation fails
        4. Creates FormData object with validated fields
        5. Posts Saved message to trigger parent app to persist the decision

        The title is required and cannot be empty or contain placeholder text.
        Other fields (decision, context, consequences) can be empty and are
        optional.
        """
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
        """Cancel editing and post cancellation message.

        Abandons any changes made to the form and posts a Cancelled message.
        The parent app will handle cleaning up the edit state and returning
        to the previous view.
        """
        self.post_message(self.Cancelled())

    def reset_for_new_decision(self, decision_id: int, status: str) -> None:
        """Reset the form for creating a new decision.

        Prepares the form to create a new decision by:
        1. Clearing all form fields
        2. Updating the header with the new decision ID
        3. Setting the status to the provided value
        4. Clearing any error messages
        5. Setting is_new_decision flag to True

        Args:
            decision_id: The next available decision ID to be displayed in the header.
            status: The initial status for the new decision (from status selection modal).
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
        if self._status_select:
            self._status_select.value = status
        if self._error_label:
            self._error_label.update("")

    def set_existing_decision_data(
        self,
        record: DecisionRecord,
    ) -> None:
        """Populate the form with existing decision data for editing.

        Loads all decision fields from the DecisionRecord into the form fields:
        - Copies title, decision, context, and consequences text
        - Updates status dropdown to show current decision status
        - Updates header to show "Edit" mode instead of "Create New"
        - Sets is_new_decision flag to False

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

        # Update status
        self.current_status = record.status
        if self._status_select:
            self._status_select.value = record.status

        # Update header to show "Edit" instead of "Create New"
        header = self.query_one("#editor-header", Label)
        header.update(f"Edit Decision #{self.decision_id}")

    def set_status(self, status: str) -> None:
        """Update the status display.

        Updates the current_status and synchronizes the status Select widget
        to show the new status value. Used when status changes are made
        programmatically.

        Args:
            status: The new status value to display.
        """
        self.current_status = status
        if self._status_select:
            self._status_select.value = status

    def get_form_data(self) -> FormData:
        """Get the current form data.

        Extracts all form field values and returns them as a FormData object.
        All text fields are trimmed of leading/trailing whitespace.

        Returns:
            FormData object with current field values (title, status, decision,
            context, consequences).
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

        Shows an error message to the user in the error label at the bottom
        of the form. Used for validation errors and other user-facing messages.

        Args:
            message: The error message to display.
        """
        if self._error_label:
            self._error_label.update(message)

    def clear_error(self) -> None:
        """Clear any error messages.

        Removes any previously displayed error messages from the form.
        """
        if self._error_label:
            self._error_label.update("")
