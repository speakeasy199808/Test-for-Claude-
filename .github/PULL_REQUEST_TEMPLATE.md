## Description

<!-- Provide a clear summary of the changes. Reference the task ID (e.g., P0-020). -->

**Task ID:** P0-XXX
**Review Tier:** <!-- Critical (k0) | Standard (k1/lyralang/shells/slices) | Docs -->

---

## Changes Made

<!-- List the key changes in this PR. -->

- 

---

## Review Checklist

### CI Gates (all PRs)

- [ ] `cargo fmt --all -- --check` passes (no formatting issues)
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes (no warnings)
- [ ] `cargo build --workspace` succeeds
- [ ] `cargo test --workspace` passes (all tests green)

### Documentation (if public API changed)

- [ ] Doc comments updated for new/modified public items
- [ ] Module-level documentation reflects changes
- [ ] CHANGELOG.md updated (if user-facing change)

### Versioning (if API changed)

- [ ] Version bump follows rules in VERSIONING.md
- [ ] Breaking changes use `!` suffix in commit type (e.g., `feat(k0)!:`)

### Constitutional Invariants (k0/ changes ONLY)

> **If this PR modifies any file under `k0/`, ALL items below MUST be checked.**
> Skip this section entirely if the PR does not touch `k0/`.

- [ ] **Determinism preserved** — identical inputs always produce identical outputs
- [ ] **No ambient nondeterminism** — no `std::time::SystemTime`, no `std::time::Instant`, no unseeded `rand`, no `HashMap` iteration-order dependence
- [ ] **Canonical serialization preserved** — all encoding remains deterministic and versioned
- [ ] **Offline-first maintained** — core truth requires no network access
- [ ] **`#![forbid(unsafe_code)]`** remains in place
- [ ] **`#![deny(missing_docs)]`** remains in place
- [ ] **k0 unit tests pass** — `cargo test -p k0 --lib` (all tests green)

### Review Tier Compliance

- [ ] Correct number of reviewers requested per review tier:
  - **Critical (k0/):** 2 reviewers from `@lyra-core`
  - **Standard (k1/, lyralang/, shells/, slices/):** 1 reviewer
  - **Docs (*.md, lyra/tasks/):** 1 reviewer

---

## Testing

<!-- Describe how you tested these changes. -->

- [ ] Existing tests pass
- [ ] New tests added (if applicable)
- [ ] Manual verification performed (describe below if applicable)

**Test command(s) run:**
```bash
cargo test --workspace
```

---

## Additional Notes

<!-- Any additional context, screenshots, or concerns for reviewers. -->

