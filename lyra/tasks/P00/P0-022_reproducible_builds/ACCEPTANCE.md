# Acceptance — P0-022 Reproducible Builds

## Acceptance Criteria

1. **Cargo.lock is committed to version control.** The `.gitignore` does not exclude `Cargo.lock`, and the file is tracked by Git. All workspace dependency versions are fully locked.

2. **`rust-toolchain.toml` exists at the repository root.** It specifies the `stable` channel and includes the `rustfmt` and `clippy` components.

3. **`rust-toolchain.toml` is syntactically valid.** Running `rustup show` in the repository root picks up the toolchain file and reports the correct channel and components.

4. **`BUILD.md` exists at the repository root.** It documents prerequisites, build commands, reproducibility guarantees, and verification procedures.

5. **`BUILD.md` documents prerequisites.** At minimum: Rust stable toolchain installed via `rustup`, with `rustfmt` and `clippy` components.

6. **`BUILD.md` documents build commands.** At minimum: `cargo build --workspace`, `cargo test --workspace`, `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`.

7. **`BUILD.md` documents the reproducibility guarantee.** Explains that `Cargo.lock` locks dependency versions and `rust-toolchain.toml` pins the compiler, ensuring bit-for-bit reproducible builds from the same source tree.

8. **`BUILD.md` documents the verification procedure.** Provides a concrete sequence of commands to build twice and compare SHA-256 hashes of the resulting binaries.

9. **No Docker dependency.** The build pipeline, CI workflow, and `BUILD.md` do not require Docker. Builds use only `cargo`, `rustup`, and standard Rust tooling.

10. **CI alignment verified.** The `rust-toolchain.toml` configuration is consistent with the CI pipeline defined in `.github/workflows/ci.yml` (P0-015), which already uses `dtolnay/rust-toolchain@stable` with `rustfmt` and `clippy`.

11. **Evidence artifact exists.** `lyra/tasks/P00/P0-022_reproducible_builds/artifacts/reproducible-builds-spec.md` documents the full reproducible build specification.

## Verification Method
- `git ls-files Cargo.lock` returns `Cargo.lock` (file is tracked)
- `cat rust-toolchain.toml` shows valid TOML with `channel = "stable"` and components
- `cat BUILD.md` contains prerequisites, build commands, reproducibility, and verification sections
- `grep -r "docker\|Docker" BUILD.md .github/workflows/ci.yml` returns no required Docker usage
- Evidence artifact exists and is comprehensive

## Evidence Required
- `rust-toolchain.toml` at repo root
- `BUILD.md` at repo root
- `Cargo.lock` tracked in Git
- `artifacts/reproducible-builds-spec.md`
