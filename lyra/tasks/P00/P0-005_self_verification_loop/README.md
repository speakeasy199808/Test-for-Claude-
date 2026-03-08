# P0-005 Self Verification Loop

**Status:** ✅ Complete
**Module:** `k0::self_verify`
**Files:** `k0/src/self_verify/mod.rs`, `k0/src/self_verify/verifier.rs`, `k0/src/self_verify/error.rs`

## Mission

Provide runtime code integrity verification by computing SHA-3-256 digests
of code bytes and comparing against expected hashes. Enable tamper detection
at runtime with deterministic, receipted verification.

## Scope

### Types
- `SelfVerifier` — holds expected SHA-3-256 hash, provides `verify()` method
- `VerificationReceipt` — records expected_hash, actual_hash, timestamp, passed
- `SelfVerifyError` — error type for verification failures

### API
- `SelfVerifier::new(expected_hash: [u8; 32])` — constructor
- `SelfVerifier::verify(code_bytes, clock) -> Result<VerificationReceipt, SelfVerifyError>`
  - Computes SHA-3-256 of code_bytes
  - Advances virtual clock by one tick
  - Returns receipt with pass/fail
- `VerificationReceipt::expected_hex()` / `actual_hex()` — hex string accessors

## Acceptance Criteria

- [x] SelfVerifier holds expected SHA-3-256 hash
- [x] verify() computes actual hash and compares
- [x] Matching hash produces receipt with passed=true
- [x] Mismatched hash produces receipt with passed=false
- [x] Empty input handled correctly (SHA-3-256 of empty = known value)
- [x] Virtual clock advances on each verification
- [x] Verification is deterministic (same inputs → same receipt)
- [x] All public items documented
- [x] No unsafe code
- [x] 13 unit tests (7 in mod.rs, 6 in verifier.rs)
- [x] Integration test coverage in foundation_integration.rs step 04

## Evidence

- 13 unit tests passing
- Integration test `step_04_self_verification_loop` exercises pass and fail paths
- Cross-module determinism test includes self-verification in pipeline
