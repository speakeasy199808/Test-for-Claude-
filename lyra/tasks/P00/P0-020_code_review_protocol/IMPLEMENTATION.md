# Implementation — P0-020 Code Review Protocol

## Summary
Established a tiered code review protocol for the LyraOS repository. Created CODEOWNERS
for automated reviewer assignment with mandatory dual-review for the constitutional kernel
(`k0/`), and a PR template with a comprehensive review checklist covering CI gates,
documentation, constitutional invariants, and versioning.

## Files Created

1. **`.github/CODEOWNERS`** — GitHub CODEOWNERS file mapping crate paths to required
   reviewers. `k0/` paths require two reviewers (`@lyra-core` team members); all other
   crate paths require one reviewer; `.github/` workflow changes require core team review.

2. **`.github/PULL_REQUEST_TEMPLATE.md`** — Pull request template auto-loaded by GitHub
   for all new PRs. Contains an interactive checklist covering:
   - CI gate verification (tests, clippy, formatting)
   - Documentation updates
   - Constitutional invariant preservation (k0-specific)
   - Review tier compliance
   - Versioning rules

3. **`lyra/tasks/P00/P0-020_code_review_protocol/artifacts/code-review-protocol-spec.md`**
   — Full protocol specification documenting tier definitions, reviewer responsibilities,
   escalation paths, checklist rationale, and integration with branch protection settings.

## Files Modified

- No existing files were modified. This task is purely additive — it introduces new
  process files without changing any Rust source code or existing configuration.

## Review Tier Summary

| Tier | Crate Paths | Required Approvals | Reviewer Assignment |
|---|---|---|---|
| Critical | `k0/**` | 2 | `@lyra-core` (dual) |
| Standard | `k1/**`, `lyralang/**`, `shells/**`, `slices/**` | 1 | `@lyra-contributors` |
| Docs | `*.md`, `docs/**`, `lyra/tasks/**` | 1 | `@lyra-contributors` |

## Verification

- `.github/CODEOWNERS` exists and follows GitHub CODEOWNERS syntax
- `.github/PULL_REQUEST_TEMPLATE.md` exists at the correct path for auto-population
- PR template contains all required checklist items
- CODEOWNERS maps `k0/` to two reviewers for dual-review enforcement
- CODEOWNERS maps standard crates to single reviewer
- Constitutional invariant checklist item is present for k0 changes
- Protocol specification artifact is comprehensive and self-contained
