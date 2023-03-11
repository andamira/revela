// revela::error::kira
//
//!
//

use super::RevelaError as Error;

pub use ::kira::{
    manager::{backend::cpal::Error as CpalError, error::PlaySoundError},
    sound::FromFileError,
};

///
#[derive(Debug)]
pub enum KiraError {
    Cpal(CpalError),
    FromFile(FromFileError),
    // IMPROVE
    PlaySound,
}

impl From<CpalError> for Error {
    fn from(err: CpalError) -> Self {
        Error::Kira(KiraError::Cpal(err))
    }
}
impl From<FromFileError> for Error {
    fn from(err: FromFileError) -> Self {
        Error::Kira(KiraError::FromFile(err))
    }
}
impl<T> From<PlaySoundError<T>> for Error {
    fn from(_err: PlaySoundError<T>) -> Self {
        Error::Kira(KiraError::PlaySound)
    }
}
