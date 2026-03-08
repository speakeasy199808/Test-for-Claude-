# Implementation — P0-015 CI/CD Pipeline

## Summary

The CI/CD pipeline is implemented as a single GitHub Actions workflow file at `.github/workflows/ci.yml`. It enforces build correctness, test passage, formatting, lint cleanliness, and dependency audit on every push and pull request.

## Implementation Details

### Workflow File
- **Path:** `.github/workflows/ci.yml`
- **Trigger:** `push` to `main` and `blackboxai/**`; `pull_request` to `main`
- **Environment:** `CARGO_TERM_COLOR=always`, `RUST_BACKTRACE=1`

### Jobs Implemented

1. **build-and-test** — Full workspace validation
   - `cargo fmt --all -- --check` (formatting gate)
   - `cargo clippy --workspace --all-targets -- -D warnings` (lint gate, zero warnings)
   - `cargo build --workspace` (compilation gate)
   - `cargo test --workspace` (test gate)

2. **test-k0** — Constitutional substrate isolation
   - `cargo test -p k0 --lib` (k0-only unit tests, independent gate)

3. **deny** — Supply-chain audit
   - `cargo deny check advisories licenses sources`
   - `continue-on-error: true` (non-blocking until P0-021 finalizes deny config)

### Toolchain
- `dtolnay/rust-toolchain@stable` with `rustfmt` and `clippy` components
- Cache keyed on `Cargo.lock` hash with fallback restore key

## Files Touched
- `.github/workflows/ci.yml` (created)

## Verification
- YAML syntax validated
- All acceptance criteria in ACCEPTANCE.md satisfied
- Workflow structure matches DESIGN.md architecture
