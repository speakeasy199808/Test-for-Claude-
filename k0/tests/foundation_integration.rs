//! P0-023 Foundation Integration — end-to-end integration test.
//!
//! This test exercises ALL k0 modules together in a single deterministic
//! pipeline, verifying that the entire foundational substrate works as
//! an integrated whole.
//!
//! # Modules Exercised
//! 1. Genesis state construction and validation
//! 2. Constitutional hash computation
//! 3. Trust roots with threshold policy
//! 4. Self-verification loop
//! 5. Codec encode/decode roundtrip
//! 6. Digest routing (SHA-3 + BLAKE3)
//! 7. Virtual time operations
//! 8. Entropy pool (seeded, deterministic)
//! 9. Determinism verifier double-run
//! 10. Drift detector clean check
//! 11. Incident creation + recovery protocol
//! 12. Error catalog lookup
//! 13. Structured logging with virtual timestamps
//! 14. Cross-module determinism assertion

use k0::codec::{decode, encode, StructField, Value, TAG_VARINT_U};
use k0::digest::{digest, DigestAlgorithm};
use k0::drift::DriftDetector;
use k0::entropy::EntropyPool;
use k0::errors::{ErrorCatalog, ErrorCode};
use k0::genesis::trust_roots::{
    ThresholdPolicy, TrustRootEntry, TrustRootKind, TrustRootSet, TrustRoots,
};
use k0::genesis::{ConstitutionalHash, GenesisState};
use k0::incident::{Incident, IncidentKind};
use k0::logging::{CorrelationId, LogEntry, LogLevel, LogSink};
use k0::recovery::RecoveryProtocol;
use k0::self_verify::SelfVerifier;
use k0::time::{VirtualClock, VirtualTime};
use k0::verifier::DeterminismVerifier;

// ── 1. Genesis State ────────────────────────────────────────────────────────

#[test]
fn step_01_genesis_state_construction_and_validation() {
    let genesis = GenesisState::canonical();
    assert!(
        genesis.validate().is_ok(),
        "canonical genesis must be valid"
    );
    assert_eq!(genesis.version, 1);
    assert_eq!(genesis.system_id, "lyra");
    assert_eq!(genesis.sequence, 0);

    // Canonical bytes are deterministic
    let bytes1 = genesis.to_canonical_bytes().unwrap();
    let bytes2 = genesis.to_canonical_bytes().unwrap();
    assert_eq!(
        bytes1, bytes2,
        "genesis serialization must be deterministic"
    );
}

// ── 2. Constitutional Hash ──────────────────────────────────────────────────

#[test]
fn step_02_constitutional_hash_computation() {
    let genesis = GenesisState::canonical();
    let hash1 = ConstitutionalHash::of(&genesis).unwrap();
    let hash2 = ConstitutionalHash::of(&genesis).unwrap();
    assert_eq!(hash1, hash2, "constitutional hash must be deterministic");

    let hex = hash1.to_hex();
    assert_eq!(hex.len(), 64, "hash hex must be 64 chars");

    // Roundtrip through hex
    let parsed = ConstitutionalHash::from_hex(&hex).unwrap();
    assert_eq!(hash1, parsed, "hex roundtrip must be lossless");

    // Mutated state produces different hash
    let mut mutated = GenesisState::canonical();
    mutated.system_id = "lyra-alt".to_string();
    let hash_mutated = ConstitutionalHash::of(&mutated).unwrap();
    assert_ne!(
        hash1, hash_mutated,
        "mutated state must produce different hash"
    );
}

// ── 3. Trust Roots with Threshold Policy ────────────────────────────────────

