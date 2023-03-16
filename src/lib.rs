// revela
//
//! A user interface abstracted over a selection of [`backend`]s.
//

#![warn(clippy::all)]
#![allow(
    uncommon_codepoints,
    clippy::module_inception,
    non_upper_case_globals,
)]
//
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(all(feature = "std", feature = "no-std"))]
compile_error!("You can't have both the `std` and `no-std` features at the same time.");

#[cfg(feature = "alloc")]
pub extern crate alloc;

pub mod backend;
pub mod error;
pub mod events;
pub mod layout;
mod text;
mod window;

/// Everything is directly available in here.
pub mod all {
    #[doc(inline)]
    pub use crate::{
        backend::all::*,
        events::all::*,
        error::*,
        layout::{Clamper, Position, Size, Zone},
        text::TextGrid, window::Window,
    };
}
