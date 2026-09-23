## ADDED Requirements
### Requirement: Source-built Nix flake
The repository SHALL provide a Nix flake whose default package builds the vrdx executable from the checked-out source for aarch64-darwin, aarch64-linux, and x86_64-linux. The package SHALL take its version from `Cargo.toml`, pin its nixpkgs input in `flake.lock`, and verify that the installed executable reports that version. Changes to the flake or Cargo manifests SHALL trigger a Linux flake build in CI.

#### Scenario: Install with Nix
- **WHEN** a user with flakes enabled runs `nix profile add github:niklas-heer/vrdx` on a supported system
- **THEN** Nix SHALL build and install a `vrdx` executable that reports the version in `Cargo.toml`

#### Scenario: Flake drift
- **WHEN** a manifest change prevents the flake from building
- **THEN** the Nix workflow SHALL fail
