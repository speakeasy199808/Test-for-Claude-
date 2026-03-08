# Digest Traceability — P0-008

## Algorithm → Module → Test Mapping

| Algorithm | Module | Tests |
|---|---|---|
| SHA-3-256 (primary) | `k0/src/digest/sha3.rs` | `sha3_256_empty_string_golden`, `sha3_256_abc_golden`, `sha3_256_output_is_32_bytes`, `sha3_256_is_deterministic`, `sha3_256_algorithm_tag_is_correct` |
| BLAKE3 (secondary) | `k0/src/digest/blake3.rs` | `blake3_empty_string_golden`, `blake3_output_is_32_bytes`, `blake3_is_deterministic`, `blake3_algorithm_tag_is_correct`, `blake3_different_inputs_differ` |
| Routing API | `k0/src/digest/mod.rs` | `digest_sha3_256_produces_32_bytes`, `digest_blake3_produces_32_bytes`, `digest_hex_is_64_chars`, `digest_is_deterministic`, `different_inputs_produce_different_digests`, `sha3_and_blake3_produce_different_outputs_for_same_input`, `algorithm_names_are_correct`, `algorithm_output_len_is_32`, `sha3_256_shorthand_matches_routing`, `blake3_shorthand_matches_routing`, `sha3_256_empty_input_golden` |
| Doc-test | `k0/src/digest/mod.rs` | `digest (line 14)` |

## Golden Vector Inventory

| Algorithm | Input | Expected Output | Source |
|---|---|---|---|
| SHA-3-256 | `""` (empty) | `a7ffc6f8bf1ed76651c14756a061d662f580ff4de43b49fa82d80a4b80f8434a` | NIST FIPS 202 |
| SHA-3-256 | `"abc"` | `3a985da74fe225b2045c172d6bd390bd855f086e3e9d525b46bfe24511431532` | NIST FIPS 202 |
| BLAKE3 | `""` (empty) | `af1349b9f5f9a1a6a0404dea36dcc9499bcb25c9adc112b7cc9a93cae41f3262` | BLAKE3 reference |

## Module Inventory

| Module | Path | Exports |
|---|---|---|
| Digest root | `k0/src/digest/mod.rs` | `digest`, `sha3_256`, `blake3`, `DigestAlgorithm`, `DigestOutput` |
| SHA-3-256 | `k0/src/digest/sha3.rs` | `sha3_256_digest` |
| BLAKE3 | `k0/src/digest/blake3.rs` | `blake3_digest` |

## Routing Contract

All system-wide hash operations MUST call `k0::digest::digest(algorithm, input)` or the shorthands `k0::digest::sha3_256(input)` / `k0::digest::blake3(input)`. Direct use of `sha3` or `blake3` crates outside `k0/src/digest/` is forbidden in production code.

## Test Summary

- Total digest tests: 22 (21 unit + 1 doc-test)
- All pass: `cargo test -p k0` → 86/86 ok (85 unit + 1 doc-test)
