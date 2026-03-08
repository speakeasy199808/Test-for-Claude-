# P0-021 — Dependency Audit

## Mission
Automated supply-chain audit for all Rust dependencies in the Lyra workspace. Establishes license compatibility policy, vulnerability scanning via advisory databases, source-origin restrictions, and auto-generated dependency inventory (`DEPS.md`). This task finalizes the `cargo-deny` configuration referenced by the CI/CD pipeline (P0-015) and transitions the `deny` CI job from non-blocking to blocking.

## Scope
- `deny.toml` configuration at repository root for `cargo-deny`
- License allowlist policy: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib, Unicode-3.0, Unicode-DFS-2016
- Advisory database vulnerability scanning (deny on known vulnerabilities)
- Source-origin restrictions (deny unknown registries and unknown git sources)
- Duplicate dependency detection (warn on multiple versions)
- `DEPS.md` dependency inventory with version, license, and purpose for each workspace dependency
- CI integration: `cargo deny check` runs as a blocking gate

## Primary Archetype
Verification / Proof

## Primary Ownership Root
`k0/`

## Secondary Touched Roots
- `lyra/tasks/`
- Repository root (`deny.toml`, `DEPS.md`)
- `.github/` (CI pipeline integration)

## Deliverables
- `deny.toml` — cargo-deny configuration with license, advisory, ban, and source policies
- `DEPS.md` — auto-generated dependency inventory for all workspace dependencies
- `lyra/tasks/P00/P0-021_dependency_audit/artifacts/dependency-audit-spec.md` — evidence artifact
- Task control-plane files and traceability
