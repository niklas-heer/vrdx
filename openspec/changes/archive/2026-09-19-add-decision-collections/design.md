## Context

Markdown and Git remain authoritative. The current editor's numeric identities cannot address a renamed standalone record. The user explicitly instructed us to replace the editor with a CLI; retain prior implementation history in Git rather than keeping parallel interfaces.

## Decisions

- A collection is the direct `.md` children of a selected directory (`decisions` by default), except `README.md`. Every candidate must parse; incompatible legacy files are diagnosed, never silently migrated.
- Each file starts with `+++` TOML metadata: schema version, ID, title, date, status, optional tags and relationship arrays. The remaining Markdown is retained verbatim. Unknown metadata keys and duplicate fields are errors to catch misspellings; arbitrary additional reasoning belongs in the body.
- ULIDs use the standard 26-character uppercase Crockford Base32 form: a 48-bit millisecond timestamp and 80 random bits. This avoids a custom UUID codec and is shorter than canonical UUIDv7. No truncated identity is stored. Query prefixes must resolve uniquely. Time ordering is approximate under clock skew or same-millisecond creation.
- New filenames are `DATE_HHMMSSmmm_slug.md`, ordered by decision date and UTC creation time; the ULID appears only in metadata, as requested. Metadata identity survives all renames; filename content is never authoritative. Date may be backdated independently of ID creation time.
- Status is proposed, accepted, rejected, deprecated or superseded. Accepted means currently applicable. Markdown edits preserve history through Git; no deletion or automatic transition commands are added.
- `supersedes` and `superseded_by` are inverse declarations normalized to newer → older edges. Either side is sufficient; identical reciprocal declarations coalesce. Each old record may have one replacement, while a replacement may consolidate several records. `related_to` is symmetric and `depends_on` directional. Only supersession cycles are invalid.
- An actual replacement must be accepted, deprecated or superseded, and its predecessor must be superseded. A superseded record must have a replacement. Proposed replacements should use related_to until accepted. Date order does not imply causality.
- Sorted maps/sets give deterministic nodes, edges, findings and query tie-breaking. Graph rebuild/export does not write a cache. Invalid collections cannot produce authoritative lists or AI context; validation and graph export expose findings.
- Tags are trimmed, nonempty free-form strings, matched case-insensitively and exactly by filters (multiple tags mean AND). They remain independent from text queries and status.
- AI context ranks case-insensitive question terms with simple transparent field weights, includes selected records plus immediate neighbors and complete replacement chains, and truncates body excerpts explicitly. Source IDs, paths, tags, status and edges remain available. No model, embedding service or credentials are required.
- Reuse serde, serde_json and tempfile. Add clap for a discoverable CLI, toml for a standard metadata grammar, ulid for standard identifiers, and minimal jiff for real date validation/UTC defaults. No graph crate is needed.

## Limits

No legacy migration, cross-collection references, persistent index, graph visualization, semantic retrieval, multi-file mutation transaction, TUI, or web app. Read commands fail on incompatible old records; the previous interface is intentionally removed. Creation is no-clobber and atomic through tempfile; concurrent manual edits are observed on the next rebuild, not as an atomic collection snapshot. Validation checks current state, not historical transitions or authors' reasoning.

## Evidence

- [ULID representation and generation](https://docs.rs/ulid/3.0.0/ulid/struct.Ulid.html)
- [UUIDv7 alternative](https://www.rfc-editor.org/rfc/rfc9562.html#section-5.7)
- The previous document/repository layers informed no-clobber publication and installed-binary verification; the obsolete implementation was removed after the user changed scope.
