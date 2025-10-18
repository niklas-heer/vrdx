#!/usr/bin/env python3
"""Test script to verify TextArea height rendering in Textual.

This script creates a simple app with TextArea widgets to test if height
settings are being applied correctly.
"""

from textual.app import App, ComposeResult
from textual.widgets import TextArea, Label, Static
from textual.containers import Vertical


class TextAreaHeightTest(App):
    """Test application for TextArea height rendering."""

    CSS = """
    Screen {
        background: $surface;
    }

    .test-section {
        width: 100%;
        height: auto;
        padding: 1;
        margin-bottom: 2;
        border: solid green;
    }

    .section-title {
        text-style: bold;
        color: yellow;
        margin-bottom: 1;
    }

    #test-area-1 {
        width: 100%;
        height: 5;
        margin-bottom: 1;
        border: solid blue;
    }

    #test-area-2 {
        width: 100%;
        height: 10;
        margin-bottom: 1;
        border: solid red;
    }

    #test-area-3 {
        width: 100%;
        height: 15;
        margin-bottom: 1;
        border: solid magenta;
    }

    #test-area-4 {
        width: 100%;
        height: 12;
        margin-bottom: 1;
        border: solid cyan;
    }
    """

    def compose(self) -> ComposeResult:
        """Compose the test UI."""
        yield Static("TextArea Height Test", classes="section-title")

        with Vertical(classes="test-section"):
            yield Label(
                "Height: 5 (should show ~3-4 lines of text)", classes="section-title"
            )
            yield TextArea(
                "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6",
                id="test-area-1",
                read_only=False,
            )

        with Vertical(classes="test-section"):
            yield Label(
                "Height: 10 (should show ~8-9 lines of text)", classes="section-title"
            )
            yield TextArea(
                "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10",
                id="test-area-2",
                read_only=False,
            )

        with Vertical(classes="test-section"):
            yield Label(
                "Height: 15 (should show ~13-14 lines of text)", classes="section-title"
            )
            yield TextArea(
                "Line 1\nLine 2\nLine 3\nLine 4\nLine 5\nLine 6\nLine 7\nLine 8\nLine 9\nLine 10\nLine 11\nLine 12\nLine 13\nLine 14\nLine 15",
                id="test-area-3",
                read_only=False,
            )

        with Vertical(classes="test-section"):
            yield Label(
                "Height: 12 (same as decision/context/consequences)",
                classes="section-title",
            )
            yield TextArea(
                "This is a test text area with height: 12\nYou should be able to see multiple lines here.\nType more text to test multi-line editing.\n\nThis simulates the decision, context, and consequences fields.",
                id="test-area-4",
                read_only=False,
            )

    def on_mount(self) -> None:
        """Set heights programmatically as well."""
        # Try to force heights programmatically
        try:
            self.query_one("#test-area-1", TextArea).styles.height = 5
            self.query_one("#test-area-2", TextArea).styles.height = 10
            self.query_one("#test-area-3", TextArea).styles.height = 15
            self.query_one("#test-area-4", TextArea).styles.height = 12
        except Exception as e:
            self.notify(f"Error setting heights: {e}", severity="error")


if __name__ == "__main__":
    app = TextAreaHeightTest()
    app.run()
