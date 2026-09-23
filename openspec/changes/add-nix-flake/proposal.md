# Install vrdx through a Nix flake

## Why
Niklas asked on 2026-09-24 for a Nix flake in every tool repository that
ships Linux packages. Nix users would then have a declarative install path
alongside Homebrew and the release archives.

## What Changes
- Add `flake.nix` and `nix/package.nix`. They build vrdx from source with
  `rustPlatform.buildRustPackage` and `cargoLock.lockFile`, take the version
  from `Cargo.toml`, and use a locked `nixpkgs-unstable`. That branch is the
  first to provide a rustc at or above the pinned 1.97.1. The flake targets
  aarch64-darwin, aarch64-linux, and x86_64-linux; nixpkgs-unstable no longer
  supports x86_64-darwin.
- The package build skips the test suite, which existing CI owns, and runs
  `vrdx --version` as its install check.
- Add a path-filtered `.github/workflows/nix.yml` that builds the flake when
  the flake, `Cargo.toml`, or `Cargo.lock` changes.
- Document `nix profile add github:niklas-heer/vrdx` in the README as a build
  of the latest `main`, and ignore the `result` link that `nix build` creates.

## Authorization and boundaries
The 2026-09-24 request authorizes the flake, its documentation, and the build
check. The change does not modify release automation, Homebrew, the binary,
or its runtime behaviour. It adds no nixpkgs submission or binary cache. The
cross-repository rationale is the hub decision "Ship source-built Nix flakes
for released CLI tools".

## Impact
New `flake.nix`, `flake.lock`, `nix/package.nix`, `.github/workflows/nix.yml`,
README install section, and `.gitignore`. Rust and Dagger checks are unchanged.
