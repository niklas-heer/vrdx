"""Parsing and serialising decision records enclosed within vrdx marker blocks.

This module focuses on Milestone 3 responsibilities: extracting structured
decision data from Markdown and rendering it back while preserving a consistent
format. Decisions are expected to follow the canonical template documented in
``docs/DESIGN.md``:

    ### 13 Sticking with Amethyst
    * **Status**: ✅ Adopted
    * **Decision**: ...
    * **Context**: ...
    * **Consequences**: ...

Multiple decisions can appear inside a single marker block. The parser is
designed to be resilient to additional whitespace and multiline field content,
but it assumes that headings start with ``###`` and field labels remain
canonical.
"""

from __future__ import annotations

import re
from typing import Iterable, Iterator, List, Optional, Sequence

from pydantic import BaseModel, Field, field_validator

HEADING_PATTERN = re.compile(r"^###\s+(?P<id>\d+)\s+(?P<title>.+)$", re.MULTILINE)
FIELD_PATTERN = re.compile(
    r"^\*\s+\*\*(?P<label>Status|Decision|Context|Consequences)\*\*:\s*(?P<value>.*)$"
)

CANONICAL_FIELDS: tuple[str, ...] = ("Status", "Decision", "Context", "Consequences")
FIELD_TO_ATTR = {
    "Status": "status",
    "Decision": "decision",
    "Context": "context",
    "Consequences": "consequences",
}


class DecisionParseError(ValueError):
    """Raised when a decision block cannot be parsed into the expected format."""


class DecisionRecord(BaseModel):
    """Structured representation of a single decision entry."""

    id: int = Field(gt=-1)
    title: str
    status: str
    decision: str
    context: str
    consequences: str
    raw: str = Field(repr=False)

    @field_validator("title", "status", "decision", "context", "consequences")
    @classmethod
    def _trim(cls, value: str) -> str:  # pragma: no cover - trivial helper
        return value.strip()

    def render(self, *, newline: str = "\n") -> str:
        """Render the decision back to Markdown using the canonical format."""
        parts = [
            f"### {self.id} {self.title}",
            f"* **Status**: {self.status}",
            f"* **Decision**: {self.decision}",
            f"* **Context**: {self.context}",
            f"* **Consequences**: {self.consequences}",
        ]
        return newline.join(parts)


def _iter_decision_sections(body: str) -> Iterator[str]:
    matches = list(HEADING_PATTERN.finditer(body))
    for index, match in enumerate(matches):
        start = match.start()
        end = matches[index + 1].start() if index + 1 < len(matches) else len(body)
        section = body[start:end].strip()
        if section:
            yield section


def _extract_field_blocks(lines: Sequence[str]) -> dict[str, str]:
    """Convert bullet-field lines into a mapping from label to value."""
    result = {field: "" for field in CANONICAL_FIELDS}
    idx = 0
    while idx < len(lines):
        line = lines[idx]
        match = FIELD_PATTERN.match(line.strip())
        if not match:
            idx += 1
            continue

        label = match.group("label")
        buffer: List[str] = [match.group("value").rstrip()]
        idx += 1

        while idx < len(lines):
            candidate = lines[idx]
            # Stop when we hit another field bullet or a heading.
            if FIELD_PATTERN.match(candidate.strip()) or candidate.startswith("### "):
                break
            buffer.append(candidate.rstrip())
            idx += 1

        result[label] = "\n".join(part for part in buffer if part).strip()
    return result


def parse_decisions(body: str) -> list[DecisionRecord]:
    """Parse all decisions contained in a marker block body."""
    decisions: list[DecisionRecord] = []
    for section in _iter_decision_sections(body):
        lines = section.splitlines()
        if not lines:
            continue

        heading_match = HEADING_PATTERN.match(lines[0])
        if not heading_match:
            raise DecisionParseError(f"Missing decision heading in block:\n{section}")

        decision_id = int(heading_match.group("id"))
        title = heading_match.group("title").strip()

        field_map = _extract_field_blocks(lines[1:])
        missing = [field for field in CANONICAL_FIELDS if not field_map[field]]
        if missing:
            raise DecisionParseError(
                f"Decision {decision_id} '{title}' is missing fields: {', '.join(missing)}"
            )

        decisions.append(
            DecisionRecord(
                id=decision_id,
                title=title,
                status=field_map["Status"],
                decision=field_map["Decision"],
                context=field_map["Context"],
                consequences=field_map["Consequences"],
                raw=section,
            )
        )
    if not decisions and body.strip():
        raise DecisionParseError(
            "No valid decision entries were found in the provided marker block."
        )
    return decisions


def render_decisions(
    decisions: Iterable[DecisionRecord],
    *,
    newline: str = "\n",
    separator: Optional[str] = None,
) -> str:
    """Render an iterable of decisions back into Markdown."""
    sep = separator if separator is not None else f"{newline}{newline}"
    rendered = [decision.render(newline=newline) for decision in decisions]
    return sep.join(rendered)


def find_next_decision_id(decisions: Sequence[DecisionRecord]) -> int:
    """Return the next available decision ID, counting downwards from the maximum."""
    if not decisions:
        return 0
    return max(decision.id for decision in decisions) + 1


def update_decision_body(
    existing_body: str,
    decisions: Sequence[DecisionRecord],
    *,
    newline: str = "\n",
) -> str:
    """Replace the decision block body with the given decisions."""
    rendered = render_decisions(
        decisions, newline=newline, separator=f"{newline}{newline}"
    )
    if rendered and not rendered.endswith(newline):
        rendered = f"{rendered}{newline}"
    return rendered
