# P0-016 — Versioning Strategy

## Mission
Semantic versioning for all crates. Git tagging convention. Changelog generation from structured commit messages. Release branch strategy.

## Scope
- Semantic versioning policy for all workspace crates
- Git tagging convention (`v{MAJOR}.{MINOR}.{PATCH}`)
- Structured commit message format (Conventional Commits)
- Changelog generation from commit history
- Release branch strategy and merge policy
- Version bump automation guidance

## Primary Archetype
Infrastructure / Release

## Primary Ownership Root
`ops/` (policy documents), repository root (CHANGELOG.md, VERSIONING.md)

## Secondary Touched Roots
`lyra/tasks/`

## Deliverables
- `VERSIONING.md` — canonical versioning policy document
- `CHANGELOG.md` — initial changelog
- `CONTRIBUTING.md` — commit message and release conventions
- Task control-plane files, artifacts, and traceability
