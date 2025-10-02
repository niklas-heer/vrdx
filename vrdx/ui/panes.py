"""Shared pane stubs used by the Textual TUI.

These classes currently provide lightweight placeholders so the layout logic can
reference importable pane types. They will be fleshed out with concrete widgets
and behaviors in later milestones.
"""

from __future__ import annotations

from dataclasses import dataclass


@dataclass
class PaneSpec:
    """Describes the metadata associated with a pane."""

    id: str
    title: str
    description: str | None = None


class BasePane:
    """Base class for future pane implementations."""

    pane_id: str = "base"
    title: str = "Base Pane"

    def __init__(self) -> None:
        self.spec = PaneSpec(id=self.pane_id, title=self.title)

    def refresh(self) -> None:  # pragma: no cover - stub method
        """Refresh the pane contents."""
        return None

    def focus(self) -> None:  # pragma: no cover - stub method
        """Handle focus events for the pane."""
        return None


class DecisionPane(BasePane):
    pane_id = "decisions"
    title = "Decisions"


class EditorPaneStub(BasePane):
    pane_id = "editor"
    title = "Editor"


class PreviewPaneStub(BasePane):
    pane_id = "preview"
    title = "Preview"


class FilePane(BasePane):
    pane_id = "files"
    title = "Files"
