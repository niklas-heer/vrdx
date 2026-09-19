#!/bin/sh
set -eu

target=${1:-}
case "$target" in
  x86_64-unknown-linux-gnu|aarch64-unknown-linux-gnu|x86_64-apple-darwin|aarch64-apple-darwin) ;;
  *)
    printf 'usage: %s <supported-rust-target>\n' "$0" >&2
    exit 2
    ;;
esac

project_dir=$(CDPATH= cd -- "$(dirname -- "$0")/.." && pwd)
version=$(awk '
  /^\[package\]$/ { package = 1; next }
  /^\[/ { package = 0 }
  package && /^version = "/ {
    value = $0
    sub(/^version = "/, "", value)
    sub(/".*$/, "", value)
    print value
    exit
  }
' "$project_dir/Cargo.toml")

if [ -z "$version" ]; then
  printf '%s\n' 'Could not read the package version from Cargo.toml' >&2
  exit 1
fi

binary="$project_dir/target/$target/release/vrdx"
if [ ! -x "$binary" ]; then
  printf 'Release binary is missing or not executable: %s\n' "$binary" >&2
  exit 1
fi

archive_name="vrdx-v${version}-${target}"
dist_dir="$project_dir/dist"
stage_dir=$(mktemp -d "${TMPDIR:-/tmp}/vrdx-package.XXXXXX")
smoke_dir=$(mktemp -d "${TMPDIR:-/tmp}/vrdx-release-smoke.XXXXXX")
trap 'rm -rf "$stage_dir" "$smoke_dir"' EXIT HUP INT TERM

mkdir -p "$dist_dir" "$stage_dir/$archive_name"
cp "$binary" "$stage_dir/$archive_name/vrdx"
cp "$project_dir/README.md" "$project_dir/LICENSE" "$stage_dir/$archive_name/"
chmod 755 "$stage_dir/$archive_name/vrdx"

COPYFILE_DISABLE=1 tar -czf "$dist_dir/$archive_name.tar.gz" -C "$stage_dir" "$archive_name"
tar -xzf "$dist_dir/$archive_name.tar.gz" -C "$smoke_dir"

cd "$smoke_dir"
"$smoke_dir/$archive_name/vrdx" --help >/dev/null
test "$("$smoke_dir/$archive_name/vrdx" --version)" = "vrdx $version"
if "$smoke_dir/$archive_name/vrdx" --dir "$smoke_dir/missing" list >error.txt 2>&1; then
  printf '%s\n' 'Expected the extracted binary to reject a missing collection directory' >&2
  exit 1
fi
test -s error.txt
# Serialize this cargo-test harness: a concurrent fork can briefly inherit the
# relocation fixture's writable executable handle and cause Linux ETXTBSY.
# Native CI uses nextest's separate processes for parallel test isolation.
VRDX_TEST_BINARY="$smoke_dir/$archive_name/vrdx" cargo test \
  --locked \
  --manifest-path "$project_dir/Cargo.toml" \
  --test authoring --test cli \
  --test ai \
  --test dashboard \
  --test simulation -- --test-threads=1

printf 'Packaged and verified %s\n' "$dist_dir/$archive_name.tar.gz"
