"""Application-level services exposed by the vrdx package."""

from __future__ import annotations

from .discovery import DiscoveryConfig, find_markdown_files, iter_markdown_files
from .logging import configure_logging, get_logger
from .persistence import ensure_marker_block_in_file, read_markdown, write_markdown

__all__ = [
    "configure_logging",
    "get_logger",
    "DiscoveryConfig",
    "find_markdown_files",
    "iter_markdown_files",
    "ensure_marker_block_in_file",
    "read_markdown",
    "write_markdown",
]
