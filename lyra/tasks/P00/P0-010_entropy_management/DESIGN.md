# Design — P0-010 Entropy Management

## Architecture

Entropy management lives at `k0/src/entropy/`. It is a pure Rust module with no I/O, no OS calls, and no ambient nondeterminism. All entropy is derived from an explicit seed via SHA-3-256 hash chaining.

### Module Decomposition

| Module | Responsibility |
|---|---|
| `mod.rs` | Public API, re-exports, integration tests |
| `pool.rs` | `EntropyPool`, `MAX_REQUEST_BYTES` |
| `error.rs` | `EntropyError` |

### Hash Chain Protocol

```text
state_0   = seed (32 bytes)
state_n+1 = SHA3-256(state_n || LE64(counter_n))
output_n  = state_n+1
counter   monotonically increments on every advance
```

Each call to `next_bytes`, `next_u64`, `next_u32`, or `fork` advances the counter by exactly one step. The output is the full 32-byte SHA-3-256 hash output, truncated to the requested length.

For requests > 32 bytes, multiple chain steps are taken and concatenated until the requested length is satisfied.

### Type Design

**`EntropyPool`**
- `state: [u8; 32]` — current hash-chain state
- `counter: u64` — monotonic draw counter
- `new(seed: [u8; 32])` — explicit seed
- `from_seed_bytes(&[u8])` — hash arbitrary bytes to 32-byte seed
- `next_bytes(n: usize) -> Result<Vec<u8>, EntropyError>` — draw n bytes (1..=256)
- `next_u64() -> u64` — LE64 from first 8 bytes of next block
- `next_u32() -> u32` — LE32 from first 4 bytes of next block
- `fork() -> EntropyPool` — derive child pool from next block
- `counter() -> u64` — inspect draw count
- `state() -> &[u8; 32]` — inspect internal state (testing only)

**`EntropyError`**
- `ZeroLengthRequest` — `next_bytes(0)` rejected
- `RequestTooLarge { requested, max }` — `next_bytes(n > 256)` rejected

**`MAX_REQUEST_BYTES: usize = 256`** — per-call limit

### Ambient Randomness Prohibition

No imports of `rand`, `getrandom`, `OsRng`, `thread_rng`, or `std::time` are permitted anywhere in `k0/src/entropy/`. The only external dependency is `sha3` (already in workspace).

### Determinism Guarantee

Two `EntropyPool` instances with the same seed, driven with the same sequence of calls, produce byte-identical output. This is enforced by the pure hash-chain construction and verified by golden-vector tests.

## Dependency Posture

- No new crate dependencies (`sha3` already in workspace)
- Consumed by: P0-011 (determinism verifier uses entropy for test vectors), P0-012 (drift detection), P0-023 (foundation integration)
