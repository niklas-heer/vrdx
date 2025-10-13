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
| `s` | Save decision |
| `esc` | Cancel editing and return to NORMAL mode |
| `p` | Cycle through status options |

### Mode Indicators

The status bar shows the current mode (vim-style):

- `-- NORMAL --` - Browse and navigate decisions
- `-- INSERT (New) --` - Creating a new decision
- `-- EDIT --` - Editing an existing decision

### Creating a New Decision

1. Press `n` to create a new decision
2. The editor opens with a helpful template including placeholders
3. Fill in the title, decision, context, and consequences
4. Press `p` to cycle through status options (📝 Draft, ✅ Accepted, ❌ Rejected, etc.)
5. Press `s` to save the decision
6. Press `esc` to cancel without saving

### Decision Format

Decisions are stored in markdown with the following format:

```markdown
&lt;!-- vrdx start -->

### 1. Decision Title

* **Status**: ✅ Accepted
* **Decision**: What was decided
* **Context**: Why this decision was needed
* **Consequences**: What are the implications

&lt;!-- vrdx end -->
```

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

### 1. Distribution Strategy: Use uv for Python-based Distribution Only

* **Status**: ✅ Accepted
* **Decision**: Distribute vrdx exclusively as a Python application via `uv tool install`. No binary compilation will be provided or supported.
* **Context**: Binary compilation (Nuitka, PyInstaller) was evaluated but rejected for multiple reasons:
  - **Startup time penalty**: Compiled binaries have noticeable startup overhead (200-500ms+) compared to native Python (~80-120ms), which degrades the interactive TUI experience
  - **Compilation overhead**: Build times range from 10-45 minutes depending on tooling and dependencies
  - **Binary bloat**: Executables range from 30-800 MB vs ~10 MB for the Python application with dependencies
  - **Development friction**: Compilation breaks Textual's hot-reload workflow (`textual run --dev`), making iteration painfully slow
  - **Maintenance burden**: Supporting multiple binary targets (macOS, Linux) adds CI/CD complexity without meaningful benefit
  - **No real standalone**: All compilation tools bundle a Python runtime anyway
  
  Modern Python distribution via `uv` is elegant, fast, and increasingly standard. Tools like `ruff`, `uv` itself, and many CLI tools successfully ship this way. The `uv tool install` command handles virtual environments, dependencies, and PATH setup automatically—providing an excellent user experience without any compilation overhead.
  
  For vrdx specifically:
  - Application is I/O-bound (Markdown parsing, file operations) where Python excels
  - Fast startup is critical for interactive TUI responsiveness
  - Development velocity matters more than theoretical "standalone" benefits
  - Target audience (developers) already has Python installed
  - Python's text processing ecosystem is a strength, not a weakness
* **Consequences**:
  - **Positive**:
    - Zero compilation time (instant `uv tool install` from source)
    - Small installation footprint (~10 MB with dependencies)
    - Fast startup (~80-120 ms, adequate for interactive TUI)
    - Preserves Textual hot-reload for rapid development
    - Users with Python installed get seamless updates
    - Standard Python packaging workflow (easier CI/CD)
  - **Negative**:
    - Users must have Python installed (acceptable tradeoff—Python is ubiquitous on Unix-like systems and required by our target audience)
  - **Mitigation**:
    - Provide clear installation instructions for `uv tool install`
    - Document that macOS and Linux are officially supported platforms
    - Maintain fast startup times and excellent developer experience

<!-- vrdx end -->
