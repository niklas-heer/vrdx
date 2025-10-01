"""Tests for the vrdx.cli module."""

import logging
from pathlib import Path

import pytest

from vrdx import cli


def _reset_logging() -> None:
    """Ensure logging is re-configurable between tests."""
    logging.shutdown()
    root = logging.getLogger()
    for handler in root.handlers[:]:
        root.removeHandler(handler)
    root.setLevel(logging.NOTSET)


def test_main_version_flag(capsys, monkeypatch):
    monkeypatch.setattr(cli, "__version__", "1.2.3")
    exit_code = cli.main(["--version"])
    captured = capsys.readouterr()
    assert exit_code == 0
    assert captured.out.strip() == "1.2.3"


def test_main_passes_resolved_directory(tmp_path, monkeypatch):
    captured: dict[str, Path] = {}

    def fake_launch(directory: Path) -> int:
        captured["directory"] = directory
        return 0

    monkeypatch.setattr(cli, "launch_interface", fake_launch)
    exit_code = cli.main([str(tmp_path)])
    assert exit_code == 0
    assert captured["directory"] == tmp_path.resolve()


def test_main_errors_on_missing_directory(tmp_path):
    missing = tmp_path / "does-not-exist"
    with pytest.raises(SystemExit) as excinfo:
        cli.main([str(missing)])
    assert excinfo.value.code == 2


def test_launch_interface_prints_directory(tmp_path, capsys):
    exit_code = cli.launch_interface(tmp_path)
    captured = capsys.readouterr()
    assert exit_code == 0
    assert str(tmp_path) in captured.out


def test_build_parser_uses_log_level_env(monkeypatch):
    monkeypatch.setenv(cli.LOG_LEVEL_ENV, "DEBUG")
    parser = cli.build_parser()
    args = parser.parse_args([])
    assert args.log_level == "DEBUG"


def test_configure_logging_accepts_valid_level():
    _reset_logging()
    cli.configure_logging("DEBUG")
    root = logging.getLogger()
    assert root.getEffectiveLevel() == logging.DEBUG


def test_configure_logging_falls_back_to_info_on_unknown_level():
    _reset_logging()
    cli.configure_logging("NOT-A-LEVEL")
    root = logging.getLogger()
    assert root.getEffectiveLevel() == logging.INFO


def test_resolve_directory_returns_canonical_path(tmp_path):
    path = tmp_path / ".." / tmp_path.name
    resolved = cli.resolve_directory(str(path))
    assert resolved == tmp_path.resolve()
