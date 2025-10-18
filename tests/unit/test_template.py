from __future__ import annotations

import pytest

from vrdx.app.commands import apply_template_to_editor
from vrdx.parser import (
    DEFAULT_STATUS,
    DecisionTemplate,
    list_status_options,
    normalise_status,
    render_template,
)


def test_render_template_uses_defaults():
    output = render_template(42)
    assert output.startswith("### 42")
    assert f"* **Status**: {DEFAULT_STATUS}" in output
    assert "* **Decision**:" in output
    assert "* **Context**:" in output
    assert "* **Consequences**:" in output


def test_render_template_accepts_custom_values():
    output = render_template(
        5,
        title="Evaluate New Shell",
        status="✅ Accepted",
        decision="Adopt nushell for daily work.",
        context="Prefer structured pipelines.",
        consequences="Need to migrate dotfiles.",
    )
    lines = output.splitlines()
    assert lines[0] == "### 5 Evaluate New Shell"
    assert lines[1].endswith("✅ Accepted")
    assert "Adopt nushell for daily work." in lines[2]
    assert "Prefer structured pipelines." in lines[3]
    assert "Need to migrate dotfiles." in lines[4]


def test_decision_template_render_matches_function():
    template = DecisionTemplate(
        next_id=3,
        title_placeholder="Review Terminal Options",
        status="❌ Rejected",
        decision_placeholder="Do not switch terminals.",
        context_placeholder="Ghostty already fits needs.",
        consequences_placeholder="No further action required.",
    )
    assert template.render() == render_template(
        3,
        title="Review Terminal Options",
        status="❌ Rejected",
        decision="Do not switch terminals.",
        context="Ghostty already fits needs.",
        consequences="No further action required.",
    )


def test_list_status_options_contains_default():
    options = list(list_status_options())
    assert DEFAULT_STATUS in options
    assert len(options) == len(set(options))  # no duplicates
    assert options == list_status_options()  # deterministic ordering


@pytest.mark.parametrize(
    ("input_status", "expected"),
    [
        ("✅ Accepted", "✅ Accepted"),
        ("📝 Draft", "📝 Draft"),
        ("Unknown Status", DEFAULT_STATUS),
    ],
)
def test_normalise_status(input_status: str, expected: str):
    assert normalise_status(input_status) == expected


def test_apply_template_to_editor_uses_default_status():
    """Test that apply_template_to_editor uses default status when none provided."""
    template = apply_template_to_editor(5)
    assert "### 5" in template
    assert f"* **Status**: {DEFAULT_STATUS}" in template


def test_apply_template_to_editor_uses_provided_status():
    """Test that apply_template_to_editor uses the provided status."""
    template = apply_template_to_editor(10, status="✅ Accepted")
    assert "### 10" in template
    assert "* **Status**: ✅ Accepted" in template


def test_apply_template_to_editor_normalises_invalid_status():
    """Test that apply_template_to_editor normalises invalid status values."""
    template = apply_template_to_editor(7, status="Invalid Status")
    assert "### 7" in template
    assert f"* **Status**: {DEFAULT_STATUS}" in template
