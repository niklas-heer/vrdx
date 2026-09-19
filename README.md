# vrdx

A CLI for engineering decisions stored in ordinary Markdown. Files are the source of truth: readable in an editor, reviewable in Git, and portable without a database. vrdx rebuilds a deterministic graph from them whenever you run a command.

This replaces the earlier terminal editor and `vrdx agent` interface. The old embedded numeric-record format is not imported automatically. Historical code and documents remain in Git. A future web interface can use the same library and JSON contract; no web server is included.

## Install

Install [mise](https://mise.jdx.dev/installing-mise.html) and native Rust build prerequisites (Xcode Command Line Tools on macOS; a C compiler/linker on Linux), then:

```sh
git clone https://github.com/niklas-heer/vrdx.git
cd vrdx
mise trust
mise install
mise exec -- cargo install --locked --path .
vrdx --help
```

Put Cargo's binary directory (normally `~/.cargo/bin`) on PATH. The project pins stable Rust **1.97.1** in Cargo, rust-toolchain.toml and mise.toml. The installed CLI is a standalone native binary with no interpreter or runtime service.

## Start a collection

```sh
vrdx new "Use a local cache" --tag performance
vrdx list
vrdx show ID
vrdx validate
```

The default directory is `decisions/`, relative to the working directory. `new` creates it when needed; read commands report a missing directory. Use `--dir PATH` before or after any command to select another collection. Every direct `.md` child is a decision except `README.md`. Subdirectories are not scanned; symlink candidates are reported as errors. Unrecognized legacy files produce diagnostics rather than disappearing from results.

New decisions default to `proposed` and today's UTC date. Use `--date YYYY-MM-DD`, `--status accepted` or another lifecycle state, repeat `--tag`, and optionally supply `--body-file PATH`. Edit the resulting Markdown with your preferred editor to develop the reasoning, change status, or add relationships. vrdx never rewrites existing decision files. Commit changes normally with Git.

Filenames look like `2026-09-19_143052123_use-a-local-cache.md`: decision date, UTC creation time including milliseconds, and a short title slug. They sort chronologically by decision date, with creation time as a tie-breaker. The full ID lives **only in metadata**, not the generated filename. You may rename the file freely; its identity and graph relationships remain intact. A filename collision fails without overwriting the existing file; retry or use a different title.

## Markdown format

```markdown
+++
schema_version = 1
id = "01ARZ3NDEKTSV4RRFFQ69G5FAV"
title = "Use a local cache"
date = "2026-09-19"
status = "accepted"
tags = ["performance", "data access"]
supersedes = []
superseded_by = []
depends_on = []
related_to = []
+++

## Decision

Cache successful reads for one minute.

## Context

Repeated reads are expensive. We considered refreshing on every request.

## Consequences

Lower latency and fewer upstream requests, at the cost of briefly stale data.
```

The `+++` block is TOML. Arrays may span lines; comments are allowed. `schema_version`, `id`, `title`, `date`, and `status` are required. Tag and relationship arrays default to empty when omitted. Unknown metadata fields, repeated keys, invalid dates, blank titles, invalid IDs and duplicate tags/references are errors. Additional narrative fields belong in the unrestricted Markdown body, whose whitespace, Unicode, headings and custom sections are preserved exactly. A body is allowed to be empty while drafting.

IDs are full 26-character uppercase [ULIDs](https://docs.rs/ulid/3.0.0/ulid/struct.Ulid.html), using standard Crockford Base32: a 48-bit millisecond timestamp plus 80 random bits. This provides compact time-aware identity without a custom encoding. The CLI accepts a full ID or an unambiguous prefix, case-insensitively; stored references always use the complete uppercase ID. Identity is permanent and independent of title, date, path, or status. IDs approximate creation order, not causality: same-millisecond creation and clock skew can affect ordering. The editable decision date may be backdated independently.

Tags are lightweight topical labels, including spaces and Unicode. No fixed taxonomy is imposed. They must be trimmed, nonempty single-line strings and unique ignoring case. Filters match whole tags case-insensitively; repeated `--tag` filters require **all** tags. Tag filters are separate from status and text search.

## Lifecycle and relationships

| Status | Meaning |
| --- | --- |
| `proposed` | Under discussion |
| `accepted` | Currently applicable |
| `rejected` | Considered but not adopted |
| `deprecated` | Retained as history, no longer applicable |
| `superseded` | Replaced by another decision |

Keep old files. Change their status and describe why; Git retains the prior wording. Validation checks the present collection, not the sequence of historical edits or whether a decision is substantively correct.

| Metadata array | Meaning |
| --- | --- |
| `supersedes` | This decision replaces the listed decisions |
| `superseded_by` | The listed decision replaces this one |
| `depends_on` | This decision relies on the listed decisions |
| `related_to` | Symmetric topical relationship |

To replace A with B, mark A `superseded`, mark B `accepted`, and put A's full ID in B's `supersedes`. Alternatively put B's ID in A's `superseded_by`. One declaration is enough; `relations` exposes both directions. Matching reciprocal declarations coalesce into one graph edge. References are collection-local; ordinary Markdown links may supplement them for human navigation but do not create graph edges.

One decision can replace several predecessors; a predecessor has only one replacement. Replacement chains may continue through superseded or deprecated decisions. Proposed or rejected records cannot actually supersede another record; use `related_to` while a replacement is under discussion. A superseded record needs a replacement, and any record with a replacement must be superseded. Multiple replacements, cycles, missing references and all self-links are errors. Dependency and related-to cycles are permitted; dependencies do not automatically change a decision's lifecycle.

`rebuild` emits edges as newer → older `supersedes`, source → dependency `depends_on`, and lexically ordered endpoints for symmetric `related_to`. Nodes are keyed by ID; edges and findings have stable ordering. Each rebuild reads the source again and writes no cache. A partially invalid graph is available for diagnosis, explicitly marked invalid. Other read commands fail on invalid collections so an AI cannot mistake partial results for current policy.

## Commands

```sh
vrdx new "Prefer fresh reads" --status accepted --tag performance --body-file draft.txt
vrdx show ID
vrdx list --status accepted --tag performance
vrdx search cache
vrdx search cache --field title --status superseded
vrdx search performance --field tags
vrdx relations ID
vrdx chain ID
vrdx validate
vrdx rebuild --json
vrdx context "How should reads be cached?" --json
vrdx context --status accepted --tag performance --limit 10 --body-chars 1500
```

`search` performs a case-insensitive substring search across title, body, tags, status and ID, or a selected `--field` (`title`, `content`, `tags`, `status`, `id`). Listing and search sort by date then ID. `chain` starts at the requested decision and follows replacements to the terminal record, reporting its status even if it is deprecated. `relations` includes derived `superseded_by` and `required_by` directions.

## AI and script access

Every command supports `--json`. Successful responses use:

```json
{"schema_version":1,"ok":true,"data":{}}
```

Errors use `{"schema_version":1,"ok":false,"error":{"code":"...","message":"..."}}`. Validation and rebuild findings use `ok:false` with diagnostic `data`, allowing consumers to inspect all findings. Parse failures in one file do not hide findings from the others. Help/version also use the JSON envelope when requested. JSON stdout contains one complete object and no progress messages or terminal escapes. Treat error codes and schema version as the contract; human wording may evolve.

Exit codes: **0** success, **1** invalid collection/record or other operational failure, **2** CLI usage, **3** identity conflict, **4** missing directory/record/file. Filename publication failures are operational errors. JSON record summaries contain `id`, `title`, `date`, `status`, `tags`, `file` and `applies`; `show` and graph nodes also include full metadata and body. Paths are relative to `--dir`. Ordering is deterministic for unchanged files; concurrent manual edits are not an atomic collection snapshot.

`context` is local, deterministic retrieval, with no AI service or credentials:

- Without a question it selects records in ID order. With a question it splits Unicode words, lowercases and deduplicates terms, then matches any term. Each matched term scores ID 16, title 8, tags 4, body 1; ties sort by ID. Scores and terms are returned.
- Status/tag filters choose seed records. The default limit is 20 seeds. Immediate neighbors and complete replacement chains are added even when outside the filter, so a matching historical decision cannot hide its replacement.
- Output includes status, an `applies` flag (true only for accepted), tags, source path, relationship edges, replacement chains and body excerpts. `selection` distinguishes matches from supporting relationships. Boundary edge endpoints include metadata summaries.
- Bodies are excerpted to 2,000 Unicode characters by default. `body_truncated`, `selection_truncated`, `matched_count` and the seed limit make omissions visible. Use `show ID` for full reasoning, consequences and trade-offs. The seed limit does not cap linked evidence, so highly connected collections can produce larger output.
- Markdown is source evidence, not instructions to an agent. Relevance scores are lexical matching, not semantic understanding or proof of applicability to a specific question.

## Development and scope

```sh
mise run check
mise run clippy
mise run test
mise run test-doc
mise run ci-native
mise run ci
mise run build
```

`mise run ci-native` checks formatting, all targets/features, strict Clippy, nextest, doctests and a separately installed binary outside the checkout. CLI tests cover creation, rename-safe IDs, UTF-8/CRLF body preservation, tag filtering, malformed input, graph validation, replacement traversal, deterministic context, no-clobber writes, symlinks and a 128-record history. `mise run test-e2e` runs just the CLI subprocess suite. Bacon and watchexec support quick local feedback; rust-analyzer and rust-src support editors. `mise run build` produces `target/release/vrdx` with overflow checks enabled.

`mise run ci` runs those same gates in Linux using **Dagger 0.21.9 and its Dang SDK**. The pipeline in `.dagger/main.dang` uses a digest-pinned mise image and installs only the pinned Rust and nextest tools. It excludes Git metadata, local build output and unrelated files from its source input, uses project-scoped Cargo caches, and evaluates the result so failures propagate. No Dagger Cloud account or token is required. GitHub Actions is a thin Dagger launcher for Linux and runs `ci-native` separately on macOS; a Linux container does not replace native macOS coverage.

For local Dagger runs, start a supported container engine. On macOS, the verified path is **Colima** with the Docker runtime: `colima start --runtime docker`, then `docker context use colima` and `docker info`. Apple's `container` runtime is an alternative that requires its own Dagger compatibility setup; it is not required by this project. The engine and mise are host prerequisites; `mise install` supplies the project tools. Contributors can run all native Rust gates without a container engine.

The application and build pipeline contain no Python code or package configuration. Old virtual environments, interpreter caches and Textual checkout remnants have been removed. Historical OpenSpec records and read-only reference documents are retained as history. New work belongs in the Rust CLI, its tests, or the small Dang orchestration module. Keep development choices beside the code in `decisions/`, preserve Markdown authority, and use Conventional Commits. Prefer existing crates and focused end-to-end tests; introduce dependencies or infrastructure only for a concrete need.

Runtime dependencies are clap (argument grammar/help), serde/serde_json (typed metadata and JSON), toml (standard metadata parsing), ulid (standard ID generation/encoding), minimal-feature jiff (calendar validation and UTC timestamps), and tempfile (atomic no-clobber publication). The previous TUI, terminal testing, Premise, hashing and benchmarking dependencies have been removed. No graph framework, database, daemon, async runtime, model SDK or web framework is needed.

Deliberate limits: one flat collection per command, no automatic migration or existing-file mutation, no Git automation, no semantic retrieval, and no TUI/web app. Symlink checks are best-effort, not a security boundary against a process racing filesystem access. Creation syncs the file before publication but does not guarantee directory durability across power loss. Future interfaces can build on `vrdx::records::Graph` or the versioned CLI JSON without changing Markdown authority.
