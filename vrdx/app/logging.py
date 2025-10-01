from __future__ import annotations

import logging
import sys
from pathlib import Path
from typing import Optional

DEFAULT_LOG_FORMAT = "%(levelname)s %(name)s - %(message)s"
LOG_FILE_ENV = "VRDX_LOG_FILE"


def configure_logging(
    level: str = "INFO",
    log_file: Optional[Path] = None,
    *,
    format_string: str = DEFAULT_LOG_FORMAT,
) -> None:
    """Configure root logging for vrdx.

    Parameters
    ----------
    level:
        Logging level name (e.g., ``"INFO"``, ``"DEBUG"``).
    log_file:
        Optional path to a log file; if absent, logs stay on stderr only.
    format_string:
        Format string for log messages.

    Notes
    -----
    - The root logger is reset to ensure deterministic output when invoked
      repeatedly (useful for tests).
    - When ``log_file`` is provided, logs are duplicated to that file while
      still showing up on stderr.
    """
    logging.shutdown()
    root_logger = logging.getLogger()
    for handler in root_logger.handlers[:]:
        root_logger.removeHandler(handler)

    handlers: list[logging.Handler] = [logging.StreamHandler(sys.stderr)]

    if log_file:
        log_file.parent.mkdir(parents=True, exist_ok=True)
        file_handler = logging.FileHandler(log_file, encoding="utf-8")
        handlers.append(file_handler)

    logging.basicConfig(
        level=getattr(logging, level.upper(), logging.INFO),
        format=format_string,
        handlers=handlers,
    )


def get_logger(name: str) -> logging.Logger:
    """Return a module-level logger with the given name."""
    return logging.getLogger(name)
