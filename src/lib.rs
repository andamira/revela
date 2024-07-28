// revela::lib
//
//! A cohesive user interface layer.
//!
#![doc = include_str!("./Lib.md")]
//

//* global config *//
//
// warnings:
#![warn(clippy::all)]
//
// safety:
#![cfg_attr(feature = "safe", forbid(unsafe_code))]
//
// environment:
#![cfg_attr(not(feature = "std"), no_std)]
//
// nightly:
#![cfg_attr(feature = "nightly", feature(doc_cfg))]

// safeguard environment:
#[cfg(all(feature = "std", feature = "no_std"))]
compile_error!("You can't enable the `std` and `no_std` features at the same time.");
// safeguard safety:
#[cfg(all(
    feature = "safe",
    any(feature = "unsafe", // includes all below:
        feature = "unsafe_array", feature = "unsafe_async", feature = "unsafe_const",
        feature = "unsafe_dyn", feature = "unsafe_niche", feature = "unsafe_ptr",
        feature = "unsafe_slice", feature = "unsafe_str",
    )
))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");

// #[cfg(feature = "alloc")]
// use devela::_liballoc;

// pub mod backend;
// pub mod error;
// pub mod event;
// pub mod visual;

/// All the items are reexported here.
pub mod all {
    // #[doc(inline)]
    // pub use super::{backend::all::*, error::*, event::all::*, visual::all::*};
}