#[test]
fn step_03_trust_roots_with_threshold_policy() {
    let fp_a = "a".repeat(64);
    let fp_b = "b".repeat(64);
    let fp_c = "c".repeat(64);

    let roots = TrustRoots {
        version: 1,
        entries: vec![
            TrustRootEntry {
                id: "alpha-key".to_string(),
                kind: TrustRootKind::SigningKey,
                fingerprint: fp_a,
                description: "Alpha signing key".to_string(),
            },
            TrustRootEntry {
                id: "beta-spec".to_string(),
                kind: TrustRootKind::SpecDocument,
                fingerprint: fp_b,
                description: "Beta spec document".to_string(),
            },
            TrustRootEntry {
                id: "gamma-const".to_string(),
                kind: TrustRootKind::ConstitutionalSpec,
                fingerprint: fp_c,
                description: "Gamma constitutional spec".to_string(),
            },
        ],
    };
    assert!(roots.validate().is_ok());
    assert_eq!(roots.len(), 3);
    assert_eq!(roots.fingerprints().len(), 3);

    // 2-of-3 threshold
    let policy = ThresholdPolicy::new(2, 3).unwrap();
    let set = TrustRootSet::new(roots, policy).unwrap();
    assert!(set.is_quorum_possible());
    assert!(!set.verify_threshold(0));
    assert!(!set.verify_threshold(1));
    assert!(set.verify_threshold(2));
    assert!(set.verify_threshold(3));
}

// ── 4. Self-Verification Loop ───────────────────────────────────────────────

#[test]
fn step_04_self_verification_loop() {
    let code = b"constitutional law v1 - immutable foundation";
    let expected = digest(DigestAlgorithm::Sha3_256, code);
    let verifier = SelfVerifier::new(*expected.as_bytes());
    let mut clock = VirtualClock::new();

    // Matching code passes
    let receipt = verifier.verify(code, &mut clock).unwrap();
    assert!(receipt.passed, "matching code must pass self-verification");
    assert_eq!(receipt.expected_hash, receipt.actual_hash);
    assert_eq!(receipt.timestamp, VirtualTime::new(1));

    // Tampered code fails
    let tampered = b"tampered law v1 - modified foundation";
    let receipt_fail = verifier.verify(tampered, &mut clock).unwrap();
    assert!(
        !receipt_fail.passed,
        "tampered code must fail self-verification"
    );
    assert_ne!(receipt_fail.expected_hash, receipt_fail.actual_hash);
    assert_eq!(receipt_fail.timestamp, VirtualTime::new(2));
}

// ── 5. Codec Encode/Decode Roundtrip ────────────────────────────────────────

#[test]
fn step_05_codec_roundtrip() {
    let values = vec![
        Value::UInt(0),
        Value::UInt(127),
        Value::UInt(128),
        Value::UInt(u64::MAX),
        Value::SInt(-1),
        Value::SInt(0),
        Value::SInt(i64::MAX),
        Value::SInt(i64::MIN),
        Value::Bytes(vec![0xde, 0xad, 0xbe, 0xef]),
        Value::Str("lyra-foundation".to_string()),
        Value::Vector {
            elem_tag: TAG_VARINT_U,
            elements: vec![Value::UInt(1), Value::UInt(2), Value::UInt(3)],
        },
        Value::Struct {
            schema_version: 1,
            fields: vec![
                StructField {
                    field_id: 1,
                    value: Value::Str("genesis".to_string()),
                },
                StructField {
                    field_id: 2,
                    value: Value::UInt(42),
                },
            ],
        },
    ];

    for value in &values {
        let encoded = encode(value).expect("encoding must succeed");
        let decoded = decode(&encoded).expect("decoding must succeed");
        assert_eq!(value, &decoded, "roundtrip must be lossless for {value:?}");

        // Determinism: encode twice, get same bytes
        let encoded2 = encode(value).unwrap();
        assert_eq!(encoded, encoded2, "encoding must be deterministic");
    }
}

// ── 6. Digest Routing ───────────────────────────────────────────────────────

