"""Top-level package for vrdx."""

from __future__ import annotations

from importlib import import_module, metadata as _metadata

_SUBMODULES = {
    "app": "vrdx.app",
    "cli": "vrdx.cli",
    "parser": "vrdx.parser",
    "ui": "vrdx.ui",
}

__all__ = ["__version__", *sorted(_SUBMODULES.keys())]


def __getattr__(name: str):
    if name == "__version__":
        return _metadata.version("vrdx")
    if name in _SUBMODULES:
        module = import_module(_SUBMODULES[name])
        globals()[name] = module
        return module
    raise AttributeError(f"module {__name__!r} has no attribute {name!r}")


def __dir__() -> list[str]:
    return sorted(__all__)
