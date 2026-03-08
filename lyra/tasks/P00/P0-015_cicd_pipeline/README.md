# P0-015 — CI/CD Pipeline

## Mission
Canonical continuous integration and delivery pipeline for the Lyra workspace. Enforces build correctness, test passage, formatting, lint cleanliness, and dependency audit on every push and pull request.

## Scope
- GitHub Actions workflow: build, test, clippy, fmt, cargo-deny
- Workspace-wide and per-crate (k0) test jobs
- Dependency audit via cargo-deny
- Cache strategy for fast CI runs

## Primary Archetype
Process / Tooling

## Primary Ownership Root
`.github/`

## Secondary Touched Roots
`lyra/tasks/`

## Deliverables
- `.github/workflows/ci.yml` — canonical CI workflow
- Task control-plane files, artifacts, and traceability
