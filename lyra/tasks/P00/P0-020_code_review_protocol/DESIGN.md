# Design — P0-020 Code Review Protocol

## Architecture

### Review Tier Model

The review protocol defines three tiers based on crate criticality. Each tier specifies
the minimum number of approving reviewers required before a PR can merge.

```
┌─────────────────────────────────────────────────────────┐
│                   Review Tier Model                     │
├──────────┬──────────────────┬───────────┬───────────────┤
│ Tier     │ Paths            │ Reviewers │ Rationale     │
├──────────┼──────────────────┼───────────┼───────────────┤
│ Critical │ k0/**            │ 2 (dual)  │ Constitutional│
│          │                  │           │ kernel — any  │
│          │                  │           │ invariant     │
│          │                  │           │ violation is  │
│          │                  │           │ catastrophic  │
├──────────┼──────────────────┼───────────┼───────────────┤
│ Standard │ k1/**            │ 1         │ Application   │
│          │ lyralang/**      │           │ layer — bugs  │
│          │ shells/**        │           │ are fixable,  │
│          │ slices/**        │           │ no invariant  │
│          │                  │           │ risk          │
├──────────┼──────────────────┼───────────┼───────────────┤
│ Docs     │ *.md             │ 1         │ Documentation │
│          │ docs/**          │           │ only — no     │
│          │ lyra/tasks/**    │           │ runtime       │
│          │                  │           │ impact        │
└──────────┴──────────────────┴───────────┴───────────────┘
```

### CODEOWNERS Strategy

GitHub CODEOWNERS enforces review requirements at the path level. The file maps directory
globs to GitHub usernames or teams. When a PR modifies files matching a CODEOWNERS pattern,
GitHub automatically requests reviews from the listed owners.

**Key design decisions:**

1. **`k0/` dual-review** — Two distinct reviewers listed for `k0/` paths. Combined with
   the GitHub branch protection rule "Require 2 approving reviews," this enforces dual-review.
2. **Wildcard fallback** — A default `*` rule assigns a single reviewer as a catch-all,
   ensuring no PR escapes review entirely.
3. **CI/workflow protection** — `.github/` changes also require review to prevent CI
   tampering.

### Automated Review Checklist

The PR template (`.github/PULL_REQUEST_TEMPLATE.md`) provides a structured checklist that
GitHub renders as interactive checkboxes. This serves as a lightweight "review gate" that
the author self-certifies before requesting review.

**Checklist sections:**

| Section | Items | Applies To |
|---|---|---|
| CI Gates | Tests pass, clippy clean, fmt check | All PRs |
| Documentation | Docs updated for public API changes | All PRs with API changes |
| Constitutional | Invariants preserved, no ambient nondeterminism | k0/ PRs only |
| Review Tier | Correct number of reviewers requested | All PRs |
| Versioning | Version bump follows VERSIONING.md rules | All PRs with API changes |

### Branch Protection Integration

The review protocol assumes the following GitHub branch protection settings on `main`:

- **Require pull request reviews before merging:** Enabled
- **Required number of approving reviews:** 2 (enforces dual-review for k0 via CODEOWNERS)
- **Dismiss stale pull request approvals when new commits are pushed:** Enabled
- **Require review from Code Owners:** Enabled
- **Require status checks to pass before merging:** Enabled (CI gates from P0-015)

> **Note:** Branch protection settings are configured in the GitHub UI, not in repository
> files. The CODEOWNERS file works in conjunction with the "Require review from Code Owners"
> branch protection setting.

## Design Decisions

1. **Three tiers, not two** — Separating Docs from Standard avoids burdening documentation
   PRs with code-review overhead while still requiring a correctness check.
2. **CODEOWNERS over manual assignment** — Automated reviewer assignment eliminates the
   risk of forgetting to request the right reviewers.
3. **Self-certification checklist** — The PR template checklist is a lightweight gate that
   does not block merging but creates a visible record of what the author verified.
4. **Constitutional invariant callout** — Explicitly listing the four constitutional
   invariants (determinism, canonical serialization, offline-first, no ambient nondeterminism)
   in the checklist ensures reviewers know exactly what to verify for k0 changes.
5. **CI gates in checklist** — Redundant with branch protection status checks, but the
   checklist makes the expectation visible to contributors who may not know the CI setup.
6. **Team-based ownership** — CODEOWNERS uses `@lyra-core` and `@lyra-contributors` team
   handles, allowing team membership changes without editing CODEOWNERS.
