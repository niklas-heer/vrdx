"""Tests for logging utilities in vrdx.app.logging."""

from __future__ import annotations

import logging
from pathlib import Path

import pytest

from vrdx.app import logging as vrdx_logging


@pytest.fixture(autouse=True)
def reset_logging():
    """Reset root logger before and after each test to avoid cross-test bleed."""
    logging.shutdown()
    root = logging.getLogger()
    for handler in root.handlers[:]:
        root.removeHandler(handler)
    root.setLevel(logging.NOTSET)
    yield
    logging.shutdown()
    for handler in root.handlers[:]:
        root.removeHandler(handler)
    root.setLevel(logging.NOTSET)


def test_configure_logging_defaults_to_stderr(capsys):
    vrdx_logging.configure_logging(level="INFO")
    logger = vrdx_logging.get_logger("vrdx.test")
    logger.info("stderr-output")

    captured = capsys.readouterr()
    assert "stderr-output" in captured.err
    assert captured.out == ""


def test_configure_logging_writes_to_file(tmp_path: Path):
    log_file = tmp_path / "vrdx.log"

    vrdx_logging.configure_logging(level="INFO", log_file=log_file)
    logger = vrdx_logging.get_logger("vrdx.file")
    logger.warning("file-output")

    assert log_file.exists()
    assert "file-output" in log_file.read_text(encoding="utf-8")


def test_configure_logging_replaces_handlers(tmp_path: Path):
    first_log = tmp_path / "first.log"
    second_log = tmp_path / "second.log"

    vrdx_logging.configure_logging(level="INFO", log_file=first_log)
    vrdx_logging.get_logger("vrdx.first").info("first")

    vrdx_logging.configure_logging(level="INFO", log_file=second_log)
    vrdx_logging.get_logger("vrdx.second").info("second")

    assert "second" not in first_log.read_text(encoding="utf-8")
    assert "second" in second_log.read_text(encoding="utf-8")


def test_configure_logging_handles_unknown_level_gracefully():
    vrdx_logging.configure_logging(level="NOT-A-LEVEL")
    root = logging.getLogger()
    assert root.getEffectiveLevel() == logging.INFO


def test_get_logger_returns_named_logger_instance():
    logger = vrdx_logging.get_logger("vrdx.sample")
    assert isinstance(logger, logging.Logger)
    assert logger.name == "vrdx.sample"
