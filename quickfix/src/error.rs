use std::ffi::NulError;

use thiserror::Error;

/// Represent all possible error that can occurs with quickfix.
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum QuickFixError {
    /// Foreign function as return a null value.
    #[error("null function return")]
    NullFunctionReturn,

    /// Foreign function as return an invalid return code.
    #[error("invalid function return code")]
    InvalidFunctionReturnCode(i8),

    /// Cannot pass function argument to quickfix.
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

impl QuickFixError {
    /// Helper to create a new `Self::InvalidArgument` value.
    pub fn invalid_argument<T: Into<String>>(msg: T) -> Self {
        Self::InvalidArgument(msg.into())
    }
}

impl From<NulError> for QuickFixError {
    fn from(value: NulError) -> Self {
        Self::InvalidArgument(value.to_string())
    }
}
