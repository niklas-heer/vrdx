# vrdx
CLI tool for capturing and managing decision records in your repo. <br>
Pronounced “verd-ex.” Derived from “verdict”.

## Why vrdx

-    Short, modern, and designed for everyday terminal use
-    Helps you keep decisions consistent, searchable, and reviewable

## Features

-    Add decisions with status, context, and consequences
-    List and search decision records
-    Show a single decision by ID or title
-    Append to a single log file or create per-record files
-    Configurable templates and storage paths
-    Zero-dependency text format (Markdown)

## Usage

### Quick Start

```bash
# Launch vrdx in the current directory
vrdx .

# Or specify a directory
vrdx /path/to/project
```

### TUI Interface

The vrdx TUI has four main panes:

1. **Decisions List** (left, top) - Shows all decisions in the current file
2. **Files List** (left, bottom) - Shows all markdown files with decisions
3. **Editor** (center) - Edit decision content
4. **Preview** (right) - Rendered markdown preview

### Keyboard Shortcuts

#### Normal Mode

| Key | Action |
|-----|--------|
| `1` | Focus decisions list |
| `2` | Focus editor |
| `3` | Focus preview |
| `4` | Focus files list |
| `j` or `↓` | Navigate to next decision |
| `k` or `↑` | Navigate to previous decision |
| `space` | Edit selected decision (enters EDIT mode) |
| `n` | Create new decision (enters INSERT mode) |
| `r` | Refresh file list |
| `?` | Show help |
| `q` | Quit |

#### Edit Mode (INSERT/EDIT)

| Key | Action |
|-----|--------|
| `ctrl+s` or `s` | Save decision |
| `esc` | Cancel editing and return to NORMAL mode |
| `tab` | Navigate between form fields |

### Mode Indicators

The status bar shows the current mode (vim-style):

- `-- NORMAL --` - Browse and navigate decisions
- `-- INSERT (New) --` - Creating a new decision (form-based editor active)
- `-- EDIT --` - Editing an existing decision (form-based editor active)

### Form-Based Editor Features

The form-based editor provides:
- **Inline validation**: Title is required and cannot be placeholder text
- **Status dropdown**: Select from all available statuses without modal disruption
- **Auto-load**: The most recent decision is automatically loaded for editing when you start or select a decision
- **Keyboard shortcuts**: 
  - `ctrl+s` to save
  - `esc` to cancel
  - `tab` to navigate between fields

### Creating a New Decision

1. Press `n` to create a new decision
2. A status selection modal appears - select the initial status using arrow keys and press `Enter`
3. The form-based editor opens with all fields ready for input:
   - **Title**: Required - give your decision a clear title
   - **Status**: Use the dropdown to select status (📝 Draft, ✅ Accepted, ❌ Rejected, 🤔 Pending, etc.)
   - **Decision**: What was decided
   - **Context**: Why this decision was needed
   - **Consequences**: What are the implications
4. Fill in the fields - only the title is required
5. Press `ctrl+s` or click Save to persist the decision
6. Press `esc` to cancel without saving

The form uses a scrollable container, so if your terminal is small, you can scroll through the fields.

### Decision Format

Decisions are stored in markdown with the following format:

```markdown
`<!-- vrdx start -->`
### 1. Decision Title

* **Status**: ✅ Accepted
* **Decision**: What was decided
* **Context**: Why this decision was needed
* **Consequences**: What are the implications
`<!-- vrdx end -->`
```
Inline marker examples can be safely written with single backticks; the parser ignores `<!-- vrdx start -->` and `<!-- vrdx end -->` when they appear inside inline code spans, so documentation can reference the delimiters without disrupting indexing.


## Install

### Using uv (recommended)

```bash
uv tool install vrdx
```

### For development

```bash
git clone https://github.com/niklas-heer/vrdx.git
cd vrdx
uv sync

# Run with hot-reload
just dev

# Or run normally
just run
```

## Decision Records

<!-- vrdx start -->

### 1 Distribution Strategy: Use uv for Python-based Distribution Only
* **Status**: ✅ Accepted
* **Decision**: Distribute vrdx exclusively via `uv tool install`. No binary compilation.
* **Context**: Binary compilation adds unnecessary overhead—slow builds (10-45 min), bloated executables (30-800MB), and startup penalty (200-500ms+)—without real benefit. For an I/O-bound TUI app, Python's fast startup (~80-120ms) and `uv`'s seamless distribution better serve both users and development velocity. Target audience (developers) already has Python installed.
* **Consequences**: Zero compilation time, small footprint (~10MB), fast startup, seamless updates—requires Python installed (acceptable tradeoff for target audience).

<!-- vrdx end -->
