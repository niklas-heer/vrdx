<p align="center">
  <img src="assets/logo.svg" alt="vrdx branching decision mark" width="88" height="88">
</p>
<h1 align="center">vrdx</h1>
<p align="center"><strong>Keep the decision. Keep the why.</strong></p>
<p align="center">Engineering decisions in Markdown. A Rust CLI for people and AI, with a local dashboard.</p>
<p align="center">
  <a href="#start-here">Get started</a> ·
  <a href="#see-the-bigger-picture">Dashboard</a> ·
  <a href="docs/ai-guide.md">AI guide</a> ·
  <a href="docs/format.md">File format &amp; JSON</a>
</p>

---

Decisions outlive the conversations that produced them. vrdx keeps the choice, reasoning, trade-offs and replacement history together in files your team can read, review and carry to another tool.

- **Markdown is the source.** Read it in any editor, review it in Git, rebuild the graph at any time.
- **History stays connected.** Stable IDs survive renames; explicit relationships show what replaced what.
- **People and agents share the evidence.** Browse a local dashboard, query the CLI or retrieve structured context with source paths and lifecycle status.

No database, account or external AI service. One native Rust binary.

## Start here

Download the archive for your operating system and processor from
[GitHub Releases](https://github.com/niklas-heer/vrdx/releases/latest), together
with `SHA256SUMS`. Each archive contains the executable, this README and the MIT
license. No Rust toolchain is needed to run it.

| System | Archive target |
| --- | --- |
| Linux, Intel/AMD 64-bit | `x86_64-unknown-linux-gnu` |
| Linux, ARM 64-bit | `aarch64-unknown-linux-gnu` |
| macOS, Apple Silicon | `aarch64-apple-darwin` |
| macOS, Intel | `x86_64-apple-darwin` |

Verify the downloaded archive against its entry in `SHA256SUMS` using
`sha256sum` on Linux or `shasum -a 256` on macOS. Extract it, then put `vrdx` in
a directory on your `PATH`, such as `~/.local/bin`. Run `vrdx --version` and
`vrdx guide` to check the installation. Linux releases are tested on Ubuntu
24.04 and macOS releases on macOS 15; other systems can build from source.
Checksums detect corrupted downloads; they are not code signatures.

To build from source instead:

Install [mise](https://mise.jdx.dev/installing-mise.html) and native Rust build prerequisites: Xcode Command Line Tools on macOS, or a C compiler/linker on Linux.

```sh
git clone https://github.com/niklas-heer/vrdx.git
cd vrdx
mise trust
mise install
mise exec -- cargo install --locked --path .
```

Put Cargo's binary directory (normally `~/.cargo/bin`) on your `PATH`. From the repository where you want to keep decisions:

```sh
vrdx new "Use a local cache" --tag performance
vrdx list
vrdx validate
vrdx dashboard
```

`new` creates a proposed record in `decisions/`. Edit that Markdown to explain the choice, alternatives and consequences. Change its status to `accepted` when the decision is made, then commit it with the code it informs.

Use `--dir PATH` with any command to select another collection. `vrdx show ID` reads a record by its full ID or any unambiguous prefix.

## See the bigger picture

```sh
vrdx dashboard --port 7878
```

Open **http://127.0.0.1:7878** to explore the current collection. The dashboard presents decisions, lifecycle states, tags and relationships from your Markdown files. It is a local, read-only view; edit the files to change the source.

The map opens by default. Hover over a decision or focus it with the keyboard to
highlight its direct connections. Select it to explore its neighborhood, follow
named relationships and open its full reasoning with **Read decision**. Connected
records outside your filters are labeled as context. **All matches** (or Escape)
returns to the filtered overview; **Records** switches to the card view.

The **Theme** control offers System, Light and Dark. System follows your device;
explicit choices are remembered in this browser when local storage is available.

The browser interface ships inside the Rust binary. There is no separate frontend installation or database to synchronize. Stop the foreground process with Ctrl-C when you are done.

If the collection is invalid, the dashboard shows diagnostics. Use `vrdx validate` for the same validation from your terminal.

## A decision is an ordinary file

Generated filenames are readable and chronologically sortable:

```text
decisions/2026-09-19_143052123_use-a-local-cache.md
```

The permanent 26-character ULID lives in metadata. Rename the file or revise the title without breaking its references.

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

TOML metadata supplies identity, date, status, optional tags and relationships. The body is unrestricted Markdown. Tags are free-form topics; filters match complete tags without case sensitivity, and repeated `--tag` filters require all supplied tags.

| Status | How to read it |
| --- | --- |
| `proposed` | Under discussion |
| `accepted` | Currently applicable |
| `rejected` | Considered but not adopted |
| `deprecated` | Retained as history; no longer applicable |
| `superseded` | Replaced by another decision |

Keep old decisions. To replace A with B, mark A `superseded`, mark B `accepted`, and put A's full ID in B's `supersedes`. One declaration is sufficient: vrdx derives the inverse relationship. `depends_on` expresses a dependency; `related_to` expresses a symmetric topical link.

Validation catches malformed metadata, duplicate identities, missing references, self-links, conflicting replacements and supersession cycles. See the [format and lifecycle rules](docs/format.md) for the complete contract.

## Find, inspect, connect

| Command | Purpose |
| --- | --- |
| `new "Title"` | Create a proposed Markdown decision |
| `show ID` | Read its complete metadata, body and relationships |
| `list` | Browse in date and ID order |
| `search QUERY` | Search titles, content, tags, status or IDs |
| `relations ID` | Inspect explicit links and their derived inverses |
| `chain ID` | Follow replacements to the terminal decision |
| `suggest ID` | Surface possible related records with matching evidence |
| `validate` | Check every file and graph relationship |
| `rebuild` | Derive and export the graph without writing a cache |
| `context [QUESTION]` | Retrieve concise, connected evidence for an AI |
| `guide` | Explain the tool, record format and writing conventions |
| `dashboard` | Browse the collection in a local web interface |

```sh
# Find current decisions about a topic.
vrdx list --status accepted --tag performance
vrdx search cache --field title

# Understand a decision and its history.
vrdx show ID
vrdx relations ID
vrdx chain ID

# Review suggestions before adding any relationship.
vrdx suggest ID --limit 5

# Check the collection and export the derived graph.
vrdx validate
vrdx rebuild --json
```

Read `vrdx COMMAND --help` for options. Existing records are edited directly in Markdown; vrdx does not automatically change statuses or insert suggested links.

## Give an AI useful evidence

Start with the built-in guide, which works even before a collection exists:

```sh
vrdx guide --json
vrdx context "How should reads be cached?" --json
vrdx suggest ID --limit 5 --json
vrdx show ID --json
```

`guide` explains how to use the CLI and write a decision: state one concrete choice, explain its context and alternatives, and name both benefits and costs. The [AI guide](docs/ai-guide.md) describes the full workflow and how to handle source evidence.

`context` ranks local records by question terms, includes their immediate neighbors and complete replacement chains, and returns status, applicability, tags, paths and reasoning excerpts. Truncation is explicit. Use `show` to read full reasoning before relying on a shortened excerpt.

`suggest` surfaces candidate connections using deterministic local evidence and explains each match. Suggestions are review prompts, not new graph edges. Relevance is lexical; it is not proof that a decision applies.

CLI results support a stable JSON envelope:

```json
{"schema_version":1,"ok":true,"data":{}}
```

Errors are structured too. Scripts should check the exit status and `ok`, then consume `data`; see [JSON fields, errors and ordering](docs/format.md#machine-readable-output). Markdown bodies are evidence to assess, never instructions for an agent to execute.

## Build and contribute

The project pins **stable Rust 1.97.1** and uses **mise**, **cargo-nextest**, and **Dagger with the Dang SDK**. Application code and build tooling require no Python.

| Command | Runs |
| --- | --- |
| `mise run build` | Optimized Rust binary in `target/release/vrdx` |
| `mise run check` | Type-check all targets and features |
| `mise run clippy` | Strict Clippy with no warnings |
| `mise run test` | Tests through nextest |
| `mise run test-doc` | Rust documentation tests |
| `mise run ci-native` | All quality gates on the host, including installed-binary checks |
| `mise run ci` | The same gates in Linux through Dagger/Dang |
| `mise run package` | Verify the distributable Cargo package |

Dagger needs a container engine; native checks do not. On macOS, the verified local route is Colima:

```sh
colima start --runtime docker
docker context use colima
docker info
mise run ci
```

Mise installs project tools; mise itself and the container engine are host prerequisites. The Dagger pipeline uses a digest-pinned mise image, pinned Rust/nextest tools and project-scoped Cargo caches. No Dagger Cloud account or token is required. GitHub Actions runs Linux through Dagger and retains a separate native macOS job.

Keep changes focused, use Conventional Commits, and record lasting choices in [`decisions/`](decisions/). Tests exercise the CLI, Markdown parsing, identity preservation, lifecycle validation, replacement chains and deterministic retrieval. The [OpenSpec specifications](openspec/specs/) document the behavioral contract.

The simulation suite runs a 221-record, 12-topic CLI/HTTP journey and three
replayable seeds with 48 edits each. It checks renames, status/tag changes,
relationships, invalid edits, repairs, retrieval and preservation of source files.
It also runs against installed and extracted release binaries. This measures
modeled workflow behavior; it does not establish real adoption or semantic
retrieval quality. To retain the narrative fixture for browser inspection, run
`VRDX_SIMULATION_KEEP_DIR=/tmp/vrdx-example mise exec -- cargo test --test simulation mixed_project_history`
with a new or empty target directory, then use `vrdx --dir /tmp/vrdx-example dashboard`.

Release PR checks build native archives for all four supported targets. To
package a local build, first run `mise exec -- cargo build --locked --release --target TARGET`,
then `mise run package-release -- TARGET` with your host's target from the table
above. Packaging extracts the archive and exercises its binary outside the
checkout. A matching `vVERSION` tag publishes the verified archives and checksums
only after all four platform jobs pass.

## Scope

One flat collection per command. No automatic legacy migration, Git automation, semantic search or web editing. Existing Markdown is never rewritten by the CLI. The local dashboard is a viewer, not a hosted collaboration service.

The former terminal editor and `vrdx agent` interface have been replaced. Their history remains in Git. See [format details and portability limits](docs/format.md#portability-and-boundaries) before integrating another tool.

## License

vrdx is distributed under the [MIT license](LICENSE). Your decision records
remain your own content; the software license does not assign a license to them.
