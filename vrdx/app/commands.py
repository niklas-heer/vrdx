from __future__ import annotations

from dataclasses import replace
from typing import Iterable, Optional

from vrdx.app.state import AppState, DecisionLink, DecisionState, FileState, PaneId
from vrdx.parser import (
    DEFAULT_STATUS,
    DecisionRecord,
    DecisionTemplate,
    DecisionParseError,
    normalise_status,
    render_template,
)

######################################################################
# Helpers
######################################################################


def _require_active_file(app_state: AppState) -> FileState:
    file_state = app_state.current_file()
    if file_state is None:
        raise RuntimeError("No markdown file is currently selected.")
    return file_state


def _require_active_decision(app_state: AppState) -> DecisionState:
    decision_state = app_state.current_decision()
    if decision_state is None:
        raise RuntimeError("No decision is currently selected.")
    return decision_state


def _clone_record(record: DecisionRecord, **updates) -> DecisionRecord:
    return record.model_copy(update=updates)


######################################################################
# Creation & Editing
######################################################################


def create_decision(
    app_state: AppState,
    *,
    title: str,
    decision: str,
    context: str,
    consequences: str,
    status: Optional[str] = None,
) -> DecisionState:
    file_state = _require_active_file(app_state)

    next_id = file_state.next_decision_id()
    record = DecisionRecord(
        id=next_id,
        title=title.strip() or f"Decision {next_id}",
        status=normalise_status(status or DEFAULT_STATUS),
        decision=decision.strip(),
        context=context.strip(),
        consequences=consequences.strip(),
        raw="",
    ).model_copy(update={"raw": ""})
    record = record.model_copy(update={"raw": record.render()})
    decision_state = DecisionState(record=record)
    file_state.decisions.insert(0, decision_state)
    app_state.selected_decision_index = 0
    app_state.mark_modified()
    return decision_state


def update_decision(
    app_state: AppState,
    *,
    decision_id: int,
    title: Optional[str] = None,
    status: Optional[str] = None,
    decision_text: Optional[str] = None,
    context: Optional[str] = None,
    consequences: Optional[str] = None,
) -> DecisionState:
    file_state = _require_active_file(app_state)
    decision_state = file_state.find_decision(decision_id)
    if decision_state is None:
        raise ValueError(f"Decision with id {decision_id} not found.")

    record = decision_state.record
    updated = _clone_record(
        record,
        title=title.strip() if title is not None else record.title,
        status=normalise_status(status) if status is not None else record.status,
        decision=(
            decision_text.strip() if decision_text is not None else record.decision
        ),
        context=context.strip() if context is not None else record.context,
        consequences=(
            consequences.strip() if consequences is not None else record.consequences
        ),
    )
    updated = updated.model_copy(update={"raw": updated.render()})
    decision_state.record = updated
    app_state.mark_modified()
    return decision_state


def apply_template_to_editor(next_id: int) -> str:
    template = DecisionTemplate(next_id=next_id)
    return template.render()


######################################################################
# Reordering & Deletion
######################################################################


def move_decision(app_state: AppState, from_index: int, to_index: int) -> None:
    file_state = _require_active_file(app_state)
    decisions = file_state.decisions
    if not (0 <= from_index < len(decisions)):
        raise IndexError(f"Source index out of range: {from_index}")
    if not (0 <= to_index < len(decisions)):
        raise IndexError(f"Target index out of range: {to_index}")

    decision = decisions.pop(from_index)
    decisions.insert(to_index, decision)
    app_state.selected_decision_index = to_index
    app_state.mark_modified()


def delete_decision(app_state: AppState, decision_id: int) -> DecisionState:
    file_state = _require_active_file(app_state)
    decisions = file_state.decisions
    for index, decision_state in enumerate(decisions):
        if decision_state.record.id == decision_id:
            removed = decisions.pop(index)
            if app_state.selected_decision_index >= len(decisions):
                app_state.selected_decision_index = max(len(decisions) - 1, 0)
            app_state.mark_modified()
            return removed
    raise ValueError(f"Decision with id {decision_id} not found.")


######################################################################
# Linking
######################################################################


def link_decisions(
    app_state: AppState, *, source_id: int, target_id: int, relation: str
) -> DecisionLink:
    if relation not in {"supersedes", "deprecated_by"}:
        raise ValueError(f"Unsupported relation: {relation}")

    file_state = _require_active_file(app_state)
    source_state = file_state.find_decision(source_id)
    if source_state is None:
        raise ValueError(f"Source decision {source_id} not found.")

    target_state = file_state.find_decision(target_id)
    if target_state is None:
        raise ValueError(f"Target decision {target_id} not found.")

    rel: Literal["supersedes", "deprecated_by"] = relation  # type: ignore[arg-type]
    source_state.add_link(rel, target_id)

    # maintain reciprocity
    if rel == "supersedes":
        target_state.add_link("deprecated_by", source_id)
    else:
        target_state.add_link("supersedes", source_id)

    app_state.mark_modified()
    return DecisionLink(source_id, target_id, rel)


def unlink_decisions(
    app_state: AppState, *, source_id: int, target_id: int, relation: str
) -> None:
    if relation not in {"supersedes", "deprecated_by"}:
        raise ValueError(f"Unsupported relation: {relation}")

    file_state = _require_active_file(app_state)
    source_state = file_state.find_decision(source_id)
    if source_state is None:
        return
    target_state = file_state.find_decision(target_id)
    if target_state is None:
        return

    rel: Literal["supersedes", "deprecated_by"] = relation  # type: ignore[arg-type]
    source_state.remove_link(rel, target_id)

    reciprocal = "deprecated_by" if rel == "supersedes" else "supersedes"
    target_state.remove_link(reciprocal, source_id)
    app_state.mark_modified()


######################################################################
# Serialization & Parsing
######################################################################


def serialize_current_file(app_state: AppState) -> str:
    file_state = _require_active_file(app_state)
    return file_state.serialize_body()


def refresh_file_from_body(
    app_state: AppState,
    *,
    path: Path,
    body: str,
    marker_present: bool,
    inserted_marker: bool = False,
) -> FileState:
    file_state = FileState(path=path, marker_present=marker_present)
    file_state.inserted_marker = inserted_marker
    file_state.parse_body(body)
    file_state.decisions = sorted(
        file_state.decisions, key=lambda d: d.record.id, reverse=True
    )
    return file_state


def load_files(
    app_state: AppState,
    file_states: Iterable[FileState],
    *,
    focus_pane: Optional[PaneId] = None,
) -> None:
    app_state.set_files(file_states)
    if focus_pane:
        app_state.focus_pane(focus_pane)
    app_state.mark_saved()


def append_file_state(app_state: AppState, file_state: FileState) -> None:
    app_state.add_file(file_state)
    app_state.mark_saved()


######################################################################
# Navigation
######################################################################


def focus_next_decision(app_state: AppState) -> None:
    file_state = _require_active_file(app_state)
    if not file_state.decisions:
        return
    app_state.selected_decision_index = min(
        app_state.selected_decision_index + 1, len(file_state.decisions) - 1
    )


def focus_previous_decision(app_state: AppState) -> None:
    file_state = _require_active_file(app_state)
    if not file_state.decisions:
        return
    app_state.selected_decision_index = max(app_state.selected_decision_index - 1, 0)


def focus_pane(app_state: AppState, pane: PaneId) -> None:
    app_state.focus_pane(pane)
