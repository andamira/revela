// revela::backend
//
//! All specific supported UI backends.
//

pub mod capabilities;

#[cfg(feature = "no-std")]
pub mod no_std;

#[cfg(all(feature = "alloc", feature = "no-std"))]
pub mod alloc;

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
        TextGridCapabilities, WindowCapabilities,
    };
    // IMPROVE: alloc
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    pub use super::capabilities::SystemCapabilities;

    // export the panic handlers and other no-std utilities.
    #[cfg(feature = "no-std")]
    #[doc(inline)]
    pub use super::no_std::*;

    // export the global allocators and other alloc utilities.
    #[cfg(all(feature = "alloc", feature = "no-std"))]
    #[doc(inline)]
    pub use super::alloc::*;

    /* terminal */

    #[cfg(feature = "crossterm")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "crossterm")))]
    #[doc(inline)]
    pub use super::crossterm::CrosstermBackend;

    #[cfg(feature = "notcurses")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "notcurses")))]
    #[doc(inline)]
    pub use super::notcurses::{NotcursesBackend, NotcursesTextGrid};

    /* */

    #[cfg(feature = "gilrs")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "gilrs")))]
    #[doc(inline)]
    pub use super::gilrs::GilrsBackend;

    #[cfg(feature = "midir")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "midir")))]
    #[doc(inline)]
    pub use super::midir::MidirBackend;
}
