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

    def populate(self, files: Iterable[FileState], selected: int) -> None:
        self.clear()
        files_list = list(files)
        for file_state in files_list:
            # Show just the filename for brevity
            label = file_state.path.name
            classes = (
                "file-has-markers" if file_state.has_marker_block else "file-no-markers"
            )
            list_item = ListItem(Label(str(label)), classes=classes)
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


class EditorPane(TextArea):
    """Editable text area used for drafting decisions."""

    can_focus = True

    def __init__(self, *, id: str | None = None) -> None:
        super().__init__(placeholder="Draft decision content…", id=id)
        self.read_only = True

    def set_content(self, content: str, *, editable: bool = False) -> None:
        self.value = content or ""
        self.cursor_position = (0, 0)
        self.read_only = not editable


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
        self._editor: Optional[EditorPane] = None
        self._pane_hints: Optional[Static] = None
        self._status_bar: Optional[Static] = None
        self._editor_mode: str = "view"
        self._editing_decision_id: Optional[int] = None
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
                self._editor = EditorPane(id="editor-pane")
                yield self._editor
                self._preview = PreviewPane(id="preview-pane")
                yield self._preview

    def on_mount(self) -> None:
        self._initialize_files()
        self.focus_pane(PaneId.DECISIONS)
        self.refresh_panes()

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
                self.app_state.files, self.app_state.selected_file_index
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
        decision_state = self.app_state.current_decision()
        if decision_state:
            rendered = decision_state.record.render()
        else:
            rendered = "Select a decision to edit."
        self._editor.set_content(rendered, editable=False)

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
        template = apply_template_to_editor(file_state.next_decision_id())
        self._editor_mode = "edit-new"
        self._editing_decision_id = None
        self._status_message = "Drafting new decision"
        self._editor.set_content(template, editable=True)
        self.focus_pane(PaneId.EDITOR)
        self._update_status_bar()

    def action_pick_status(self) -> None:
        if self._editor is None or self._editor.read_only:
            return
        try:
            record = self._parse_editor_record()
        except DecisionParseError as exc:
            self._show_message(f"Status change failed: {exc}")
            return
        try:
            current_index = self._status_options.index(record.status)
            next_status = self._status_options[
                (current_index + 1) % len(self._status_options)
            ]
        except ValueError:
            next_status = self._status_options[0]
        lines = self._editor.value.splitlines()
        for idx, line in enumerate(lines):
            if line.startswith("* **Status**:"):
                lines[idx] = f"* **Status**: {next_status}"
                break
        self._editor.set_content("\n".join(lines), editable=True)

    def action_save(self) -> None:
        if self._editor is None or self._editor.read_only:
            return
        try:
            record = self._parse_editor_record()
        except DecisionParseError as exc:
            self._show_message(f"Unable to save: {exc}")
            return
        if self._editor_mode == "edit-new":
            commands.create_decision(
                self.app_state,
                title=record.title,
                decision=record.decision,
                context=record.context,
                consequences=record.consequences,
                status=record.status,
            )
            self.app_state.selected_decision_index = 0
        elif (
            self._editor_mode == "edit-existing"
            and self._editing_decision_id is not None
        ):
            commands.update_decision(
                self.app_state,
                decision_id=self._editing_decision_id,
                title=record.title,
                status=record.status,
                decision_text=record.decision,
                context=record.context,
                consequences=record.consequences,
            )
        else:
            self._show_message("Nothing to save.")
            return
        self._persist_current_file()
        self._status_message = "Saved"
        self._reset_edit_state()
        self.refresh_panes()

    def action_refresh(self) -> None:
        self._reset_edit_state()
        self.refresh_panes()

    def action_cancel(self) -> None:
        """Cancel editing and return to view mode."""
        if self._editor_mode in ("edit-new", "edit-existing"):
            self._reset_edit_state()
            self.focus_pane(PaneId.DECISIONS)
            self._show_message("Editing cancelled")

    def action_show_help(self) -> None:
        self._show_message(
            "[space] edit  [n] new  [p] status  [s] save  [esc] cancel  [j/k or arrows] navigate"
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
        self._editor.set_content(decision_state.record.render(), editable=True)
        self.focus_pane(PaneId.EDITOR)
        self._update_status_bar()

    def _parse_editor_record(self):
        if self._editor is None:
            raise DecisionParseError("Editor unavailable.")
        content = self._editor.value
        records = parse_decisions(content)
        if len(records) != 1:
            raise DecisionParseError("Editor must contain exactly one decision entry.")
        return records[0]

    def _persist_current_file(self) -> None:
        file_state = self.app_state.current_file()
        if not file_state:
            return
        try:
            original_text = read_markdown(file_state.path)
        except FileNotFoundError:
            original_text = ""

        from vrdx.parser.markers import ensure_marker_block

        updated_text, block, _ = ensure_marker_block(original_text)
        body = commands.serialize_current_file(self.app_state)
        if body and not body.endswith("\n"):
            body += "\n"
        new_text = block.replace_body(updated_text, body)
        write_markdown(file_state.path, new_text)
        self.app_state.mark_saved()

    def _reset_edit_state(self) -> None:
        self._editor_mode = "view"
        self._editing_decision_id = None
        self._status_message = ""
        if self._editor is not None:
            self.refresh_editor()
        self._update_status_bar()

    def _show_message(self, message: str) -> None:
        self._status_message = message
        self.log(message)
        self._update_status_bar()

    def on_list_view_selected(self, event: ListView.Selected) -> None:
        """Handle selection changes in both decision and file lists."""
        if event.list_view == self._decision_list:
            # Decision list selection changed
            if event.list_view.index is not None:
                self.app_state.selected_decision_index = event.list_view.index
                self._reset_edit_state()
                self.refresh_panes()
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