#[test]
fn step_06_digest_routing() {
    let input = b"lyra-foundation-integration-test";

    // SHA-3-256 (primary)
    let sha3 = digest(DigestAlgorithm::Sha3_256, input);
    assert_eq!(sha3.as_bytes().len(), 32);
    assert_eq!(sha3.algorithm, DigestAlgorithm::Sha3_256);
    assert_eq!(sha3.to_hex().len(), 64);

    // BLAKE3 (secondary)
    let b3 = digest(DigestAlgorithm::Blake3, input);
    assert_eq!(b3.as_bytes().len(), 32);
    assert_eq!(b3.algorithm, DigestAlgorithm::Blake3);
    assert_eq!(b3.to_hex().len(), 64);

    // Different algorithms produce different outputs
    assert_ne!(sha3.as_bytes(), b3.as_bytes());

    // Determinism
    let sha3_again = digest(DigestAlgorithm::Sha3_256, input);
    assert_eq!(sha3, sha3_again);
    let b3_again = digest(DigestAlgorithm::Blake3, input);
    assert_eq!(b3, b3_again);
}

// ── 7. Virtual Time Operations ──────────────────────────────────────────────

#[test]
fn step_07_virtual_time_operations() {
    let mut clock = VirtualClock::new();
    assert_eq!(clock.now(), VirtualTime::ZERO);

    // Tick
    let t1 = clock.tick();
    assert_eq!(t1, VirtualTime::new(1));
    let t2 = clock.tick();
    assert_eq!(t2, VirtualTime::new(2));
    assert!(t2 > t1);

    // Advance
    clock.advance(10);
    assert_eq!(clock.now(), VirtualTime::new(12));

    // Merge
    let mut other = VirtualClock::new();
    other.advance(20);
    clock.merge(&other);
    assert_eq!(clock.now(), VirtualTime::new(20));

    // Reset forward
    clock.reset_to(VirtualTime::new(100)).unwrap();
    assert_eq!(clock.now(), VirtualTime::new(100));

    // Reset backward fails
    assert!(clock.reset_to(VirtualTime::new(50)).is_err());
}

// ── 8. Entropy Pool ─────────────────────────────────────────────────────────

#[test]
fn step_08_entropy_pool() {
    let seed = [42u8; 32];
    let mut pool_a = EntropyPool::new(seed);
    let mut pool_b = EntropyPool::new(seed);

    // Same seed → same output (determinism)
    for _ in 0..10 {
        assert_eq!(pool_a.next_u64(), pool_b.next_u64());
    }

    // Bytes draw
    let mut pool = EntropyPool::new(seed);
    let bytes = pool.next_bytes(32).unwrap();
    assert_eq!(bytes.len(), 32);

    // Fork produces independent stream
    let mut parent = EntropyPool::new(seed);
    let mut child = parent.fork();
    assert_ne!(
        parent.next_bytes(16).unwrap(),
        child.next_bytes(16).unwrap()
    );

    // from_seed_bytes is deterministic
    let mut x = EntropyPool::from_seed_bytes(b"lyra-integration");
    let mut y = EntropyPool::from_seed_bytes(b"lyra-integration");
    assert_eq!(x.next_bytes(32).unwrap(), y.next_bytes(32).unwrap());
}

// ── 9. Determinism Verifier ─────────────────────────────────────────────────

#[test]
fn step_09_determinism_verifier() {
    let mut v = DeterminismVerifier::new();

    // Verify codec is deterministic
    v.verify("codec-uint-42", || {
        encode(&Value::UInt(42)).unwrap_or_default()
    })
    .unwrap();

    // Verify digest is deterministic
    v.verify("sha3-lyra", || {
        digest(DigestAlgorithm::Sha3_256, b"lyra")
            .as_bytes()
            .to_vec()
    })
    .unwrap();

    // Verify genesis serialization is deterministic
    v.verify("genesis-canonical", || {
        GenesisState::canonical()
            .to_canonical_bytes()
            .unwrap_or_default()
    })
    .unwrap();

    assert!(v.all_pass());
    assert_eq!(v.pass_count(), 3);
    assert_eq!(v.fail_count(), 0);
}

