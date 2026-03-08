//! [`VirtualClock`] and [`VirtualTime`] — deterministic monotonic time (P0-009).
//!
//! No wall clock. No `std::time`. Time advances only on explicit events.

use thiserror::Error;

/// A monotonic virtual timestamp — a `u64` counter with causal ordering.
///
/// `VirtualTime` is a pure value type. It carries no ambient state and
/// has no connection to wall-clock time.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct VirtualTime(u64);

impl VirtualTime {
    /// The zero timestamp — the origin of virtual time.
    pub const ZERO: VirtualTime = VirtualTime(0);

    /// Construct a `VirtualTime` from a raw counter value.
    pub fn new(value: u64) -> Self {
        VirtualTime(value)
    }

    /// Return the raw `u64` counter value.
    pub fn as_u64(self) -> u64 {
        self.0
    }

    /// Return the successor timestamp (saturating at `u64::MAX`).
    pub fn next(self) -> Self {
        VirtualTime(self.0.saturating_add(1))
    }
}

impl std::fmt::Display for VirtualTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "t:{}", self.0)
    }
}

/// Errors produced by [`VirtualClock`] operations.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum TimeError {
    /// Attempted to reset the clock to a time in the past.
    ///
    /// Virtual time is monotonic — it never goes backward.
    #[error("cannot reset clock backward: current={current}, requested={requested}")]
    BackwardReset {
        /// The current clock value.
        current: u64,
        /// The rejected requested value.
        requested: u64,
    },
}

/// A deterministic monotonic virtual clock.
///
/// `VirtualClock` advances only via explicit [`tick`](VirtualClock::tick) or
/// [`advance`](VirtualClock::advance) calls. It has no connection to wall-clock
/// time and no ambient nondeterminism.
///
/// # Causal Ordering
/// Time never goes backward. [`reset_to`](VirtualClock::reset_to) rejects
/// any value less than the current time. [`merge`](VirtualClock::merge) takes
/// the maximum of two clocks for distributed causal merge.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VirtualClock {
    now: VirtualTime,
}

impl VirtualClock {
    /// Create a new clock starting at [`VirtualTime::ZERO`].
    pub fn new() -> Self {
        VirtualClock {
            now: VirtualTime::ZERO,
        }
    }

    /// Return the current virtual time without advancing.
    pub fn now(&self) -> VirtualTime {
        self.now
    }

    /// Advance the clock by one tick and return the new time.
    ///
    /// This is the primary event-driven time advance operation.
    pub fn tick(&mut self) -> VirtualTime {
        self.now = self.now.next();
        self.now
    }

    /// Advance the clock by `n` ticks.
    ///
    /// If `n` is zero, this is a no-op. Saturates at `u64::MAX`.
    pub fn advance(&mut self, n: u64) {
        self.now = VirtualTime(self.now.0.saturating_add(n));
    }

    /// Reset the clock to `t`, which must be >= the current time.
    ///
    /// Returns [`TimeError::BackwardReset`] if `t < self.now()`.
    /// Used for replay and state restoration.
    pub fn reset_to(&mut self, t: VirtualTime) -> Result<(), TimeError> {
        if t < self.now {
            return Err(TimeError::BackwardReset {
                current: self.now.as_u64(),
                requested: t.as_u64(),
            });
        }
        self.now = t;
        Ok(())
    }

    /// Merge another clock into this one by taking the maximum.
    ///
    /// After merge, `self.now() >= other.now()`. This implements the
    /// causal merge rule for distributed virtual clocks.
    pub fn merge(&mut self, other: &VirtualClock) {
        if other.now > self.now {
            self.now = other.now;
        }
    }
}

impl Default for VirtualClock {
    fn default() -> Self {
        VirtualClock::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn virtual_time_zero_is_zero() {
        assert_eq!(VirtualTime::ZERO.as_u64(), 0);
    }

    #[test]
    fn virtual_time_new_stores_value() {
        assert_eq!(VirtualTime::new(42).as_u64(), 42);
    }

    #[test]
    fn virtual_time_next_increments() {
        assert_eq!(VirtualTime::new(5).next(), VirtualTime::new(6));
    }

    #[test]
    fn virtual_time_next_saturates_at_max() {
        assert_eq!(
            VirtualTime::new(u64::MAX).next(),
            VirtualTime::new(u64::MAX)
        );
    }

    #[test]
    fn virtual_time_ordering_is_correct() {
        assert!(VirtualTime::ZERO < VirtualTime::new(1));
        assert!(VirtualTime::new(10) > VirtualTime::new(9));
        assert_eq!(VirtualTime::new(7), VirtualTime::new(7));
    }

    #[test]
    fn clock_starts_at_zero() {
        let c = VirtualClock::new();
        assert_eq!(c.now(), VirtualTime::ZERO);
    }

    #[test]
    fn tick_returns_new_time() {
        let mut c = VirtualClock::new();
        assert_eq!(c.tick(), VirtualTime::new(1));
        assert_eq!(c.tick(), VirtualTime::new(2));
    }

    #[test]
    fn advance_adds_n() {
        let mut c = VirtualClock::new();
        c.advance(100);
        assert_eq!(c.now(), VirtualTime::new(100));
    }

    #[test]
    fn advance_zero_is_noop() {
        let mut c = VirtualClock::new();
        c.advance(5);
        c.advance(0);
        assert_eq!(c.now(), VirtualTime::new(5));
    }

    #[test]
    fn reset_to_forward_succeeds() {
        let mut c = VirtualClock::new();
        c.advance(3);
        assert!(c.reset_to(VirtualTime::new(10)).is_ok());
        assert_eq!(c.now(), VirtualTime::new(10));
    }

    #[test]
    fn reset_to_same_value_succeeds() {
        let mut c = VirtualClock::new();
        c.advance(5);
        assert!(c.reset_to(VirtualTime::new(5)).is_ok());
    }

    #[test]
    fn reset_to_backward_fails() {
        let mut c = VirtualClock::new();
        c.advance(10);
        let err = c.reset_to(VirtualTime::new(3)).unwrap_err();
        assert_eq!(
            err,
            TimeError::BackwardReset {
                current: 10,
                requested: 3
            }
        );
    }

    #[test]
    fn merge_takes_max_when_other_larger() {
        let mut a = VirtualClock::new();
        a.advance(5);
        let mut b = VirtualClock::new();
        b.advance(15);
        a.merge(&b);
        assert_eq!(a.now(), VirtualTime::new(15));
    }

    #[test]
    fn merge_keeps_self_when_larger() {
        let mut a = VirtualClock::new();
        a.advance(20);
        let mut b = VirtualClock::new();
        b.advance(7);
        a.merge(&b);
        assert_eq!(a.now(), VirtualTime::new(20));
    }

    #[test]
    fn merge_equal_clocks_unchanged() {
        let mut a = VirtualClock::new();
        a.advance(10);
        let mut b = VirtualClock::new();
        b.advance(10);
        a.merge(&b);
        assert_eq!(a.now(), VirtualTime::new(10));
    }

    #[test]
    fn default_clock_starts_at_zero() {
        let c = VirtualClock::default();
        assert_eq!(c.now(), VirtualTime::ZERO);
    }

    #[test]
    fn virtual_time_display() {
        assert_eq!(format!("{}", VirtualTime::new(42)), "t:42");
        assert_eq!(format!("{}", VirtualTime::ZERO), "t:0");
    }
}
