from __future__ import annotations

import sys

from vrdx.cli import main as cli_main


def main() -> int:
    """Entrypoint for vrdx when invoked as a module or script."""
    return cli_main()


if __name__ == "__main__":
    sys.exit(main())
