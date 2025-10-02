from pathlib import Path

import pytest

from vrdx.app import commands
from vrdx.app.state import AppState, FileState, PaneId
from vrdx.parser import DecisionParseError

EXAMPLE_BODY = (
    "### 2 Ship Parser\n"
    "* **Status**: ✅ Accepted\n"
    "* **Decision**: Ship the parser milestone.\n"
    "* **Context**: Enables the TUI to surface records.\n"
    "* **Consequences**: Users can browse decisions.\n"
    "\n"
    "### 1 Draft UI\n"
    "* **Status**: 📝 Draft\n"
    "* **Decision**: Build the basic panes.\n"
    "* **Context**: Required for iteration.\n"
    "* **Consequences**: Faster feedback.\n"
)


def _build_file_state(tmp_path: Path) -> FileState:
    file_state = FileState(path=tmp_path / "README.md", marker_present=True)
    file_state.parse_body(EXAMPLE_BODY)
    return file_state


def test_file_state_parsing_round_trip(tmp_path: Path):
    file_state = _build_file_state(tmp_path)
    assert len(file_state.decisions) == 2
    serialized = file_state.serialize_body()
    reparsed = FileState(path=file_state.path, marker_present=True)
    reparsed.parse_body(serialized)
    assert [d.record.id for d in reparsed.decisions] == [2, 1]


def test_file_state_next_decision_id(tmp_path: Path):
    file_state = _build_file_state(tmp_path)
    assert file_state.next_decision_id() == 3


def test_app_state_file_selection(tmp_path: Path):
    app_state = AppState(base_directory=tmp_path)
    file_a = _build_file_state(tmp_path)
    file_b = FileState(path=tmp_path / "notes.md", marker_present=False)
    app_state.set_files([file_a, file_b])

    assert app_state.current_file() is file_a
    app_state.select_file(1)
    assert app_state.current_file() is file_b

    with pytest.raises(IndexError):
        app_state.select_file(2)


def test_app_state_decision_selection(tmp_path: Path):
    app_state = AppState(base_directory=tmp_path)
    file_state = _build_file_state(tmp_path)
    app_state.set_files([file_state])

    assert app_state.current_decision().record.id == 2
    app_state.select_decision(1)
    assert app_state.current_decision().record.id == 1

    with pytest.raises(IndexError):
        app_state.select_decision(5)


def test_create_update_delete_decision(tmp_path: Path):
    app_state = AppState(base_directory=tmp_path)
    file_state = _build_file_state(tmp_path)
    app_state.set_files([file_state])

    created = commands.create_decision(
        app_state,
        title="Evaluate Editor",
        decision="Prototype Textual editor pane.",
        context="Needed for Milestone 6.",
        consequences="Allows editing workflow.",
        status="✅ Accepted",
    )
    assert created.record.id == 3
    assert app_state.is_modified

    updated = commands.update_decision(
        app_state,
        decision_id=3,
        title="Evaluate Editor Pane",
        status="📝 Draft",
    )
    assert updated.record.title == "Evaluate Editor Pane"
    assert updated.record.status == "📝 Draft"

    removed = commands.delete_decision(app_state, decision_id=3)
    assert removed.record.id == 3
    assert all(dec.record.id != 3 for dec in file_state.decisions)


def test_move_decision_updates_selection(tmp_path: Path):
    app_state = AppState(base_directory=tmp_path)
    file_state = _build_file_state(tmp_path)
    app_state.set_files([file_state])
    assert app_state.current_decision().record.id == 2

    commands.move_decision(app_state, from_index=0, to_index=1)
    assert [d.record.id for d in file_state.decisions] == [1, 2]
    assert app_state.selected_decision_index == 1


def test_link_decisions_are_reciprocal(tmp_path: Path):
    app_state = AppState(base_directory=tmp_path)
    file_state = _build_file_state(tmp_path)
    app_state.set_files([file_state])

    link = commands.link_decisions(
        app_state, source_id=2, target_id=1, relation="supersedes"
    )
    assert link.relation == "supersedes"
    decision_two = file_state.find_decision(2)
    decision_one = file_state.find_decision(1)
    assert any(
        link.relation == "supersedes" and link.target_id == 1
        for link in decision_two.links
    )
    assert any(
        link.relation == "deprecated_by" and link.target_id == 2
        for link in decision_one.links
    )

    commands.unlink_decisions(
        app_state, source_id=2, target_id=1, relation="supersedes"
    )
    assert not decision_two.links
    assert not decision_one.links


def test_focus_navigation_helpers(tmp_path: Path):
    app_state = AppState(base_directory=tmp_path)
    file_state = _build_file_state(tmp_path)
    app_state.set_files([file_state])

    commands.focus_next_decision(app_state)
    assert app_state.current_decision().record.id == 1
    commands.focus_previous_decision(app_state)
    assert app_state.current_decision().record.id == 2

    commands.focus_pane(app_state, PaneId.PREVIEW)
    assert app_state.active_pane == PaneId.PREVIEW


def test_refresh_file_from_body(tmp_path: Path):
    app_state = AppState(base_directory=tmp_path)
    refreshed = commands.refresh_file_from_body(
        app_state,
        path=tmp_path / "README.md",
        body=EXAMPLE_BODY,
        marker_present=True,
    )
    assert isinstance(refreshed, FileState)
    assert len(refreshed.decisions) == 2


def test_parse_failure_propagates(tmp_path: Path):
    app_state = AppState(base_directory=tmp_path)
    bad_body = "### Missing Fields\n* **Status**: 📝 Draft\n"
    with pytest.raises(DecisionParseError):
        commands.refresh_file_from_body(
            app_state,
            path=tmp_path / "bad.md",
            body=bad_body,
            marker_present=True,
        )
