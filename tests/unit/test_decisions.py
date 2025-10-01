from __future__ import annotations

import pytest

from vrdx.parser import (
    DecisionParseError,
    DecisionRecord,
    find_next_decision_id,
    parse_decisions,
    render_decisions,
    update_decision_body,
)


def test_parse_single_decision():
    body = (
        "### 1 Adopt Tool\n"
        "* **Status**: 📝 Draft\n"
        "* **Decision**: Use the new tool.\n"
        "* **Context**: Simpler workflow.\n"
        "* **Consequences**: Less maintenance.\n"
    )

    decisions = parse_decisions(body)
    assert len(decisions) == 1
    decision = decisions[0]
    assert decision.id == 1
    assert decision.title == "Adopt Tool"
    assert decision.status == "📝 Draft"
    assert "Use the new tool." in decision.decision


def test_parse_multiple_decisions_preserves_order():
    body = (
        "### 3 Upgrade Stack\n"
        "* **Status**: ✅ Accepted\n"
        "* **Decision**: Upgrade to latest stack.\n"
        "* **Context**: Align with company standards.\n"
        "* **Consequences**: Training required.\n"
        "\n"
        "### 2 Sunset Legacy\n"
        "* **Status**: ❌ Rejected\n"
        "* **Decision**: Drop the legacy module.\n"
        "* **Context**: Customers still depend on it.\n"
        "* **Consequences**: Revisit next quarter.\n"
    )

    decisions = parse_decisions(body)
    assert [d.id for d in decisions] == [3, 2]
    assert decisions[0].title == "Upgrade Stack"
    assert decisions[1].status == "❌ Rejected"


def test_parse_decision_missing_field_raises():
    body = (
        "### 5 Incomplete Decision\n"
        "* **Status**: 📝 Draft\n"
        "* **Decision**: TBD\n"
        "* **Context**: TBD\n"
    )

    with pytest.raises(DecisionParseError):
        parse_decisions(body)


def test_render_decisions_roundtrip():
    decisions = [
        DecisionRecord(
            id=10,
            title="Test Decision",
            status="✅ Accepted",
            decision="Do the thing.",
            context="Because it helps.",
            consequences="Improved morale.",
            raw="",
        )
    ]
    rendered = render_decisions(decisions)
    assert "### 10 Test Decision" in rendered
    reparsed = parse_decisions(rendered)
    assert reparsed[0].context == "Because it helps."


def test_update_decision_body_inserts_newline():
    decisions = [
        DecisionRecord(
            id=1,
            title="Example",
            status="📝 Draft",
            decision="Initial choice.",
            context="Evaluating options.",
            consequences="Requires follow-up.",
            raw="",
        ),
        DecisionRecord(
            id=0,
            title="Prior Decision",
            status="✅ Accepted",
            decision="Baseline.",
            context="Historical context.",
            consequences="Already in place.",
            raw="",
        ),
    ]
    body = update_decision_body("ignored", decisions)
    assert body.endswith("\n")
    assert body.count("###") == 2


def test_find_next_decision_id_returns_increment():
    decisions = [
        DecisionRecord(
            id=7,
            title="Existing",
            status="✅ Accepted",
            decision="Existing decision.",
            context="Already decided.",
            consequences="Stable.",
            raw="",
        )
    ]
    assert find_next_decision_id(decisions) == 8
    assert find_next_decision_id([]) == 0
