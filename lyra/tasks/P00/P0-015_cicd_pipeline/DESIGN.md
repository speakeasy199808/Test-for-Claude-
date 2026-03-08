# Design — P0-015 CI/CD Pipeline

## Pipeline Architecture

### Jobs

```
push / pull_request
        │
        ├── build-and-test   (ubuntu-latest)
        │     ├── cargo fmt --all -- --check
        │     ├── cargo clippy --workspace --all-targets -- -D warnings
        │     ├── cargo build --workspace
        │     └── cargo test --workspace
        │
        ├── test-k0          (ubuntu-latest)
        │     └── cargo test -p k0 --lib
        │
        └── deny             (ubuntu-latest)
              └── cargo deny check advisories licenses sources
```

### Trigger Policy
- `push` to `main` — full pipeline
- `push` to `blackboxai/**` — full pipeline (agent branches)
- `pull_request` to `main` — full pipeline (gate before merge)

### Toolchain
- `dtolnay/rust-toolchain@stable` — pinned to stable channel
- Components: `rustfmt`, `clippy`
- `CARGO_TERM_COLOR=always` — readable CI output
- `RUST_BACKTRACE=1` — full backtraces on test failures

### Cache Strategy
- Key: `{os}-cargo-{Cargo.lock hash}`
- Paths: `~/.cargo/registry`, `~/.cargo/git`, `target/`
- Restore key fallback: `{os}-cargo-` (partial cache hit)

### Dependency Audit
- `cargo-deny` — checks advisories, licenses, and source origins
- `continue-on-error: true` — non-blocking until deny config is finalized (P0-021)

## Design Decisions
1. **Separate k0 job** — k0 is the constitutional substrate; its tests are independently gated
2. **Clippy as error** — `-D warnings` enforces zero-warning policy at CI level
3. **fmt check before build** — formatting failures are caught cheapest first
4. **cargo-deny non-blocking** — advisory database may lag; blocking enabled after P0-021 audit
5. **Cargo.lock hash cache key** — deterministic cache invalidation on dependency changes
