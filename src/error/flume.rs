// revela::error::flume
//
//!
//

use super::UiError;

pub use ::flume::{
    RecvError, RecvTimeoutError, SendError, SendTimeoutError, TryRecvError, TrySendError,
};

impl From<RecvError> for UiError {
    fn from(_err: RecvError) -> Self {
        UiError::Flume
    }
}
impl From<TryRecvError> for UiError {
    fn from(_err: TryRecvError) -> Self {
        UiError::Flume
    }
}
impl From<RecvTimeoutError> for UiError {
    fn from(_err: RecvTimeoutError) -> Self {
        UiError::Flume
    }
}
impl<T> From<SendError<T>> for UiError {
    fn from(_err: SendError<T>) -> Self {
        UiError::Flume
    }
}
impl<T> From<TrySendError<T>> for UiError {
    fn from(_err: TrySendError<T>) -> Self {
        UiError::Flume
    }
}

impl<T> From<SendTimeoutError<T>> for UiError {
    fn from(_err: SendTimeoutError<T>) -> Self {
        UiError::Flume
    }
}
