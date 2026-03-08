# Design — P0-021 Dependency Audit

## Architecture

### cargo-deny Configuration (`deny.toml`)

The `deny.toml` file at the repository root configures four audit dimensions:

```
deny.toml
  ├── [advisories]   — vulnerability scanning against RustSec advisory DB
  ├── [licenses]     — license compatibility allowlist
  ├── [bans]         — duplicate version detection
  └── [sources]      — dependency origin restrictions
```

### Advisory Database
- **Database:** RustSec Advisory Database (https://github.com/rustsec/advisory-db)
- **Local path:** `~/.cargo/advisory-db` (default cargo-deny location)
- **Policy:** `vulnerability = "deny"` — any crate with a known vulnerability fails the audit
- **Unmaintained:** `unmaintained = "warn"` — flag unmaintained crates without blocking
- **Yanked:** `yanked = "warn"` — flag yanked crate versions without blocking
- **Notice:** `notice = "warn"` — informational advisories are non-blocking

### License Allowlist

Only the following licenses are permitted in the dependency tree:

| License | SPDX Identifier | Rationale |
|---|---|---|
| MIT License | `MIT` | Permissive, compatible with workspace dual-license |
| Apache License 2.0 | `Apache-2.0` | Permissive, workspace primary license |
| BSD 2-Clause | `BSD-2-Clause` | Permissive, minimal obligations |
| BSD 3-Clause | `BSD-3-Clause` | Permissive, minimal obligations |
| ISC License | `ISC` | Permissive, functionally equivalent to MIT |
| Zlib License | `Zlib` | Permissive, common in compression libraries |
| Unicode License v3 | `Unicode-3.0` | Required by unicode-ident and related crates |
| Unicode DFS 2016 | `Unicode-DFS-2016` | Required by unicode data file consumers |

**Policy:** Any dependency with a license not on this list causes a hard failure. Unlicensed crates are denied.

### Confidence Threshold
- `confidence-threshold = 0.8` — license detection confidence must be at least 80%

### Ban Policy
- `multiple-versions = "warn"` — having two versions of the same crate in the dependency tree produces a warning but does not block
- `wildcards = "allow"` — wildcard dependencies are permitted (workspace uses version ranges)
- No specific crate denials at this time

### Source Restrictions
- `unknown-registry = "deny"` — only crates.io is permitted as a registry source
- `unknown-git = "deny"` — no git dependencies from unknown origins
- `allow-registry` includes `https://github.com/rust-lang/crates.io-index` (crates.io)
- `allow-git` is empty — no git source overrides permitted

### DEPS.md Generation

The `DEPS.md` file is a manually maintained dependency inventory that documents:
- Crate name and version (from `[workspace.dependencies]`)
- SPDX license identifier
- Purpose within the Lyra workspace
- Category (cryptographic, serialization, error handling, logging, testing, benchmarking)

### Integration with CI Pipeline (P0-015)

The `deny` job in `.github/workflows/ci.yml` runs:
```
cargo deny check advisories licenses sources
```

After P0-021 completion, the `continue-on-error: true` flag on the deny job should be removed, making the audit a blocking CI gate.

## Design Decisions
1. **Explicit allowlist over denylist** — safer default; new licenses must be explicitly approved
2. **Advisory deny, unmaintained warn** — vulnerabilities are hard failures; unmaintained is informational
3. **Multiple versions warn** — avoids blocking on transitive dependency version splits that are common in the Rust ecosystem
4. **No git sources** — all dependencies must come from crates.io for reproducibility and auditability
5. **Unicode licenses included** — required by `unicode-ident` (transitive dependency of `proc-macro2`, `syn`, `serde`)
6. **Manual DEPS.md** — human-reviewed inventory ensures purpose documentation; auto-generation can supplement but not replace review
