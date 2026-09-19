## ADDED Requirements

### Requirement: Reproducible usage simulation
The project SHALL exercise realistic mixed-topic decision collections and repeatable lifecycle, rename, invalid-edit and recovery sequences through the public executable. Assertions SHALL cover source identity, current applicability, replacement history, evidence retrieval and deterministic results. Failures SHALL identify the scenario or reproducible seed. Reported results SHALL distinguish modeled coverage from real adoption.

#### Scenario: Recover from a broken replacement edit
- **WHEN** an edited collection temporarily contains an inconsistent replacement and the edit is then completed or reverted
- **THEN** validation and evidence queries SHALL reject the invalid state and recover using the same stable record identities

### Requirement: Verified native distribution
The project SHALL distribute versioned Linux and macOS native executable archives with the project license, usage documentation and SHA-256 checksums. Release automation SHALL build from the selected source revision, verify the version and exercise each native executable outside the source checkout before publishing. The documented platform and architecture SHALL match the artifact.

#### Scenario: Install without a Rust toolchain
- **WHEN** a user verifies and extracts a supported release archive
- **THEN** the executable SHALL provide guide, creation, validation and retrieval workflows without a source checkout, database, Python or Rust toolchain
