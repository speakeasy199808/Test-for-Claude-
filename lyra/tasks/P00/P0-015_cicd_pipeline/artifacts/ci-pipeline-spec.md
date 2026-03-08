# CI/CD Pipeline Specification — Evidence Artifact

## Pipeline Identity
- **Workflow file:** `.github/workflows/ci.yml`
- **Workflow name:** `Lyra CI`
- **Task:** P0-015

## Trigger Matrix

| Event | Branch Filter | Jobs Triggered |
|---|---|---|
| `push` | `main` | build-and-test, test-k0, deny |
| `push` | `blackboxai/**` | build-and-test, test-k0, deny |
| `pull_request` | `main` | build-and-test, test-k0, deny |

## Job Specifications

### Job: build-and-test
- **Runner:** `ubuntu-latest`
- **Toolchain:** `dtolnay/rust-toolchain@stable` (rustfmt, clippy)
- **Steps:**
  1. Checkout (`actions/checkout@v4`)
  2. Install Rust stable
  3. Cache cargo registry (`actions/cache@v4`, key: `{os}-cargo-{Cargo.lock hash}`)
  4. `cargo fmt --all -- --check` — formatting gate
  5. `cargo clippy --workspace --all-targets -- -D warnings` — lint gate
  6. `cargo build --workspace` — compilation gate
  7. `cargo test --workspace` — test gate

### Job: test-k0
- **Runner:** `ubuntu-latest`
- **Toolchain:** `dtolnay/rust-toolchain@stable`
- **Steps:**
  1. Checkout
  2. Install Rust stable
  3. Cache cargo registry (key: `{os}-cargo-k0-{Cargo.lock hash}`)
  4. `cargo test -p k0 --lib` — k0 unit test gate

### Job: deny
- **Runner:** `ubuntu-latest`
- **Steps:**
  1. Checkout
  2. Install `cargo-deny` (`--locked`)
  3. `cargo deny check advisories licenses sources` (`continue-on-error: true`)

## Environment Variables
- `CARGO_TERM_COLOR=always`
- `RUST_BACKTRACE=1`

## Cache Strategy
- **Paths cached:** `~/.cargo/registry`, `~/.cargo/git`, `target/`
- **Primary key:** `{runner.os}-cargo-{Cargo.lock hash}`
- **Restore fallback:** `{runner.os}-cargo-`

## Acceptance Verification
All 8 acceptance criteria from ACCEPTANCE.md are satisfied:
1. ✅ Workflow exists at `.github/workflows/ci.yml`
2. ✅ Triggers on push to `main`/`blackboxai/**` and PR to `main`
3. ✅ build-and-test runs fmt, clippy, build, test
4. ✅ test-k0 runs `cargo test -p k0 --lib`
5. ✅ deny runs `cargo deny check advisories licenses sources`
6. ✅ Cache keyed on `Cargo.lock` hash
7. ✅ `RUST_BACKTRACE=1` set
8. ✅ Uses `dtolnay/rust-toolchain@stable` with rustfmt + clippy
