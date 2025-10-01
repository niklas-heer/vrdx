from __future__ import annotations

from pathlib import Path
from typing import Optional, Tuple

from .logging import get_logger
from vrdx.parser import markers

ENCODING = "utf-8"
LOGGER = get_logger(__name__)


def read_markdown(path: Path, *, encoding: str = ENCODING) -> str:
    """Read and return the contents of ``path`` as UTF-8 text by default."""
    LOGGER.debug("Reading markdown file: %s", path)
    return path.read_text(encoding=encoding)


def write_markdown(path: Path, content: str, *, encoding: str = ENCODING) -> None:
    """Write ``content`` to ``path`` using UTF-8 encoding by default."""
    LOGGER.debug("Writing markdown file: %s", path)
    path.write_text(content, encoding=encoding)


def ensure_marker_block_in_file(
    path: Path,
    *,
    newline: Optional[str] = None,
    encoding: str = ENCODING,
) -> Tuple[markers.MarkerBlock, bool]:
    """Ensure ``path`` contains a canonical vrdx marker block.

    Parameters
    ----------
    path:
        The markdown file to examine or update.
    newline:
        Optional preferred newline sequence to use when inserting a new block.
        If ``None``, the newline sequence is inferred from the current file
        contents.
    encoding:
        Encoding used to read and write the file (defaults to UTF-8).

    Returns
    -------
    tuple
        A tuple ``(marker_block, inserted)`` where ``marker_block`` describes
        the span of the markers after the operation and ``inserted`` indicates
        whether a new block was added.

    Raises
    ------
    markers.MarkerError
        Propagated when marker detection fails due to malformed input.
    """
    LOGGER.debug("Ensuring marker block in file: %s", path)
    original = read_markdown(path, encoding=encoding)

    if newline is None:
        updated_text, marker_block, inserted = markers.ensure_marker_block(original)
    else:
        updated_text, marker_block, inserted = markers.ensure_marker_block(
            original,
            newline=newline,
        )

    if inserted:
        LOGGER.info("Inserted marker block into %s", path)
        write_markdown(path, updated_text, encoding=encoding)

    return marker_block, inserted
