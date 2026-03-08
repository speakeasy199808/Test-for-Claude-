# Lyra Build Guide

## Prerequisites

### Required
- **Rust stable toolchain** — installed via [rustup](https://rustup.rs/)
- **rustfmt** — Rust code formatter (installed automatically via `rust-toolchain.toml`)
- **clippy** — Rust linter (installed automatically via `rust-toolchain.toml`)

### Installation

```bash
# Install rustup (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# On Windows, download and run rustup-init.exe from https://rustup.rs/
```

When you clone this repository and run any `cargo` command, `rustup` will automatically read `rust-toolchain.toml` and install the correct toolchain and components. No manual toolchain configuration is needed.

### Optional
- **cargo-deny** — dependency audit tool (used in CI, not required for local builds)
  ```bash
  cargo install cargo-deny --locked
  ```

### Not Required
- **Docker** — Lyra builds do not use Docker. No Dockerfile, no container runtime, no daemon.
- **Nix** — not currently required, but compatible if added later.

## Build Commands

### Full Workspace Build
```bash
cargo build --workspace
```

### Release Build
```bash
cargo build --workspace --release
```

### Run All Tests
```bash
cargo test --workspace
```

### Run k0 Unit Tests Only
```bash
cargo test -p k0 --lib
```

### Check Formatting
```bash
cargo fmt --all -- --check
```

### Run Linter
```bash
cargo clippy --workspace --all-targets -- -D warnings
```

### Full CI-Equivalent Local Check
Run all gates that CI enforces (see `.github/workflows/ci.yml`):
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo build --workspace
cargo test --workspace
```

### Dependency Audit (Optional)
```bash
cargo deny check advisories licenses sources
```

## Reproducibility

Lyra builds are designed to be **bit-for-bit reproducible** from source. This means that building the same commit on two different machines (with the same toolchain) produces identical binaries.

### How Reproducibility Is Achieved

| Mechanism | File | Purpose |
|---|---|---|
| **Dependency locking** | `Cargo.lock` | Records the exact resolved version of every direct and transitive dependency. Committed to version control. |
| **Toolchain pinning** | `rust-toolchain.toml` | Specifies the Rust toolchain channel and components. Automatically read by `rustup`. |
| **No ambient nondeterminism** | Source code (k0 constitutional invariant) | No `std::time::SystemTime`, no unseeded randomness, no environment-dependent behavior in core logic. |
| **Deterministic serialization** | `k0/src/codec/` | All encoding is canonical and versioned — same input always produces same bytes. |

### What Is Locked

- **All dependency versions** — `Cargo.lock` pins every crate (direct and transitive) to an exact version and checksum.
- **Rust compiler channel** — `rust-toolchain.toml` specifies `stable` with `rustfmt` and `clippy`.
- **Workspace configuration** — `Cargo.toml` at the repo root defines the workspace members, shared version, edition, and dependency versions.

### What Is Not Locked (and Why)

- **Specific stable compiler version** — The project uses `channel = "stable"` rather than a pinned version like `1.78.0`. This is intentional for the pre-1.0 development phase. For release builds, the channel can be tightened to a specific version.
- **Operating system** — Builds are reproducible across runs on the same OS. Cross-OS binary identity is not guaranteed (different linkers, system libraries).

## Verification

### Verify Reproducible Builds

To verify that builds are bit-for-bit reproducible, build twice from a clean state and compare hashes:

#### Linux / macOS
```bash
# Build 1
cargo build --workspace --release
find target/release -maxdepth 1 -type f -executable | sort | xargs sha256sum > /tmp/build1.sha256

# Clean
cargo clean

# Build 2
cargo build --workspace --release
find target/release -maxdepth 1 -type f -executable | sort | xargs sha256sum > /tmp/build2.sha256

# Compare
diff /tmp/build1.sha256 /tmp/build2.sha256
# Empty diff = bit-for-bit identical builds
```

#### Windows (PowerShell)
```powershell
# Build 1
cargo build --workspace --release
Get-ChildItem target\release\*.exe | ForEach-Object { Get-FileHash $_.FullName -Algorithm SHA256 } | Out-File build1.sha256

# Clean
cargo clean

# Build 2
cargo build --workspace --release
Get-ChildItem target\release\*.exe | ForEach-Object { Get-FileHash $_.FullName -Algorithm SHA256 } | Out-File build2.sha256

# Compare
Compare-Object (Get-Content build1.sha256) (Get-Content build2.sha256)
# No output = bit-for-bit identical builds
```

### Verify Cargo.lock Is Tracked
```bash
git ls-files Cargo.lock
# Should output: Cargo.lock
```

### Verify Toolchain File
```bash
rustup show
# Should show the stable toolchain with rustfmt and clippy
```

## Environment Variables

The following environment variables are used in CI and recommended for local development:

| Variable | Value | Purpose |
|---|---|---|
| `CARGO_TERM_COLOR` | `always` | Colored terminal output |
| `RUST_BACKTRACE` | `1` | Show backtraces on panic |

## No Docker Dependency

Lyra intentionally does not use Docker for builds. The rationale:

1. **Simplicity** — fewer moving parts, fewer failure modes
2. **Transparency** — the build environment is fully described by two files (`rust-toolchain.toml`, `Cargo.lock`)
3. **Speed** — no container build overhead, no image pulls
4. **Accessibility** — contributors need only Rust installed via `rustup`
5. **Reproducibility** — Docker introduces its own reproducibility challenges (base image updates, layer caching, platform differences)

If fully hermetic builds are needed in the future, Nix can be layered on top without introducing Docker.
