"""Parsing utilities exposed by the vrdx.parser package."""

from __future__ import annotations

from . import decisions, markers, template
from .decisions import (
    DecisionParseError,
    DecisionRecord,
    find_next_decision_id,
    parse_decisions,
    render_decisions,
    update_decision_body,
)
from .markers import (
    MARKER_END,
    MARKER_START,
    MarkerBlock,
    MarkerError,
    DuplicateMarkerError,
    MissingMarkerError,
    MarkerOrderError,
    build_marker_scaffold,
    detect_marker_block,
    detect_preferred_newline,
    ensure_marker_block,
)
from .template import (
    DEFAULT_STATUS,
    DecisionTemplate,
    list_status_options,
    normalise_status,
    render_template,
)

__all__ = (
    "DEFAULT_STATUS",
    "DecisionParseError",
    "DecisionRecord",
    "DecisionTemplate",
    "DuplicateMarkerError",
    "MARKER_END",
    "MARKER_START",
    "MarkerBlock",
    "MarkerError",
    "MarkerOrderError",
    "MissingMarkerError",
    "build_marker_scaffold",
    "decisions",
    "detect_marker_block",
    "detect_preferred_newline",
    "ensure_marker_block",
    "find_next_decision_id",
    "list_status_options",
    "markers",
    "normalise_status",
    "parse_decisions",
    "render_decisions",
    "render_template",
    "template",
    "update_decision_body",
)
