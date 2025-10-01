"""Tests for logging utilities in vrdx.app.logging."""

from __future__ import annotations

import logging
from pathlib import Path

import pytest

from vrdx.app import logging as vrdx_logging


@pytest.fixture(autouse=True)
def reset_logging():
    """Ensure the root logger is clean before each test."""
    logging.shutdown()
    root_logger = logging.getLogger()
    for handler in root_logger.handlers[:]:
        root_logger.removeHandler(handler)
    root_logger.setLevel(logging.NOTSET)
    yield
    logging.shutdown()
    for handler in root_logger.handlers[:]:
        root_logger.removeHandler(handler)
    root_logger.setLevel(logging.NOTSET)


def test_configure_logging_defaults_to_stderr_only(capsys):
    vrdx_logging.configure_logging(level="INFO")
    logger = logging.getLogger("vrdx.test")
    logger.info("hello")
    captured = capsys.readouterr()
    assert "hello" in captured.err
    assert captured.out == ""


def test_configure_logging_writes_to_file(tmp_path: Path):
    log_file = tmp_path / "vrdx.log"
    vrdx_logging.configure_logging(level="INFO", log_file=log_file)
    logger = vrdx_logging.get_logger("vrdx.file")
    logger.warning("file output")
    assert log_file.exists()
    assert "file output" in log_file.read_text(encoding="utf-8")


def test_configure_logging_replaces_previous_handlers(tmp_path: Path, capsys):
    first_log = tmp_path / "first.log"
    second_log = tmp_path / "second.log"

    # First configuration writes to first log file.
    vrdx_logging.configure_logging(level="INFO", log_file=first_log)
    logging.getLogger("vrdx.first").info("first")
    assert "first" in first_log.read_text(encoding="utf-8")

    # Reconfigure to write to second log file only.
    vrdx_logging.configure_logging(level="INFO", log_file=second_log)
    logging.getLogger("vrdx.second").info("second")

    # Confirm subsequent emits are not appended to the first file.
    assert "second" not in first_log.read_text(encoding="utf-8")
    assert "second" in second_log.read_text(encoding="utf-8")

    # Logging should still stream to stderr.
    captured = capsys.readouterr()
    assert "second" in captured.err


def test_get_logger_returns_named_logger():
    logger = vrdx_logging.get_logger("vrdx.named")
    assert isinstance(logger, logging.Logger)
    assert logger.name == "vrdx.named"
