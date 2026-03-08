# Code Review Protocol Specification — Evidence Artifact

## Protocol Identity
- **Task:** P0-020
- **Title:** Code Review Protocol
- **Archetype:** Interface/Bridge
- **Status:** Complete

---

## 1. Purpose

This document specifies the code review protocol for the LyraOS repository. The protocol
ensures that all changes receive appropriate human review before merging to `main`, with
heightened scrutiny for the constitutional kernel (`k0/`).

The protocol addresses three risks:
1. **Constitutional invariant violation** — a change to `k0/` breaks determinism, canonical
   serialization, offline-first guarantees, or introduces ambient nondeterminism.
2. **Quality regression** — a change introduces bugs, missing docs, or untested code paths.
3. **Process bypass** — a change merges without any review, circumventing team oversight.

---

## 2. Review Tier Definitions

### 2.1 Tier: Critical

| Property | Value |
|---|---|
| **Paths** | `k0/**` |
| **Required Approvals** | 2 (dual-review) |
| **Reviewer Pool** | `@lyra-core/kernel-reviewers`, `@lyra-core/kernel-reviewers-2` |
| **Rationale** | `k0/` is the constitutional kernel. Any invariant violation is catastrophic and may be subtle. Two independent reviewers reduce the probability of missing a violation. |

**Reviewer responsibilities for Critical tier:**
- Verify all four constitutional invariants are preserved:
  1. Determinism — identical inputs produce identical outputs
  2. No ambient nondeterminism — no wall clock, no unseeded randomness, no HashMap iteration
  3. Canonical serialization — encoding is deterministic and versioned
  4. Offline-first — core truth requires no network access
- Verify `#![forbid(unsafe_code)]` and `#![deny(missing_docs)]` remain in place
- Run `cargo test -p k0 --lib` locally before approving
- Check that new types implement required derives (Serialize, Deserialize, Debug, etc.)
- Verify no new dependencies are added without justification

### 2.2 Tier: Standard

| Property | Value |
|---|---|
| **Paths** | `k1/**`, `lyralang/**`, `shells/**`, `slices/**` |
| **Required Approvals** | 1 (single-review) |
| **Reviewer Pool** | `@lyra-contributors` |
| **Rationale** | Application-layer crates. Bugs are fixable without constitutional risk. Single review balances quality with velocity. |

**Reviewer responsibilities for Standard tier:**
- Verify tests pass and new code has test coverage
- Check clippy compliance and documentation
- Verify the change follows Conventional Commits
- Check for unnecessary dependencies

### 2.3 Tier: Docs

| Property | Value |
|---|---|
| **Paths** | `*.md`, `docs/**`, `lyra/tasks/**`, `interfaces/**` |
| **Required Approvals** | 1 (single-review) |
| **Reviewer Pool** | `@lyra-contributors` |
| **Rationale** | Documentation-only changes have no runtime impact. Single review ensures correctness and consistency. |

**Reviewer responsibilities for Docs tier:**
- Verify factual accuracy
- Check formatting and link validity
- Ensure consistency with existing documentation style

---

## 3. CODEOWNERS Configuration

### 3.1 File Location
`.github/CODEOWNERS`

### 3.2 Pattern Precedence
GitHub CODEOWNERS uses last-match-wins precedence. Patterns are ordered from least specific
(default `*` catch-all) to most specific (individual crate paths).

### 3.3 Pattern Mapping

| Pattern | Owner(s) | Tier | Effect |
|---|---|---|---|
| `*` | `@lyra-contributors` | Default | Catch-all: no PR escapes review |
| `/k1/` | `@lyra-contributors` | Standard | Single review for k1 |
| `/lyralang/` | `@lyra-contributors` | Standard | Single review for lyralang |
| `/shells/` | `@lyra-contributors` | Standard | Single review for shells |
| `/slices/` | `@lyra-contributors` | Standard | Single review for slices |
| `/k0/` | `@lyra-core/kernel-reviewers` `@lyra-core/kernel-reviewers-2` | Critical | Dual review for k0 |
| `/.github/` | `@lyra-core` | Infra | Core team reviews CI changes |
| `/.github/workflows/` | `@lyra-core` | Infra | Core team reviews workflow changes |
| `/lyra/tasks/` | `@lyra-contributors` | Docs | Single review for task docs |
| `*.md` | `@lyra-contributors` | Docs | Single review for markdown |
| `/Cargo.toml` | `@lyra-core` | Infra | Core team reviews workspace config |
| `/Cargo.lock` | `@lyra-core` | Infra | Core team reviews lockfile changes |

### 3.4 Team Structure

| Team Handle | Purpose | Minimum Members |
|---|---|---|
| `@lyra-core` | Core maintainers with constitutional knowledge | 2 |
| `@lyra-core/kernel-reviewers` | First k0 reviewer slot | 1+ |
| `@lyra-core/kernel-reviewers-2` | Second k0 reviewer slot | 1+ |
| `@lyra-contributors` | All active contributors | 1+ |

---

## 4. PR Template Checklist

### 4.1 File Location
`.github/PULL_REQUEST_TEMPLATE.md`

