// revela::lib
//
//! A modular user interface abstracted over multiple backends.
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
        feature = "unsafe_hint", feature = "unsafe_layout", feature = "unsafe_niche",
        feature = "unsafe_ptr", feature = "unsafe_slice", feature = "unsafe_str",
        feature = "unsafe_sync", feature = "unsafe_syscall", feature = "unsafe_thread",
    )
))]
compile_error!("You can't enable the `safe` and `unsafe*` features at the same time.");

// #[cfg(feature = "alloc")]
// use devela::_liballoc;

// #[cfg(feature = "audio")]
// #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "audio")))]
// pub mod audio;
// #[cfg(feature = "color")]
// #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "color")))]
// pub mod color;
// #[cfg(feature = "font")]
// #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "font")))]
// pub mod font;
// #[cfg(feature = "gfx")]
// #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "gfx")))]
// pub mod gfx;
// #[cfg(feature = "ui")]
// #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "ui")))]
// pub mod ui;
// #[cfg(feature = "video")]
// #[cfg_attr(feature = "nightly_doc", doc(cfg(feature = "video")))]
// pub mod video;

///
pub mod _dep {
    #[doc(inline)]
    pub use devela;
}

/// All the items are reexported here.
pub mod _all {
    // #[doc(inline)]
    // pub use super::{backend::all::*, error::*, event::all::*, visual::all::*};
}

/// Information about the library
pub mod _info {
    // /// Documented examples.
    // #[cfg(any(doc, test))]
    // pub mod examples;

    /// Cargo features.
    pub mod features {
        #![cfg_attr(not(feature = "all"), allow(rustdoc::private_intra_doc_links))]
        #![doc = include_str!("./_info/features.md")]
    }
}
