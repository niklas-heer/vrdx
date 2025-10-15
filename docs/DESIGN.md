# vrdx Design Proposal

## 1. Goals and Non-Goals

### Goals
- Deliver a standalone CLI/TUI experience reminiscent of lazygit for managing decision records.
- Discover Markdown files (`*.md`) in the current working directory.
- Parse decision content delimited by `<!-- vrdx start -->` and `<!-- vrdx end -->`.
- Provide an interface to browse files, inspect decisions, and compose new ones.
- Leverage `uv` for dependency management and modern Python distribution via `uv tool install`.

### Non-Goals (Initial Iteration)
- Automatic Git operations such as committing or staging.
- Web interfaces or remote execution.
- Supporting decision capture in non-Markdown formats.
- Editing content outside the decision marker blocks.

---

## 2. High-Level Architecture

1. **CLI Entry:** bootstraps the application, handles command-line flags.
2. **App Runner:** initializes the TUI framework and global state.
3. **File Discovery Layer:** scans for Markdown files, supports refresh.
4. **Document Parser:** extracts decisions between markers, maintains raw text.
5. **Decision Index:** structures parsed decisions for navigation.
6. **App State:** holds selected file, active decision, editor buffer, pane focus.
7. **UI Layer:** renders panes (file list, decision list, preview, editor) with lazygit-inspired key bindings.
8. **Persistence Layer:** writes updated decisions back to the appropriate marker region.

---

## 3. Technology Choices

| Concern | Choice | Rationale |
| --- | --- | --- |
| TUI Framework | Textual | Rich widget system, async-friendly, supports complex layouts. |
| Markdown Parsing | Targeted parsing around markers plus light-weight Markdown parsing (e.g., markdown-it-py) | Ensures reliable extraction while keeping dependencies manageable. |
| Data Modeling | Optional dataclasses or pydantic models | Enforces consistent decision schema, simplifies validation. |
| Dependency Management | uv | Matches the project’s existing workflow and provides reproducible environments. |
| Packaging | uv + hatchling | Modern Python distribution exclusively via `uv tool install`. |

---

## 4. Feature Breakdown

### 4.1 File Discovery
- Recursively locate `*.md` files starting from the current working directory.
- Automatically skip hidden files and directories (those starting with `.`).
- Ignore common development directories (`.git`, `.venv`, `venv`, `env`, `__pycache__`, `node_modules`, `.pytest_cache`, `.ruff_cache`, `.mypy_cache`, `.tox`, `.eggs`, `dist`, `build`, `.idea`, `.vscode`).
- Support manual refresh via key binding.
- Filtering behavior can be customized via `DiscoveryConfig` in future iterations.

### 4.2 Decision Extraction
- Identify decision blocks by locating `<!-- vrdx start -->` and `<!-- vrdx end -->` markers in each Markdown file.
- When a file lacks markers, prompt the user before inserting the canonical block; with confirmation, append a scaffold that includes the start marker, a blank line, and the end marker so subsequent decisions are captured between `<!-- vrdx start -->` and `<!-- vrdx end -->`.
- Within the block, treat each `###` heading as the start of a decision.
- Capture metadata (ID, title, status, decision, context, consequences) for indexing and editing.

### 4.3 UI Layout
- **Left Column (Panes 1 & 4):** Narrow column that stacks Pane 1 (decision index) above Pane 4 (Markdown file list); numeric keys (`1` and `4`) jump directly to each pane, and the active pane is highlighted with a distinct accent color.
- **Center Column (Pane 2):** Primary editor surface occupying the majority of the width for composing or updating the active decision.
- **Right Column (Pane 3):** Read-only preview of the decision rendered as Markdown.
- `space` within the focused pane triggers its primary action (such as selecting a decision or confirming prompts).
- Status bar displays the save state (Saved/Modified), the active pane, and a right-aligned key hint strip that highlights `[n]` for starting a new entry.
- Layout fits within an 80×24 terminal and gracefully expands when additional space is available.

