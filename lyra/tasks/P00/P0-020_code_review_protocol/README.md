# P0-020 — Code Review Protocol

## Mission
Establish code review requirements scaled by crate criticality. Enforce mandatory dual-review
for all changes to `k0/` (the constitutional kernel). Provide automated review checklist
enforcement via PR templates and CODEOWNERS to guarantee that no change to safety-critical
code merges without appropriate human oversight.

## Scope
- Define three review tiers: Critical (k0), Standard (k1, lyralang, shells, slices), and Docs
- Enforce mandatory dual-review for all `k0/` pull requests via GitHub CODEOWNERS
- Create a PR template with an automated review checklist covering tests, clippy, docs,
  and constitutional invariant preservation
- Establish CODEOWNERS file mapping crate paths to required reviewers
- Document the review protocol so all contributors understand the expectations

## Primary Archetype
Interface/Bridge

## Primary Ownership Root
`k0/`

## Secondary Ownership Roots
- `.github/` — CODEOWNERS and PR template
- `lyra/tasks/` — task documentation

## Deliverables
- `.github/CODEOWNERS` — ownership and review requirements per crate path
- `.github/PULL_REQUEST_TEMPLATE.md` — PR checklist template with tier-aware review guidance
- `lyra/tasks/P00/P0-020_code_review_protocol/artifacts/code-review-protocol-spec.md` — full protocol specification

## Dependencies
- **P0-004** (Repo Architecture) — crate layout must be established
- **P0-015** (CI/CD Pipeline) — CI gates referenced in review checklist
- **P0-016** (Versioning Strategy) — version bump rules referenced in checklist

## Context
The Lyra system treats `k0/` as the constitutional kernel. Changes to `k0/` can violate
determinism, canonical serialization, or offline-first invariants. A single reviewer may miss
subtle invariant violations. Dual-review for `k0/` ensures that at least two humans verify
constitutional compliance before any kernel change reaches `main`. Standard crates (`k1`,
`lyralang`, `shells`, `slices`) carry lower risk and require single-reviewer approval.
Documentation-only changes require a single reviewer for correctness but have no
constitutional implications.
