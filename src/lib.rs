// revela
//
//! A user interface abstracted over a selection of [`backend`]s.
//

#![warn(clippy::all)]
#![allow(
    clippy::float_arithmetic,
    clippy::implicit_return,
    clippy::needless_return,
    clippy::blanket_clippy_restriction_lints,
    clippy::pattern_type_mismatch,
    uncommon_codepoints
)]
//
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

pub mod backend;
pub mod core;
pub mod error;
pub mod layout;

/// Everything is directly available in here.
pub mod all {
    #[doc(inline)]
    pub use crate::{
        backend::capabilities::{Capabilities, PixelCapabilities, TextGridCapabilities},
        core::all::*,
        error::{RevelaError, RevelaResult},
        layout::{Clamper, Position, Size, Zone},
    };

    #[cfg(feature = "notcurses")]
    #[doc(inline)]
    pub use crate::backend::notcurses::{NotcursesTextGrid, NotcursesUi};

    #[cfg(feature = "gilrs")]
    #[doc(inline)]
    pub use crate::backend::gilrs::GilrsUi;

    #[cfg(feature = "midir")]
    #[doc(inline)]
    pub use crate::backend::midir::MidirUi;
}
