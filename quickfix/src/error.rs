use std::ffi::NulError;

use thiserror::Error;

/// Represent all possible error that can occurs with quickfix.
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum QuickFixError {
    /// Foreign function as return a null value.
    #[error("null function return")]
    NullFunctionReturn,

    /// Foreign function as return an invalid return code.
    #[error("invalid function return code: {0}")]
    InvalidFunctionReturnCode(i8),

    /// Cannot compute required buffer len to move data from cpp to rust.
    #[error("invalid buffer len")]
    InvalidBufferLen,

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

impl From<i8> for QuickFixError {
    fn from(value: i8) -> Self {
        Self::InvalidFunctionReturnCode(value)
    }
}
