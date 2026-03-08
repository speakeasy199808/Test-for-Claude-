# Acceptance — P0-010 Entropy Management

## Acceptance Criteria

1. `EntropyPool::new(seed: [u8; 32])` creates a pool with explicit seed — no ambient entropy.
2. `EntropyPool::from_seed_bytes(&[u8])` hashes arbitrary bytes to a 32-byte seed via SHA-3-256.
3. `next_bytes(n)` returns exactly `n` bytes; valid range is `1..=256`.
4. `next_bytes(0)` returns `EntropyError::ZeroLengthRequest`.
5. `next_bytes(n > 256)` returns `EntropyError::RequestTooLarge`.
6. Two pools with identical seeds produce byte-identical output for identical call sequences.
7. Two pools with different seeds produce different output.
8. Sequential draws from the same pool produce different values (hash chain advances).
9. `fork()` produces a child pool that diverges from the parent; two forks from identical parents are identical.
10. No `rand`, `getrandom`, `OsRng`, `thread_rng`, or `std::time` appears in `k0/src/entropy/`.
11. Golden vector: `EntropyPool::new([0u8;32]).next_u64()` matches `SHA3-256([0u8;32] || LE64(0))[0..8]`.

## Verification Method
- Unit tests in `k0/src/entropy/` covering all operations, error paths, and golden vectors
- `cargo test -p k0 --lib` passes with 0 failures

## Evidence Required
- `artifacts/entropy-traceability.md`
- `fixtures/entropy/golden_vectors.json`
- `cargo test -p k0 --lib` output: all entropy tests pass (25 entropy tests)
