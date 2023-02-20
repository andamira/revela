// revela::backend
//
//! All specific supported UI backends.
//

pub mod capabilities;

#[cfg(feature = "crossterm")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "crossterm")))]
pub mod crossterm;
#[cfg(feature = "gilrs")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "gilrs")))]
pub mod gilrs;
#[cfg(feature = "midir")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "midir")))]
pub mod midir;
#[cfg(feature = "notcurses")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "notcurses")))]
pub mod notcurses;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::capabilities::{
        Capabilities, ColorCapabilities, InputCapabilities, PixelCapabilities, SoundCapabilities,
        SystemCapabilities, TextGridCapabilities, WindowCapabilities,
    };

    #[cfg(feature = "crossterm")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "crossterm")))]
    #[doc(inline)]
    pub use super::crossterm::CrosstermBackend;

    #[cfg(feature = "notcurses")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "notcurses")))]
    #[doc(inline)]
    pub use super::notcurses::{NotcursesBackend, NotcursesTextGrid};

    #[cfg(feature = "gilrs")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "gilrs")))]
    #[doc(inline)]
    pub use super::gilrs::GilrsBackend;

    #[cfg(feature = "midir")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "midir")))]
    #[doc(inline)]
    pub use super::midir::MidirBackend;
}
