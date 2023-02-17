// revela::error::midir
//
//!
//

use super::UiError;

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

impl From<MidirInitError> for UiError {
    fn from(err: MidirInitError) -> Self {
        UiError::Midir(MidirError::Init(err))
    }
}
impl<T> From<MidirConnectError<T>> for UiError {
    fn from(_err: MidirConnectError<T>) -> Self {
        UiError::Midir(MidirError::Connect)
    }
}
