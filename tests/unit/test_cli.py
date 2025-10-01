"""Tests for the vrdx.cli module."""

from __future__ import annotations

from pathlib import Path
from typing import Iterable

import pytest

from vrdx import cli


def test_main_version_flag(capsys, monkeypatch):
    monkeypatch.setattr(cli, "__version__", "1.2.3")
    exit_code = cli.main(["--version"])
    captured = capsys.readouterr()
    assert exit_code == 0
    assert captured.out.strip() == "1.2.3"


def test_main_passes_resolved_directory(tmp_path: Path, monkeypatch):
    recorded: dict[str, Path] = {}

    def fake_launch(path: Path) -> int:
        recorded["path"] = path
        return 0

    monkeypatch.setattr(cli, "launch_interface", fake_launch)
    exit_code = cli.main([str(tmp_path)])
    assert exit_code == 0
    assert recorded["path"] == tmp_path.resolve()


def test_main_errors_on_missing_directory(tmp_path: Path):
    missing = tmp_path / "does-not-exist"
    with pytest.raises(SystemExit) as excinfo:
        cli.main([str(missing)])
    assert excinfo.value.code == 2


def test_launch_interface_lists_markdown_files(tmp_path: Path, capsys, monkeypatch):
    files = [
        tmp_path / "README.md",
        tmp_path / "docs" / "adr.md",
    ]
    for file in files:
        file.parent.mkdir(parents=True, exist_ok=True)
        file.write_text("# stub\n", encoding="utf-8")

    monkeypatch.setattr(
        cli.discovery,
        "find_markdown_files",
        lambda base: [path.resolve() for path in files],
    )

    exit_code = cli.launch_interface(tmp_path)
    captured = capsys.readouterr()

    assert exit_code == 0
    assert "Discovered Markdown files:" in captured.out
    assert "README.md" in captured.out
    assert "docs/adr.md" in captured.out


def test_launch_interface_handles_empty_results(tmp_path: Path, capsys, monkeypatch):
    monkeypatch.setattr(cli.discovery, "find_markdown_files", lambda base: [])
    exit_code = cli.launch_interface(tmp_path)
    output = capsys.readouterr().out
    assert exit_code == 0
    assert "No Markdown files found." in output


def test_configure_logging_delegates_to_app_logging(tmp_path: Path, monkeypatch):
    called: dict[str, Iterable[Path | str]] = {}

    def fake_configure(*, level: str, log_file: Path | None, format_string: str):
        called["level"] = level
        called["log_file"] = log_file
        called["format"] = format_string

    monkeypatch.setattr(cli.app_logging, "configure_logging", fake_configure)

    log_path = tmp_path / "vrdx.log"
    cli.configure_logging("DEBUG", str(log_path))

    assert called["level"] == "DEBUG"
    assert called["log_file"] == log_path.resolve()


def test_configure_logging_accepts_none_log_file(monkeypatch):
    called: dict[str, Path | None] = {}

    def fake_configure(*, level: str, log_file: Path | None, format_string: str):
        called["log_file"] = log_file

    monkeypatch.setattr(cli.app_logging, "configure_logging", fake_configure)
    cli.configure_logging("INFO", None)
    assert called["log_file"] is None


def test_build_parser_uses_env_default(monkeypatch):
    monkeypatch.setenv(cli.LOG_LEVEL_ENV, "DEBUG")
    parser = cli.build_parser()
    args = parser.parse_args([])
    assert args.log_level == "DEBUG"


def test_resolve_directory_returns_canonical_path(tmp_path: Path):
    nested = tmp_path / "dir"
    nested.mkdir()
    mixed_path = nested / ".." / "dir"
    resolved = cli.resolve_directory(str(mixed_path))
    assert resolved == nested.resolve()
