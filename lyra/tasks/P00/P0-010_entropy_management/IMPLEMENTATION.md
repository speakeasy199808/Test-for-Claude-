# Implementation Notes — P0-010 Entropy Management

## Work Package Shape
Three-module Rust implementation in `k0/src/entropy/` with task-local control plane, fixtures, and artifacts.

## Produced Components

| File | Description |
|---|---|
| `k0/src/entropy/mod.rs` | Public API: re-exports `EntropyPool`, `EntropyError`, `MAX_REQUEST_BYTES`; integration tests |
| `k0/src/entropy/pool.rs` | `EntropyPool` struct, `MAX_REQUEST_BYTES` constant, hash-chain implementation |
| `k0/src/entropy/error.rs` | `EntropyError` enum: `ZeroLengthRequest`, `RequestTooLarge` |

## Hash Chain Implementation

```rust
fn advance(&mut self) -> [u8; 32] {
    let mut hasher = Sha3_256::new();
    hasher.update(self.state);
    hasher.update(self.counter.to_le_bytes());
    let result = hasher.finalize();
    // update state and counter
    self.state = result.into();
    self.counter = self.counter.saturating_add(1);
    self.state
}
```

Every `next_bytes`, `next_u64`, `next_u32`, and `fork` call invokes `advance()` at least once.

## Test Coverage (25 entropy tests)

**pool.rs (18):** `new_pool_counter_starts_at_zero`, `next_bytes_advances_counter`, `next_bytes_deterministic_same_seed`, `different_seeds_produce_different_output`, `next_bytes_zero_length_rejected`, `next_bytes_too_large_rejected`, `next_bytes_max_allowed_succeeds`, `next_bytes_returns_correct_length`, `next_u64_deterministic`, `next_u32_deterministic`, `sequential_draws_differ`, `fork_produces_independent_stream`, `fork_is_deterministic`, `from_seed_bytes_deterministic`, `from_seed_bytes_different_inputs_differ`, `counter_increments_on_each_draw`, `state_changes_after_draw`, `golden_first_u64_from_zero_seed`

**mod.rs (7):** `pool_is_accessible_from_mod`, `max_request_bytes_is_256`, `entropy_error_zero_length_accessible`, `two_pools_same_seed_same_output`, `from_seed_bytes_stable`, `fork_child_diverges_from_parent`, `sequential_u64_draws_are_unique`

## Ownership Placement
- Production code: `k0/src/entropy/` (primary ownership root `k0/`)
- Task control plane: `lyra/tasks/P00/P0-010_entropy_management/`

## Ambient Randomness Prohibition Verification
`grep -r "rand\|getrandom\|OsRng\|thread_rng\|std::time" k0/src/entropy/` returns no matches.

## Dependency Posture
- No new crate dependencies (`sha3` already in workspace)
- Enables: P0-011 (determinism verifier), P0-012 (drift detection), P0-023 (foundation integration)
