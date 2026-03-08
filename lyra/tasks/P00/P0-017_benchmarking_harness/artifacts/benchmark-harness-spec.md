# Benchmark Harness Specification — Evidence Artifact

## Task: P0-017 Benchmarking Harness

## Framework
- **Library:** Criterion 0.5 with `html_reports` feature
- **Runner:** Custom (`harness = false`)
- **Reports:** HTML output in `target/criterion/`

## Benchmark Inventory

| File | Groups | Benchmarks | Throughput |
|---|---|---|---|
| `digest_bench.rs` | 3 | 14 | Yes (bytes/sec) |
| `codec_bench.rs` | 3 | 13 | No |
| `time_bench.rs` | 4 | 5 | No |
| `entropy_bench.rs` | 5 | 9 | Yes (bytes/sec) |
| **Total** | **15** | **41** | — |

## Running Benchmarks

```bash
# Run all k0 benchmarks
cargo bench -p k0

# Run specific benchmark
cargo bench -p k0 --bench digest_bench

# Compile-check only (no execution)
cargo bench --no-run -p k0
```

## Acceptance Verification
1. ✅ Criterion 0.5 declared as workspace dependency with html_reports
2. ✅ k0 Cargo.toml includes criterion dev-dependency and [[bench]] entries
3. ✅ digest_bench covers SHA-3-256 and BLAKE3 at 6 input sizes with throughput
4. ✅ codec_bench covers all value types for encode, decode, and roundtrip
5. ✅ time_bench covers tick, advance, merge, and VirtualTime::next
6. ✅ entropy_bench covers next_u64, next_u32, next_bytes, fork, from_seed_bytes
7. ✅ All benchmarks compile with `cargo bench --no-run -p k0`
8. ✅ All benchmarks use `black_box`
9. ✅ Throughput metrics on digest and entropy benchmarks
