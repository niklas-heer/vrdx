# vrdx

Keep engineering decisions beside the code, in ordinary Markdown. vrdx is a Rust terminal application for macOS and Linux, pronounced “verd-ex.”

Browse a repository's Markdown files, create and edit structured decisions, and preview a record before saving. Files remain readable in your editor and reviewable in Git. Saves preserve content outside the decision being edited, check for external changes, and replace files atomically.

## Install and run

Install [mise](https://mise.jdx.dev/installing-mise.html) and your platform's native build tools (Xcode Command Line Tools on macOS; a C compiler and linker on Linux), then:

```sh
git clone https://github.com/niklas-heer/vrdx.git
cd vrdx
mise trust
mise install
mise exec -- cargo install --locked --path .
vrdx /path/to/project
```

Ensure Cargo's installation directory (normally `~/.cargo/bin`) is on your `PATH`. No Python environment or runtime is needed for the installed binary. This checkout is the source of the Rust version; the historical PyPI package is the previous Python implementation.

```sh
vrdx                 # Current directory
vrdx /path/to/repo
vrdx --help
vrdx --version
```

## Working with decisions

The interface has decisions and files on the left, with an editor and preview on the right. It supports an 80 × 24 terminal and expands with larger windows.

Select a Markdown file and press `n` to add a decision. If the file has no decision block, confirm its creation. Choose a status, enter a title and any supporting fields, then press `Ctrl+s`. Use `Enter` or `Space` to edit an existing record. A save failure leaves your draft available for correction or retry. In an empty repository, `n` offers to create `DECISIONS.md`; cancellation creates no file. Save/Discard/Stay protects drafts when navigating away. At 80 × 24, use `Alt+4` while editing to show the preview.

| Key | Action |
| --- | --- |
| `1`, `2`, `3`, `4` | Focus decisions, files, editor, preview while browsing |
| `Alt+1` … `Alt+4` | Switch panes while editing |
| `j` / `k`, arrow keys | Navigate the focused list |
| `Enter` / `Space` | Edit the selected decision |
| `n` | Create a decision |
| `d` | Delete the selected decision; only `y` confirms, `Enter`/`Esc` cancel |
| `J` / `K` | Move the selected decision down / up without changing its ID |
| `/` | Search decisions across the repository; type to filter and `Enter` to select |
| `t` | Create a decision using a repository template |
| `l` | Follow decision relationships; `a` opens the target picker to add one |
| `h` | Browse Git revisions and inspect the selected decision at a revision |
| `Tab` / `Shift+Tab` | Move between editor fields and Save/Cancel |
| `Ctrl+s` | Save the draft |
| `Alt+m` | Merge and save independent local and external field changes |
| `Esc` | Cancel editing |
| `r` | Refresh repository files |
| `?` | Show help |
| `q` | Quit while browsing |
| `Ctrl+q` | Quit, with an unsaved-draft check |
| `Ctrl+c` | Interrupt and restore the terminal |

Search covers decision content across files. Search, template, relationship, and history dialogs support arrow-key navigation and `Esc` to return. In history, `n` loads older commits and `p` loads newer ones; historical content supports scrolling and Home/End. History reads Git without checking out a revision or changing the working tree. Moving and deleting records use the same conflict checks as editing. The Status field's `[Change]` control also opens the picker by mouse without losing draft text.

Repository templates live in `.vrdx/templates/NAME.md`. Copy the complete Markdown example below into a template file and customize its fields. Each template must contain exactly one decision; creation assigns a fresh ID. Templates are data and are never executed. Selecting a template opens an editable draft; cancelling writes nothing.

Relationships are ordinary Markdown links in the Context field, using a repository-relative file and stable decision ID, for example `[Decision 1](vrdx:DECISIONS.md#1)`. The picker and headless `link` command validate the target and percent-encode special characters in paths. Following a missing target reports an error. Deletion records a `<!-- vrdx high-water: ID -->` comment before the decision block, so deleted IDs are never reused and a broken link cannot silently point to a replacement record.

## Agent and script interface

`vrdx agent` works without a terminal. Every response is a JSON object with `schema_version: 1` and `ok`; successful command results appear under `data`, and failures under `error` with `code` and `message`. Help returns a `help` string. IDs are decimal strings to avoid precision loss in JSON clients.

```sh
vrdx agent --help
vrdx agent list --root /path/to/project
vrdx agent search --root /path/to/project --query caching
vrdx agent show --root /path/to/project --file DECISIONS.md --id 1
vrdx agent validate --root /path/to/project
```

| Command | Required options, in addition to optional `--root DIRECTORY` |
| --- | --- |
| `list` / `validate` / `templates` | None |
| `search` | `--query TEXT` |
| `show` | `--file FILE`; optional `--id ID` selects one record |
| `create` | `--file FILE --if-match TOKEN`, plus `--record JSON` or `--template NAME` |
| `update` | `--file FILE --id ID --if-match TOKEN --record JSON` |
| `delete` | `--file FILE --id ID --if-match TOKEN` |
| `move` | `--file FILE --id ID --if-match TOKEN --position INDEX` (zero-based final position) |
| `merge` | `--file FILE --id ID --if-match TOKEN --baseline-source TEXT --record JSON` |
| `template` | `--name NAME` |
| `relationships` | `--file FILE --id ID` |
| `link` / `follow` | `--file FILE --id ID --target-file FILE --target-id ID`; `link` also requires `--if-match TOKEN` |
| `history` | `--file FILE`; optional `--offset INDEX --limit COUNT` for paging |
| `history-show` | `--file FILE --id ID --revision COMMIT` |

`show` returns the original Markdown `source` and a SHA-256 `snapshot` token; `list` includes a token with each record. Supply that token as `--if-match` for a mutation. Use `missing` only when creating a file that does not exist. A stale token fails with exit status 3. Paths must be relative to the selected root, and writes cannot escape it through parent components or symlinks.

```sh
vrdx agent create --root /path/to/project --file DECISIONS.md \
  --if-match missing --record '{"title":"Keep decisions beside code"}'
```

Record JSON contains any of `title`, `status`, `decision`, `context`, and `consequences` as strings; omitted values use the current record or draft defaults. An explicitly supplied `id` must match the assigned identity. Unknown or duplicate fields, incorrect types, duplicate options, and unknown options fail. Use `--record-file PATH` instead of `--record JSON`, and `--baseline-file PATH` instead of `--baseline-source TEXT`, for larger inputs. For a merge, retain the original `source` from `show`, read the latest snapshot token, and submit the original source together with your local record edits. Different fields merge automatically; divergent changes to the same field require a human decision.

Exit status is 0 for success, 2 for usage errors, 3 for conflicts, 4 for missing resources, and 1 for other validation, filesystem, or Git errors. `validate` checks documents, templates, and relationship targets. Git history commands require the Git executable and a repository containing the selected directory. History pages default to 200 commits, accept limits from 1 to 200, and return `has_more` and `next_offset` for retrieving older entries.

## Markdown format

Copy this complete block into a Markdown file:

```markdown
<!-- vrdx start -->
### 1 Choose repository-local decision records
* **Status**: ✅ Accepted
* **Decision**: Store decisions alongside the code.
* **Context**: Reviewers need the reasoning behind a change.
* **Consequences**: Decisions are reviewed and versioned in Git.
<!-- vrdx end -->
```

Markers inside fenced or inline code examples are ignored when scanning documentation. In your actual decision block, use bare markers as shown in the example. A file can contain one decision block; malformed markers or records are reported rather than rewritten. Titles and statuses must be nonempty; narrative fields may be empty. Internal paragraph breaks and indentation are preserved. Trailing whitespace that cannot round-trip through the format is rejected with a validation error.

Saves compare the whole loaded file with disk before replacement. A detected external edit leaves the draft intact. `Alt+m` attempts a three-way merge using the original, local, and disk records: independent fields and changes to other records or surrounding text are retained; divergent edits to the same field fail without writing. There is no force overwrite. Independent editors can still race the final check and rename. Existing permission bits are retained, but ACLs, extended attributes, and directory durability after power loss are not guaranteed.

## Development

`mise.toml` pins Rust **nightly-2026-09-06** and the developer tools. The nightly toolchain is an explicit project choice; pinning its date keeps compiler and lint behavior consistent. `rust-toolchain.toml` uses the same date for direct Cargo and editor invocations. Update both pins deliberately and rerun the full checks when upgrading. Stable Rust compatibility is not claimed.

```sh
mise install
mise run check
mise run test
mise run ci
mise run run -- /path/to/project
```

| Task | Purpose |
| --- | --- |
| `mise run fmt` / `fmt-fix` | Check / apply rustfmt |
| `mise run check` | Check all targets and features with Cargo.lock |
| `mise run clippy` | Strict Clippy, with warnings treated as errors |
| `mise run test` | Unit and integration tests through nextest |
| `mise run test-doc` | Cargo documentation tests |
| `mise run test-e2e` | Real terminal tests using a pseudo-terminal |
| `mise run contracts` | Premise named-field round trips, identity, and bulk transport contracts |
| `mise run bench` | Criterion parsing benchmarks for 1, 100 and 1,000 records |
| `mise run dev` | Bacon diagnostics; `c` for Clippy, `t` for tests |
| `mise run watch` | Watch source changes and rerun checks/tests |
| `mise run run -- DIRECTORY` | Launch the application in another terminal |
| `mise run verify-install` | Install temporarily; run headless agent commands and real terminal journeys outside the checkout |
| `mise run package` | Verify a Cargo source package from a clean checkout without publishing |
| `mise run ci` | Formatting, checking, Clippy, tests, doctests, and installed-binary checks |

The watch tasks run checks or tests; they never repeatedly launch the application or take over its terminal. GitHub Actions runs the same CI task on macOS and Linux. Nextest enforces timeouts without retrying failures, so flaky terminal behavior remains visible.

The VS Code workspace settings enable rustfmt and Clippy through rust-analyzer. Other editors can use the pinned toolchain's `rust-analyzer` component.

### Tool and dependency choices

- **Ratatui and Crossterm** provide terminal rendering and input. `color-eyre` reports errors and `signal-hook` supports terminal cleanup on signals.
- **pulldown-cmark**, **tempfile**, and **unicode-width** handle Markdown boundaries, atomic file replacement, and Unicode display width.
- **Premise** supplies named record fields (`Fielded`), keyed lookup (`Keyed`), and attached rationale in the domain core. The JSON interface uses this same representation. Executable contracts check identity, bulk transport, rejected coercions, Unicode, and numeric boundaries. IDs use decimal text to preserve the full `u64` range.
- **serde_json** provides the agent wire format, with **serde** rejecting duplicate input fields; **sha2** provides stable content revision tokens for mutation preconditions.
- **nextest**, **portable-pty**, and **vt100** exercise logic and real terminal behavior; **Criterion** measures parser throughput.
- **bacon** and **watchexec** provide continuous feedback. **cargo-binstall**, **cargo-generate**, and **cargo-seek** are pinned developer utilities for installing binaries, optional scaffolding, and dependency discovery; none is a runtime dependency.

Mise downloads the pinned developer utilities from their upstream binary releases, using cargo-binstall for bacon where a prebuilt binary is available. Dependencies are locked in Cargo.lock. The strict lint policy rejects unsafe code, unchecked indexing, unwraps, panic paths, unfinished implementations, and unchecked arithmetic. Database, web-server, and async-runtime dependencies are unnecessary for this synchronous local-file application.

Premise's library contracts govern the record core. Its separate four-law framework also mandates a different workspace architecture and bans all source comments and Rust documentation comments. This project retains its requested Rust documentation tests, OpenSpec workflow, and standard Rust architecture; that framework is not the application's conformance standard.

## Decision Records

The earlier Python distribution decision below is preserved as historical context. Decision 2 supersedes it for the Rust implementation.

<!-- vrdx start -->
### 2 Distribution Strategy: Native Rust Application and Mise Development Tools
* **Status**: ✅ Accepted
* **Decision**: Supersede decision 1 with a native Rust application distributed through Cargo and built with the pinned nightly toolchain. Manage development tools and tasks through mise.
* **Context**: The project is moving to Rust and Ratatui, with reproducible tooling, strict compiler checks, and real terminal tests requested as part of the migration.
* **Consequences**: The installed application runs without Python. Source builds require the pinned Rust toolchain and native build tools. macOS and Linux remain the supported platforms.

### 1 Distribution Strategy: Use uv for Python-based Distribution Only
* **Status**: ✅ Accepted
* **Decision**: Distribute vrdx exclusively via `uv tool install`. No binary compilation.
* **Context**: Binary compilation adds unnecessary overhead—slow builds (10-45 min), bloated executables (30-800MB), and startup penalty (200-500ms+)—without real benefit. For an I/O-bound TUI app, Python's fast startup (~80-120ms) and `uv`'s seamless distribution better serve both users and development velocity. Target audience (developers) already has Python installed.
* **Consequences**: Zero compilation time, small footprint (~10MB), fast startup, seamless updates—requires Python installed (acceptable tradeoff for target audience).
<!-- vrdx end -->
