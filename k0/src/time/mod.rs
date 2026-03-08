//! Virtual Time — monotonic counter, no wall clock (P0-009).
//!
//! This module provides a deterministic virtual clock for the Lyra system.
//! Time advances only on explicit events — there is no ambient wall-clock
//! input, no `std::time::SystemTime`, and no `std::time::Instant` anywhere
//! in this module or its callers.
//!
//! # Design
//! - [`VirtualTime`] is a monotonic `u64` counter (newtype).
//! - [`VirtualClock`] holds the current time and advances only via [`VirtualClock::tick`].
//! - Causal ordering is enforced: time never goes backward.
//! - [`VirtualClock::merge`] takes the maximum of two clocks for distributed causal merge.
//!
//! # Usage
//! ```rust
//! use k0::time::{VirtualClock, VirtualTime};
//!
//! let mut clock = VirtualClock::new();
//! assert_eq!(clock.now(), VirtualTime::ZERO);
//! let t1 = clock.tick();
//! let t2 = clock.tick();
//! assert!(t2 > t1);
//! ```

pub mod clock;

pub use clock::{TimeError, VirtualClock, VirtualTime};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_clock_starts_at_zero() {
        let clock = VirtualClock::new();
        assert_eq!(clock.now(), VirtualTime::ZERO);
    }

    #[test]
    fn tick_advances_by_one() {
        let mut clock = VirtualClock::new();
        let t = clock.tick();
        assert_eq!(t, VirtualTime::new(1));
        assert_eq!(clock.now(), VirtualTime::new(1));
    }

    #[test]
    fn tick_is_monotonic() {
        let mut clock = VirtualClock::new();
        let t1 = clock.tick();
        let t2 = clock.tick();
        let t3 = clock.tick();
        assert!(t1 < t2);
        assert!(t2 < t3);
    }

    #[test]
    fn advance_by_n() {
        let mut clock = VirtualClock::new();
        clock.advance(10);
        assert_eq!(clock.now(), VirtualTime::new(10));
    }

    #[test]
    fn merge_takes_max() {
        let mut a = VirtualClock::new();
        a.advance(5);
        let mut b = VirtualClock::new();
        b.advance(10);
        a.merge(&b);
        assert_eq!(a.now(), VirtualTime::new(10));
    }

    #[test]
    fn merge_keeps_own_if_larger() {
        let mut a = VirtualClock::new();
        a.advance(20);
        let mut b = VirtualClock::new();
        b.advance(5);
        a.merge(&b);
        assert_eq!(a.now(), VirtualTime::new(20));
    }

    #[test]
    fn virtual_time_ordering() {
        let t0 = VirtualTime::ZERO;
        let t1 = VirtualTime::new(1);
        let t100 = VirtualTime::new(100);
        assert!(t0 < t1);
        assert!(t1 < t100);
        assert!(t100 > t0);
        assert_eq!(t0, VirtualTime::ZERO);
    }

    #[test]
    fn reset_to_advances_forward() {
        let mut clock = VirtualClock::new();
        clock.advance(5);
        clock.reset_to(VirtualTime::new(10)).unwrap();
        assert_eq!(clock.now(), VirtualTime::new(10));
    }

    #[test]
    fn reset_to_rejects_backward() {
        let mut clock = VirtualClock::new();
        clock.advance(10);
        let result = clock.reset_to(VirtualTime::new(5));
        assert!(result.is_err());
    }

    #[test]
    fn advance_zero_is_noop() {
        let mut clock = VirtualClock::new();
        clock.advance(5);
        clock.advance(0);
        assert_eq!(clock.now(), VirtualTime::new(5));
    }

    #[test]
    fn virtual_time_as_u64() {
        let t = VirtualTime::new(42);
        assert_eq!(t.as_u64(), 42);
    }
}
