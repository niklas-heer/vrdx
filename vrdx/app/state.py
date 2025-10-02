from __future__ import annotations

from dataclasses import dataclass, field
from enum import Enum
from pathlib import Path
from typing import Iterable, Literal, Optional

from vrdx.parser import (
    DecisionRecord,
    DecisionParseError,
    find_next_decision_id,
    list_status_options,
    parse_decisions,
    render_decisions,
)


class PaneId(str, Enum):
    """Identifiers for focusable panes in the TUI."""

    DECISIONS = "decisions"
    EDITOR = "editor"
    PREVIEW = "preview"
    FILES = "files"


LinkRelation = Literal["supersedes", "deprecated_by"]


@dataclass(frozen=True)
class DecisionLink:
    """Represents a directional relationship between decision identifiers."""

    source_id: int
    target_id: int
    relation: LinkRelation


@dataclass
class DecisionState:
    """Mutable wrapper around a parsed decision and its relationships."""

    record: DecisionRecord
    links: list[DecisionLink] = field(default_factory=list)

    def add_link(self, relation: LinkRelation, target_id: int) -> None:
        if any(
            link.relation == relation and link.target_id == target_id
            for link in self.links
        ):
            return
        self.links.append(DecisionLink(self.record.id, target_id, relation))

    def remove_link(self, relation: LinkRelation, target_id: int) -> None:
        self.links = [
            link
            for link in self.links
            if not (link.relation == relation and link.target_id == target_id)
        ]


@dataclass
class FileState:
    """Holds the decisions and metadata associated with a single markdown file."""

    path: Path
    decisions: list[DecisionState] = field(default_factory=list)
    marker_present: bool = False
    inserted_marker: bool = False

    @property
    def decision_records(self) -> list[DecisionRecord]:
        return [decision.record for decision in self.decisions]

    def parse_body(self, body: str) -> None:
        try:
            records = parse_decisions(body)
        except DecisionParseError as exc:  # pragma: no cover - defensive
            raise exc
        self.decisions = [DecisionState(record) for record in records]

    def serialize_body(self) -> str:
        return render_decisions(self.decision_records)

    def next_decision_id(self) -> int:
        return find_next_decision_id(self.decision_records)

    def find_decision(self, decision_id: int) -> Optional[DecisionState]:
        return next(
            (
                decision
                for decision in self.decisions
                if decision.record.id == decision_id
            ),
            None,
        )


@dataclass
class AppState:
    """Aggregates the core state for the vrdx TUI application."""

    base_directory: Path
    files: list[FileState] = field(default_factory=list)
    selected_file_index: int = 0
    selected_decision_index: int = 0
    active_pane: PaneId = PaneId.DECISIONS
    is_modified: bool = False

    def reset(self) -> None:
        self.files.clear()
        self.selected_file_index = 0
        self.selected_decision_index = 0
        self.is_modified = False

    def set_files(self, files: Iterable[FileState]) -> None:
        self.files = list(files)
        self.selected_file_index = 0 if self.files else -1
        self.selected_decision_index = 0
        self.is_modified = False

    def current_file(self) -> Optional[FileState]:
        if 0 <= self.selected_file_index < len(self.files):
            return self.files[self.selected_file_index]
        return None

    def select_file(self, index: int) -> None:
        if not (0 <= index < len(self.files)):
            raise IndexError(f"File index out of range: {index}")
        self.selected_file_index = index
        self.selected_decision_index = 0

    def current_decision(self) -> Optional[DecisionState]:
        file_state = self.current_file()
        if not file_state:
            return None
        if 0 <= self.selected_decision_index < len(file_state.decisions):
            return file_state.decisions[self.selected_decision_index]
        return None

    def select_decision(self, index: int) -> None:
        file_state = self.current_file()
        if not file_state:
            raise IndexError("No file selected")
        if not (0 <= index < len(file_state.decisions)):
            raise IndexError(f"Decision index out of range: {index}")
        self.selected_decision_index = index

    def mark_modified(self) -> None:
        self.is_modified = True

    def mark_saved(self) -> None:
        self.is_modified = False

    def focus_pane(self, pane: PaneId) -> None:
        self.active_pane = pane

    @property
    def status_options(self) -> list[str]:
        return list(list_status_options())

    def add_file(self, file_state: FileState) -> None:
        self.files.append(file_state)
        if self.selected_file_index == -1:
            self.selected_file_index = 0

    def remove_file(self, path: Path) -> None:
        self.files = [file for file in self.files if file.path != path]
        if not self.files:
            self.selected_file_index = -1
            self.selected_decision_index = -1
        elif self.selected_file_index >= len(self.files):
            self.selected_file_index = len(self.files) - 1
            self.selected_decision_index = 0
