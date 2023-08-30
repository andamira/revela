// revela::event::time
//
//!
//

use core::{fmt, num::NonZeroU64};
#[cfg(feature = "unsafe_constructors")]
use devela::iif;

/// The time at which the event actually occurs.
///
/// Backend dependant, relative to an arbitrary moment, usually in microseconds.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct EventTimeStamp(NonZeroU64);

impl EventTimeStamp {
    /// New time stamp, starting at 1 µs.
    pub fn new(µs: u64) -> EventTimeStamp {
        #[cfg(not(feature = "unsafe_constructors"))]
        return EventTimeStamp(NonZeroU64::new(µs).or(NonZeroU64::new(1)).unwrap());

        // TODO:CHECK:BENCH
        #[cfg(feature = "unsafe_constructors")]
        // SAFETY: we make sure to never pass 0
        return EventTimeStamp(unsafe { NonZeroU64::new_unchecked(iif![µs==0;1;µs]) });
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
