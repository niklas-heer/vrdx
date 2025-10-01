"""Tests for Markdown discovery helpers."""

from __future__ import annotations

from pathlib import Path

import pytest

from vrdx.app.discovery import DiscoveryConfig, find_markdown_files, iter_markdown_files


def create_files(base: Path, structure: dict[str, str | dict]) -> None:
    """Recursively create files and directories for tests."""
    for name, value in structure.items():
        path = base / name
        if isinstance(value, dict):
            path.mkdir(parents=True, exist_ok=True)
            create_files(path, value)
        else:
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(value, encoding="utf-8")


def test_find_markdown_files_discovers_md_and_markdown(tmp_path: Path):
    create_files(
        tmp_path,
        {
            "README.md": "# Title\n",
            "notes.markdown": "# Notes\n",
            "nested": {
                "inner.md": "content",
                "ignored.txt": "not md",
            },
        },
    )

    results = find_markdown_files(tmp_path)
    assert results == sorted(
        [
            (tmp_path / "README.md").resolve(),
            (tmp_path / "nested" / "inner.md").resolve(),
            (tmp_path / "notes.markdown").resolve(),
        ]
    )


def test_iter_markdown_files_skips_default_directories(tmp_path: Path):
    create_files(
        tmp_path,
        {
            ".git": {
                "ignored.md": "should not appear",
            },
            "docs": {
                "adr.md": "Decision",
            },
            ".venv": {
                "excluded.markdown": "virtual env",
            },
        },
    )

    results = list(iter_markdown_files(tmp_path))
    assert results == [(tmp_path / "docs" / "adr.md").resolve()]


def test_iter_markdown_files_respects_custom_extensions(tmp_path: Path):
    create_files(
        tmp_path,
        {
            "docs": {
                "decision.adoc": "not markdown",
                "decision.mdown": "custom extension",
            }
        },
    )
    config = DiscoveryConfig(extensions=(".mdown",))
    results = list(iter_markdown_files(tmp_path, config=config))
    assert results == [(tmp_path / "docs" / "decision.mdown").resolve()]


def test_iter_markdown_files_errors_when_directory_missing(tmp_path: Path):
    missing = tmp_path / "does-not-exist"
    with pytest.raises(FileNotFoundError):
        list(iter_markdown_files(missing))


def test_iter_markdown_files_errors_when_path_is_file(tmp_path: Path):
    file_path = tmp_path / "README.md"
    file_path.write_text("hi", encoding="utf-8")
    with pytest.raises(NotADirectoryError):
        list(iter_markdown_files(file_path))


def test_discovery_config_normalizes_extensions():
    config = DiscoveryConfig(extensions=(".MD", ".Markdown"))
    assert config.normalized_extensions() == (".md", ".markdown")
