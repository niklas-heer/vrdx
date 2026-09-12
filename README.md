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
| `Tab` / `Shift+Tab` | Move between editor fields and Save/Cancel |
| `Ctrl+s` | Save the draft |
| `Esc` | Cancel editing |
| `r` | Refresh repository files |
| `?` | Show help |
| `q` | Quit while browsing |
| `Ctrl+q` | Quit, with an unsaved-draft check |
| `Ctrl+c` | Interrupt and restore the terminal |

Search commands, configurable templates, and automatic Git operations are not currently available.

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

Saves compare the whole loaded file with disk before replacement. A detected external edit requires cancelling/reloading and reapplying the draft; there is no force overwrite. Independent editors can still race the final check and rename. Existing permission bits are retained, but ACLs, extended attributes, and directory durability after power loss are not guaranteed.

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
| `mise run bench` | Criterion parsing benchmarks for 1, 100 and 1,000 records |
| `mise run dev` | Bacon diagnostics; `c` for Clippy, `t` for tests |
| `mise run watch` | Watch source changes and rerun checks/tests |
| `mise run run -- DIRECTORY` | Launch the application in another terminal |
| `mise run verify-install` | Install temporarily; check CLI startup and real terminal journeys outside the checkout |
| `mise run package` | Verify a Cargo source package from a clean checkout without publishing |
| `mise run ci` | Formatting, checking, Clippy, tests, doctests, and installed-binary checks |

The watch tasks run checks or tests; they never repeatedly launch the application or take over its terminal. GitHub Actions runs the same CI task on macOS and Linux. Nextest enforces timeouts without retrying failures, so flaky terminal behavior remains visible.

The VS Code workspace settings enable rustfmt and Clippy through rust-analyzer. Other editors can use the pinned toolchain's `rust-analyzer` component.

### Tool and dependency choices

- **Ratatui and Crossterm** provide terminal rendering and input. `color-eyre` reports errors and `signal-hook` supports terminal cleanup on signals.
- **pulldown-cmark**, **tempfile**, and **unicode-width** handle Markdown boundaries, atomic file replacement, and Unicode display width.
- **Premise** supplies named record fields (`Fielded`), keyed lookup (`Keyed`), and attached rationale in the domain core. IDs use decimal text to preserve the full `u64` range. This integration does not claim conformance to the upstream four-law checker.
- **nextest**, **portable-pty**, and **vt100** exercise logic and real terminal behavior; **Criterion** measures parser throughput.
- **bacon** and **watchexec** provide continuous feedback. **cargo-binstall**, **cargo-generate**, and **cargo-seek** are pinned developer utilities for installing binaries, optional scaffolding, and dependency discovery; none is a runtime dependency.

Mise downloads the pinned developer utilities from their upstream binary releases, using cargo-binstall for bacon where a prebuilt binary is available. Dependencies are locked in Cargo.lock. The strict lint policy rejects unsafe code, unchecked indexing, unwraps, panic paths, unfinished implementations, and unchecked arithmetic. Database, web-server, and async-runtime dependencies are unnecessary for this synchronous local-file application.

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
