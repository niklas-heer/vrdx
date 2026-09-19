# Concise, composable decision authoring

## Why
Niklas requested a fast single executable, multiple forms of testing, formatting,
actionable validation and built-in AI guidance. Records should capture one choice,
why and consequences, with short memorable titles and minimal ceremony.

## What Changes
- Keep noninteractive creation; add structured JSON file/stdin input and an explicit editor flow with recoverable drafts.
- Add a provider-independent copyable prompt and a concise guide with an executable input example/schema.
- Add explicit formatting and check-only mode. Normalize metadata key order and spacing while preserving comments, IDs and all Markdown body bytes. Generated structured bodies are already formatted; arbitrary prose/code is never reflowed.
- Add repair hints to human and JSON diagnostics.
- Exercise subprocess, editor failure/recovery, formatting idempotence and standalone executable journeys. Measure release command latency with a reproducible benchmark and remove demonstrated hot spots.

## Authorization and boundaries
The user's design-and-build request authorizes this work. Explicit fmt is the
new exception to existing-file immutability; queries remain read-only. The editor
only edits a staged new draft. No hosted AI, database, interactive wizard, Git
automation or general Markdown rewriting. Existing bodies and record IDs remain
authoritative. Historical long records are not automatically shortened.

## Research
- https://cognitect.com/blog/2011/11/15/documenting-architecture-decisions — one small record per decision; title, context, choice and consequences.
- https://clig.dev/ — composable stdin/JSON, explicit interactivity, useful errors and examples.
- https://www.anthropic.com/engineering/writing-tools-for-agents — clear inputs, actionable errors, meaningful output and measured task coverage.

The shorter writing target and exact command design are project choices, not claims
that these sources prescribe this implementation. Context7 and CORE Memory are
unavailable; official documentation and repository context are used instead.
