// revela::lib
//
//! A user interface abstracted over a selection of [`backend`]s.
//

// warnings
#![warn(clippy::all)]
#![allow(uncommon_codepoints, clippy::module_inception, non_upper_case_globals)]
// environment
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
#![cfg_attr(feature = "nightly", feature(doc_cfg))]
#[cfg(feature = "alloc")]
extern crate alloc;

// safeguards
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
#[cfg(all(feature = "safe", feature = "nonsafe"))]
compile_error!("You can't enable the `safe` and `unsafe` features at the same time.");
// deprecated
devela::deprecate_feature![old: "no-std", new: "no_std", since: "0.0.8"];
devela::deprecate_feature![old: "backends_no-std", new: "backends_no_std", since: "0.0.8"];

pub mod backend;
pub mod error;
pub mod event;
pub mod visual;

/// All items are reexported here.
pub mod all {
    #[doc(inline)]
    pub use super::{backend::all::*, error::*, event::all::*, visual::all::*};
}
