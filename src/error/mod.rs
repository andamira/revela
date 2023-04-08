// revela::error
//
//! Error types.
//

use core::result;

// use ladata::error::LadataError;

#[cfg(feature = "alloc")]
use alloc::string::String;

// NOTE: crossterm error type is an alias of std::io::Error
#[cfg(feature = "std")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
pub use std::io::Error as IoError;

/// `notcurses` error type.
///
#[cfg(feature = "notcurses")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "notcurses")))]
pub use ::notcurses::NotcursesError;

/// `gilrs` error type.
///
#[cfg(feature = "gilrs")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "gilrs")))]
pub use ::gilrs::Error as GilrsError;

#[cfg(feature = "midir")]
mod midir;
#[cfg(feature = "midir")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "midir")))]
pub use self::midir::{MidirError, MidirInitError, MidirPortInfoError};

/// Midi parsing error.
///
// #[cfg(feature = "midi-convert")]
pub use ::midi_convert::MidiParseError as MidiConvertParseError;

#[cfg(feature = "kira")]
mod kira;
#[cfg(feature = "kira")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "kira")))]
pub use self::kira::KiraError;

#[cfg(feature = "flume")]
mod flume;

/// `sdl` error type.
#[cfg(feature = "sdl2")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sdl2")))]
pub use sdl2::Error as Sdl2Error;

/// `png` error type.
#[cfg(feature = "png")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "png")))]
pub use png::EncodingError as PngEncodingError;

/// Main *revela* result type.
pub type RevelaResult<N> = result::Result<N, RevelaError>;

/// Main *revela* error type.
#[non_exhaustive]
#[derive(Debug)]
pub enum RevelaError {
    /// A [`notcurses`][::notcurses] error.
    // https://docs.rs/notcurses/latest/notcurses/enum.Error.html
    #[cfg(feature = "notcurses")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "notcurses")))]
    Notcurses(NotcursesError),

    /// An [`sdl2`] error.
    #[cfg(feature = "sdl2")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "sdl2")))]
    Sdl2(Sdl2Error),

    /// An io error.
    #[cfg(feature = "std")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "std")))]
    Io(IoError),

    /// A [`gilrs`][`::gilrs`] error.
    // https://docs.rs/gilrs/latest/gilrs/enum.Error.html
    #[cfg(feature = "gilrs")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "gilrs")))]
    Gilrs(GilrsError),

    /// A [`midir`][::midir] error.
    #[cfg(feature = "midir")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "midir")))]
    Midir(MidirError),

    /// A [`flume`][::flume] error.
    #[cfg(feature = "flume")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "flume")))]
    Flume,

    #[cfg(feature = "kira")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "kira")))]
    Kira(KiraError),

    /// A [`midi-convert`][::midi-convert] error.
    // #[cfg(feature = "midi-convert")]
    MidiConvert(MidiConvertParseError),

    /// A [`png`] encoding error.
    #[cfg(feature = "png")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "png")))]
    PngEncoding(PngEncodingError),

    // TODO: e.g. for poll_event, etc.
    // Unsupported,
    // /// A failed conversion error.
    // FailedConversion(String, String),
    /// This functionality is not supported.
    NotSupported,

    // /// A `ladata` error.
    // Ladata(LadataError),
    /// A custom error message.
    #[cfg(feature = "alloc")]
    #[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
    String(String),
}

#[cfg(feature = "gilrs")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "gilrs")))]
mod gilrs_impls {
    use super::{GilrsError, RevelaError};

    impl From<GilrsError> for RevelaError {
        fn from(err: GilrsError) -> Self {
            RevelaError::Gilrs(err)
        }
    }
}

#[cfg(feature = "notcurses")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "notcurses")))]
mod notcurses_impls {
    use super::{NotcursesError, RevelaError};

    impl From<NotcursesError> for RevelaError {
        fn from(err: NotcursesError) -> Self {
            RevelaError::Notcurses(err)
        }
    }
}

#[cfg(feature = "png")]
mod png_impls {
    use super::{PngEncodingError, RevelaError};

    impl From<PngEncodingError> for RevelaError {
        fn from(err: PngEncodingError) -> Self {
            RevelaError::PngEncoding(err)
        }
    }
}

#[cfg(feature = "sdl2")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "sdl2")))]
mod sdl2_impls {
    use super::{RevelaError, Sdl2Error};

    impl From<Sdl2Error> for RevelaError {
        fn from(err: Sdl2Error) -> Self {
            RevelaError::Sdl2(err)
        }
    }
}

mod core_impls {
    // use super::{LadataError, RevelaError};
    use super::RevelaError;
    use core::fmt;

    impl fmt::Display for RevelaError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                #[cfg(feature = "notcurses")]
                RevelaError::Notcurses(e) => fmt::Debug::fmt(e, f),

                #[cfg(feature = "sdl2")]
                RevelaError::Sdl2(e) => fmt::Debug::fmt(e, f),

                #[cfg(feature = "std")]
                RevelaError::Io(e) => fmt::Debug::fmt(e, f),

                #[cfg(feature = "gilrs")]
                RevelaError::Gilrs(e) => fmt::Debug::fmt(e, f),

                #[cfg(feature = "midir")]
                RevelaError::Midir(e) => fmt::Debug::fmt(e, f),

                // #[cfg(feature = "midi-convert")]
                RevelaError::MidiConvert(e) => fmt::Debug::fmt(e, f),

                #[cfg(feature = "kira")]
                RevelaError::Kira(e) => fmt::Debug::fmt(e, f),

                #[cfg(feature = "flume")]
                RevelaError::Flume => write!(f, "Flume error"),

                #[cfg(feature = "png")]
                RevelaError::PngEncoding(e) => fmt::Debug::fmt(e, f),

                // RevelaError::FailedConversion(from, to) => write!(f, "FailedConversion {from} -> {to}"),
                RevelaError::NotSupported => write!(f, "NotSupported"),

                // WIP
                // RevelaError::Ladata(e) => fmt::Debug::fmt(e, f),
                #[cfg(feature = "alloc")]
                RevelaError::String(e) => write!(f, "{}", e),
                // #[allow(unreachable_patterns)]
                // _ => write!(f, "RevelaError"),
            }
        }
    }

    // impl From<LadataError> for RevelaError {
    //     fn from(err: LadataError) -> Self {
    //         RevelaError::Ladata(err)
    //     }
    // }
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
}

#[cfg(feature = "alloc")]
#[cfg_attr(feature = "nightly", doc(cfg(feature = "alloc")))]
mod alloc_impls {
    use super::{RevelaError, String};
    use alloc::string::ToString;

    impl RevelaError {
        /// Returns a `string` error.
        pub fn string(string: impl ToString) -> Self {
            Self::String(string.to_string())
        }
    }

    impl From<String> for RevelaError {
        fn from(err: String) -> Self {
            RevelaError::String(err)
        }
    }
}
