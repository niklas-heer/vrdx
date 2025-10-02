build:
    uv run python -m nuitka --standalone --onefile --output-filename=vrdx --output-dir=bin main.py

lint:
    uv run ruff check .

test:
    uv run --with pytest pytest
