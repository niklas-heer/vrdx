"""Tests for persistence helpers ensuring marker blocks in markdown files."""

from __future__ import annotations

from pathlib import Path

import pytest

from vrdx.app.persistence import (
    ENCODING,
    ensure_marker_block_in_file,
    read_markdown,
    write_markdown,
)
from vrdx.parser import MARKER_END, MARKER_START, MarkerBlock


def test_read_and_write_markdown_round_trip(tmp_path: Path):
    file_path = tmp_path / "doc.md"
    content = "# Header\nSome content.\n"
    write_markdown(file_path, content)
    assert file_path.read_text(encoding=ENCODING) == content

    loaded = read_markdown(file_path)
    assert loaded == content


def test_ensure_marker_block_in_file_detects_existing_block(tmp_path: Path):
    file_path = tmp_path / "decisions.md"
    file_path.write_text(
        "\n".join(
            [
                "# Decisions",
                "",
                MARKER_START,
                "",
                MARKER_END,
                "",
            ]
        ),
        encoding=ENCODING,
    )

    block, inserted = ensure_marker_block_in_file(file_path)
    assert isinstance(block, MarkerBlock)
    assert inserted is False
    # File content should remain unchanged.
    assert file_path.read_text(encoding=ENCODING).startswith("# Decisions")


def test_ensure_marker_block_in_file_inserts_when_missing(tmp_path: Path):
    file_path = tmp_path / "notes.md"
    file_path.write_text("# Notes\n", encoding=ENCODING)

    block, inserted = ensure_marker_block_in_file(file_path)
    assert inserted is True
    content = file_path.read_text(encoding=ENCODING)
    assert MARKER_START in content
    assert MARKER_END in content
    # The block body should currently be empty.
    assert block.body(content).strip() == ""


def test_ensure_marker_block_in_file_raises_for_malformed_markers(tmp_path: Path):
    file_path = tmp_path / "bad.md"
    file_path.write_text(MARKER_START, encoding=ENCODING)

    with pytest.raises(Exception):
        ensure_marker_block_in_file(file_path)
