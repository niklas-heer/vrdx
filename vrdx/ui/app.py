from __future__ import annotations

from dataclasses import dataclass
from importlib import resources
from pathlib import Path
from typing import Iterable, Optional

from textual.app import App, ComposeResult
from textual.binding import Binding
from textual.containers import Horizontal, Vertical
from textual.message import Message
from textual.reactive import reactive
from textual.widgets import (
    Header,
    Label,
    ListItem,
    ListView,
    Markdown,
    Static,
    TextArea,
)

from vrdx.app import commands
from vrdx.app.commands import apply_template_to_editor
from vrdx.app.discovery import find_markdown_files
from vrdx.app.persistence import read_markdown, write_markdown
from vrdx.app.state import AppState, FileState, PaneId
from vrdx.parser import DecisionParseError, list_status_options, parse_decisions
from vrdx.parser.markers import detect_marker_block, MarkerError
from vrdx.ui.modals import StatusSelectionModal
from vrdx.ui.forms import FormBasedDecisionEditor, FormData


try:
    _CSS_TEXT = (
        resources.files("vrdx.ui").joinpath("styles.tcss").read_text(encoding="utf-8")
    )
except (FileNotFoundError, OSError, AttributeError):
    _CSS_TEXT = """/* Layout and styling for the vrdx Textual TUI */

Screen {
    background: #111827;
    color: #f9fafb;
}

#main-layout {
    width: 100%;
    height: 100%;
}

#left-column {
    width: 25%;
    padding: 1;
    border-right: solid #374151;
}

#decisions-title,
#files-title {
    text-style: bold;
    padding-bottom: 0;
}

#pane-hints {
    padding: 0 1;
    color: #9ca3af;
}

#decision-list,
#file-list {
    border: solid #374151;
    background: #1f2937;
    padding: 0;
}

#file-list ListItem.file-no-markers {
    color: #9ca3af;
}

#file-list ListItem.file-no-markers Label {
    color: #9ca3af;
}

#editor-pane,
#preview-pane {
    border: solid #374151;
    padding: 1;
    background: #0f172a;
}

#editor-pane {
    width: 45%;
}

#preview-pane {
    width: 30%;
}

#status-bar {
    background: #1f2937;
    color: #f9fafb;
    border-top: solid #374151;
    padding: 0 1;
}
"""


@dataclass
class PaneFocusChanged(Message):
    pane: PaneId


class DecisionList(ListView):
    """Displays the list of decisions for the active file."""

    can_focus = True

    def populate(self, file_state: Optional[FileState]) -> None:
        self.clear()
        if not file_state or not file_state.decisions:
            self.append(ListItem(Label("No decisions found.", id="empty-decisions")))
            return
        for decision in file_state.decisions:
            label = f"{decision.record.id}: {decision.record.title}"
            status = decision.record.status
            self.append(ListItem(Label(f"{label} ({status})")))
        if self.children:
            self.index = 0


class FileList(ListView):
    """Displays discovered markdown files."""

    can_focus = True

    def populate(
        self, files: Iterable[FileState], selected: int, base_directory: Path
    ) -> None:
        self.clear()
        files_list = list(files)
        for file_state in files_list:
            # Show relative path from base directory
            try:
                relative_path = file_state.path.relative_to(base_directory)
                label = str(relative_path)
            except ValueError:
                # Fallback to just filename if path is not relative to base
                label = file_state.path.name
            classes = (
                "file-has-markers" if file_state.has_marker_block else "file-no-markers"
            )
            list_item = ListItem(Label(label), classes=classes)
            self.append(list_item)
        if self.children and 0 <= selected < len(self.children):
            self.index = selected


class PreviewPane(Markdown):
    """Renders the selected decision in read-only form as rendered Markdown."""

    can_focus = True

    def show_decision(self, markdown_text: str) -> None:
        if markdown_text:
            self.update(markdown_text)
        else:
            self.update(
                "# No Decision Selected\n\nSelect a decision from the list to preview it here."
            )


# EditorPane has been replaced with FormBasedDecisionEditor
# See vrdx/ui/forms.py for the new form-based editor implementation


