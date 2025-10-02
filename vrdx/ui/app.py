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
from vrdx.app.state import AppState, FileState, PaneId


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
        for index, file_state in enumerate(files):
            label = file_state.path.relative_to(file_state.path.parents[0])
            item = ListItem(Label(str(label)))
            self.append(item)
        if self.children and 0 <= selected < len(self.children):
            self.index = selected


class PreviewPane(Static):
    """Renders the selected decision in read-only form."""

    def show_decision(self, markdown: str) -> None:
        self.update(markdown or "No decision selected.")


class EditorPane(TextArea):
    """Editable text area for composing or updating decisions."""

    def __init__(self, *, id: str | None = None) -> None:
        super().__init__(placeholder="Draft decision content…", id=id)
        self.read_only = True

    def set_content(self, content: str, editable: bool = False) -> None:
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
        Binding("space", "select_decision", "Select decision", show=False),
        Binding("r", "refresh", "Refresh", show=False),
        Binding("?", "show_help", "Help", show=False),
        Binding("q", "quit", "Quit vrdx", show=True),
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
        self.refresh_panes()
        self.focus_pane(PaneId.DECISIONS)

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
        self.refresh_editor()
        self.update_dirty_indicator()

    def refresh_preview(self) -> None:
        decision_state = self.app_state.current_decision()
        if self._preview:
            content = decision_state.record.raw if decision_state else ""
            self._preview.show_decision(content)

    def refresh_editor(self) -> None:
        decision_state = self.app_state.current_decision()
        if self._editor:
            editable = False
            if decision_state:
                rendered = decision_state.record.render()
            else:
                rendered = "Select a decision to edit."
            self._editor.set_content(rendered, editable)

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
        self.refresh_panes()

    def action_previous_decision(self) -> None:
        commands.focus_previous_decision(self.app_state)
        self.refresh_panes()

    def action_select_decision(self) -> None:
        file_state = self.app_state.current_file()
        if not self._editor or file_state is None:
            return

        decision_state = self.app_state.current_decision()
        if decision_state:
            content = decision_state.record.render()
        else:
            content = commands.apply_template_to_editor(file_state.next_decision_id())

        self._editor.set_content(content, editable=True)
        self.focus_pane(PaneId.EDITOR)

    def action_refresh(self) -> None:
        self.refresh_panes()

    def action_show_help(self) -> None:
        self.push_screen(Static("Key bindings:\n" + "\n".join(self.help_actions())))

    def watch_dirty_indicator(self, dirty_indicator: str) -> None:
        if footer := self.query_one(Footer):
            footer.add_text(dirty_indicator)
