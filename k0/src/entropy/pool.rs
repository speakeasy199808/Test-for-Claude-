//! [`EntropyPool`] — seeded deterministic hash-chained entropy (P0-010).
//!
//! No ambient randomness. No OS entropy. No `rand::thread_rng()`. No `OsRng`.
//! The pool is seeded explicitly and advances via SHA-3-256 hash chaining.
//!
//! # Hash Chain Protocol
//! ```text
//! state_0  = seed (32 bytes)
//! state_n+1 = SHA3-256(state_n || LE64(counter_n))
//! output_n  = state_n+1
//! counter   monotonically increments on every draw
//! ```
//!
//! This produces a deterministic, reproducible byte stream from any seed.
//! Identical seeds produce identical output sequences.

use sha3::{Digest, Sha3_256};

use super::error::EntropyError;

/// Maximum bytes returnable in a single [`EntropyPool::next_bytes`] call.
///
/// Callers needing more bytes must make multiple calls.
pub const MAX_REQUEST_BYTES: usize = 256;

/// A deterministic hash-chained entropy pool.
///
/// Seeded with 32 bytes; produces an unbounded deterministic byte stream
/// via SHA-3-256 hash chaining. No ambient nondeterminism is permitted.
///
/// # Determinism Guarantee
/// Two `EntropyPool` instances created with the same seed and driven with
/// the same sequence of calls will produce byte-identical output.
#[derive(Debug, Clone)]
pub struct EntropyPool {
    /// Current internal state (32 bytes, SHA-3-256 output width).
    state: [u8; 32],
    /// Monotonic draw counter — included in each hash to prevent cycles.
    counter: u64,
}

impl EntropyPool {
    /// Create a new pool from an explicit 32-byte seed.
    ///
    /// No ambient entropy is used. The caller is responsible for providing
    /// a seed appropriate to the security requirements of the use case.
    pub fn new(seed: [u8; 32]) -> Self {
        EntropyPool {
            state: seed,
            counter: 0,
        }
    }

    /// Create a pool seeded from a byte slice (hashed to 32 bytes via SHA-3-256).
    ///
    /// Useful when the seed material is not already 32 bytes.
    pub fn from_seed_bytes(seed_material: &[u8]) -> Self {
        let mut hasher = Sha3_256::new();
        hasher.update(seed_material);
        let hash = hasher.finalize();
        let mut seed = [0u8; 32];
        seed.copy_from_slice(&hash);
        EntropyPool::new(seed)
    }

    /// Advance the hash chain by one step and return the new state.
    ///
    /// `new_state = SHA3-256(current_state || LE64(counter))`
    fn advance(&mut self) -> [u8; 32] {
        let mut hasher = Sha3_256::new();
        hasher.update(self.state);
        hasher.update(self.counter.to_le_bytes());
        let result = hasher.finalize();
        let mut new_state = [0u8; 32];
        new_state.copy_from_slice(&result);
        self.state = new_state;
        self.counter = self.counter.saturating_add(1);
        new_state
    }

    /// Draw `n` deterministic bytes from the pool.
    ///
    /// Each call advances the hash chain. `n` must be in `1..=MAX_REQUEST_BYTES`.
    ///
    /// # Errors
    /// - [`EntropyError::ZeroLengthRequest`] if `n == 0`
    /// - [`EntropyError::RequestTooLarge`] if `n > MAX_REQUEST_BYTES`
    pub fn next_bytes(&mut self, n: usize) -> Result<Vec<u8>, EntropyError> {
        if n == 0 {
            return Err(EntropyError::ZeroLengthRequest);
        }
        if n > MAX_REQUEST_BYTES {
            return Err(EntropyError::RequestTooLarge {
                requested: n,
                max: MAX_REQUEST_BYTES,
            });
        }

        // Collect enough 32-byte blocks to satisfy the request.
        let mut buf = Vec::with_capacity(n);
        while buf.len() < n {
            let block = self.advance();
            let remaining = n - buf.len();
            let take = remaining.min(32);
            buf.extend_from_slice(&block[..take]);
        }
        Ok(buf)
    }

