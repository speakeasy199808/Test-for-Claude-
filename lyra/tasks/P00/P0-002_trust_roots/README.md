# P0-002 Trust Roots

**Status:** ✅ Complete
**Module:** `k0::genesis::trust_roots`
**File:** `k0/src/genesis/trust_roots.rs`

## Mission

Declare and validate trusted authority anchors at genesis. Provide threshold
policy for quorum-based verification, key ceremony stubs for multi-party
key generation, and HSM binding stubs for hardware security module integration.

## Scope

### Core Types (existing, validated)
- `TrustRootKind` — SigningKey, SpecDocument, ConstitutionalSpec
- `TrustRootEntry` — id, kind, fingerprint (SHA-3-256 hex), description
- `TrustRoots` — versioned, canonically ordered collection with validation

### New Types (P0-002 expansion)
- `ThresholdPolicy` — m-of-n quorum policy with validation
- `TrustRootSet` — TrustRoots + ThresholdPolicy with verify_threshold and is_quorum_possible
- `CeremonyStep` — Init, ContributeShare, VerifyShares, Finalize
- `CeremonyRecord` — ceremony execution record with steps and result fingerprint
- `HsmCapability` — KeyGeneration, Signing, Verification, SecureStorage
- `HsmBinding` — HSM identity fingerprint + capabilities

## Acceptance Criteria

- [x] ThresholdPolicy validates required <= total, both > 0
- [x] TrustRootSet combines roots with threshold policy
- [x] verify_threshold checks verified count against required
- [x] is_quorum_possible checks entries count against required
- [x] CeremonyStep and CeremonyRecord provide ceremony workflow stubs
- [x] HsmCapability and HsmBinding provide HSM integration stubs
- [x] All types serialize/deserialize via serde
- [x] All public items documented
- [x] No unsafe code
- [x] 30+ unit tests covering all new types
- [x] All existing tests continue to pass

## Evidence

- 30 unit tests in `k0/src/genesis/trust_roots.rs` (all passing)
- Serialization roundtrip tests for ThresholdPolicy, TrustRootSet, CeremonyRecord, HsmBinding
- Integration test coverage in `k0/tests/foundation_integration.rs` step 03
