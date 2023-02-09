// uiuiui::error
//
//! Error types.
//

use core::result;

#[cfg(feature = "std")]
use std::io::Error as IoError;

#[cfg(feature = "notcurses")]
use notcurses::Error as NotcursesError;

#[cfg(feature = "gilrs")]
use gilrs::Error as GilrsError;

// #[cfg(feature = "sdl2")]
// use sdl2::Error as Sdl2Error;

// use png::EncodingError as PngEncodingError;

/// Main *revela* result type.
pub type UiResult<N> = result::Result<N, UiError>;

/// Main *revela* error type.
#[non_exhaustive]
#[derive(Debug)]
pub enum UiError {
    /// A [`notcurses`] error.
    // https://docs.rs/notcurses/latest/notcurses/enum.Error.html
    #[cfg(feature = "notcurses")]
    Notcurses(NotcursesError),

    // /// An [`sdl2`] error.
    // #[cfg(feature = "sdl2")]
    // Sdl2(Sdl2Error),
    /// An io error.
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    Io(IoError),

    /// A [`gilrs`] error.
    // https://docs.rs/gilrs/latest/gilrs/enum.Error.html
    #[cfg(feature = "gilrs")]
    Gilrs(GilrsError),

    // /// A [`png`] encoding error.
    // PngEncoding(PngEncodingError),

    // TODO: e.g. for poll_event, etc.
    // Unsupported,
    // /// A failed conversion error.
    // FailedConversion(String, String),
    /// This functionality is not supported.
    NotSupported,
}

// mod png_impls {
//     use super::{PngEncodingError, UiError};
//
//     impl From<PngEncodingError> for UiError {
//         fn from(err: PngEncodingError) -> Self {
//             UiError::PngEncoding(err)
//         }
//     }
// }

#[cfg(feature = "notcurses")]
mod notcurses_impls {
    use super::{NotcursesError, UiError};

    impl From<NotcursesError> for UiError {
        fn from(err: NotcursesError) -> Self {
            UiError::Notcurses(err)
        }
    }
}
// #[cfg(feature = "sdl2")]
// mod sdl2_impls {
//     use super::{UiError, Sdl2Error};
//
//     impl UiError {
//         // https://docs.rs/sdl2/latest/sdl2/fn.get_error.html
//         // https://docs.rs/sdl2-sys/latest/sdl2_sys/fn.SDL_GetError.html
//         pub fn get_error() -> Self {
//             Self::new(&sdl2::get_error())
//         }
//     }
//
//     impl From<Sdl2Error> for UiError {
//         fn from(err: Sdl2Error) -> Self {
//             UiError::Sdl2(err)
//         }
//     }
// }

mod core_impls {
    use super::UiError;
    use core::fmt::{self, Debug};

    #[cfg(feature = "gilrs")]
    use super::GilrsError;

    impl fmt::Display for UiError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                #[cfg(feature = "notcurses")]
                UiError::Notcurses(e) => Debug::fmt(e, f),

                // #[cfg(feature = "sdl2")]
                // UiError::Sdl2(e) => Debug::fmt(e, f),
                #[cfg(feature = "std")]
                UiError::Io(e) => Debug::fmt(e, f),

                #[cfg(feature = "gilrs")]
                UiError::Gilrs(e) => Debug::fmt(e, f),

                // UiError::PngEncoding(e) => Debug::fmt(e, f),
                //
                // UiError::FailedConversion(from, to) => write!(f, "FailedConversion {from} -> {to}"),
                UiError::NotSupported => write!(f, "NotSupported"),
                // #[allow(unreachable_patterns)]
                // _ => write!(f, "UiError"),
            }
        }
    }

    #[cfg(feature = "gilrs")]
    impl From<GilrsError> for UiError {
        fn from(err: GilrsError) -> Self {
            UiError::Gilrs(err)
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
mod std_impls {
    use super::{IoError, UiError};
    use std::error::Error as StdError;

    impl StdError for UiError {}

    impl From<IoError> for UiError {
        fn from(err: IoError) -> Self {
            UiError::Io(err)
        }
    }
}
