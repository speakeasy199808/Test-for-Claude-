# Implementation — P0-017 Benchmarking Harness

## Summary

Implemented a Criterion 0.5 benchmark suite for the k0 crate covering all four core subsystems: digest, codec, time, and entropy.

## Changes Made

### Workspace Cargo.toml
- Added `criterion = { version = "0.5", features = ["html_reports"] }` to `[workspace.dependencies]`

### k0/Cargo.toml
- Added `criterion = { workspace = true }` to `[dev-dependencies]`
- Added four `[[bench]]` entries with `harness = false`

### Benchmark Files Created
1. **`k0/benches/digest_bench.rs`** — 3 benchmark groups, 14 individual benchmarks
2. **`k0/benches/codec_bench.rs`** — 3 benchmark groups, 13 individual benchmarks
3. **`k0/benches/time_bench.rs`** — 4 benchmark groups, 5 individual benchmarks
4. **`k0/benches/entropy_bench.rs`** — 5 benchmark groups, 9 individual benchmarks

### Bug Fix
- Fixed unused variable warning in `k0/src/drift/detector.rs:150` (prefixed with `_`)

## Verification
- `cargo bench --no-run -p k0` compiles all 4 benchmark executables successfully ✅
- All benchmarks use `black_box` for optimization prevention ✅
- Throughput metrics configured for digest and entropy benchmarks ✅
