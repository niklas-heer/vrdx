"""Tests for vrdx.parser.markers utilities."""

from __future__ import annotations

import pytest

from vrdx.parser import markers


def test_detect_marker_block_none_when_no_markers():
    text = "# Title\n\nNo markers here.\n"
    assert markers.detect_marker_block(text) is None


def test_detect_marker_block_happy_path():
    text = "\n".join(
        [
            "# Heading",
            "",
            markers.MARKER_START,
            "### 1 Decision",
            markers.MARKER_END,
            "",
        ]
    )
    block = markers.detect_marker_block(text)
    assert block is not None
    body = block.body(text)
    assert "### 1 Decision" in body
    replaced = block.replace_body(text, "New Body\n")
    assert "New Body" in replaced
    assert "### 1 Decision" not in replaced


@pytest.mark.parametrize(
    "content",
    [
        markers.MARKER_START,
        markers.MARKER_END,
        f"{markers.MARKER_START}\n{markers.MARKER_START}",
        f"{markers.MARKER_END}\n{markers.MARKER_END}",
    ],
)
def test_detect_marker_block_errors_on_invalid_marker_counts(content: str):
    with pytest.raises(markers.MarkerError):
        markers.detect_marker_block(content)


def test_detect_marker_block_errors_when_end_precedes_start():
    text = "\n".join(
        [
            markers.MARKER_END,
            markers.MARKER_START,
        ]
    )
    with pytest.raises(markers.MarkerOrderError):
        markers.detect_marker_block(text)


@pytest.mark.parametrize(
    ("text", "expected"),
    [
        ("line1\nline2\n", "\n"),
        ("line1\r\nline2\r\n", "\r\n"),
        ("line1\rline2\r", "\r"),
        ("", "\n"),
    ],
)
def test_detect_preferred_newline(text: str, expected: str):
    assert markers.detect_preferred_newline(text) == expected


def test_build_marker_scaffold_uses_custom_newline():
    scaffold = markers.build_marker_scaffold("\r\n")
    assert scaffold.count("\r\n") == 3
    assert scaffold.startswith(markers.MARKER_START)


def test_ensure_marker_block_returns_existing_block():
    text = "\n".join(
        [
            markers.MARKER_START,
            "Existing body",
            markers.MARKER_END,
            "",
        ]
    )
    updated, block, inserted = markers.ensure_marker_block(text)
    assert updated == text
    assert block.body(updated).strip() == "Existing body"
    assert inserted is False


def test_ensure_marker_block_appends_scaffold_when_missing():
    text = "# Intro\n"
    updated, block, inserted = markers.ensure_marker_block(text)
    assert inserted is True
    assert updated.endswith(markers.MARKER_END + "\n")
    assert block.body(updated).strip() == ""


def test_ensure_marker_block_preserves_custom_newline_choice():
    text = "# Intro\r\n"
    updated, _, inserted = markers.ensure_marker_block(text, newline="\r\n")
    assert inserted is True
    assert "\r\n" in updated
