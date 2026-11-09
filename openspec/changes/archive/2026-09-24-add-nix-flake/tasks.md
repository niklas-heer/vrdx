## Implementation
- [x] Add the flake, package definition and lockfile.
- [x] Add the path-filtered Nix build workflow.
- [x] Document Nix installation in the README.

## Verification
- [x] Build and version-check the flake on aarch64-darwin and aarch64-linux.
- [x] Run `nix flake check --all-systems`, nixfmt, actionlint and zizmor.
- [x] Exercise the Nix-built executable outside the checkout.

## Outcomes
- `nix build` passed on aarch64-darwin with Determinate Nix 3.17.2 and on
  aarch64-linux in a `nixos/nix` container with Nix 2.35.2. Both runs used
  nixpkgs-unstable `8825beb` with rustc 1.98.1, and `versionCheckHook` found
  0.4.0. The GitHub workflow covers x86_64-linux.
- `nix flake check --all-systems`, nixfmt, actionlint and zizmor passed.
- Copied into an empty directory, the Nix-built executable ran `guide --json`,
  `init --dry-run`, `new` and `validate` without repository assets.
- nixpkgs-unstable no longer evaluates x86_64-darwin, so the flake omits Intel
  Macs, which Homebrew and the release archives still cover.
