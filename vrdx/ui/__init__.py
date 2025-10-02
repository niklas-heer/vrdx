"""User interface components built with Textual for the vrdx application."""

from pathlib import Path

from .app import VrdxApp

STYLES_PATH = Path(__file__).with_name("styles.tcss")

__all__ = ["VrdxApp", "STYLES_PATH"]
