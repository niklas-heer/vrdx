"""Utilities for discovering Markdown files within a project tree.

This module provides simple helpers for enumerating Markdown files that may
contain vrdx decision records. The discovery rules are intentionally
straightforward for now:

* The caller supplies the base directory.
* Files ending in ``.md`` or ``.markdown`` (case-insensitive) are considered.
* Certain directories that rarely contain decision logs (e.g. ``.git`` or
  virtual environment folders) are skipped by default, but callers can override
  this behaviour.

In later milestones we can expand this module to honour repository-specific
ignore files or user configuration.
"""

from __future__ import annotations

import os
from dataclasses import dataclass, field
from pathlib import Path
from typing import Iterable, Iterator, Sequence


_DEFAULT_IGNORED_DIRS: tuple[str, ...] = (
    ".git",
    ".hg",
    ".svn",
    ".venv",
    "__pycache__",
    "node_modules",
)


@dataclass(frozen=True)
class DiscoveryConfig:
    """Configuration for Markdown discovery.

    Parameters
    ----------
    extensions:
        Iterable of file suffixes to include when scanning. All comparisons are
        case-insensitive and should include the leading dot.
    ignored_directories:
        Directory names that should be skipped during traversal. Matching is
        case-sensitive and applies to directory stems (not absolute paths).
    """

    extensions: Sequence[str] = field(
        default_factory=lambda: (".md", ".markdown"),
    )
    ignored_directories: Sequence[str] = field(
        default_factory=lambda: _DEFAULT_IGNORED_DIRS,
    )

    def normalized_extensions(self) -> tuple[str, ...]:
        """Return a tuple of lowercase extensions."""
        return tuple(ext.lower() for ext in self.extensions)


def iter_markdown_files(
    base_directory: Path,
    *,
    config: DiscoveryConfig | None = None,
) -> Iterator[Path]:
    """Yield Markdown files discovered beneath ``base_directory``.

    Parameters
    ----------
    base_directory:
        Root directory to scan. The directory must exist.
    config:
        Optional :class:`DiscoveryConfig` to customise discovery.

    Yields
    ------
    :class:`pathlib.Path`
        Paths to Markdown files relative to ``base_directory`` (resolved).

    Raises
    ------
    FileNotFoundError
        If ``base_directory`` does not exist.
    NotADirectoryError
        If ``base_directory`` is not a directory.
    """
    if not base_directory.exists():
        raise FileNotFoundError(f"Base directory does not exist: {base_directory}")
    if not base_directory.is_dir():
        raise NotADirectoryError(f"Base directory is not a directory: {base_directory}")

    cfg = config or DiscoveryConfig()
    normalized_exts = cfg.normalized_extensions()
    ignored = set(cfg.ignored_directories)

    for root, dirs, files in os.walk(base_directory):
        dirs[:] = [d for d in dirs if d not in ignored]
        root_path = Path(root)
        for filename in files:
            if Path(filename).suffix.lower() in normalized_exts:
                yield (root_path / filename).resolve()


def find_markdown_files(
    base_directory: Path,
    *,
    config: DiscoveryConfig | None = None,
) -> list[Path]:
    """Return a sorted list of Markdown files beneath ``base_directory``."""
    files = list(iter_markdown_files(base_directory, config=config))
    return sorted(files)


__all__ = [
    "DiscoveryConfig",
    "find_markdown_files",
    "iter_markdown_files",
]