// ── 10. Drift Detector ─────────────────────────────────────────────────────

#[test]
fn step_10_drift_detector() {
    let mut d = DriftDetector::new();

    d.check("sha3-empty", || {
        digest(DigestAlgorithm::Sha3_256, b"").as_bytes().to_vec()
    })
    .unwrap();

    d.check("blake3-empty", || {
        digest(DigestAlgorithm::Blake3, b"").as_bytes().to_vec()
    })
    .unwrap();

    d.check("codec-str", || {
        encode(&Value::Str("lyra".to_string())).unwrap_or_default()
    })
    .unwrap();

    let report = d.report();
    assert!(report.is_clean(), "all checks must be drift-free");
    assert_eq!(report.total_checks, 3);
    assert_eq!(report.passed, 3);
    assert!(!report.has_constitutional_drift());
}

// ── 11. Incident + Recovery ─────────────────────────────────────────────────

#[test]
fn step_11_incident_and_recovery() {
    let t = VirtualTime::new(100);

    // Constitutional incident → halt
    let constitutional = Incident::new(
        IncidentKind::DeterminismViolation,
        "integration-test",
        t,
        "first=cafe second=dead",
    );
    assert!(constitutional.is_constitutional());
    assert_eq!(constitutional.code(), "INC-001");

    let outcome = RecoveryProtocol::execute(&constitutional, VirtualTime::new(101));
    assert!(outcome.is_halted(), "constitutional violation must halt");

    // Operational incident → escalate
    let operational = Incident::new(
        IncidentKind::EncodingError,
        "codec-boundary",
        VirtualTime::new(200),
        "bad tag 0xff",
    );
    assert!(!operational.is_constitutional());

    let outcome2 = RecoveryProtocol::execute(&operational, VirtualTime::new(201));
    assert!(outcome2.is_escalated(), "encoding error must escalate");

    // Low severity → recover
    let low = Incident::new_bare(
        IncidentKind::RecoverableError,
        "retry-ok",
        VirtualTime::new(300),
    );
    let outcome3 = RecoveryProtocol::execute(&low, VirtualTime::new(301));
    assert!(outcome3.is_recovered(), "low severity must recover");
}

// ── 12. Error Catalog ───────────────────────────────────────────────────────

#[test]
fn step_12_error_catalog() {
    let catalog = ErrorCatalog::default_catalog();
    assert!(!catalog.is_empty(), "default catalog must not be empty");

    // Lookup E0001 (constitutional/determinism)
    let e0001 = catalog.lookup(ErrorCode::new(1).unwrap());
    assert!(e0001.is_some(), "E0001 must exist in catalog");
    let entry = e0001.unwrap();
    assert_eq!(entry.code.number(), 1);

    // All codes are in valid range
    for entry in catalog.entries() {
        assert!(
            entry.code.number() >= 1 && entry.code.number() <= 9999,
            "code {} out of range",
            entry.code
        );
    }

    // Serialization is deterministic
    let entry = catalog.lookup(ErrorCode::new(1).unwrap()).unwrap();
    let json1 = serde_json::to_string(entry).unwrap();
    let json2 = serde_json::to_string(entry).unwrap();
    assert_eq!(json1, json2, "error entry JSON must be deterministic");
}

// ── 13. Structured Logging ──────────────────────────────────────────────────

