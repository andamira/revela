// revela::error::midir
//
//!
//

use super::RevelaError as Error;

pub use ::midir::{
    ConnectError as MidirConnectError, ConnectErrorKind as MidirConnectErrorKind,
    InitError as MidirInitError, PortInfoError as MidirPortInfoError,
};

///
#[derive(Debug)]
pub enum MidirError {
    Connect,
    Init(MidirInitError),
    PortInfo(MidirPortInfoError),
}

impl From<MidirInitError> for Error {
    fn from(err: MidirInitError) -> Self {
        Error::Midir(MidirError::Init(err))
    }
}
impl<T> From<MidirConnectError<T>> for Error {
    fn from(_err: MidirConnectError<T>) -> Self {
        Error::Midir(MidirError::Connect)
    }
}