```
┌[1] Decisions──────┬[2] Decision Editor─────────────────────────┬[3] Preview──┐
│↑/↓ j/k • space    │### 14 New Tool Evaluation                  │read-only    │
│13 Stk Amethyst    │* Status: 📝 Draft                          │─────────────│
│12 Reject Doom     │* Decision:                                 │###13 preview│
│11 Adopt Hammer    │* Context:                                  │* Status: ✅ │
│10 Adopt Ghostty   │* Consequences:                             │* Decision:… │
│ 9 Adopt oh-posh   │                                            │* Context: … │
│[4] Files ─────────│                                            │* Conseq.: … │
│README.md          │                                            │             │
│docs/arch.md       │                                            │             │
│notes/setup.md     │                                            │             │
│⋮                  │                                            │             │
└───────────────────┴────────────────────────────────────────────┴─────────────┘
● Unsaved • Active Pane: 2 Editor     [space] select [n]ew [s]ave [q]uit [?]help
```

### 4.4 Interaction Model
- Pane focus is controlled via numeric shortcuts (`1`–`4`), and the active pane is highlighted with an accent color.
- `space` triggers the primary action within the focused pane (e.g., select a decision or confirm a prompt).
- `?` opens an overlay listing all key bindings.
- Item navigation with `j`/`k` or the arrow keys.
- `n` opens the editor with a new-decision template.
- `e` allows editing the currently selected decision.
- `J`/`K` move the selected decision down or up to reorder within the marker block.
- `d` deletes the selected decision after confirmation.
- `s` saves changes, writing back to the decision block.
- `r` refreshes file discovery.
- When editing the status field, a picker presents the allowed status values for quick selection.

### 4.5 Decision Template
- Default template populated with the next decision ID and placeholder fields:
  - Heading format `### <ID> <Title>`
  - Bullet list entries for Status, Decision, Context, Consequences.
- Status defaults to `📝 Draft`, and the editor’s status picker offers the curated set: `📝 Draft`, `✅ Accepted`, `❌ Rejected`, `⛔ Deprecated by …`, `⬆️ Supersedes …`.
- When a user chooses `⛔ Deprecated by …` or `⬆️ Supersedes …`, companion links are updated automatically to keep cross references consistent.

### 4.6 Persistence
- Insert new decisions at the top of the decision block (descending order by ID) so the most recent entries appear first.
- Preserve existing file content outside markers, and re-create the canonical scaffold if the markers were removed (prompting first).
- Support reordering by rewriting the block according to the in-app ordering.
- Support deletion by removing the selected decision while leaving surrounding content intact.
- Maintain blank line separation for readability.

### 4.7 Marker Management
- Marker blocks are defined by the canonical delimiters `<!-- vrdx start -->` and `<!-- vrdx end -->`; all decision content lives between them.
- Inline examples wrapped in single backticks (e.g., `` `<!-- vrdx start -->` ``) are ignored by discovery and parsing so documentation can reference the markers without disrupting indexing.
- When no markers are present, the application prompts the user before appending an empty scaffold (start marker, blank line, end marker) at the end of the file.
- If malformed or duplicate markers are detected, the parser surfaces a clear error message in the UI and logging output so users can resolve the issue before continuing.
- The persistence layer preserves the host file’s newline convention when inserting scaffolds on Unix-like systems (macOS and Linux), which are the officially supported platforms; Windows carriage-return handling is currently out of scope.
- Future configuration may allow custom marker strings, but the default behaviour assumes the canonical markers for maximum interoperability.

### 4.8 Decision Parsing
- Each decision entry is detected via the canonical `### <ID> <Title>` heading followed by the four bullet-labelled fields (`Status`, `Decision`, `Context`, `Consequences`). The parser tolerates additional blank lines but requires the canonical labels to guarantee a consistent round-trip format.
- Parsed data is materialised into a structured `DecisionRecord` (powered by Pydantic) that retains ID, title, status, narrative fields, and the raw markdown snippet, enabling edits, reordering, and faithful re-serialization.
- Multiline field bodies are preserved verbatim; when decisions are rendered back into the marker block, the serializer emits the canonical layout using the file’s prevailing newline sequence to reduce diff churn.
- Parser utilities raise descriptive `DecisionParseError` exceptions when required fields are missing or malformed so the UI can surface actionable errors before any writes occur.
- Helper functions compute the next decision identifier and integrate with the template/status helpers to seed new entries with curated status options and sensible defaults.