#[test]
fn step_13_structured_logging() {
    let mut sink = LogSink::new();
    let cid = CorrelationId::new(1);

    sink.log(LogEntry::new(
        VirtualTime::new(1),
        LogLevel::Info,
        cid,
        "k0::genesis",
        "Genesis state initialized",
    ));

    sink.log(LogEntry::new(
        VirtualTime::new(2),
        LogLevel::Debug,
        cid,
        "k0::digest",
        "Constitutional hash computed",
    ));

    let mut warn_entry = LogEntry::new(
        VirtualTime::new(3),
        LogLevel::Warn,
        CorrelationId::new(2),
        "k0::self_verify",
        "Self-verification mismatch detected",
    );
    warn_entry.add_context("expected", "cafe...");
    warn_entry.add_context("actual", "dead...");
    sink.log(warn_entry);

    // Verify ordering
    assert_eq!(sink.len(), 3);
    assert_eq!(sink.entries()[0].message(), "Genesis state initialized");
    assert_eq!(
        sink.entries()[2].message(),
        "Self-verification mismatch detected"
    );

    // Filter by level
    let warns = sink.by_level(LogLevel::Warn);
    assert_eq!(warns.len(), 1);

    // Filter by correlation ID
    let cid1_entries = sink.by_correlation_id(cid);
    assert_eq!(cid1_entries.len(), 2);

    // JSON serialization is deterministic
    let json1 = serde_json::to_string(sink.entries().first().unwrap()).unwrap();
    let json2 = serde_json::to_string(sink.entries().first().unwrap()).unwrap();
    assert_eq!(json1, json2);

    // Virtual timestamps, not wall clock
    assert_eq!(sink.entries()[0].timestamp(), VirtualTime::new(1));
    assert_eq!(sink.entries()[1].timestamp(), VirtualTime::new(2));
    assert_eq!(sink.entries()[2].timestamp(), VirtualTime::new(3));
}

// ── 14. Cross-Module Determinism ────────────────────────────────────────────

#[test]
fn step_14_cross_module_determinism() {
    // Run the entire pipeline twice and verify identical results

    fn run_pipeline() -> Vec<u8> {
        let mut output = Vec::new();

        // Genesis
        let genesis = GenesisState::canonical();
        let genesis_bytes = genesis.to_canonical_bytes().unwrap();
        output.extend_from_slice(&genesis_bytes);

        // Constitutional hash
        let hash = ConstitutionalHash::of(&genesis).unwrap();
        output.extend_from_slice(hash.as_bytes());

        // Digest routing
        let sha3 = digest(DigestAlgorithm::Sha3_256, &genesis_bytes);
        output.extend_from_slice(sha3.as_bytes());
        let b3 = digest(DigestAlgorithm::Blake3, &genesis_bytes);
        output.extend_from_slice(b3.as_bytes());

        // Codec roundtrip
        let value = Value::Struct {
            schema_version: 1,
            fields: vec![
                StructField {
                    field_id: 1,
                    value: Value::Str("lyra".to_string()),
                },
                StructField {
                    field_id: 2,
                    value: Value::UInt(42),
                },
            ],
        };
        let encoded = encode(&value).unwrap();
        output.extend_from_slice(&encoded);

        // Entropy
        let mut pool = EntropyPool::new([0u8; 32]);
        let entropy_bytes = pool.next_bytes(32).unwrap();
        output.extend_from_slice(&entropy_bytes);

        // Self-verification
        let verifier = SelfVerifier::new(*sha3.as_bytes());
        let mut clock = VirtualClock::new();
        let receipt = verifier.verify(&genesis_bytes, &mut clock).unwrap();
        output.push(if receipt.passed { 1 } else { 0 });
        output.extend_from_slice(&receipt.actual_hash);

        output
    }

    let run1 = run_pipeline();
    let run2 = run_pipeline();
    assert_eq!(
        run1, run2,
        "full pipeline must produce identical output on every run"
    );
    assert!(!run1.is_empty(), "pipeline output must not be empty");
}

// ── Full Pipeline Smoke Test ────────────────────────────────────────────────

