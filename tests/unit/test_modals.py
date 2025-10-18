"""Tests for modal screens used in the vrdx TUI."""

from __future__ import annotations

import pytest

from vrdx.parser import DEFAULT_STATUS, list_status_options
from vrdx.ui.modals import StatusSelectionModal, StatusOptionDisplay


class TestStatusSelectionModal:
    """Tests for the StatusSelectionModal screen."""

    def test_modal_initializes_with_status_options(self) -> None:
        """Test that the modal initializes with all available status options."""
        modal = StatusSelectionModal()
        assert modal.status_options == list(list_status_options())
        assert len(modal.status_options) == 5

    def test_modal_starts_with_first_option_selected(self) -> None:
        """Test that the first status option is selected by default."""
        modal = StatusSelectionModal()
        assert modal.selected_index == 0
        assert modal.status_options[modal.selected_index] == DEFAULT_STATUS

    def test_action_next_status_advances_selection(self) -> None:
        """Test that next_status action advances the selected index."""
        modal = StatusSelectionModal()
        initial_index = modal.selected_index
        modal.action_next_option()
        assert modal.selected_index == initial_index + 1

    def test_action_next_status_wraps_around(self) -> None:
        """Test that next_status wraps from last to first option."""
        modal = StatusSelectionModal()
        modal.selected_index = len(modal.status_options) - 1
        modal.action_next_option()
        assert modal.selected_index == 0

    def test_action_previous_status_decrements_selection(self) -> None:
        """Test that previous_status action decrements the selected index."""
        modal = StatusSelectionModal()
        modal.selected_index = 2
        modal.action_previous_option()
        assert modal.selected_index == 1

    def test_action_previous_status_wraps_around(self) -> None:
        """Test that previous_status wraps from first to last option."""
        modal = StatusSelectionModal()
        modal.selected_index = 0
        modal.action_previous_option()
        assert modal.selected_index == len(modal.status_options) - 1

    def test_status_options_in_correct_order(self) -> None:
        """Test that status options are in the expected order."""
        modal = StatusSelectionModal()
        expected_options = [
            "📝 Draft",
            "✅ Accepted",
            "❌ Rejected",
            "⛔ Deprecated by …",
            "⬆️ Supersedes …",
        ]
        assert modal.status_options == expected_options

    def test_modal_cycles_through_all_options(self) -> None:
        """Test cycling through all options using next_status."""
        modal = StatusSelectionModal()
        num_options = len(modal.status_options)

        # Cycle through all options
        for expected_index in range(num_options):
            assert modal.selected_index == expected_index
            modal.action_next_option()

        # Should be back at the start
        assert modal.selected_index == 0


class TestStatusOptionDisplay:
    """Tests for the StatusOptionDisplay widget."""

    def test_display_initializes_with_options(self) -> None:
        """Test that the display widget initializes with status options."""
        options = ["Option 1", "Option 2", "Option 3"]
        display = StatusOptionDisplay(options, selected_index=0)
        assert display.status_options == options
        assert display.selected_index == 0

    def test_display_renders_with_selected_indicator(self) -> None:
        """Test that the display renders with an arrow for the selected option."""
        options = ["Option 1", "Option 2", "Option 3"]
        display = StatusOptionDisplay(options, selected_index=1)
        rendered = display.render()
        lines = rendered.split("\n")

        # First option should have spaces, not arrow
        assert lines[0].startswith("  ")
        # Second option should have arrow
        assert "→" in lines[1]
        # Third option should have spaces, not arrow
        assert lines[2].startswith("  ")

    def test_display_renders_all_options(self) -> None:
        """Test that all options are rendered."""
        options = ["Option 1", "Option 2", "Option 3"]
        display = StatusOptionDisplay(options, selected_index=0)
        rendered = display.render()
        for option in options:
            assert option in rendered

    def test_display_update_selection_changes_index(self) -> None:
        """Test that update_selection changes the selected index."""
        options = ["Option 1", "Option 2", "Option 3"]
        display = StatusOptionDisplay(options, selected_index=0)
        display.update_selection(2)
        assert display.selected_index == 2

    def test_display_selected_index_in_bounds(self) -> None:
        """Test that selected_index is properly validated when rendering."""
        options = ["Option 1", "Option 2", "Option 3"]
        display = StatusOptionDisplay(options, selected_index=1)
        # Should render without error even with middle index selected
        rendered = display.render()
        assert "Option 2" in rendered
