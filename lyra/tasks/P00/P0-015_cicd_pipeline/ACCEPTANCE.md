# Acceptance — P0-015 CI/CD Pipeline

## Acceptance Criteria
1. A GitHub Actions workflow exists at `.github/workflows/ci.yml`.
2. The workflow triggers on push to `main` and `blackboxai/**` branches, and on pull requests to `main`.
3. The `build-and-test` job runs:
   - `cargo fmt --all -- --check`
   - `cargo clippy --workspace --all-targets -- -D warnings`
   - `cargo build --workspace`
   - `cargo test --workspace`
4. The `test-k0` job runs `cargo test -p k0 --lib` independently.
5. The `deny` job runs `cargo deny check advisories licenses sources`.
6. Cargo registry and build artifacts are cached by `Cargo.lock` hash.
7. `RUST_BACKTRACE=1` is set for all jobs.
8. All jobs use `dtolnay/rust-toolchain@stable` with `rustfmt` and `clippy` components.

## Verification Method
- Workflow file review against declared scope
- YAML syntax validity
- Job dependency and trigger correctness

## Evidence Required
- `.github/workflows/ci.yml`
- `artifacts/ci-pipeline-spec.md`
