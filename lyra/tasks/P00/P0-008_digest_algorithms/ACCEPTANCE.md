# Acceptance — P0-008 Digest Algorithms

## Acceptance Criteria

1. SHA-3-256 is implemented as the primary digest algorithm.
2. BLAKE3 is implemented as the secondary digest algorithm.
3. A unified routing API exists: `k0::digest::digest(DigestAlgorithm, &[u8]) -> DigestOutput`.
4. All system-wide hash operations route through `k0::digest` — no direct crate calls in production code outside this module.
5. `DigestAlgorithm` enum explicitly names both algorithms; callers must select explicitly.
6. `DigestOutput` is algorithm-tagged: carries both the 32-byte hash and the algorithm that produced it.
7. SHA-3-256 golden vectors match known test vectors (empty string, "abc").
8. BLAKE3 golden vector matches known test vector (empty string).
9. Both algorithms are deterministic: identical inputs always produce identical outputs.
10. Hex serialization of `DigestOutput` produces a 64-character lowercase string.

## Verification Method
- Unit tests in `k0/src/digest/` covering golden vectors, determinism, algorithm tagging, and routing
- `cargo test -p k0` passes with 0 failures

## Evidence Required
- `artifacts/digest-traceability.md`
- `fixtures/digest/golden_vectors.json`
- `cargo test -p k0` output: all digest tests pass (21 digest tests)
