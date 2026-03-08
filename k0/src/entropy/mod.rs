//! Entropy Management — seeded deterministic hash-chained pool (P0-010).
//!
//! This module provides a deterministic entropy source for the Lyra system.
//! No ambient randomness is permitted: no `rand::thread_rng()`, no `OsRng`,
//! no `getrandom`, no `std::time`-based seeding.
//!
//! # Design
//! - [`EntropyPool`] is seeded with an explicit `[u8; 32]` value.
//! - Each draw advances the internal state via SHA-3-256 hash chaining.
//! - Identical seeds + identical call sequences produce byte-identical output.
//! - [`EntropyPool::fork`] creates a child pool with a derived seed.
//!
//! # Hash Chain Protocol
//! ```text
//! state_0   = seed
//! state_n+1 = SHA3-256(state_n || LE64(counter_n))
//! output_n  = state_n+1
//! ```
//!
//! # Usage
//! ```rust
//! use k0::entropy::EntropyPool;
//!
//! let seed = [0u8; 32];
//! let mut pool = EntropyPool::new(seed);
//! let bytes = pool.next_bytes(16).unwrap();
//! assert_eq!(bytes.len(), 16);
//! let v = pool.next_u64();
//! let _ = v; // deterministic value
//! ```

pub mod error;
pub mod pool;

pub use error::EntropyError;
pub use pool::{EntropyPool, MAX_REQUEST_BYTES};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pool_is_accessible_from_mod() {
        let mut pool = EntropyPool::new([0u8; 32]);
        let bytes = pool.next_bytes(8).unwrap();
        assert_eq!(bytes.len(), 8);
    }

    #[test]
    fn max_request_bytes_is_256() {
        assert_eq!(MAX_REQUEST_BYTES, 256);
    }

    #[test]
    fn entropy_error_zero_length_accessible() {
        let mut pool = EntropyPool::new([0u8; 32]);
        assert_eq!(pool.next_bytes(0), Err(EntropyError::ZeroLengthRequest));
    }

    #[test]
    fn two_pools_same_seed_same_output() {
        let seed = [42u8; 32];
        let mut a = EntropyPool::new(seed);
        let mut b = EntropyPool::new(seed);
        for _ in 0..10 {
            assert_eq!(a.next_u64(), b.next_u64());
        }
    }

    #[test]
    fn from_seed_bytes_stable() {
        let mut a = EntropyPool::from_seed_bytes(b"lyra-entropy-test");
        let mut b = EntropyPool::from_seed_bytes(b"lyra-entropy-test");
        assert_eq!(a.next_bytes(32).unwrap(), b.next_bytes(32).unwrap());
    }

    #[test]
    fn fork_child_diverges_from_parent() {
        let mut parent = EntropyPool::new([1u8; 32]);
        let mut child = parent.fork();
        assert_ne!(
            parent.next_bytes(32).unwrap(),
            child.next_bytes(32).unwrap()
        );
    }

    #[test]
    fn sequential_u64_draws_are_unique() {
        let mut pool = EntropyPool::new([0u8; 32]);
        let a = pool.next_u64();
        let b = pool.next_u64();
        let c = pool.next_u64();
        // All three must differ (hash chain property)
        assert_ne!(a, b);
        assert_ne!(b, c);
        assert_ne!(a, c);
    }
}
