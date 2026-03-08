# P0-010 — Entropy Management

## Mission
Seeded deterministic PRNG. Hash-chained entropy pool. No ambient randomness permitted.

## Scope
- `EntropyPool` — explicit 32-byte seed, SHA-3-256 hash-chained state
- `EntropyPool::new(seed)` — explicit seed constructor
- `EntropyPool::from_seed_bytes(&[u8])` — seed from arbitrary bytes (hashed to 32)
- `EntropyPool::next_bytes(n)` — draw n deterministic bytes (1..=256)
- `EntropyPool::next_u64()` / `next_u32()` — typed draws
- `EntropyPool::fork()` — derive independent child pool
- `EntropyError` — structured errors for invalid requests
- No `rand`, no `getrandom`, no `OsRng`, no `thread_rng`

## Primary Archetype
Core Module Implementation

## Primary Ownership Root
`k0/`

## Secondary Touched Roots
`fixtures/`, `lyra/tasks/`

## Deliverables
- `k0/src/entropy/mod.rs` — public API, re-exports, integration tests
- `k0/src/entropy/pool.rs` — `EntropyPool`, `MAX_REQUEST_BYTES`
- `k0/src/entropy/error.rs` — `EntropyError`
- Task control plane: README, ACCEPTANCE, DESIGN, IMPLEMENTATION, task.toml
- Fixtures: entropy golden vectors
- Artifacts: entropy traceability
