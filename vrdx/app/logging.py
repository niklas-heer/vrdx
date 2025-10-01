"""Logging utilities for the vrdx application."""

from __future__ import annotations

import logging
import sys
from pathlib import Path
from typing import Optional

DEFAULT_LOG_FORMAT = "%(levelname)s %(name)s - %(message)s"


def configure_logging(
    *,
    level: str = "INFO",
    log_file: Optional[Path] = None,
    format_string: str = DEFAULT_LOG_FORMAT,
) -> None:
    """Configure root logging for the vrdx application.

    Parameters
    ----------
    level:
        Logging level name (e.g., "INFO", "DEBUG"). Unknown values default to INFO.
    log_file:
        Optional path to a log file. When provided, logs are duplicated to this file.
    format_string:
        Formatter template applied to all handlers.
    """
    logging.shutdown()
    root_logger = logging.getLogger()

    # Remove existing handlers to avoid duplicate logs when reconfiguring.
    for handler in root_logger.handlers[:]:
        root_logger.removeHandler(handler)

    handlers: list[logging.Handler] = [logging.StreamHandler(sys.stderr)]

    if log_file:
        log_file.parent.mkdir(parents=True, exist_ok=True)
        handlers.append(logging.FileHandler(log_file, encoding="utf-8"))

    effective_level = getattr(logging, level.upper(), logging.INFO)
    logging.basicConfig(
        level=effective_level,
        format=format_string,
        handlers=handlers,
    )


def get_logger(name: str) -> logging.Logger:
    """Return a module-level logger configured for the vrdx application."""
    return logging.getLogger(name)
