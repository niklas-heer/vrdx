#!/bin/sh
set -eu

project_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
smoke_dir=$(mktemp -d "${TMPDIR:-/tmp}/vrdx-install.XXXXXX")
trap 'rm -rf "$smoke_dir"' EXIT HUP INT TERM

cargo install --locked --path "$project_dir" --root "$smoke_dir/install"
mkdir "$smoke_dir/project"
cd "$smoke_dir/project"
"$smoke_dir/install/bin/vrdx" --help
"$smoke_dir/install/bin/vrdx" --version
# CLI errors must be readable and must not enter the alternate terminal screen.
if "$smoke_dir/install/bin/vrdx" "$smoke_dir/missing" > "$smoke_dir/error.txt" 2>&1; then
  printf '%s\n' 'Expected a missing directory to fail' >&2
  exit 1
fi
test -s "$smoke_dir/error.txt"
# The terminal harness launches this installed binary in its own temporary project.
VRDX_TEST_BINARY="$smoke_dir/install/bin/vrdx" cargo nextest run \
  --locked --manifest-path "$project_dir/Cargo.toml" --test terminal
printf '%s\n' 'Installed binary smoke test passed outside the source checkout.'
