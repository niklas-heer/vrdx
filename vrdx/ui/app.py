from __future__ import annotations

from dataclasses import dataclass
from pathlib import Path
from typing import Iterable, Optional

from textual.app import App, ComposeResult
from textual.binding import Binding
from textual.containers import Horizontal, Vertical
from textual.message import Message
from textual.reactive import reactive
from textual.widgets import Footer, Header, Label, ListItem, ListView, Static, TextArea

from vrdx.app import commands
from vrdx.app.persistence import read_markdown, write_markdown
from vrdx.app.state import AppState, FileState, PaneId
from vrdx.parser import DecisionParseError, list_status_options, parse_decisions
from vrdx.parser.markers import ensure_marker_block
from vrdx.parser.template import render_template


@dataclass
class PaneFocusChanged(Message):
    pane: PaneId


class DecisionList(ListView):
    """Displays the list of decisions for the active file."""

    def populate(self, file_state: Optional[FileState]) -> None:
        self.clear()
        if not file_state or not file_state.decisions:
            self.append(ListItem(Label("No decisions found.", id="empty-decisions")))
            return
        for decision in file_state.decisions:
            label = f"{decision.record.id}: {decision.record.title}"
            status = decision.record.status
            self.append(ListItem(Label(f"{label} ({status})")))
        self.index = 0


class FileList(ListView):
    """Displays discovered markdown files."""

    def populate(self, files: Iterable[FileState], selected: int) -> None:
        self.clear()
        for file_state in files:
            label = file_state.path.relative_to(file_state.path.parents[0])
            self.append(ListItem(Label(str(label))))
        if self.children and 0 <= selected < len(self.children):
            self.index = selected


class PreviewPane(Static):
    """Renders the selected decision in read-only form."""

    def show_decision(self, markdown: str) -> None:
        self.update(markdown or "No decision selected.")


class EditorPane(TextArea):
    """Editable text area used for drafting decisions."""

    def __init__(self, *, id: str | None = None) -> None:
        super().__init__(placeholder="Draft decision content…", id=id)
        self.read_only = True

    def set_content(self, content: str, *, editable: bool = False) -> None:
        self.value = content or ""
        self.cursor_position = len(self.value)
        self.read_only = not editable


class VrdxApp(App[None]):
    """Textual application shell for the vrdx decision manager."""

    CSS_PATH = "styles.tcss"

    BINDINGS = [
        Binding("1", "focus_decisions", "Decisions", show=False),
        Binding("2", "focus_editor", "Editor", show=False),
        Binding("3", "focus_preview", "Preview", show=False),
        Binding("4", "focus_files", "Files", show=False),
        Binding("j,down", "next_decision", "Next decision", show=False),
        Binding("k,up", "previous_decision", "Previous decision", show=False),
        Binding("space", "select_decision", "Edit", show=True),
        Binding("n", "new_decision", "New", show=True),
        Binding("p", "pick_status", "Status", show=True),
        Binding("s", "save", "Save", show=True),
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
        self._editor_mode: str = "view"
        self._editing_decision_id: Optional[int] = None
        self._status_options = list(list_status_options())
        self._status_message: str = ""

    def compose(self) -> ComposeResult:
        with Vertical():
            yield Header()
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
            yield Footer()

    def on_mount(self) -> None:
        self.focus_pane(PaneId.DECISIONS)
        self.refresh_panes()

    def refresh_panes(self) -> None:
        file_state = self.app_state.current_file()
        if self._decision_list:
            self._decision_list.populate(file_state)
            self._decision_list.index = self.app_state.selected_decision_index
        if self._file_list:
            self._file_list.populate(
                self.app_state.files, self.app_state.selected_file_index
            )
        self.refresh_preview()
        if self._editor_mode == "view":
            self.refresh_editor()
        self.update_dirty_indicator()
        self._update_footer()

    def refresh_preview(self) -> None:
        decision_state = self.app_state.current_decision()
        if self._preview:
            content = decision_state.record.render() if decision_state else ""
            self._preview.show_decision(content)

    def refresh_editor(self) -> None:
        if not self._editor:
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
        match pane:
            case PaneId.DECISIONS:
                if self._decision_list:
                    super().set_focus(self._decision_list)
            case PaneId.EDITOR:
                if self._editor:
                    super().set_focus(self._editor)
            case PaneId.PREVIEW:
                if self._preview:
                    super().set_focus(self._preview)
            case PaneId.FILES:
                if self._file_list:
                    super().set_focus(self._file_list)
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
        if not file_state or not self._editor:
            return
        template = render_template(file_state.next_decision_id())
        self._editor_mode = "edit-new"
        self._editing_decision_id = None
        self._status_message = "Drafting new decision"
        self._editor.set_content(template, editable=True)
        self.focus_pane(PaneId.EDITOR)
        self._update_footer()

    def action_pick_status(self) -> None:
        if not self._editor or self._editor.read_only:
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
        if not self._editor or self._editor.read_only:
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

    def action_show_help(self) -> None:
        help_lines = [
            "[space] edit",
            "[n] new decision",
            "[p] cycle status",
            "[s] save",
            "[q] quit",
            "[?] help",
        ]
        self.push_screen(
            Static("Key bindings:\n" + "\n".join(f"- {line}" for line in help_lines))
        )

    def watch_dirty_indicator(self, dirty_indicator: str) -> None:
        self._update_footer()

    def _begin_edit_existing(self) -> None:
        if not self._editor:
            return
        decision_state = self.app_state.current_decision()
        if not decision_state:
            return
        self._editor_mode = "edit-existing"
        self._editing_decision_id = decision_state.record.id
        self._status_message = f"Editing decision #{decision_state.record.id}"
        self._editor.set_content(decision_state.record.render(), editable=True)
        self.focus_pane(PaneId.EDITOR)
        self._update_footer()

    def _parse_editor_record(self):
        if not self._editor:
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
        self.refresh_editor()
        self._update_footer()

    def _show_message(self, message: str) -> None:
        self._status_message = message
        self.log(message)
        self._update_footer()

    def _update_footer(self) -> None:
        if footer := self.query_one(Footer):
            hint_text = (
                "[space] edit  [n] new  [p] status  [s] save  [q] quit  [?] help"
            )
            footer.update(
                f"{self.dirty_indicator}  {self._status_message or hint_text}"
            )
