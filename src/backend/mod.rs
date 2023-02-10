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
pub use self::notcurses::NotcursesUi;

// TODO
// #[cfg(feature = "gilrs")]
// #[cfg_attr(feature = "nightly", doc(cfg(feature = "gilrs")))]
// pub mod gilrs;
// #[cfg(feature = "gilrs")]
// #[cfg_attr(feature = "nightly", doc(cfg(feature = "gilrs")))]
// #[doc(inline)]
// pub use self::gilrs::GilrsUi;
