//! Globally Unique Error Code System (P0-018).
//!
//! Provides a structured, machine-readable error code system for the
//! entire Lyra system. Error codes are globally unique identifiers
//! in the range E0001–E9999, organized by category.
//!
//! # Design
//! - [`ErrorCode`] is a newtype wrapping a `u16` code number.
//! - [`ErrorCategory`] classifies errors into system domains.
//! - [`ErrorEntry`] provides full metadata: code, category, message,
//!   explanation, and suggested fix.
//! - [`ErrorCatalog`] is the registry of all known error entries.
//!
//! # Category Ranges
//! | Range | Category |
//! |---|---|
//! | E0001–E0999 | Constitutional / Determinism |
//! | E1000–E1999 | Codec / Serialization |
//! | E2000–E2999 | Digest / Cryptographic |
//! | E3000–E3999 | Time / Causal Ordering |
//! | E4000–E4999 | Entropy / Randomness |
//! | E5000–E5999 | Incident / Recovery |
//! | E6000–E6999 | Verification / Proof |
//! | E7000–E7999 | Resource / Capacity |
//! | E8000–E8999 | Policy / Governance |
//! | E9000–E9999 | Reserved / Extension |

pub mod catalog;
pub mod code;

pub use catalog::ErrorCatalog;
pub use code::{ErrorCategory, ErrorCode, ErrorEntry};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_catalog_is_not_empty() {
        let catalog = ErrorCatalog::default_catalog();
        assert!(!catalog.is_empty());
    }

    #[test]
    fn all_codes_in_valid_range() {
        let catalog = ErrorCatalog::default_catalog();
        for entry in catalog.entries() {
            assert!(
                entry.code.number() >= 1 && entry.code.number() <= 9999,
                "Code {} out of range",
                entry.code
            );
        }
    }

    #[test]
    fn all_codes_are_unique() {
        let catalog = ErrorCatalog::default_catalog();
        let mut seen = std::collections::HashSet::new();
        for entry in catalog.entries() {
            assert!(
                seen.insert(entry.code.number()),
                "Duplicate error code: {}",
                entry.code
            );
        }
    }

    #[test]
    fn category_matches_code_range() {
        let catalog = ErrorCatalog::default_catalog();
        for entry in catalog.entries() {
            let expected = ErrorCategory::from_code(entry.code);
            assert_eq!(
                entry.category, expected,
                "Code {} has category {:?} but range implies {:?}",
                entry.code, entry.category, expected
            );
        }
    }

    #[test]
    fn lookup_by_code_works() {
        let catalog = ErrorCatalog::default_catalog();
        let entry = catalog.lookup(ErrorCode::new(1).unwrap());
        assert!(entry.is_some());
        assert_eq!(entry.unwrap().code.number(), 1);
    }

    #[test]
    fn lookup_missing_code_returns_none() {
        let catalog = ErrorCatalog::default_catalog();
        assert!(catalog.lookup(ErrorCode::new(9999).unwrap()).is_none());
    }

    #[test]
    fn error_code_display_format() {
        let code = ErrorCode::new(1).unwrap();
        assert_eq!(format!("{code}"), "E0001");
        let code = ErrorCode::new(1234).unwrap();
        assert_eq!(format!("{code}"), "E1234");
    }

    #[test]
    fn error_code_zero_is_rejected() {
        assert!(ErrorCode::new(0).is_none());
    }

    #[test]
    fn error_code_above_9999_is_rejected() {
        assert!(ErrorCode::new(10000).is_none());
    }

    #[test]
    fn error_entry_to_json_is_deterministic() {
        let catalog = ErrorCatalog::default_catalog();
        let entry = catalog.lookup(ErrorCode::new(1).unwrap()).unwrap();
        let json1 = serde_json::to_string(entry).unwrap();
        let json2 = serde_json::to_string(entry).unwrap();
        assert_eq!(json1, json2);
    }
}