### 4.2 Checklist Sections

| Section | Applies To | Items |
|---|---|---|
| CI Gates | All PRs | fmt check, clippy, build, test |
| Documentation | PRs with API changes | doc comments, module docs, CHANGELOG |
| Versioning | PRs with API changes | version bump, breaking change annotation |
| Constitutional Invariants | k0/ PRs only | determinism, no ambient nondeterminism, canonical serialization, offline-first, forbid(unsafe_code), deny(missing_docs), k0 unit tests |
| Review Tier Compliance | All PRs | correct reviewer count requested |
| Testing | All PRs | existing tests pass, new tests added |

### 4.3 Checklist Rationale

The checklist serves as a **self-certification gate**. It does not block merging
programmatically (that is the role of CI status checks and branch protection). Instead,
it creates a visible, auditable record of what the PR author verified before requesting
review. This reduces reviewer burden by ensuring basic hygiene is already confirmed.

The constitutional invariants section is intentionally verbose. Each invariant is listed
explicitly so that authors working on `k0/` cannot overlook any of them. The cost of
checking a box is low; the cost of missing an invariant violation is high.

---

## 5. Branch Protection Integration

The review protocol requires the following GitHub branch protection settings on `main`
to be fully effective:

| Setting | Value | Purpose |
|---|---|---|
| Require pull request reviews before merging | Enabled | No direct pushes to main |
| Required number of approving reviews | 2 | Enforces dual-review for k0 (CODEOWNERS assigns 2 owners) |
| Dismiss stale pull request approvals | Enabled | Force re-review after new commits |
| Require review from Code Owners | Enabled | CODEOWNERS patterns are enforced, not advisory |
| Require status checks to pass | Enabled | CI gates (P0-015) must pass |
| Required status checks | `build-and-test`, `test-k0`, `deny` | All three CI jobs must succeed |
| Require branches to be up to date | Enabled | PR must be rebased on latest main |
| Include administrators | Enabled | Admins cannot bypass review requirements |

> **Note:** These settings are configured in the GitHub repository settings UI under
> Settings > Branches > Branch protection rules > `main`. They are not stored in
> repository files but are documented here for completeness.

---

## 6. Escalation Paths

### 6.1 Disagreement Between Reviewers
If two reviewers on a Critical-tier PR disagree, the PR is blocked until consensus is
reached. Either reviewer may escalate to the full `@lyra-core` team for a tiebreaker
discussion.

### 6.2 Emergency Bypass
In an emergency (e.g., security vulnerability fix), a single `@lyra-core` member may
approve a Critical-tier PR with the following conditions:
1. The PR description includes `EMERGENCY:` prefix and justification
2. A follow-up review is scheduled within 24 hours
3. The emergency bypass is logged in the PR comments

### 6.3 Reviewer Unavailability
If no reviewer from the required pool is available within 48 hours:
1. The PR author pings the `@lyra-core` team in a comment
2. Any `@lyra-core` member may substitute as reviewer
3. For Critical-tier PRs, the substitute must still verify all constitutional invariants

---

## 7. Review Workflow

### 7.1 Author Workflow
1. Create a feature branch from `main` (see CONTRIBUTING.md for branch naming)
2. Make changes following project conventions
3. Run CI gates locally:
   ```bash
   cargo fmt --all -- --check
   cargo clippy --workspace --all-targets -- -D warnings
   cargo test --workspace
   ```
4. For k0 changes, also run: `cargo test -p k0 --lib`
5. Open a PR — the template auto-populates the checklist
6. Complete all applicable checklist items
7. GitHub auto-assigns reviewers via CODEOWNERS
8. Address reviewer feedback, push updates
9. After required approvals, squash-merge or rebase-merge to `main`

### 7.2 Reviewer Workflow
1. Receive review request (auto-assigned via CODEOWNERS)
2. Check the PR checklist — verify the author completed applicable items
3. Review the diff:
   - For Critical tier: verify all four constitutional invariants
   - For Standard tier: verify tests, docs, and code quality
   - For Docs tier: verify accuracy and formatting
4. Leave comments or approve
5. If changes requested, re-review after author updates

---

## 8. Acceptance Verification

All 10 acceptance criteria from ACCEPTANCE.md are satisfied:

1. **Review tiers defined** — Three tiers documented in this specification (Section 2)
2. **Dual-review for k0** — CODEOWNERS assigns two reviewer teams for `/k0/` (Section 3)
3. **Single-review for standard crates** — CODEOWNERS assigns `@lyra-contributors` (Section 3)
4. **Review checklist exists** — `.github/PULL_REQUEST_TEMPLATE.md` created (Section 4)
5. **Checklist covers CI gates** — fmt, clippy, build, test items present
6. **Checklist covers documentation** — doc comments and CHANGELOG items present
7. **Checklist covers constitutional invariants** — all four invariants listed explicitly
8. **CODEOWNERS file exists** — `.github/CODEOWNERS` created with correct syntax
9. **PR template auto-loaded** — placed at `.github/PULL_REQUEST_TEMPLATE.md`
10. **Protocol specification artifact** — this document
