# Acceptance — P0-021 Dependency Audit

## Acceptance Criteria
1. `deny.toml` exists at the repository root and is valid cargo-deny configuration.
2. `[advisories]` section configures `db-path` and sets `vulnerability = "deny"` to block known vulnerabilities.
3. `[licenses]` section defines an explicit allowlist: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, Zlib, Unicode-3.0, Unicode-DFS-2016.
4. `[licenses]` section sets `unlicensed = "deny"` to reject crates with no declared license.
5. `[bans]` section sets `multiple-versions = "warn"` to flag duplicate dependency versions without blocking.
6. `[sources]` section sets `unknown-registry = "deny"` and `unknown-git = "deny"` to restrict dependency origins.
7. `DEPS.md` exists at the repository root and lists all workspace dependencies with version, license, and purpose.
8. `DEPS.md` covers all 9 workspace dependencies: sha3, blake3, serde, serde_json, thiserror, anyhow, tracing, proptest, criterion.
9. `cargo deny check advisories licenses sources` passes without error (or only warnings for `multiple-versions`).
10. The CI pipeline `deny` job in `.github/workflows/ci.yml` references the finalized `deny.toml` configuration.

## Verification Method
- `cargo deny check advisories licenses sources` — passes with exit code 0
- Manual review of `deny.toml` against declared license allowlist
- Manual review of `DEPS.md` for completeness and accuracy
- CI pipeline `deny` job transitions from `continue-on-error: true` to blocking

## Evidence Required
- `deny.toml`
- `DEPS.md`
- `artifacts/dependency-audit-spec.md`
