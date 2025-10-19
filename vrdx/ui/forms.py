"""Form-based decision editor widget for the vrdx TUI.

This module provides a structured form interface for creating and editing decisions,
with dedicated sections for Title, Status, Decision, Context, and Consequences fields.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Optional

from textual.app import ComposeResult
from textual.binding import Binding
from textual.containers import Container, Grid, Horizontal, Vertical
from textual.reactive import reactive
from textual.widgets import Button, Label, Static, TextArea, Select
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
    - Title textarea field (required, validated)
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
        background: $panel;
    }

    #editor-header {
        display: none;
    }

    #form-scroll {
        width: 100%;
        height: 1fr;
        overflow: auto;
        padding: 0;
    }

    .form-section {
        width: 100%;
        height: auto;
        padding: 0;
        margin: 0;
        border: none;
        text-opacity: 100%;
    }

    .form-section-title {
        text-style: bold;
        color: $accent;
        padding: 0;
        height: auto;
        margin-bottom: 0;
        margin-top: 0;
        text-opacity: 100%;
    }

    .form-separator {
        display: none;
    }

    #title-area {
        width: 100%;
        height: 1;
        margin: 0;
        border: solid $accent;
        background: $surface;
    }

    #title-area:focus {
        border: solid $primary;
        background: $surface;
    }

    .status-section {
        width: 100%;
        height: auto;
        margin: 0;
    }

    #status-select {
        width: 100%;
        height: auto;
        border: solid $accent;
        background: $surface;
        margin: 0;
    }

    #status-select:focus {
        border: solid $primary;
        background: $surface;
    }

    #decision-area {
        width: 100%;
        height: 1;
        margin: 0;
        border: solid $accent;
        background: $surface;
    }

    #decision-area:focus {
        border: solid $primary;
        background: $surface;
    }

    #context-area {
        width: 100%;
        height: 1;
        margin: 0;
        border: solid $accent;
        background: $surface;
    }

    #context-area:focus {
        border: solid $primary;
        background: $surface;
    }

    #consequences-area {
        width: 100%;
        height: 1;
        margin: 0;
        border: solid $accent;
        background: $surface;
    }

    #consequences-area:focus {
        border: solid $primary;
        background: $surface;
    }

    #button-row {
        width: auto;
        height: auto;
        layout: horizontal;
        align-horizontal: left;
        padding: 0;
        margin: 0;
        border: none;
        text-opacity: 100%;
    }



    .validation-error {
        color: $error;
        text-style: bold;
        margin-top: 1;
        text-opacity: 0%;
    }

    .validation-error.show {
        text-opacity: 100%;
    }

    Input:focus,
    TextArea:focus,
    Select:focus {
        border: solid $primary;
    }

    Input {
        background: $surface;
    }

    TextArea {
        background: $surface;
    }

    Select {
        background: $surface;
    }

    Button {
        width: auto;
        padding: 0;
        height: 1;
        margin: 0;
        border: none;
    }

    #save-btn {
        background: $success;
        color: $text;
    }

    #save-btn:hover {
        background-tint: white 20%;
        text-style: b;
    }

    #save-btn:focus {
        border: solid $primary;
    }

    #cancel-btn {
        background: $error;
        color: $text;
    }

    #cancel-btn:hover {
        background-tint: white 20%;
        text-style: b;
    }

    #cancel-btn:focus {
        border: solid $primary;
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
        self._title_area: Optional[TextArea] = None
        self._decision_area: Optional[TextArea] = None
        self._context_area: Optional[TextArea] = None
        self._consequences_area: Optional[TextArea] = None
        self._status_select: Optional[Select] = None
        self._error_label: Optional[Label] = None

        # Dynamic textarea sizing - textareas will grow as needed up to max
        self._title_height = reactive(1, init=False)
        self._decision_height = reactive(1, init=False)
        self._context_height = reactive(1, init=False)
        self._consequences_height = reactive(1, init=False)

    def compose(self) -> ComposeResult:
        """Compose the form layout.

        Creates a vertical layout with:
        1. Header showing decision ID (updated when editing)
        2. Scrollable form content with all input fields on same line as labels
        3. Button row with Save and Cancel actions

        The form uses a scrollable container to handle content that exceeds
        available screen height, with visual separators between sections.
        """
        # Header with decision ID
        yield Label(
            f"Create New Decision #{self.decision_id}",
            id="editor-header",
        )

        # Scrollable form content
        with Vertical(id="form-scroll"):
            # Use Grid for consistent label/input alignment
            with Grid(id="form-grid"):
                # Title row
                yield Label("Title:", classes="form-section-title")
                self._title_area = TextArea(
                    id="title-area",
                    read_only=False,
                )
                yield self._title_area

                # Status row
                yield Label("Status:", classes="form-section-title")
                # Create Select with status options
                status_options = [(status, status) for status in list_status_options()]
                self._status_select = Select(
                    status_options,
                    value=self.current_status,
                    id="status-select",
                    compact=True,
                )
                yield self._status_select

                # Decision row
                yield Label("Decision:", classes="form-section-title")
                self._decision_area = TextArea(
                    id="decision-area",
                    read_only=False,
                    compact=True,
                )
                yield self._decision_area

                # Context row
                yield Label("Context:", classes="form-section-title")
                self._context_area = TextArea(
                    id="context-area",
                    read_only=False,
                    compact=True,
                )
                yield self._context_area

                # Consequences row
                yield Label("Consequences:", classes="form-section-title")
                self._consequences_area = TextArea(
                    id="consequences-area",
                    read_only=False,
                    compact=True,
                )
                yield self._consequences_area

            # Error message (hidden by default)
            self._error_label = Label("", classes="validation-error")
            yield self._error_label

        # Button row
        with Horizontal(id="button-row"):
            yield Button("Save", id="save-btn", variant="default")
            yield Button("Cancel", id="cancel-btn", variant="default")

    def _calculate_textarea_height(
        self, textarea: TextArea, min_height: int, max_height: int
    ) -> int:
        """Calculate appropriate height for a textarea based on content.

        Args:
            textarea: The TextArea widget to measure.
            min_height: Minimum height in lines.
            max_height: Maximum height in lines.

        Returns:
            The calculated height in lines, clamped between min and max.
        """
        text = textarea.text
        if not text:
            return min_height

        # Count lines in the text
        lines = len(text.split("\n"))

        # Clamp between min and max
        return max(min_height, min(lines, max_height))

    def _update_textarea_height(
        self, textarea: Optional[TextArea], height: int
    ) -> None:
        """Update the height style of a textarea.

        Args:
            textarea: The TextArea widget to update.
            height: The new height in lines.
        """
        if textarea:
            textarea.styles.height = height

    def on_mount(self) -> None:
        """Set up the form when mounted.

        Initializes the form by focusing on the title field,
        making it ready for immediate user interaction, with
        entrance animations for visual polish.
        """
        # Animate header fade-in
        header = self.query_one("#editor-header", Label)
        header.styles.animate(
            "opacity",
            value=1.0,
            duration=0.5,
            easing="in_out_cubic",
        )

        # Animate form sections with staggered entrance
        sections = self.query(".form-section")
        for i, section in enumerate(sections):
            section.styles.animate(
                "opacity",
                value=1.0,
                duration=0.4,
                easing="out_cubic",
                delay=0.1 + (i * 0.08),
            )

        # Animate separators with staggered entrance
        separators = self.query(".form-separator")
        for i, separator in enumerate(separators):
            separator.styles.animate(
                "opacity",
                value=1.0,
                duration=0.4,
                easing="out_cubic",
                delay=0.15 + (i * 0.1),
            )

        # Animate button row fade-in
        button_row = self.query_one("#button-row", Horizontal)
        button_row.styles.animate(
            "opacity",
            value=1.0,
            duration=0.5,
            easing="in_out_cubic",
            delay=0.5,
        )

        # Focus on title area
        if self._title_area:
            self._title_area.focus()

    def watch_title_height(self, height: int) -> None:
        """Watch for changes to title height reactive attribute."""
        self._update_textarea_height(self._title_area, height)

    def watch_decision_height(self, height: int) -> None:
        """Watch for changes to decision height reactive attribute."""
        self._update_textarea_height(self._decision_area, height)

    def watch_context_height(self, height: int) -> None:
        """Watch for changes to context height reactive attribute."""
        self._update_textarea_height(self._context_area, height)

    def watch_consequences_height(self, height: int) -> None:
        """Watch for changes to consequences height reactive attribute."""
        self._update_textarea_height(self._consequences_area, height)

    def on_button_pressed(self, event: Button.Pressed) -> None:
        """Handle button presses.

        Routes Save and Cancel button clicks to their respective action handlers.
        Provides visual feedback with animations on interaction.

        Args:
            event: The button pressed event containing button information.
        """
        if event.button.id == "save-btn":
            # Add scale animation feedback
            event.button.styles.animate(
                "scale",
                value=(0.95, 0.95),
                duration=0.2,
                easing="in_out_cubic",
            )
            self.action_save()
        elif event.button.id == "cancel-btn":
            # Add scale animation feedback
            event.button.styles.animate(
                "scale",
                value=(0.95, 0.95),
                duration=0.2,
                easing="in_out_cubic",
            )
            self.action_cancel()

    def on_text_area_changed(self, event: TextArea.Changed) -> None:
        """Handle textarea content changes for dynamic sizing.

        Updates textarea heights when content changes to fit the current content.

        Args:
            event: The TextArea changed event.
        """
        textarea = event.text_area

        if textarea.id == "title-area":
            self._title_height = self._calculate_textarea_height(textarea, 1, 2)
        elif textarea.id == "decision-area":
            self._decision_height = self._calculate_textarea_height(textarea, 1, 3)
        elif textarea.id == "context-area":
            self._context_height = self._calculate_textarea_height(textarea, 1, 3)
        elif textarea.id == "consequences-area":
            self._consequences_height = self._calculate_textarea_height(textarea, 1, 3)

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
        title = (self._title_area.text if self._title_area else "").strip()
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
        if self._title_area:
            self._title_area.text = ""
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
        if self._title_area:
            self._title_area.text = record.title
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
            title=(self._title_area.text if self._title_area else "").strip(),
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
        Error messages fade in with visual animation.

        Args:
            message: The error message to display.
        """
        if self._error_label:
            self._error_label.update(message)
            self._error_label.add_class("show")
            # Animate error message fade-in
            self._error_label.styles.animate(
                "text_opacity",
                value=1.0,
                duration=0.4,
                easing="in_out_cubic",
            )

    def clear_error(self) -> None:
        """Clear any error messages.

        Removes any previously displayed error messages from the form.
        Error messages fade out before being cleared.
        """
        if self._error_label:
            self._error_label.remove_class("show")
            # Animate error message fade-out
            self._error_label.styles.animate(
                "text_opacity",
                value=0.0,
                duration=0.3,
                easing="in_out_cubic",
                on_complete=lambda: self._error_label.update("")
                if self._error_label
                else None,
            )
