from __future__ import annotations

import argparse
import logging
import os
import sys
from pathlib import Path
from typing import Iterable, Optional

from . import __version__

LOG_LEVEL_ENV = "VRDX_LOG_LEVEL"


def build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(
        prog="vrdx",
        description="TUI tool for managing decision records in Markdown files.",
    )
    parser.add_argument(
        "directory",
        nargs="?",
        default=".",
        help="Base directory to scan for Markdown decision records (default: current directory).",
    )
    parser.add_argument(
        "--version",
        action="store_true",
        help="Show the vrdx version and exit.",
    )
    parser.add_argument(
        "--log-level",
        default=os.environ.get(LOG_LEVEL_ENV, "INFO"),
        help=(
            "Logging level (DEBUG, INFO, WARNING, ERROR). "
            f"Defaults to ${LOG_LEVEL_ENV} or INFO if unset."
        ),
    )
    return parser


def configure_logging(level: str) -> None:
    logging.basicConfig(
        level=getattr(logging, level.upper(), logging.INFO),
        format="%(levelname)s %(name)s - %(message)s",
    )


def resolve_directory(raw: str) -> Path:
    directory = Path(raw).expanduser().resolve()
    if not directory.exists():
        raise FileNotFoundError(f"Directory not found: {directory}")
    if not directory.is_dir():
        raise NotADirectoryError(f"Not a directory: {directory}")
    return directory


def launch_interface(base_dir: Path) -> int:
    # Placeholder until the Textual app is implemented in later milestones.
    logging.getLogger(__name__).info("Launching vrdx in %s", base_dir)
    print(f"vrdx initialized for directory: {base_dir}")
    return 0


def main(argv: Optional[Iterable[str]] = None) -> int:
    parser = build_parser()
    args = parser.parse_args(list(argv) if argv is not None else None)

    if args.version:
        print(__version__)
        return 0

    configure_logging(args.log_level)
    try:
        directory = resolve_directory(args.directory)
    except (FileNotFoundError, NotADirectoryError) as exc:
        parser.error(str(exc))
        return 2  # pragma: no cover (argparse.error exits)
    return launch_interface(directory)


if __name__ == "__main__":
    sys.exit(main())
