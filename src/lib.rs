// revela::lib
//
//! A user interface abstracted over a selection of [`backend`]s.
//!
#![doc = include_str!("./Lib.md")]
//

// warnings
#![warn(clippy::all)]
#![allow(uncommon_codepoints, clippy::module_inception, non_upper_case_globals)]
// nightly, safety, environment
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(not(feature = "std"), no_std)]
#[cfg(feature = "alloc")]
extern crate alloc;

// safeguards
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
#[cfg(all(
    feature = "safe",
    any(feature = "unsafe", feature = "unsafe_init", feature = "unsafe_libc")
))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");

pub mod backend;
pub mod error;
pub mod event;
pub mod visual;

/// All items are reexported here.
pub mod all {
    #[doc(inline)]
    pub use super::{backend::all::*, error::*, event::all::*, visual::all::*};
}
