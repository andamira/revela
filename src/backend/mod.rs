// revela::backend
//
//! All specific supported UI backends.
//

pub mod capabilities;
#[doc(inline)]
pub use capabilities::{Capabilities, PixelCapabilities, TextGridCapabilities};

#[cfg(feature = "crossterm")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "crossterm")))]
pub mod crossterm;
#[cfg(feature = "crossterm")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "crossterm")))]
#[doc(inline)]
pub use self::crossterm::CrosstermBackend;

#[cfg(feature = "notcurses")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "notcurses")))]
pub mod notcurses;
#[cfg(feature = "notcurses")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "notcurses")))]
#[doc(inline)]
pub use self::notcurses::{NotcursesBackend, NotcursesTextGrid};

#[cfg(feature = "gilrs")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "gilrs")))]
pub mod gilrs;
#[cfg(feature = "gilrs")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "gilrs")))]
#[doc(inline)]
pub use self::gilrs::GilrsBackend;

#[cfg(feature = "midir")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "midir")))]
pub mod midir;
#[cfg(feature = "midir")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "midir")))]
#[doc(inline)]
pub use self::midir::MidirBackend;
