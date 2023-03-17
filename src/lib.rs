// revela
//
//! A user interface abstracted over a selection of [`backend`]s.
//

#![warn(clippy::all)]
#![allow(uncommon_codepoints, clippy::module_inception, non_upper_case_globals)]
//
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

#[cfg(all(feature = "std", feature = "no-std"))]
compile_error!("You can't have both the `std` and `no-std` features at the same time.");

#[cfg(feature = "alloc")]
extern crate alloc;

pub mod backend;
mod canvas;
pub mod error;
pub mod event;
pub mod layout;

/// Everything is directly available in here.
pub mod all {
    #[doc(inline)]
    pub use super::{
        backend::all::*,
        canvas::*,
        error::*,
        event::all::*,
        layout::{Clamper, Position, Size, Zone},
    };
}
