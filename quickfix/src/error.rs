use std::ffi::NulError;

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum QuickFixError {
    #[error("invalid function return")]
    InvalidFunctionReturn,

    #[error("invalid function return code")]
    InvalidFunctionReturnCode(i32),

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

impl QuickFixError {
    pub fn invalid_argument<T: Into<String>>(msg: T) -> Self {
        Self::InvalidArgument(msg.into())
    }
}

impl From<NulError> for QuickFixError {
    fn from(value: NulError) -> Self {
        Self::InvalidArgument(value.to_string())
    }
}
