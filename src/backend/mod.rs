// revela::backend
//
//! All specific supported UI backends.
//

mod backend;
pub mod capabilities;
mod text;
mod window;

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
#[cfg(feature = "sdl2")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sdl2")))]
pub mod sdl2;

pub use all::*;
pub(crate) mod all {
    #[doc(inline)]
    pub use super::{
        backend::Backend,
        capabilities::{
            Capabilities, ColorCapabilities, InputCapabilities, PixelCapabilities,
            SoundCapabilities, TextGridCapabilities, WindowCapabilities,
        },
        text::TextGrid,
        window::Window,
    };
    // IMPROVE: alloc
    #[doc(inline)]
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    pub use super::capabilities::SystemCapabilities;

    // export the panic handlers and other no-std utilities.
    #[doc(inline)]
    #[cfg(feature = "no-std")]
    pub use super::no_std::*;

    // export the global allocators and other alloc utilities.
    #[doc(inline)]
    #[cfg(all(feature = "alloc", feature = "no-std"))]
    pub use super::alloc::*;

    /* terminal */

    #[doc(inline)]
    #[cfg(feature = "crossterm")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "crossterm")))]
    pub use super::crossterm::CrosstermBackend;

    #[doc(inline)]
    #[cfg(feature = "notcurses")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "notcurses")))]
    pub use super::notcurses::{NotcursesBackend, NotcursesTextGrid};

    /* */

    #[doc(inline)]
    #[cfg(feature = "sdl2")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "sdl2")))]
    pub use super::sdl2::{Sdl2Backend, Sdl2EventSource};

    /* */

    #[doc(inline)]
    #[cfg(feature = "gilrs")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "gilrs")))]
    pub use super::gilrs::GilrsBackend;

    #[doc(inline)]
    #[cfg(feature = "midir")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "midir")))]
    pub use super::midir::MidirBackend;
}
