from __future__ import annotations

import argparse
import os
import sys
from pathlib import Path
from typing import Iterable, Optional

from vrdx import __version__
from vrdx.app import discovery, logging as app_logging

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
    parser.add_argument(
        "--log-file",
        help="Optional file path to tee logs in addition to stderr.",
    )
    return parser


def configure_logging(level: str, log_file: Optional[str]) -> None:
    file_path = Path(log_file).expanduser().resolve() if log_file else None
    app_logging.configure_logging(
        level=level,
        log_file=file_path,
        format_string=app_logging.DEFAULT_LOG_FORMAT,
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
    markdown_files = discovery.find_markdown_files(base_dir)
    print(f"vrdx initialized for directory: {base_dir}")
    if markdown_files:
        print("Discovered Markdown files:")
        for path in markdown_files:
            print(f"  • {path.relative_to(base_dir)}")
    else:
        print("No Markdown files found.")
    return 0


def main(argv: Optional[Iterable[str]] = None) -> int:
    parser = build_parser()
    args = parser.parse_args(list(argv) if argv is not None else None)

    if args.version:
        print(__version__)
        return 0

    configure_logging(args.log_level, args.log_file)
    try:
        directory = resolve_directory(args.directory)
    except (FileNotFoundError, NotADirectoryError) as exc:
        parser.error(str(exc))
        return 2  # pragma: no cover (argparse.error exits)

    return launch_interface(directory)


if __name__ == "__main__":
    sys.exit(main())
