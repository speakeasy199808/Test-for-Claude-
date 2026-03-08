# P0-017 — Benchmarking Harness

## Mission
Criterion-based benchmark suite. Regression detection with statistical significance thresholds. Performance budgets per crate. Historical tracking.

## Scope
- Criterion 0.5 benchmark suite for k0 crate
- Benchmarks for digest (SHA-3-256, BLAKE3), codec (encode/decode), time (tick/advance/merge), entropy (next_u64/next_bytes/fork)
- Throughput measurements with parameterized input sizes
- HTML report generation for historical comparison
- Foundation for regression detection in CI

## Primary Archetype
Verification / Proof

## Primary Ownership Root
`k0/benches/`

## Secondary Touched Roots
`lyra/tasks/`

## Deliverables
- `k0/benches/digest_bench.rs` — SHA-3-256 and BLAKE3 throughput benchmarks
- `k0/benches/codec_bench.rs` — LyraCodec encode/decode benchmarks
- `k0/benches/time_bench.rs` — Virtual clock operation benchmarks
- `k0/benches/entropy_bench.rs` — Entropy pool throughput benchmarks
- Criterion workspace dependency and k0 bench configuration
- Task control-plane files