class VrdxApp(App[None]):
    """Textual application shell for the vrdx decision manager."""

    CSS = _CSS_TEXT

    BINDINGS = [
        Binding("1", "focus_decisions", "Decisions", show=False),
        Binding("2", "focus_files", "Files", show=False),
        Binding("3", "focus_editor", "Editor", show=False),
        Binding("4", "focus_preview", "Preview", show=False),
        Binding("j,down", "next_decision", "Next decision", show=False),
        Binding("k,up", "previous_decision", "Previous decision", show=False),
        Binding("space", "select_decision", "Edit", show=True),
        Binding("n", "new_decision", "New", show=True),
        Binding("p", "pick_status", "Status", show=True),
        Binding("s", "save", "Save", show=True),
        Binding("escape", "cancel", "Cancel", show=False),
        Binding("r", "refresh", "Refresh", show=False),
        Binding("?", "show_help", "Help", show=True),
        Binding("q", "quit", "Quit", show=True),
    ]

    app_state: AppState
    dirty_indicator = reactive("● Saved")

    def __init__(self, app_state: Optional[AppState] = None) -> None:
        super().__init__()
        self.app_state = app_state or AppState(base_directory=Path("."))
        self._decision_list: Optional[DecisionList] = None
        self._file_list: Optional[FileList] = None
        self._preview: Optional[PreviewPane] = None
        self._editor: Optional[FormBasedDecisionEditor] = None
        self._pane_hints: Optional[Static] = None
        self._status_bar: Optional[Static] = None
        self._editor_mode: str = "view"
        self._editing_decision_id: Optional[int] = None
        self._pending_new_decision_id: Optional[int] = None
        self._pending_new_status: Optional[str] = None
        self._status_options = list(list_status_options())
        self._status_message: str = "Ready"

    def compose(self) -> ComposeResult:
        with Vertical():
            yield Header()
            self._status_bar = Static("", id="status-bar")
            yield self._status_bar
            self._pane_hints = Static(
                "1·Decisions  2·Files  3·Editor  4·Preview", id="pane-hints"
            )
            yield self._pane_hints
            with Horizontal(id="main-layout"):
                with Vertical(id="left-column"):
                    yield Label("Decisions", id="decisions-title")
                    self._decision_list = DecisionList(id="decision-list")
                    yield self._decision_list
                    yield Label("Files", id="files-title")
                    self._file_list = FileList(id="file-list")
                    yield self._file_list
                self._editor = FormBasedDecisionEditor(
                    decision_id=0,
                    current_status="📝 Draft",
                    id="editor-pane",
                )
                yield self._editor
                self._preview = PreviewPane(id="preview-pane")
                yield self._preview

    def on_mount(self) -> None:
        self._initialize_files()
        self.focus_pane(PaneId.DECISIONS)
        self.refresh_panes()
        self._load_most_recent_decision()

    def _initialize_files(self) -> None:
        base_directory = self.app_state.base_directory
        markdown_paths = find_markdown_files(base_directory)
        file_states = []
        for path in markdown_paths:
            try:
                text = read_markdown(path)
            except FileNotFoundError:
                continue
            try:
                block = detect_marker_block(text)
            except MarkerError as exc:
                self._show_message(f"{path.name}: {exc}")
                block = None

            inserted_marker = False
            body = block.body(text) if block else ""
            try:
                file_state = commands.refresh_file_from_body(
                    self.app_state,
                    path=path,
                    body=body,
                    marker_present=block is not None,
                    inserted_marker=inserted_marker,
                )
            except DecisionParseError as exc:
                self._show_message(f"{path.name}: {exc}")
                continue
            file_states.append(file_state)
        if file_states:
            file_states.sort(
                key=lambda fs: (not fs.has_marker_block, fs.path.name.lower())
            )
            commands.load_files(self.app_state, file_states)
            first_with_markers = next(
                (
                    idx
                    for idx, fs in enumerate(self.app_state.files)
                    if fs.has_marker_block
                ),
                0,
            )
            self.app_state.selected_file_index = first_with_markers
            self.app_state.selected_decision_index = 0
        else:
            self._show_message("No markdown files found in the current directory.")

    def refresh_panes(self) -> None:
        file_state = self.app_state.current_file()
        if self._decision_list is not None:
            self._decision_list.populate(file_state)
            self._decision_list.index = self.app_state.selected_decision_index
        if self._file_list is not None:
            self._file_list.populate(
                self.app_state.files,
                self.app_state.selected_file_index,
                self.app_state.base_directory,
            )
        self.refresh_preview()
        if self._editor_mode == "view":
            self.refresh_editor()
        self.update_dirty_indicator()
        self._update_status_bar()

    def refresh_preview(self) -> None:
        decision_state = self.app_state.current_decision()
        if self._preview is not None:
            content = decision_state.record.render() if decision_state else ""
            self._preview.show_decision(content)

    def refresh_editor(self) -> None:
        if self._editor is None:
            return
        # Form displays are managed by form state, not by refresh_editor
        pass

    def update_dirty_indicator(self) -> None:
        self.dirty_indicator = "● Unsaved" if self.app_state.is_modified else "● Saved"

    def focus_pane(self, pane: PaneId) -> None:
        self.app_state.focus_pane(pane)
        widget_to_focus = None
        match pane:
            case PaneId.DECISIONS:
                widget_to_focus = self._decision_list
            case PaneId.EDITOR:
                widget_to_focus = self._editor
            case PaneId.PREVIEW:
                widget_to_focus = self._preview
            case PaneId.FILES:
                widget_to_focus = self._file_list

        if widget_to_focus:
            try:
                self.set_focus(widget_to_focus)
            except Exception:
                # Widget might not be focusable, continue anyway
                pass
        self.post_message(PaneFocusChanged(pane))

    def action_focus_decisions(self) -> None:
        self.focus_pane(PaneId.DECISIONS)

    def action_focus_editor(self) -> None:
        self.focus_pane(PaneId.EDITOR)

    def action_focus_preview(self) -> None:
        self.focus_pane(PaneId.PREVIEW)

    def action_focus_files(self) -> None:
        self.focus_pane(PaneId.FILES)

    def action_next_decision(self) -> None:
        commands.focus_next_decision(self.app_state)
        self._reset_edit_state()
        self.refresh_panes()

    def action_previous_decision(self) -> None:
        commands.focus_previous_decision(self.app_state)
        self._reset_edit_state()
        self.refresh_panes()

    def action_select_decision(self) -> None:
        self._begin_edit_existing()

    def action_new_decision(self) -> None:
        file_state = self.app_state.current_file()
        if not file_state or self._editor is None:
            return
        # Store the next decision ID for use in the modal callback
        self._pending_new_decision_id = file_state.next_decision_id()
        # Show the status selection modal
        self.push_screen(StatusSelectionModal(), callback=self._on_status_selected)

    def _on_status_selected(self, status: Optional[str]) -> None:
        """Handle the result from the status selection modal.

        Args:
            status: The selected status, or None if the selection was cancelled.
        """
        if (
            status is None
            or self._pending_new_decision_id is None
            or self._editor is None
        ):
            self._pending_new_decision_id = None
            return

        # Configure the form for new decision
        self._editor.reset_for_new_decision(self._pending_new_decision_id, status)
        self._editor_mode = "edit-new"
        self._editing_decision_id = None
        self._status_message = "Creating new decision"
        self.focus_pane(PaneId.EDITOR)
        self._update_status_bar()
        self._pending_new_decision_id = None
        self._pending_new_status = status

    def action_pick_status(self) -> None:
        if self._editor is None or self._editor_mode == "view":
            return
        # Show status selection modal
        self.push_screen(StatusSelectionModal(), callback=self._on_status_changed)

    def _on_status_changed(self, status: Optional[str]) -> None:
        """Handle status change from modal while editing.

        Args:
            status: The selected status, or None if cancelled.
        """
        if status is None or self._editor is None:
            return
        self._editor.set_status(status)
        self._status_message = f"Status changed to {status}"
        self._update_status_bar()

    def action_save(self) -> None:
        if self._editor is None:
            return
        # The form editor will validate and post a Saved message
        self._editor.action_save()

    def action_refresh(self) -> None:
        self._reset_edit_state()
        self.refresh_panes()

    def action_cancel(self) -> None:
        """Cancel editing and return to view mode."""
        if self._editor is None:
            return
        # The form editor will post a Cancelled message
        self._editor.action_cancel()

    def action_show_help(self) -> None:
        self._show_message(
            "[space] edit  [n] new (select status)  [p] cycle status  [s] save  [esc] cancel  [j/k or arrows] navigate"
        )

    def watch_dirty_indicator(self, dirty_indicator: str) -> None:
        self._update_status_bar()

    def _begin_edit_existing(self) -> None:
        if self._editor is None:
            return
        decision_state = self.app_state.current_decision()
        if not decision_state:
            return
        self._editor_mode = "edit-existing"
        self._editing_decision_id = decision_state.record.id
        self._status_message = f"Editing decision #{decision_state.record.id}"
        # Configure the form for existing decision
        self._editor.decision_id = decision_state.record.id
        self._editor.current_status = decision_state.record.status
        self._editor.is_new_decision = False
        # Clear and prepare form for editing
        try:
            header = self._editor.query_one("#editor-header", Label)
            header.update(f"Edit Decision #{decision_state.record.id}")
        except Exception:
            # Header widget might not be available, continue anyway
            pass
        self._editor.set_existing_decision_data(decision_state.record)
        self.focus_pane(PaneId.EDITOR)
        self._update_status_bar()

    def _handle_form_saved(self, form_data: FormData) -> None:
        """Handle form saved event.

        Args:
            form_data: The form data that was submitted.
        """
        if self._editor_mode == "edit-new":
            commands.create_decision(
                self.app_state,
                title=form_data.title,
                decision=form_data.decision,
                context=form_data.context,
                consequences=form_data.consequences,
                status=form_data.status,
            )
            self.app_state.selected_decision_index = 0
        elif (
            self._editor_mode == "edit-existing"
            and self._editing_decision_id is not None
        ):
            commands.update_decision(
                self.app_state,
                decision_id=self._editing_decision_id,
                title=form_data.title,
                status=form_data.status,
                decision_text=form_data.decision,
                context=form_data.context,
                consequences=form_data.consequences,
            )
        else:
            self._show_message("Nothing to save.")
            return
        self._persist_current_file()
        self._status_message = "Saved"
        self._reset_edit_state()
        self.refresh_panes()

    def _handle_form_cancelled(self) -> None:
        """Handle form cancelled event."""
        self._reset_edit_state()
        self.focus_pane(PaneId.DECISIONS)
        self._show_message("Editing cancelled")

    def _handle_status_change_requested(self) -> None:
        """Handle status change request from form."""
        self.push_screen(StatusSelectionModal(), callback=self._on_status_changed)

    def _persist_current_file(self) -> None:
        file_state = self.app_state.current_file()
        if not file_state:
            return
        try:
            original_text = read_markdown(file_state.path)
        except FileNotFoundError:
            return

        from vrdx.parser.markers import detect_marker_block

        try:
            block = detect_marker_block(original_text)
        except MarkerError:
            self._show_message(
                f"Cannot save: {file_state.path.name} has invalid markers"
            )
            return

        if block is None:
            self._show_message(
                f"Cannot save: {file_state.path.name} has no decision marker block"
            )
            return

        body = commands.serialize_current_file(self.app_state)
        if body and not body.endswith("\n"):
            body += "\n"
        new_text = block.replace_body(original_text, body)
        write_markdown(file_state.path, new_text)
        self.app_state.mark_saved()

    def _reset_edit_state(self) -> None:
        self._editor_mode = "view"
        self._editing_decision_id = None
        self._status_message = ""
        if self._editor is not None:
            self.refresh_editor()
        self._update_status_bar()

    def _load_most_recent_decision(self) -> None:
        """Load the most recent decision into the editing pane.

        The most recent decision is the first one in the list since
        decisions are ordered by ID in descending order.
        """
        file_state = self.app_state.current_file()
        if file_state and file_state.decisions:
            # Set the selection to the first decision (most recent)
            self.app_state.selected_decision_index = 0
            self._begin_edit_existing()

    def _show_message(self, message: str) -> None:
        self._status_message = message
        self.log(message)
        self._update_status_bar()

    def on_form_based_decision_editor_saved(
        self, event: FormBasedDecisionEditor.Saved
    ) -> None:
        """Handle form saved event."""
        self._handle_form_saved(event.data)

    def on_form_based_decision_editor_cancelled(
        self, event: FormBasedDecisionEditor.Cancelled
    ) -> None:
        """Handle form cancelled event."""
        self._handle_form_cancelled()

    def on_form_based_decision_editor_status_change_requested(
        self, event: FormBasedDecisionEditor.StatusChangeRequested
    ) -> None:
        """Handle status change request from form."""
        self._handle_status_change_requested()

    def on_list_view_selected(self, event: ListView.Selected) -> None:
        """Handle selection changes in both decision and file lists."""
        if event.list_view == self._decision_list:
            # Decision list selection changed
            if event.list_view.index is not None:
                self.app_state.selected_decision_index = event.list_view.index
                self.refresh_panes()
                # Load the selected decision into the editing pane
                self._begin_edit_existing()
        elif event.list_view == self._file_list:
            # File list selection changed
            if event.list_view.index is not None and 0 <= event.list_view.index < len(
                self.app_state.files
            ):
                self.app_state.select_file(event.list_view.index)
                self._reset_edit_state()
                self.refresh_panes()

    def _update_status_bar(self) -> None:
        if self._status_bar is not None:
            # Build mode indicator (vim-style)
            mode_indicator = ""
            if self._editor_mode == "edit-new":
                mode_indicator = "-- INSERT (New) --"
            elif self._editor_mode == "edit-existing":
                mode_indicator = "-- EDIT --"
            elif self._editor_mode == "view":
                mode_indicator = "-- NORMAL --"

            # Build hint text
            if self._editor_mode in ("edit-new", "edit-existing"):
                hint_text = "[s] save  [esc] cancel  [p] cycle status"
            else:
                hint_text = (
                    "[space] edit  [n] new  [1-4] focus panes  [?] help  [q] quit"
                )

            # Combine status message or hint with mode
            status_text = self._status_message or hint_text
            full_status = f"{self.dirty_indicator}  {mode_indicator}  {status_text}"

            self._status_bar.update(full_status)
