// revela::error::flume
//
//!
//

use super::RevelaError as Error;

pub use ::flume::{
    RecvError, RecvTimeoutError, SendError, SendTimeoutError, TryRecvError, TrySendError,
};

impl From<RecvError> for Error {
    fn from(_err: RecvError) -> Self {
        Error::Flume
    }
}
impl From<TryRecvError> for Error {
    fn from(_err: TryRecvError) -> Self {
        Error::Flume
    }
}
impl From<RecvTimeoutError> for Error {
    fn from(_err: RecvTimeoutError) -> Self {
        Error::Flume
    }
}
impl<T> From<SendError<T>> for Error {
    fn from(_err: SendError<T>) -> Self {
        Error::Flume
    }
}
impl<T> From<TrySendError<T>> for Error {
    fn from(_err: TrySendError<T>) -> Self {
        Error::Flume
    }
}

impl<T> From<SendTimeoutError<T>> for Error {
    fn from(_err: SendTimeoutError<T>) -> Self {
        Error::Flume
    }
}
