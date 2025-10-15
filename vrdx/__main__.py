"""Entry point for running vrdx as a module with `python -m vrdx`."""

from __future__ import annotations

import sys

from vrdx.cli import main

if __name__ == "__main__":
    sys.exit(main())
