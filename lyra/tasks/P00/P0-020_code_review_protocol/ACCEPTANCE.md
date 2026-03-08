# Acceptance — P0-020 Code Review Protocol

## Acceptance Criteria

1. **Review tiers defined** — Three review tiers are documented: Critical (`k0/`, dual-review required), Standard (`k1/`, `lyralang/`, `shells/`, `slices/`, single-review required), and Docs (documentation-only changes, single-review required).
2. **Dual-review enforced for k0** — The `.github/CODEOWNERS` file assigns at least two required reviewers (or a team requiring 2 approvals) for all paths under `k0/`.
3. **Single-review for standard crates** — The `.github/CODEOWNERS` file assigns at least one required reviewer for `k1/`, `lyralang/`, `shells/`, and `slices/`.
4. **Review checklist exists** — `.github/PULL_REQUEST_TEMPLATE.md` contains a checklist that PR authors must complete before requesting review.
5. **Checklist covers CI gates** — The PR template checklist includes: tests pass (`cargo test --workspace`), clippy clean (`cargo clippy -- -D warnings`), formatting verified (`cargo fmt --check`).
6. **Checklist covers documentation** — The PR template checklist includes a documentation-updated item for public API changes.
7. **Checklist covers constitutional invariants** — The PR template checklist includes an explicit item for `k0/` changes: "Constitutional invariants preserved (determinism, canonical serialization, offline-first, no ambient nondeterminism)."
8. **CODEOWNERS file exists** — `.github/CODEOWNERS` is present at the repository root `.github/` directory and follows GitHub CODEOWNERS syntax.
9. **PR template is auto-loaded** — `.github/PULL_REQUEST_TEMPLATE.md` is placed in the correct location for GitHub to auto-populate new PRs.
10. **Protocol specification artifact** — `artifacts/code-review-protocol-spec.md` documents the full protocol including tier definitions, reviewer responsibilities, escalation paths, and checklist rationale.

## Verification Method
- Manual inspection of `.github/CODEOWNERS` for correct path-to-reviewer mappings
- Manual inspection of `.github/PULL_REQUEST_TEMPLATE.md` for checklist completeness
- Open a test PR touching `k0/` and verify GitHub requests two reviewers
- Open a test PR touching `k1/` and verify GitHub requests one reviewer
- Confirm PR body is auto-populated with the checklist template

## Evidence Required
- `.github/CODEOWNERS` file contents
- `.github/PULL_REQUEST_TEMPLATE.md` file contents
- `artifacts/code-review-protocol-spec.md` specification document
