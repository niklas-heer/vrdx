<!-- OPENSPEC:START -->
# OpenSpec Instructions

These instructions are for AI assistants working in this project.

Always open `@/openspec/AGENTS.md` when the request:
- Mentions planning or proposals (words like proposal, spec, change, plan)
- Introduces new capabilities, breaking changes, architecture shifts, or big performance/security work
- Sounds ambiguous and you need the authoritative spec before coding

Use `@/openspec/AGENTS.md` to learn:
- How to create and apply change proposals
- Spec format and conventions
- Project structure and guidelines

Keep this managed block so 'openspec update' can refresh the instructions.

<!-- OPENSPEC:END -->
# General conventions

## Commit message format
- All commits MUST follow the [Conventional Commits](https://www.conventionalcommits.org/) specification.
- Format: `<type>[optional scope]: <description>`
- Common types:
  - `feat:` – new feature (triggers MINOR version bump)
  - `fix:` – bug fix (triggers PATCH version bump)
  - `docs:` – documentation changes
  - `test:` – add or update tests
  - `refactor:` – code change that neither fixes a bug nor adds a feature
  - `chore:` – maintenance tasks, dependency updates
  - `ci:` – CI/CD configuration changes
  - `perf:` – performance improvements
- Use `!` after the type or add `BREAKING CHANGE:` footer for breaking changes (triggers MAJOR version bump).
- Examples:
  - `feat(api): add user authentication endpoint`
  - `fix: resolve memory leak in data processor`
  - `docs: update installation instructions`
  - `feat!: remove deprecated API endpoints`

## Using Context7
- Before starting any task, check if you have access to [Context7](https://context7.com/) MCP tools.
- ALWAYS use Context7 automatically for:
  - Code generation and scaffolding
  - Setup and configuration steps
  - Library and API documentation lookups
- Use Context7 MCP tools to resolve library IDs and fetch docs without waiting for explicit requests.
- Context7 provides up-to-date documentation and examples—prefer it over outdated knowledge.
- If Context7 is unavailable, fall back to standard methods and mention it in your response.
<!-- vrdx start -->

<!-- vrdx end -->
