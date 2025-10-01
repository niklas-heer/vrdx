"""Top-level package for the vrdx application.

This module exposes lightweight metadata that can be consumed by
CLI entry points and external tools without triggering heavy imports.
"""

from __future__ import annotations

from importlib import metadata as _metadata


def __getattr__(name: str):
    if name == "__version__":
        return _metadata.version("vrdx")
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")


__all__ = ["__version__"]
