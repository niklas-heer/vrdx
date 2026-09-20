## MODIFIED Requirements

### Requirement: Measured standalone CLI workflows
The application SHALL remain one native executable with bundled dashboard, guidance and agent skill assets, requiring no runtime interpreter or external AI service. Verification SHALL cover structured and human workflows through the executable outside the checkout. A reproducible opt-in latency benchmark SHALL report release-command timings and workload size without treating machine-specific measurements as universal guarantees.

#### Scenario: Relocated executable
- **WHEN** only the executable is copied into an otherwise empty working directory
- **THEN** guide, prompt, JSON creation, validation, formatting and skill installation SHALL work without accessing repository assets or invoking a runtime interpreter
