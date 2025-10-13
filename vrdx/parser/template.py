"""Helpers for generating decision templates and status metadata.

This module centralises the canonical template content used when creating new
decision records. It also exposes the curated set of status labels surfaced in
the UI so both the renderer and the interface share a single source of truth.

The template output mirrors the structure documented in `docs/DESIGN.md`.
"""

from __future__ import annotations

from dataclasses import dataclass
from typing import Iterable, List

DEFAULT_STATUS = "📝 Draft"

# Ordered list so the UI can present statuses predictably.
STATUS_OPTIONS: List[str] = [
    "📝 Draft",
    "✅ Accepted",
    "❌ Rejected",
    "⛔ Deprecated by …",
    "⬆️ Supersedes …",
]


@dataclass(frozen=True)
class DecisionTemplate:
    """Callable container for producing new decision Markdown blocks."""

    next_id: int
    title_placeholder: str = "Decision Title (e.g., 'Use PostgreSQL for data storage')"
    status: str = DEFAULT_STATUS
    decision_placeholder: str = "What decision was made? (e.g., 'We will use PostgreSQL as our primary database')"
    context_placeholder: str = "What is the context? Why is this decision needed? (e.g., 'We need a reliable, ACID-compliant database for our application')"
    consequences_placeholder: str = "What are the consequences? Pros and cons? (e.g., 'Pros: ACID compliance, mature ecosystem. Cons: Requires separate hosting')"

    def render(self, *, newline: str = "\n") -> str:
        """Render the decision template using the specified newline."""
        parts = [
            f"### {self.next_id} {self.title_placeholder}".rstrip(),
            f"* **Status**: {self.status}",
            f"* **Decision**: {self.decision_placeholder}",
            f"* **Context**: {self.context_placeholder}",
            f"* **Consequences**: {self.consequences_placeholder}",
        ]
        return newline.join(parts)


def is_status_supported(status: str) -> bool:
    """Return True when ``status`` is one of the curated status options."""
    return status in STATUS_OPTIONS


def normalise_status(status: str) -> str:
    """Return the provided status when it is supported; otherwise, fall back to the default draft status."""
    return status if is_status_supported(status) else DEFAULT_STATUS


def render_template(
    next_id: int,
    *,
    title: str = "Decision Title (e.g., 'Use PostgreSQL for data storage')",
    status: str = DEFAULT_STATUS,
    decision: str = "What decision was made? (e.g., 'We will use PostgreSQL as our primary database')",
    context: str = "What is the context? Why is this decision needed? (e.g., 'We need a reliable, ACID-compliant database for our application')",
    consequences: str = "What are the consequences? Pros and cons? (e.g., 'Pros: ACID compliance, mature ecosystem. Cons: Requires separate hosting')",
    newline: str = "\n",
) -> str:
    """Convenience function to create and render a decision template."""
    template = DecisionTemplate(
        next_id=next_id,
        title_placeholder=title,
        status=normalise_status(status),
        decision_placeholder=decision,
        context_placeholder=context,
        consequences_placeholder=consequences,
    )
    return template.render(newline=newline)


def list_status_options() -> Iterable[str]:
    """Return the curated status values."""
    return STATUS_OPTIONS