#[test]
fn full_foundation_pipeline() {
    // This test exercises the complete flow in a single sequential pipeline,
    // simulating what a real system boot would look like.

    let mut clock = VirtualClock::new();
    let mut sink = LogSink::new();
    let cid = CorrelationId::new(1);

    // 1. Genesis
    let genesis = GenesisState::canonical();
    genesis.validate().unwrap();
    sink.log(LogEntry::new(
        clock.tick(),
        LogLevel::Info,
        cid,
        "k0::genesis",
        "Genesis state validated",
    ));

    // 2. Constitutional hash
    let const_hash = ConstitutionalHash::of(&genesis).unwrap();
    sink.log(LogEntry::new(
        clock.tick(),
        LogLevel::Info,
        cid,
        "k0::genesis::hash",
        "Constitutional hash computed",
    ));

    // 3. Trust roots
    let roots = TrustRoots {
        version: 1,
        entries: vec![TrustRootEntry {
            id: "const-spec-v1".to_string(),
            kind: TrustRootKind::ConstitutionalSpec,
            fingerprint: const_hash.to_hex(),
            description: "Constitutional spec v1".to_string(),
        }],
    };
    let policy = ThresholdPolicy::new(1, 1).unwrap();
    let trust_set = TrustRootSet::new(roots, policy).unwrap();
    assert!(trust_set.is_quorum_possible());
    assert!(trust_set.verify_threshold(1));
    sink.log(LogEntry::new(
        clock.tick(),
        LogLevel::Info,
        cid,
        "k0::genesis::trust_roots",
        "Trust roots validated with threshold policy",
    ));

    // 4. Self-verification
    let genesis_bytes = genesis.to_canonical_bytes().unwrap();
    let expected_digest = digest(DigestAlgorithm::Sha3_256, &genesis_bytes);
    let self_verifier = SelfVerifier::new(*expected_digest.as_bytes());
    let receipt = self_verifier.verify(&genesis_bytes, &mut clock).unwrap();
    assert!(receipt.passed);
    sink.log(LogEntry::new(
        clock.tick(),
        LogLevel::Info,
        cid,
        "k0::self_verify",
        "Self-verification passed",
    ));

    // 5. Determinism verification
    let mut det_verifier = DeterminismVerifier::new();
    det_verifier
        .verify("genesis-bytes", || {
            GenesisState::canonical()
                .to_canonical_bytes()
                .unwrap_or_default()
        })
        .unwrap();
    assert!(det_verifier.all_pass());
    sink.log(LogEntry::new(
        clock.tick(),
        LogLevel::Info,
        cid,
        "k0::verifier",
        "Determinism verification passed",
    ));

    // 6. Drift check
    let mut drift = DriftDetector::new();
    drift
        .check("genesis-hash", || {
            ConstitutionalHash::of(&GenesisState::canonical())
                .unwrap()
                .as_bytes()
                .to_vec()
        })
        .unwrap();
    assert!(drift.is_clean());
    sink.log(LogEntry::new(
        clock.tick(),
        LogLevel::Info,
        cid,
        "k0::drift",
        "Drift check clean",
    ));

    // 7. Error catalog
    let catalog = ErrorCatalog::default_catalog();
    assert!(!catalog.is_empty());
    sink.log(LogEntry::new(
        clock.tick(),
        LogLevel::Info,
        cid,
        "k0::errors",
        "Error catalog loaded",
    ));

    // 8. Entropy
    let mut pool = EntropyPool::from_seed_bytes(const_hash.as_bytes());
    let _entropy = pool.next_bytes(32).unwrap();
    sink.log(LogEntry::new(
        clock.tick(),
        LogLevel::Info,
        cid,
        "k0::entropy",
        "Entropy pool seeded from constitutional hash",
    ));

    // Final assertions
    assert!(sink.len() >= 8, "should have logged at least 8 entries");
    assert!(
        sink.by_level(LogLevel::Error).is_empty(),
        "no errors should have been logged"
    );
    assert!(
        clock.now().as_u64() >= 8,
        "clock should have advanced at least 8 ticks"
    );
}
