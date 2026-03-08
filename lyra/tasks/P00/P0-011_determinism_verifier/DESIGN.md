# Design — P0-011 Determinism Verifier

## Constitutional Basis
Determinism is a constitutional invariant (P0-003): identical canonical input state + identical event sequence must produce identical outputs. The verifier enforces this at runtime by running each computation twice and comparing outputs byte-for-byte.

## Architecture

### Module Layout
```
k0/src/verifier/
├── mod.rs          — public API, re-exports, integration tests
├── determinism.rs  — DeterminismVerifier, VerificationOutcome, VerificationRecord, verify_once
└── error.rs        — VerifierError (DeterminismViolation, EmptyOutput)
```

### Core Types

**`DeterminismVerifier`**
- `clock: VirtualClock` — monotonic virtual clock for timestamping events
- `records: Vec<VerificationRecord>` — append-only audit log
- `verify(label, f)` — runs `f()` twice, compares, records, returns `Ok(output)` or `Err`
- `verify_allow_empty(label, f)` — same but permits empty output
- `all_pass()`, `pass_count()`, `fail_count()`, `record_count()` — audit queries

**`VerificationOutcome`**
- `Pass { output_hex: String, timestamp: VirtualTime }` — both runs identical
- `Fail { first_hex, second_hex, timestamp }` — constitutional violation

**`VerificationRecord`**
- `label: String` — human-readable verification point name
- `outcome: VerificationOutcome` — Pass or Fail with timestamp

**`VerifierError`**
- `DeterminismViolation { label, first_hex, second_hex }` — constitutional violation
- `EmptyOutput { label }` — suspicious empty output (use `verify_allow_empty` if intentional)

**`verify_once(label, f)`**
- Stateless helper: runs `f()` twice, returns `Ok(output)` or `Err(DeterminismViolation)`

## Design Decisions
1. **Byte-level comparison** — outputs are `Vec<u8>` for unambiguous comparison
2. **Hex evidence** — all violation records include hex-encoded first/second outputs for debugging
3. **Empty output guard** — empty outputs are rejected by default (likely a bug in the function under test)
4. **VirtualTime timestamps** — all events use the internal virtual clock, no wall clock
5. **Audit log** — all events recorded regardless of outcome for post-hoc analysis
6. **No ambient state** — verifier is fully deterministic given the same sequence of calls
