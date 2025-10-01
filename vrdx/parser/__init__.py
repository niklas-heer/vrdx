"""Parsing utilities exposed by the vrdx.parser package."""

from __future__ import annotations

from . import markers
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

__all__ = [
    "markers",
    "MARKER_END",
    "MARKER_START",
    "MarkerBlock",
    "MarkerError",
    "DuplicateMarkerError",
    "MissingMarkerError",
    "MarkerOrderError",
    "build_marker_scaffold",
    "detect_marker_block",
    "detect_preferred_newline",
    "ensure_marker_block",
]
