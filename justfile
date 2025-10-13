# Install dependencies
install:
    uv sync

# Run in development mode with hot reload
dev:
    uv run textual run --dev vrdx/ui/app.py:VrdxApp

# Run with Python hot reload (restarts on code changes)
dev-watch:
    uv run ptw --runner "textual run --dev vrdx/ui/app.py:VrdxApp"

# Run the application normally
run:
    uv run vrdx

# Run with a specific directory
run-dir DIR:
    uv run vrdx {{DIR}}

# Run tests
test:
    uv run pytest -v

# Run tests in watch mode
test-watch:
    uv run ptw

# Run CI tests across Linux distros with Earthly
ci-linux:
    earthly +ubuntu +alpine +centos

# Run linter
lint:
    uv run ruff check .

# Fix linting issues automatically
lint-fix:
    uv run ruff check --fix .

# Install as a tool (for end users)
install-tool:
    uv tool install .

# Clean build artifacts
clean:
    rm -rf __pycache__ .pytest_cache
    find . -type d -name "*.egg-info" -exec rm -rf {} + 2>/dev/null || true
    find . -type d -name "__pycache__" -exec rm -rf {} + 2>/dev/null || true

# Open Textual console for debugging
console:
    uv run textual console

# Show help for debugging workflow
debug:
    @echo "To debug with Textual console:"
    @echo "  1. Run 'just console' in one terminal"
    @echo "  2. Run 'just dev' in another terminal"
    @echo "  3. View live logs in the console terminal"

# Show all available commands
help:
    @just --list
