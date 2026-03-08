# Dependency Audit Specification — Evidence Artifact

## Audit Identity
- **Configuration file:** `deny.toml`
- **Inventory file:** `DEPS.md`
- **Task:** P0-021
- **Tool:** cargo-deny

## Audit Dimensions

### 1. Advisory Scanning
| Setting | Value | Effect |
|---|---|---|
| `db-path` | `~/.cargo/advisory-db` | Local advisory database cache |
| `db-urls` | RustSec advisory-db | Upstream advisory source |
| `vulnerability` | `deny` | Known vulnerabilities cause hard failure |
| `unmaintained` | `warn` | Unmaintained crates flagged, non-blocking |
| `yanked` | `warn` | Yanked versions flagged, non-blocking |
| `notice` | `warn` | Informational advisories, non-blocking |

### 2. License Policy
| Setting | Value | Effect |
|---|---|---|
| `unlicensed` | `deny` | Crates with no license are rejected |
| `confidence-threshold` | `0.8` | License detection must be 80%+ confident |
| `allow` | 8 licenses | Only listed SPDX identifiers permitted |

**Allowed Licenses:**
1. MIT
2. Apache-2.0
3. BSD-2-Clause
4. BSD-3-Clause
5. ISC
6. Zlib
7. Unicode-3.0
8. Unicode-DFS-2016

### 3. Ban Policy
| Setting | Value | Effect |
|---|---|---|
| `multiple-versions` | `warn` | Duplicate crate versions produce warnings |
| `wildcards` | `allow` | Wildcard version specs permitted |
| `highlight` | `all` | Full dependency path shown for duplicates |

### 4. Source Restrictions
| Setting | Value | Effect |
|---|---|---|
| `unknown-registry` | `deny` | Only crates.io permitted |
| `unknown-git` | `deny` | No git dependencies from unknown origins |
| `allow-registry` | crates.io index | Explicit crates.io allowance |
| `allow-git` | (empty) | No git source overrides |

## Workspace Dependency Inventory

| # | Crate | Version | License | Category |
|---|---|---|---|---|
| 1 | sha3 | 0.10 | MIT OR Apache-2.0 | Cryptographic |
| 2 | blake3 | 1 | MIT OR Apache-2.0 | Cryptographic |
| 3 | serde | 1 | MIT OR Apache-2.0 | Serialization |
| 4 | serde_json | 1 | MIT OR Apache-2.0 | Serialization |
| 5 | thiserror | 1 | MIT OR Apache-2.0 | Error Handling |
| 6 | anyhow | 1 | MIT OR Apache-2.0 | Error Handling |
| 7 | tracing | 0.1 | MIT | Logging |
| 8 | proptest | 1 | MIT OR Apache-2.0 | Testing |
| 9 | criterion | 0.5 | MIT OR Apache-2.0 | Benchmarking |

## CI Integration

- **Workflow:** `.github/workflows/ci.yml`
- **Job:** `deny`
- **Command:** `cargo deny check advisories licenses sources`
- **Status:** Blocking gate (after P0-021 completion, `continue-on-error` removed)
- **Triggers:** push to `main`/`blackboxai/**`, pull request to `main`

## Acceptance Verification

All 10 acceptance criteria from ACCEPTANCE.md are satisfied:

1. `deny.toml` exists at repository root ✅
2. `[advisories]` configures `db-path` and `vulnerability = "deny"` ✅
3. `[licenses]` defines allowlist with 8 SPDX identifiers ✅
4. `[licenses]` sets `unlicensed = "deny"` ✅
5. `[bans]` sets `multiple-versions = "warn"` ✅
6. `[sources]` sets `unknown-registry = "deny"` and `unknown-git = "deny"` ✅
7. `DEPS.md` exists at repository root with version, license, and purpose ✅
8. `DEPS.md` covers all 9 workspace dependencies ✅
9. `cargo deny check` configuration is valid and enforceable ✅
10. CI pipeline `deny` job references finalized `deny.toml` ✅

## Traceability
- **Upstream dependency:** P0-004 (Repo Architecture — workspace structure)
- **Integration dependency:** P0-015 (CI/CD Pipeline — deny job)
- **Downstream consumers:** All future dependency additions must pass this audit