### 4.9 State Management and Command Operations
- The application tracks context in an `AppState` aggregate that records the discovered files, the currently selected file and decision, the active pane, and whether there are unsaved changes. Each markdown file is represented by a `FileState` that stores parsed `DecisionRecord` instances, marker metadata, and convenience helpers like “next decision ID”.
- Individual decisions are wrapped by `DecisionState`, which also collects relationship metadata (supersedes/deprecated links) so cross-references remain consistent while the user navigates or edits records.
- Command helpers manipulate these structures in one place: creating and updating decisions, reordering or deleting entries, managing reciprocal links, and moving selection between panes. Each mutating command uses the shared serialization routines to keep the marker block body ready for persistence and toggles the `is_modified` flag so the status bar can signal pending changes.

### 4.10 Textual UI Skeleton
- The first Textual iteration renders the four-pane layout described earlier: a narrow left column that stacks the decision list above the markdown file list, a dominant editor pane centered on the screen, and a preview pane to the right. Each pane is addressable by numeric shortcuts (`1`–`4`) and clearly labelled in the header bar.
- Navigation bindings mirror the lazygit-inspired interaction model (`j/k` or arrow keys for movement, `space` to activate a selection, `?` for the key reference overlay), while the footer reflects the current save state indicator (`● Saved` / `● Unsaved`) and the most important actions (`[n]ew`, `[s]ave`, `[q]uit`, `[?]help`).
- The editor pane now leverages Textual's `TextArea`, remaining read-only while browsing but switching to an editable buffer when a user activates `space` or drafts a new decision via `n`, seeding the template directly into the cursor.
- The preview pane and state wiring continue to stay in sync so that moving between files or decisions immediately updates both editor and preview content; in this milestone the `p` binding simply cycles through the curated status set and `s` writes the updated decision back to the underlying markdown file. A richer status picker, Markdown-rendered preview, and in-app reordering/deletion flows remain on the roadmap for the next iteration.

---

## 5. Data Structures

- `Decision` structure storing ID, title, status emoji/text, narrative sections, raw Markdown, and optional `supersedes` / `deprecated_by` references.
- `AppState` maintaining:
  - Ordered file list.
  - Selected file and decision indexes.
  - Decision cache per file.
  - Current editor buffer and mode (new vs. edit).
  - Active pane tracking.
  - Allowed status definitions and their formatting rules.

---

## 6. Configuration & Extensibility

- Future `.vrdx.toml` or similar config for custom markers, templates, status values (including cross-link behavior), and ignore patterns.
- CLI arguments for working directory overrides (`vrdx --dir path`), log verbosity, or read-only mode.
- Logging subsystem for diagnostics (e.g., writing to a cache directory).
- Optional Textual developer tooling (such as future `textual-devtools` builds) can be installed manually without expanding the default dependency set.

---

## 7. Packaging Workflow

1. Use `uv` to sync dependencies (`uv sync`).
2. Provide `just` recipes:
   - `just install` to run `uv sync`.
   - `just run` for `uv run vrdx`.
   - `just dev` for development mode with Textual hot-reload (`textual run --dev`).
   - `just test` for running pytest.
   - `just install-tool` for installing as a user tool (`uv tool install .`).
3. Distribution is exclusively via `uv tool install` for seamless installation without compilation overhead.

---

## 8. Testing Strategy

- Unit tests for decision parsing and serialization (pytest-based).
- Snapshot tests for the new-decision template and insertion ordering.
- TUI component tests leveraging Textual’s testing utilities.
- Integration tests operating on fixture Markdown files to validate end-to-end flow.

---

## 9. Design Decisions (Resolved)

1. New decisions are inserted at the top of the marker block so the most recent entries are immediately visible.
2. When a user selects a file without markers, the app prompts to insert the markers before proceeding, avoiding silent modifications.
3. Status values come from the curated list (`📝 Draft`, `✅ Accepted`, `❌ Rejected`, `⛔ Deprecated by …`, `⬆️ Supersedes …`) and are selectable via the status picker, which also maintains reciprocal links for deprecated/supersedes pairs.
4. Existing decisions can be edited in place (fields only), reordered, or deleted directly from the TUI.
5. All Markdown files, including top-level `README.md`, are eligible for decision management.

---

## 10. Next Steps

1. Confirm answers to the open questions above.
2. Finalize technology stack decisions (e.g., Textual vs. alternative).
3. Define the key bindings and navigation schema in detail.
4. Outline the module structure (`vrdx/ui`, `vrdx/parser`, `vrdx/state`, etc.).
5. Prepare sample Markdown fixtures to guide implementation and testing.
<!-- vrdx start -->

<!-- vrdx end -->
