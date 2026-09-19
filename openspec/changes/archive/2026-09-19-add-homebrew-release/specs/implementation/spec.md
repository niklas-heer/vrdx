## ADDED Requirements
### Requirement: Tested Homebrew distribution
The release SHALL be installable through the existing niklas-heer/tap/vrdx formula using checksummed native archives. Automated formula updates SHALL select a published stable release, require all supported archives and valid checksums, and pass native installation tests before updating the tap. Tag publication SHALL require the full quality workflow and all native archive checks.

#### Scenario: Homebrew installation
- **WHEN** a user installs niklas-heer/tap/vrdx on a supported platform
- **THEN** Homebrew SHALL install the matching verified executable without a Rust toolchain

#### Scenario: Failed release or formula checks
- **WHEN** a quality, packaging or native formula test fails
- **THEN** the corresponding publication SHALL not proceed
