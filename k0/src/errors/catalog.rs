//! Error catalog: the registry of all known Lyra error codes.

use crate::errors::code::{ErrorCategory, ErrorCode, ErrorEntry};

/// The error catalog — a registry of all known error entries.
///
/// The catalog is constructed at startup and provides O(n) lookup
/// by error code. For the expected catalog size (< 1000 entries),
/// this is efficient and avoids HashMap nondeterminism concerns.
#[derive(Debug, Clone)]
pub struct ErrorCatalog {
    entries: Vec<ErrorEntry>,
}

impl ErrorCatalog {
    /// Create an empty catalog.
    pub fn new() -> Self {
        ErrorCatalog {
            entries: Vec::new(),
        }
    }

    /// Register an error entry. Panics if the code is already registered.
    pub fn register(&mut self, entry: ErrorEntry) {
        debug_assert!(
            !self.entries.iter().any(|e| e.code == entry.code),
            "Duplicate error code: {}",
            entry.code
        );
        self.entries.push(entry);
    }

    /// Look up an error entry by code. Returns `None` if not found.
    pub fn lookup(&self, code: ErrorCode) -> Option<&ErrorEntry> {
        self.entries.iter().find(|e| e.code == code)
    }

    /// Returns the number of registered error entries.
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true if the catalog is empty.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// Returns an iterator over all registered error entries.
    pub fn entries(&self) -> &[ErrorEntry] {
        &self.entries
    }

    /// Returns all entries in a given category.
    pub fn by_category(&self, category: ErrorCategory) -> Vec<&ErrorEntry> {
        self.entries
            .iter()
            .filter(|e| e.category == category)
            .collect()
    }

