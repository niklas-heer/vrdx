"""Support for detecting and creating vrdx marker blocks within Markdown files."""

from __future__ import annotations

import re
from dataclasses import dataclass
from typing import Optional, Tuple

MARKER_START = "<!-- vrdx start -->"
MARKER_END = "<!-- vrdx end -->"

_START_RE = re.compile(re.escape(MARKER_START))
_END_RE = re.compile(re.escape(MARKER_END))


class MarkerError(ValueError):
    """Base exception for marker related issues."""


class MissingMarkerError(MarkerError):
    """Raised when only one of the start/end markers is present."""


class DuplicateMarkerError(MarkerError):
    """Raised when multiple marker pairs are detected."""


class MarkerOrderError(MarkerError):
    """Raised when the end marker appears before the start marker."""


@dataclass(frozen=True)
class MarkerBlock:
    """Span details for the decision block enclosed by markers.

    Attributes
    ----------
    start_index:
        Offset of the start marker (inclusive).
    content_start:
        Offset immediately after the start marker.
    content_end:
        Offset at the beginning of the end marker.
    end_index:
        Offset immediately after the end marker (exclusive).
    """

    start_index: int
    content_start: int
    content_end: int
    end_index: int

    def body(self, text: str) -> str:
        """Return the substring located between the markers."""
        return text[self.content_start : self.content_end]

    def replace_body(self, text: str, new_body: str) -> str:
        """Return ``text`` with the block body replaced by ``new_body``."""
        return "".join((text[: self.content_start], new_body, text[self.content_end :]))


def detect_marker_block(text: str) -> Optional[MarkerBlock]:
    """Identify the canonical marker block within ``text``.

    Parameters
    ----------
    text:
        Markdown document contents.

    Returns
    -------
    MarkerBlock or None
        Span information describing the first marker block, or ``None`` when no
        markers are present.

    Raises
    ------
    MissingMarkerError
        Occurs when only a single marker is found.
    DuplicateMarkerError
        Raised if multiple start or end markers are detected.
    MarkerOrderError
        Raised when the end marker precedes the start marker.
    """
    start_matches = list(_START_RE.finditer(text))
    end_matches = list(_END_RE.finditer(text))

    if not start_matches and not end_matches:
        return None
    if not start_matches or not end_matches:
        raise MissingMarkerError(
            "Detected only one vrdx marker; both start and end markers must be present."
        )
    if len(start_matches) > 1 or len(end_matches) > 1:
        raise DuplicateMarkerError("Multiple vrdx marker blocks detected.")
    start_match = start_matches[0]
    end_match = end_matches[0]

    if start_match.start() > end_match.start():
        raise MarkerOrderError("End marker appears before start marker.")

    return MarkerBlock(
        start_index=start_match.start(),
        content_start=start_match.end(),
        content_end=end_match.start(),
        end_index=end_match.end(),
    )


def detect_preferred_newline(text: str) -> str:
    """Infer the dominant newline sequence used in ``text``."""
    for idx, char in enumerate(text):
        if char == "\n":
            if idx > 0 and text[idx - 1] == "\r":
                return "\r\n"
            return "\n"
        if char == "\r":
            if idx + 1 < len(text) and text[idx + 1] == "\n":
                return "\r\n"
            return "\r"
    return "\n"


def build_marker_scaffold(newline: str = "\n") -> str:
    """Return the canonical empty marker block using ``newline`` separators."""
    return f"{MARKER_START}{newline}{newline}{MARKER_END}{newline}"


def ensure_marker_block(
    text: str,
    *,
    newline: Optional[str] = None,
) -> Tuple[str, MarkerBlock, bool]:
    """Guarantee that ``text`` contains exactly one marker block.

    Parameters
    ----------
    text:
        Markdown document content.
    newline:
        Preferred newline sequence to use if a block must be inserted. When
        ``None``, the sequence is inferred from ``text`` (defaulting to ``\\n``).

    Returns
    -------
    tuple
        ``(updated_text, marker_block, inserted)`` where ``inserted`` indicates
        whether a new block was appended.

    Raises
    ------
    DuplicateMarkerError
        If multiple marker blocks already exist.
    MissingMarkerError
        If only one of the markers is present.
    MarkerOrderError
        If the end marker precedes the start marker.
    """
    block = detect_marker_block(text)
    if block is not None:
        return text, block, False

    newline_to_use = newline or detect_preferred_newline(text)

    updated_text = text
    if updated_text:
        if updated_text.endswith("\r\n"):
            trailing = "\r\n"
        elif updated_text.endswith("\n"):
            trailing = "\n"
        elif updated_text.endswith("\r"):
            trailing = "\r"
        else:
            trailing = ""
        if trailing:
            if trailing != newline_to_use:
                updated_text = updated_text[: -len(trailing)] + newline_to_use
        else:
            updated_text = f"{updated_text}{newline_to_use}"

    updated = f"{updated_text}{build_marker_scaffold(newline_to_use)}"

    block = detect_marker_block(updated)
    if block is None:
        raise MarkerError("Failed to create the vrdx marker block.")
    return updated, block, True
