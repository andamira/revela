// revela::error
//
//! Error types.
//

use core::result;

// NOTE: crossterm error type is an alias of std::io::Error
#[cfg(feature = "std")]
use std::io::Error as IoError;

#[cfg(feature = "notcurses")]
use ::notcurses::Error as NotcursesError;

#[cfg(feature = "gilrs")]
use ::gilrs::Error as GilrsError;

#[cfg(feature = "midir")]
mod midir;
#[cfg(feature = "midir")]
pub use self::midir::{MidirError, MidirInitError, MidirPortInfoError};

// #[cfg(feature = "midi-convert")]
pub use ::midi_convert::MidiParseError as MidiConvertParseError;

#[cfg(feature = "flume")]
mod flume;

// #[cfg(feature = "sdl2")]
// use sdl2::Error as Sdl2Error;

// use png::EncodingError as PngEncodingError;

/// Main *revela* result type.
pub type RevelaResult<N> = result::Result<N, RevelaError>;

/// Main *revela* error type.
#[non_exhaustive]
#[derive(Debug)]
pub enum RevelaError {
    /// A [`notcurses`][::notcurses] error.
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

    /// A [`gilrs`][`::gilrs`] error.
    // https://docs.rs/gilrs/latest/gilrs/enum.Error.html
    #[cfg(feature = "gilrs")]
    Gilrs(GilrsError),

    /// A [`midir`][::midir] error.
    #[cfg(feature = "midir")]
    Midir(MidirError),

    /// A [`flume`][::flume] error.
    #[cfg(feature = "flume")]
    Flume,

    /// A [`midi-convert`][::midi-convert] error.
    // #[cfg(feature = "midi-convert")]
    MidiConvert(MidiConvertParseError),

    // /// A [`png`] encoding error.
    // PngEncoding(PngEncodingError),

    // TODO: e.g. for poll_event, etc.
    // Unsupported,
    // /// A failed conversion error.
    // FailedConversion(String, String),
    /// This functionality is not supported.
    NotSupported,

    /// A custom error message.
    #[cfg(feature = "std")]
    String(String),
}
impl RevelaError {
    /// Returns a `string` error.
    #[cfg(feature = "std")]
    pub fn string(string: impl ToString) -> Self {
        Self::String(string.to_string())
    }
}

#[cfg(feature = "gilrs")]
mod gilrs_impls {
    use super::{GilrsError, RevelaError};

    impl From<GilrsError> for RevelaError {
        fn from(err: GilrsError) -> Self {
            RevelaError::Gilrs(err)
        }
    }
}

#[cfg(feature = "notcurses")]
mod notcurses_impls {
    use super::{NotcursesError, RevelaError};

    impl From<NotcursesError> for RevelaError {
        fn from(err: NotcursesError) -> Self {
            RevelaError::Notcurses(err)
        }
    }
}

// mod png_impls {
//     use super::{PngEncodingError, RevelaError};
//
//     impl From<PngEncodingError> for RevelaError {
//         fn from(err: PngEncodingError) -> Self {
//             RevelaError::PngEncoding(err)
//         }
//     }
// }

// #[cfg(feature = "sdl2")]
// mod sdl2_impls {
//     use super::{RevelaError, Sdl2Error};
//
//     impl RevelaError {
//         // https://docs.rs/sdl2/latest/sdl2/fn.get_error.html
//         // https://docs.rs/sdl2-sys/latest/sdl2_sys/fn.SDL_GetError.html
//         pub fn get_error() -> Self {
//             Self::new(&sdl2::get_error())
//         }
//     }
//
//     impl From<Sdl2Error> for RevelaError {
//         fn from(err: Sdl2Error) -> Self {
//             RevelaError::Sdl2(err)
//         }
//     }
// }

mod core_impls {
    use super::RevelaError;
    use core::fmt;

    impl fmt::Display for RevelaError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                #[cfg(feature = "notcurses")]
                RevelaError::Notcurses(e) => fmt::Debug::fmt(e, f),

                // #[cfg(feature = "sdl2")]
                // RevelaError::Sdl2(e) => Debug::fmt(e, f),
                #[cfg(feature = "std")]
                RevelaError::Io(e) => fmt::Debug::fmt(e, f),

                #[cfg(feature = "gilrs")]
                RevelaError::Gilrs(e) => fmt::Debug::fmt(e, f),

                #[cfg(feature = "midir")]
                RevelaError::Midir(e) => fmt::Debug::fmt(e, f),

                // #[cfg(feature = "midi-convert")]
                RevelaError::MidiConvert(e) => fmt::Debug::fmt(e, f),

                #[cfg(feature = "flume")]
                RevelaError::Flume => write!(f, "Flume error"),

                // RevelaError::PngEncoding(e) => Debug::fmt(e, f),
                //
                // RevelaError::FailedConversion(from, to) => write!(f, "FailedConversion {from} -> {to}"),
                RevelaError::NotSupported => write!(f, "NotSupported"),

                #[cfg(feature = "std")]
                RevelaError::String(e) => write!(f, "{}", e),
                // #[allow(unreachable_patterns)]
                // _ => write!(f, "RevelaError"),
            }
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
mod std_impls {
    use super::{IoError, RevelaError};
    use std::error::Error as StdError;

    impl StdError for RevelaError {}

    impl From<IoError> for RevelaError {
        fn from(err: IoError) -> Self {
            RevelaError::Io(err)
        }
    }

    impl From<String> for RevelaError {
        fn from(err: String) -> Self {
            RevelaError::String(err)
        }
    }
}