    /// Build the default Lyra error catalog with all known error codes.
    pub fn default_catalog() -> Self {
        let mut catalog = Self::new();

        // === Constitutional (E0001–E0999) ===
        catalog.register(ErrorEntry {
            code: ErrorCode::new(1).unwrap(),
            category: ErrorCategory::Constitutional,
            message: "Determinism violation detected",
            explanation: "Two executions of the same computation with identical inputs produced different outputs. This is a constitutional violation of the Lyra determinism invariant.",
            suggestion: "Check for ambient nondeterministic inputs: wall clock, unseeded randomness, HashMap iteration order, or floating-point operations.",
        });
        catalog.register(ErrorEntry {
            code: ErrorCode::new(2).unwrap(),
            category: ErrorCategory::Constitutional,
            message: "Constitutional hash mismatch",
            explanation: "The computed constitutional hash does not match the expected genesis hash. The system state may have been tampered with.",
            suggestion: "Verify the genesis state has not been modified. Re-derive the constitutional hash from the canonical genesis state.",
        });
        catalog.register(ErrorEntry {
            code: ErrorCode::new(3).unwrap(),
            category: ErrorCategory::Constitutional,
            message: "Trust root verification failed",
            explanation: "A trust root fingerprint could not be verified against the known trust root set.",
            suggestion: "Check that the trust root fingerprint is correctly formatted and present in the genesis state trust root list.",
        });
        catalog.register(ErrorEntry {
            code: ErrorCode::new(4).unwrap(),
            category: ErrorCategory::Constitutional,
            message: "Forbidden ambient input detected",
            explanation: "A module attempted to access a forbidden ambient input source (wall clock, system randomness, environment variable, etc.).",
            suggestion: "Replace the ambient input with the appropriate k0 primitive: VirtualClock for time, EntropyPool for randomness.",
        });
        catalog.register(ErrorEntry {
            code: ErrorCode::new(5).unwrap(),
            category: ErrorCategory::Constitutional,
            message: "Genesis state validation failed",
            explanation: "The genesis state does not satisfy one or more constitutional invariants (version, system ID, trust roots, or sequence number).",
            suggestion: "Use the canonical genesis state constructor and verify all fields meet the constitutional math specification.",
        });

        // === Codec (E1000–E1999) ===
        catalog.register(ErrorEntry {
            code: ErrorCode::new(1000).unwrap(),
            category: ErrorCategory::Codec,
            message: "Unknown type tag in encoded data",
            explanation: "The decoder encountered a type tag byte that does not correspond to any known LyraCodec type.",
            suggestion: "Verify the encoded data was produced by a compatible LyraCodec encoder version.",
        });
        catalog.register(ErrorEntry {
            code: ErrorCode::new(1001).unwrap(),
            category: ErrorCategory::Codec,
            message: "Non-canonical varint encoding",
            explanation: "A varint was encoded with unnecessary trailing zero bytes, violating the canonical encoding requirement.",
            suggestion: "Use the canonical varint encoder which produces minimal-length encodings.",
        });
        catalog.register(ErrorEntry {
            code: ErrorCode::new(1002).unwrap(),
            category: ErrorCategory::Codec,
            message: "Non-canonical struct field order",
            explanation: "Struct fields are not in ascending field_id order, violating the canonical encoding requirement.",
            suggestion: "Sort struct fields by field_id before encoding.",
        });
        catalog.register(ErrorEntry {
            code: ErrorCode::new(1003).unwrap(),
            category: ErrorCategory::Codec,
            message: "Non-canonical map key order",
            explanation: "Map entries are not sorted by lexicographic order of encoded key bytes.",
            suggestion: "Sort map entries by the canonical encoded bytes of each key.",
        });
        catalog.register(ErrorEntry {
            code: ErrorCode::new(1004).unwrap(),
            category: ErrorCategory::Codec,
            message: "Unexpected end of input",
            explanation: "The decoder reached the end of the input buffer before completing the current value.",
            suggestion: "Verify the encoded data is complete and not truncated.",
        });

        // === Digest (E2000–E2999) ===
        catalog.register(ErrorEntry {
            code: ErrorCode::new(2000).unwrap(),
            category: ErrorCategory::Digest,
            message: "Invalid digest length",
            explanation: "A digest output does not have the expected 32-byte length for the specified algorithm.",
            suggestion: "Ensure digest outputs are exactly 32 bytes for both SHA-3-256 and BLAKE3.",
        });
        catalog.register(ErrorEntry {
            code: ErrorCode::new(2001).unwrap(),
            category: ErrorCategory::Digest,
            message: "Digest algorithm mismatch",
            explanation: "A digest was produced with one algorithm but is being verified against a different algorithm's expected output.",
            suggestion: "Check the DigestAlgorithm tag on the DigestOutput and ensure it matches the expected algorithm.",
        });

        // === Time (E3000–E3999) ===
        catalog.register(ErrorEntry {
            code: ErrorCode::new(3000).unwrap(),
            category: ErrorCategory::Time,
            message: "Backward time reset attempted",
            explanation: "An attempt was made to reset the virtual clock to a time earlier than the current time, violating monotonicity.",
            suggestion: "Virtual time can only advance forward. Use merge() to reconcile divergent clocks.",
        });
        catalog.register(ErrorEntry {
            code: ErrorCode::new(3001).unwrap(),
            category: ErrorCategory::Time,
            message: "Causal ordering violation",
            explanation: "An event was timestamped with a virtual time that precedes a causally prior event.",
            suggestion: "Ensure all events are timestamped after their causal predecessors by ticking the clock between dependent operations.",
        });

        // === Entropy (E4000–E4999) ===
        catalog.register(ErrorEntry {
            code: ErrorCode::new(4000).unwrap(),
            category: ErrorCategory::Entropy,
            message: "Entropy request exceeds maximum",
            explanation: "A request for random bytes exceeded the maximum allowed size (256 bytes per request).",
            suggestion: "Split large entropy requests into multiple calls of at most 256 bytes each.",
        });
        catalog.register(ErrorEntry {
            code: ErrorCode::new(4001).unwrap(),
            category: ErrorCategory::Entropy,
            message: "Zero-length entropy request",
            explanation: "A request for zero random bytes was made, which is not meaningful.",
            suggestion: "Request at least 1 byte from the entropy pool.",
        });

        // === Incident (E5000–E5999) ===
        catalog.register(ErrorEntry {
            code: ErrorCode::new(5000).unwrap(),
            category: ErrorCategory::Incident,
            message: "Unrecoverable incident",
            explanation: "An incident was classified as unrecoverable and the system must halt.",
            suggestion:
                "Review the incident record for root cause. Manual intervention may be required.",
        });
        catalog.register(ErrorEntry {
            code: ErrorCode::new(5001).unwrap(),
            category: ErrorCategory::Incident,
            message: "Recovery protocol failed",
            explanation: "The recovery state machine could not transition to a safe state after an incident.",
            suggestion: "Check the recovery protocol logs and escalate to a higher recovery tier.",
        });

        // === Verification (E6000–E6999) ===
        catalog.register(ErrorEntry {
            code: ErrorCode::new(6000).unwrap(),
            category: ErrorCategory::Verification,
            message: "Verification run mismatch",
            explanation: "The double-run determinism verifier detected different outputs for the same computation.",
            suggestion: "This is a constitutional violation. Investigate the computation for nondeterministic behavior.",
        });
        catalog.register(ErrorEntry {
            code: ErrorCode::new(6001).unwrap(),
            category: ErrorCategory::Verification,
            message: "Drift threshold exceeded",
            explanation: "Statistical drift detection found output variance exceeding the configured threshold.",
            suggestion: "Review the flagged computation for subtle nondeterminism sources.",
        });

        // === Resource (E7000–E7999) ===
        catalog.register(ErrorEntry {
            code: ErrorCode::new(7000).unwrap(),
            category: ErrorCategory::Resource,
            message: "Memory allocation limit exceeded",
            explanation: "A subsystem attempted to allocate memory beyond its configured budget.",
            suggestion:
                "Reduce the working set size or increase the memory budget for the subsystem.",
        });

        // === Policy (E8000–E8999) ===
        catalog.register(ErrorEntry {
            code: ErrorCode::new(8000).unwrap(),
            category: ErrorCategory::Policy,
            message: "Policy gate rejected operation",
            explanation: "A policy gate denied the requested operation based on the current governance rules.",
            suggestion: "Review the policy configuration and ensure the operation is permitted under current rules.",
        });

        catalog
    }
}

impl Default for ErrorCatalog {
    fn default() -> Self {
        Self::default_catalog()
    }
}
