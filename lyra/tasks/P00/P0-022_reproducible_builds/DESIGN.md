# Design — P0-022 Reproducible Builds

## Architecture

### Reproducibility Stack

The reproducible build guarantee is achieved through three layers, each addressing a different source of nondeterminism:

```
Layer 3: BUILD.md              (human-readable environment documentation)
Layer 2: rust-toolchain.toml   (compiler version pinning)
Layer 1: Cargo.lock            (dependency version locking)
Layer 0: Source code            (deterministic by constitutional invariant)
```

### Layer 1 — Dependency Locking (Cargo.lock)

**Problem:** Cargo resolves dependency versions at build time. Without a lockfile, `cargo build` on two different machines (or at two different times) may resolve to different dependency versions, producing different binaries.

**Solution:** Commit `Cargo.lock` to version control. This file records the exact resolved version of every direct and transitive dependency.

**Verification:**
```bash
git ls-files Cargo.lock   # Must return "Cargo.lock"
grep -c "Cargo.lock" .gitignore   # Must return 0 or not match
```

**Existing state:** `Cargo.lock` is already committed. `.gitignore` contains only `/target` and `.blackboxcli/`.

### Layer 2 — Toolchain Pinning (rust-toolchain.toml)

**Problem:** Different Rust compiler versions may produce different machine code from the same source. Codegen changes, optimization differences, and standard library updates all affect binary output.

**Solution:** Create `rust-toolchain.toml` at the repository root. This file is automatically read by `rustup` and ensures every developer and CI runner uses the same toolchain.

**Configuration:**
```toml
[toolchain]
channel = "stable"
components = ["rustfmt", "clippy"]
```

**Design decisions:**
1. **`stable` channel** — not pinned to a specific version (e.g., `1.78.0`) because the project is pre-1.0 and benefits from stable compiler updates. For release builds, this can be tightened to a specific version.
2. **`rustfmt` and `clippy` components** — required by CI (P0-015) and CONTRIBUTING.md. Including them in the toolchain file ensures they are installed automatically.
3. **No `targets` field** — the project currently targets the host platform only. Cross-compilation targets can be added later.
4. **No `profile` field** — defaults to the `default` profile, which is appropriate for development.

**CI alignment:** The existing CI workflow (`.github/workflows/ci.yml`) uses `dtolnay/rust-toolchain@stable` with `components: rustfmt clippy`. The `rust-toolchain.toml` file is consistent with this configuration. When `rustup` detects the toolchain file, it will use the specified channel, making the CI action's toolchain specification redundant but harmless.

### Layer 3 — Environment Documentation (BUILD.md)

**Problem:** Even with locked dependencies and a pinned toolchain, builds can fail or produce different results if the build environment is not documented. Developers need to know what tools to install, what commands to run, and how to verify reproducibility.

**Solution:** Create `BUILD.md` at the repository root with four sections:
1. **Prerequisites** — what to install
2. **Build Commands** — how to build, test, lint, and format
3. **Reproducibility** — how the build is made reproducible
4. **Verification** — how to verify bit-for-bit reproducibility

### Docker-Free Approach

**Problem:** Docker adds complexity, requires a daemon, and introduces its own reproducibility challenges (base image updates, layer caching, platform differences).

**Solution:** The Lyra build pipeline uses only `cargo` and `rustup`. No Dockerfile, no `docker-compose.yml`, no container runtime required.

**Rationale:**
1. **Simplicity** — fewer moving parts, fewer failure modes
2. **Transparency** — the build environment is fully described by `rust-toolchain.toml` and `Cargo.lock`
3. **Speed** — no container build overhead
4. **Accessibility** — contributors need only Rust, not Docker
5. **Nix compatibility** — if hermetic builds are needed later, Nix can be layered on top without Docker

### Verification Procedure

To verify bit-for-bit reproducibility:

```bash
# Build 1
cargo build --workspace --release
sha256sum target/release/lyra* > /tmp/build1.sha256

# Clean
cargo clean

# Build 2
cargo build --workspace --release
sha256sum target/release/lyra* > /tmp/build2.sha256

# Compare
diff /tmp/build1.sha256 /tmp/build2.sha256
```

If the diff is empty, the builds are bit-for-bit identical.

**Note:** On Windows, use `Get-FileHash` in PowerShell instead of `sha256sum`.

## Design Decisions Summary

| Decision | Rationale |
|---|---|
| Commit `Cargo.lock` | Locks all dependency versions for reproducibility |
| `rust-toolchain.toml` at repo root | Pins compiler via `rustup` auto-detection |
| `stable` channel (not pinned version) | Pre-1.0 project benefits from stable updates |
| Include `rustfmt` + `clippy` components | Required by CI and CONTRIBUTING.md |
| `BUILD.md` at repo root | Human-readable build environment documentation |
| No Docker | Simplicity, transparency, speed, accessibility |
| Cargo-only build | Standard Rust tooling, no external build systems |
| SHA-256 hash comparison for verification | Simple, reliable, well-understood |

## Future Considerations
- **Pinned stable version:** For release builds, pin to a specific stable version (e.g., `channel = "1.78.0"`)
- **Nix flake:** For fully hermetic builds, add a `flake.nix` that wraps the Cargo build
- **`cargo-deny` integration:** Already in CI (P0-015), ensures no banned dependencies
- **Cross-compilation:** Add `targets` to `rust-toolchain.toml` when needed
- **Reproducible release pipeline:** Extend CI to produce signed, reproducible release artifacts
