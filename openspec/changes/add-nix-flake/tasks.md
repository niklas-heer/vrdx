## Implementation
- [ ] Add the flake, package definition and lockfile.
- [ ] Add the path-filtered Nix build workflow.
- [ ] Document Nix installation in the README.

## Verification
- [ ] Build and version-check the flake on aarch64-darwin and aarch64-linux.
- [ ] Run `nix flake check --all-systems`, nixfmt, actionlint and zizmor.
- [ ] Exercise the Nix-built executable outside the checkout.
