// revela::backend
//
//! All specific supported UI backends.
//

#[cfg(feature = "notcurses")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "notcurses")))]
pub mod notcurses;
#[cfg(feature = "notcurses")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "notcurses")))]
#[doc(inline)]
pub use self::notcurses::*;
/// Re-export of the [`notcurses`](https://docs.rs/notcurses) crate.
#[doc(inline)]
#[cfg(feature = "notcurses")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "notcurses")))]
pub use ::notcurses as notcurses_crate;

// TODO
// #[cfg(feature = "gilrs")]
// #[cfg_attr(feature = "nightly", doc(cfg(feature = "gilrs")))]
// pub mod gilrs;
// #[cfg(feature = "gilrs")]
// #[cfg_attr(feature = "nightly", doc(cfg(feature = "gilrs")))]
// #[doc(inline)]
// pub use self::gilrs::GilrsUi;
// /// Re-export of the [`gilrs`](https://docs.rs/gilrs) crate.
// #[doc(inline)]
// #[cfg(feature = "gilrs")]
// #[cfg_attr(feature = "nightly", doc(cfg(feature = "gilrs")))]
// pub use ::gilrs as gilrs_crate;
