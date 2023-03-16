// revela::core::events::time
//
//!
//

use core::{fmt, num::NonZeroU64};

/// The time at which the event actually occurs.
///
/// Backend dependant, relative to an arbitrary moment, usually in microseconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventTimeStamp(NonZeroU64);

impl EventTimeStamp {
    /// New time stamp, starting at 1 µs.
    pub fn new(µs: u64) -> EventTimeStamp {
        #[cfg(feature = "safe")]
        return EventTimeStamp(NonZeroU64::new(µs).or(NonZeroU64::new(1)).unwrap());

        // CHECK BENCH
        #[cfg(not(feature = "safe"))]
        return EventTimeStamp(
            NonZeroU64::new(µs).unwrap_or(unsafe { NonZeroU64::new_unchecked(1) }),
        );
    }
}

impl Default for EventTimeStamp {
    fn default() -> Self {
        EventTimeStamp(NonZeroU64::new(1).unwrap())
    }
}

impl From<u64> for EventTimeStamp {
    fn from(µs: u64) -> Self {
        Self::new(µs)
    }
}

impl fmt::Display for EventTimeStamp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
