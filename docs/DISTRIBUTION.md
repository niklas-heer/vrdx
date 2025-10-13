# Distribution Strategy

## Overview

vrdx is distributed **exclusively as a Python application** via `uv tool install`. No binary compilation (Nuitka, PyInstaller, etc.) is provided or supported.

## Installation

### End Users

Install vrdx globally using uv:

```bash
uv tool install vrdx
```

This command:
- Creates an isolated virtual environment for vrdx
- Installs all dependencies (~10 MB total)
- Adds the `vrdx` command to your PATH
- Handles updates seamlessly with `uv tool upgrade vrdx`

### Developers

Clone and set up for development:

```bash
git clone https://github.com/niklas-heer/vrdx.git
cd vrdx
uv sync
just dev  # Launches with hot-reload
```

## Why uv-only Distribution?

See Decision Record #1 in the README for the full rationale. Key points:

### Rejected: Binary Compilation (Nuitka, PyInstaller)

- **Startup time penalty**: 200-500ms+ vs ~80-120ms for Python
- **Compilation overhead**: 10-45 minutes build time
- **Binary bloat**: 30-800 MB vs ~10 MB for Python + deps
- **Development friction**: Breaks Textual hot-reload
- **Maintenance burden**: Multi-platform CI/CD complexity
- **No real benefit**: All tools bundle Python runtime anyway

### Benefits of uv Distribution

- ✅ Zero compilation time
- ✅ Small installation footprint (~10 MB)
- ✅ Fast startup (~80-120 ms)
- ✅ Preserves development velocity
- ✅ Standard Python packaging workflow
- ✅ Seamless updates via `uv tool upgrade`

## Configuration Verification

### Package Configuration (`pyproject.toml`)

```toml
[project]
name = "vrdx"
version = "0.1.0"
requires-python = ">=3.13"

[project.scripts]
vrdx = "vrdx.cli:main"

[build-system]
requires = ["hatchling"]
build-backend = "hatchling.build"
```

**Key points**:
- Lightweight `hatchling` build backend (no compilation)
- Simple entry point to `vrdx.cli:main`
- No compilation tools in dependencies

### Development Workflow (`justfile`)

```bash
just install      # uv sync
just dev          # Textual hot-reload
just test         # pytest
just install-tool # uv tool install .
```

**Key points**:
- All commands use `uv run` or `uv sync`
- No build/compile/bundle targets
- Development mode preserves hot-reload

### CI/CD (`.github/workflows/ci.yml`)

```yaml
jobs:
  test:
    steps:
      - uses: astral-sh/setup-uv@v1
      - run: uv sync
      - run: uv run --with pytest pytest
```

**Key points**:
- Tests only, no build/release steps
- No binary compilation in CI
- Fast feedback loop

## Distribution Checklist

- [x] No Nuitka configuration
- [x] No PyInstaller configuration
- [x] No cx_Freeze, py2exe, py2app references
- [x] `pyproject.toml` uses hatchling only
- [x] Entry point defined as `vrdx = "vrdx.cli:main"`
- [x] `justfile` uses `uv tool install`
- [x] README documents `uv tool install vrdx`
- [x] CI/CD runs tests only (no builds)
- [x] Decision record exists in README
- [x] `.gitignore` excludes build artifacts

## Testing Installation

Verify the package installs correctly:

```bash
# From source directory
uv tool install .

# Verify it works
vrdx --version

# Test basic functionality
vrdx .
```

## Supported Platforms

- **macOS**: Primary development platform ✅
- **Linux**: Officially supported ✅
- **Windows**: Not currently supported (out of scope)

## Future Considerations

This distribution strategy is **final** for vrdx. Binary compilation will not be revisited unless:
- Python startup time regresses significantly (unlikely)
- Target audience changes to non-developers (scope change)
- uv tool installation becomes problematic (very unlikely)

For now, the uv-only approach provides the best balance of:
- User experience (fast install, fast startup)
- Developer experience (hot-reload, rapid iteration)
- Maintenance burden (minimal CI/CD complexity)
- Distribution size (small footprint)

## Troubleshooting

### Issue: "Multiple vrdx marker blocks detected" Error

**Symptom**: The app crashes when scanning documentation files that contain marker strings in example code or documentation.

**Cause**: The parser detects `&lt;!-- vrdx start -->` and `&lt;!-- vrdx end -->` strings in documentation/example text as actual markers.

**Solution**: 
1. Escape marker strings in documentation using HTML entities:
   - Replace `&lt;!-- vrdx start -->` with `&lt;!-- vrdx start -->`
   - Replace `&lt;!-- vrdx end -->` with `&lt;!-- vrdx end -->`

2. Or run vrdx on a different directory that doesn't contain documentation files:
   ```bash
   vrdx /path/to/target/directory
   ```

3. Consider adding `.vrdxignore` support in future versions to exclude specific files or directories.

### Issue: Slow Startup or Large Installation

**Symptom**: Installation takes a long time or uses excessive disk space.

**Expected**: 
- Installation: < 30 seconds
- Disk space: ~10 MB with dependencies
- Startup time: ~80-120 ms

**If experiencing issues**:
- Ensure you're using `uv tool install` (not pip)
- Check for stale virtual environments: `uv cache clean`
- Verify uv version: `uv --version` (should be 0.1.0+)

### Issue: Command Not Found After Installation

**Symptom**: Running `vrdx` returns "command not found"

**Solution**:
1. Verify installation: `uv tool list | grep vrdx`
2. Check PATH includes uv's bin directory:
   - macOS/Linux: `~/.local/bin` should be in PATH
   - Add to `.zshrc` or `.bashrc`: `export PATH="$HOME/.local/bin:$PATH"`
3. Reload shell: `source ~/.zshrc` or restart terminal

### Getting Help

For additional support:
- Check the README.md for usage examples
- Review `docs/DESIGN.md` for architecture details
- Enable debug logging: `vrdx --log-level DEBUG --log-file vrdx.log`
- Open an issue on GitHub with logs and error messages

## References

- Decision Record #1 in README.md
- [uv documentation](https://docs.astral.sh/uv/)
- [hatchling documentation](https://hatch.pypa.io/latest/)
- Design doc: `docs/DESIGN.md` (Section 7: Packaging Workflow)
- Implementation doc: `docs/IMPLEMENTATION.md` (Section 4: Dependencies)
<!-- vrdx start -->

<!-- vrdx end -->
