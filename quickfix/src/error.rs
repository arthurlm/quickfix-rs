use std::ffi::{CStr, NulError};

use quickfix_ffi::{Fix_clearLastErrorMessage, Fix_getLastErrorMessage};
use thiserror::Error;

/// Represent all possible error that can occurs with quickfix.
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum QuickFixError {
    /// Foreign function as return a null value.
    #[error("null function return: {0}")]
    NullFunctionReturn(String),

    /// Foreign function as return an invalid return code.
    #[error("invalid function return code: code={0}, msg={1}")]
    InvalidFunctionReturnCode(i8, String),

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

    /// Build a null function return and read associated error if any.
    pub fn null() -> Self {
        Self::NullFunctionReturn(last_quickfix_error_message_or_default())
    }
}

impl From<NulError> for QuickFixError {
    fn from(value: NulError) -> Self {
        Self::InvalidArgument(value.to_string())
    }
}

impl From<i8> for QuickFixError {
    fn from(value: i8) -> Self {
        Self::InvalidFunctionReturnCode(value, last_quickfix_error_message_or_default())
    }
}

fn last_quickfix_error_message_or_default() -> String {
    last_quickfix_error_message()
        .unwrap_or_else(|| "Cannot get last error message from quickfix library".to_string())
}

fn last_quickfix_error_message() -> Option<String> {
    unsafe {
        let raw_error = Fix_getLastErrorMessage()?;
        let error = CStr::from_ptr(raw_error.as_ptr().cast());
        let msg = error.to_string_lossy().to_string();
        Fix_clearLastErrorMessage();
        Some(msg)
    }
}