    /// Draw a deterministic `u64` from the pool (little-endian from first 8 bytes).
    pub fn next_u64(&mut self) -> u64 {
        let block = self.advance();
        u64::from_le_bytes(block[..8].try_into().expect("slice is 8 bytes"))
    }

    /// Draw a deterministic `u32` from the pool (little-endian from first 4 bytes).
    pub fn next_u32(&mut self) -> u32 {
        let block = self.advance();
        u32::from_le_bytes(block[..4].try_into().expect("slice is 4 bytes"))
    }

    /// Fork this pool into a child pool with a derived seed.
    ///
    /// The child pool is seeded from the parent's next hash-chain output.
    /// After forking, parent and child produce independent byte streams.
    pub fn fork(&mut self) -> EntropyPool {
        let child_seed = self.advance();
        EntropyPool::new(child_seed)
    }

    /// Return the current draw counter (number of hash-chain advances so far).
    pub fn counter(&self) -> u64 {
        self.counter
    }

    /// Return the current internal state (for inspection/testing only).
    ///
    /// Do not use the raw state as entropy output — use [`next_bytes`](Self::next_bytes).
    pub fn state(&self) -> &[u8; 32] {
        &self.state
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn zero_seed() -> [u8; 32] {
        [0u8; 32]
    }

    fn one_seed() -> [u8; 32] {
        let mut s = [0u8; 32];
        s[0] = 1;
        s
    }

    #[test]
    fn new_pool_counter_starts_at_zero() {
        let pool = EntropyPool::new(zero_seed());
        assert_eq!(pool.counter(), 0);
    }

    #[test]
    fn next_bytes_advances_counter() {
        let mut pool = EntropyPool::new(zero_seed());
        pool.next_bytes(1).unwrap();
        assert_eq!(pool.counter(), 1);
        pool.next_bytes(32).unwrap();
        assert_eq!(pool.counter(), 2);
    }

    #[test]
    fn next_bytes_deterministic_same_seed() {
        let mut a = EntropyPool::new(zero_seed());
        let mut b = EntropyPool::new(zero_seed());
        assert_eq!(a.next_bytes(32).unwrap(), b.next_bytes(32).unwrap());
        assert_eq!(a.next_bytes(16).unwrap(), b.next_bytes(16).unwrap());
    }

    #[test]
    fn different_seeds_produce_different_output() {
        let mut a = EntropyPool::new(zero_seed());
        let mut b = EntropyPool::new(one_seed());
        assert_ne!(a.next_bytes(32).unwrap(), b.next_bytes(32).unwrap());
    }

    #[test]
    fn next_bytes_zero_length_rejected() {
        let mut pool = EntropyPool::new(zero_seed());
        assert_eq!(pool.next_bytes(0), Err(EntropyError::ZeroLengthRequest));
    }

    #[test]
    fn next_bytes_too_large_rejected() {
        let mut pool = EntropyPool::new(zero_seed());
        let err = pool.next_bytes(MAX_REQUEST_BYTES + 1).unwrap_err();
        assert_eq!(
            err,
            EntropyError::RequestTooLarge {
                requested: MAX_REQUEST_BYTES + 1,
                max: MAX_REQUEST_BYTES,
            }
        );
    }

    #[test]
    fn next_bytes_max_allowed_succeeds() {
        let mut pool = EntropyPool::new(zero_seed());
        let bytes = pool.next_bytes(MAX_REQUEST_BYTES).unwrap();
        assert_eq!(bytes.len(), MAX_REQUEST_BYTES);
    }

    #[test]
    fn next_bytes_returns_correct_length() {
        let mut pool = EntropyPool::new(zero_seed());
        for n in [1, 7, 16, 31, 32, 33, 64, 128, 256] {
            let bytes = pool.next_bytes(n).unwrap();
            assert_eq!(bytes.len(), n, "expected {n} bytes");
        }
    }

    #[test]
    fn next_u64_deterministic() {
        let mut a = EntropyPool::new(zero_seed());
        let mut b = EntropyPool::new(zero_seed());
        assert_eq!(a.next_u64(), b.next_u64());
        assert_eq!(a.next_u64(), b.next_u64());
    }

    #[test]
    fn next_u32_deterministic() {
        let mut a = EntropyPool::new(zero_seed());
        let mut b = EntropyPool::new(zero_seed());
        assert_eq!(a.next_u32(), b.next_u32());
    }

    #[test]
    fn sequential_draws_differ() {
        let mut pool = EntropyPool::new(zero_seed());
        let a = pool.next_bytes(32).unwrap();
        let b = pool.next_bytes(32).unwrap();
        assert_ne!(a, b, "sequential draws must differ");
    }

    #[test]
    fn fork_produces_independent_stream() {
        let mut parent = EntropyPool::new(zero_seed());
        let mut child = parent.fork();
        // Parent and child should produce different output
        let p = parent.next_bytes(32).unwrap();
        let c = child.next_bytes(32).unwrap();
        assert_ne!(p, c, "forked child must diverge from parent");
    }

    #[test]
    fn fork_is_deterministic() {
        let mut a = EntropyPool::new(zero_seed());
        let mut b = EntropyPool::new(zero_seed());
        let mut ca = a.fork();
        let mut cb = b.fork();
        assert_eq!(ca.next_bytes(32).unwrap(), cb.next_bytes(32).unwrap());
    }

    #[test]
    fn from_seed_bytes_deterministic() {
        let mut a = EntropyPool::from_seed_bytes(b"lyra-test-seed");
        let mut b = EntropyPool::from_seed_bytes(b"lyra-test-seed");
        assert_eq!(a.next_bytes(32).unwrap(), b.next_bytes(32).unwrap());
    }

    #[test]
    fn from_seed_bytes_different_inputs_differ() {
        let mut a = EntropyPool::from_seed_bytes(b"seed-a");
        let mut b = EntropyPool::from_seed_bytes(b"seed-b");
        assert_ne!(a.next_bytes(32).unwrap(), b.next_bytes(32).unwrap());
    }

    #[test]
    fn counter_increments_on_each_draw() {
        let mut pool = EntropyPool::new(zero_seed());
        assert_eq!(pool.counter(), 0);
        pool.next_u64();
        assert_eq!(pool.counter(), 1);
        pool.next_u32();
        assert_eq!(pool.counter(), 2);
        pool.next_bytes(1).unwrap();
        assert_eq!(pool.counter(), 3);
    }

    #[test]
    fn state_changes_after_draw() {
        let mut pool = EntropyPool::new(zero_seed());
        let s0 = *pool.state();
        pool.next_u64();
        let s1 = *pool.state();
        assert_ne!(s0, s1);
    }

    /// Golden vector: SHA3-256([0u8;32] || LE64(0)) first 8 bytes as u64.
    /// This pins the hash-chain output to a known value for regression detection.
    #[test]
    fn golden_first_u64_from_zero_seed() {
        let mut pool = EntropyPool::new(zero_seed());
        let v = pool.next_u64();
        // The value is deterministic — record it here as a golden anchor.
        // If this test fails, the hash chain implementation changed.
        let expected = {
            use sha3::{Digest, Sha3_256};
            let mut h = Sha3_256::new();
            h.update([0u8; 32]);
            h.update(0u64.to_le_bytes());
            let out = h.finalize();
            u64::from_le_bytes(out[..8].try_into().unwrap())
        };
        assert_eq!(v, expected, "golden hash-chain anchor mismatch");
    }
}
