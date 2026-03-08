# Entropy Traceability — P0-010

| Acceptance Criterion | Implementation | Test | Status |
|---|---|---|---|
| `new(seed)` — explicit seed, no ambient entropy | `pool.rs` — `EntropyPool { state: seed, counter: 0 }` | `new_pool_counter_starts_at_zero` | ✅ |
| `from_seed_bytes(&[u8])` — hashes to 32-byte seed | `pool.rs` — `SHA3-256(seed_material)` | `from_seed_bytes_deterministic`, `from_seed_bytes_different_inputs_differ`, `from_seed_bytes_stable` | ✅ |
| `next_bytes(n)` returns exactly n bytes | `pool.rs` — multi-block fill loop | `next_bytes_returns_correct_length`, `next_bytes_max_allowed_succeeds` | ✅ |
| `next_bytes(0)` → `ZeroLengthRequest` | `pool.rs` — early return | `next_bytes_zero_length_rejected`, `entropy_error_zero_length_accessible` | ✅ |
| `next_bytes(n > 256)` → `RequestTooLarge` | `pool.rs` — early return | `next_bytes_too_large_rejected` | ✅ |
| Same seed → same output | `pool.rs` — pure hash chain | `next_bytes_deterministic_same_seed`, `next_u64_deterministic`, `next_u32_deterministic`, `two_pools_same_seed_same_output` | ✅ |
| Different seeds → different output | `pool.rs` — hash chain diverges | `different_seeds_produce_different_output` | ✅ |
| Sequential draws differ | `pool.rs` — counter increments each step | `sequential_draws_differ`, `sequential_u64_draws_are_unique` | ✅ |
| `fork()` — child diverges from parent; two forks identical | `pool.rs` — `advance()` then `EntropyPool::new(child_seed)` | `fork_produces_independent_stream`, `fork_is_deterministic`, `fork_child_diverges_from_parent` | ✅ |
| No ambient randomness | No `rand`/`getrandom`/`OsRng`/`thread_rng`/`std::time` in `k0/src/entropy/` | Code review | ✅ |
| Golden vector: zero seed first u64 | `pool.rs` — `golden_first_u64_from_zero_seed` self-verifying test | `golden_first_u64_from_zero_seed` | ✅ |

## Fixture Reference
`lyra/tasks/P00/P0-010_entropy_management/fixtures/entropy/golden_vectors.json` — 6 scenario vectors covering seed construction, sequential draws, fork, and golden anchors.

## Test Evidence
```
cargo test -p k0 --lib
test result: ok. 138 passed; 0 failed; 0 ignored
```
Entropy-specific tests: 25 (18 in `pool.rs`, 7 in `mod.rs`)
