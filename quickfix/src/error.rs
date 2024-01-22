use std::ffi::{CStr, NulError};

use quickfix_ffi::{Fix_clearLastErrorMessage, Fix_getLastErrorCode, Fix_getLastErrorMessage};
use thiserror::Error;

/// Represent all possible error that can occurs with quickfix.
#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum QuickFixError {
    /// Foreign function as return an invalid return code.
    #[error("invalid function return code: code={0}, msg={1}")]
    InvalidFunctionReturnCode(i8, String),

    /// Cannot compute required buffer len to move data from cpp to rust.
    #[error("invalid buffer len")]
    InvalidBufferLen,

    /// Cannot pass function argument to quickfix.
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    /// Direct mapping to quickfix `FIX::DataDictionaryNotFound` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    DataDictionaryNotFound(String),

    /// Direct mapping to quickfix `FIX::FieldNotFound` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    FieldNotFound(String),

    /// Direct mapping to quickfix `FIX::FieldConvertError` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    FieldConvertError(String),

    /// Direct mapping to quickfix `FIX::MessageParseError` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    MessageParseError(String),

    /// Direct mapping to quickfix `FIX::InvalidMessage` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    InvalidMessage(String),

    /// Direct mapping to quickfix `FIX::ConfigError` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    ConfigError(String),

    /// Direct mapping to quickfix `FIX::RuntimeError` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    RuntimeError(String),

    /// Direct mapping to quickfix `FIX::InvalidTagNumber` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    InvalidTagNumber(String),

    /// Direct mapping to quickfix `FIX::RequiredTagMissing` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    RequiredTagMissing(String),

    /// Direct mapping to quickfix `FIX::TagNotDefinedForMessage` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    TagNotDefinedForMessage(String),

    /// Direct mapping to quickfix `FIX::NoTagValue` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    NoTagValue(String),

    /// Direct mapping to quickfix `FIX::IncorrectTagValue` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    IncorrectTagValue(String),

    /// Direct mapping to quickfix `FIX::IncorrectDataFormat` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    IncorrectDataFormat(String),

    /// Direct mapping to quickfix `FIX::IncorrectMessageStructure` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    IncorrectMessageStructure(String),

    /// Direct mapping to quickfix `FIX::DuplicateFieldNumber` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    DuplicateFieldNumber(String),

    /// Direct mapping to quickfix `FIX::InvalidMessageType` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    InvalidMessageType(String),

    /// Direct mapping to quickfix `FIX::UnsupportedMessageType` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    UnsupportedMessageType(String),

    /// Direct mapping to quickfix `FIX::UnsupportedVersion` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    UnsupportedVersion(String),

    /// Direct mapping to quickfix `FIX::TagOutOfOrder` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    TagOutOfOrder(String),

    /// Direct mapping to quickfix `FIX::RepeatedTag` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    RepeatedTag(String),

    /// Direct mapping to quickfix `FIX::RepeatingGroupCountMismatch` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    RepeatingGroupCountMismatch(String),

    /// Direct mapping to quickfix `FIX::DoNotSend` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    DoNotSend(String),

    /// Direct mapping to quickfix `FIX::RejectLogon` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    RejectLogon(String),

    /// Direct mapping to quickfix `FIX::SessionNotFound` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    SessionNotFound(String),

    /// Direct mapping to quickfix `FIX::IOException` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    IOException(String),

    /// Direct mapping to quickfix `FIX::SocketException` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    SocketException(String),

    /// Direct mapping to quickfix `FIX::SocketSendFailed` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    SocketSendFailed(String),

    /// Direct mapping to quickfix `FIX::SocketRecvFailed` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    SocketRecvFailed(String),

    /// Direct mapping to quickfix `FIX::SocketCloseFailed` exception found in `Exceptions.h`.
    #[error("quickfix: {0}")]
    SocketCloseFailed(String),
}

impl QuickFixError {
    /// Helper to create a new `Self::InvalidArgument` value.
    pub fn invalid_argument<T: Into<String>>(msg: T) -> Self {
        Self::InvalidArgument(msg.into())
    }

    /// Build a null function return and read associated error if any.
    pub fn from_last_error() -> Self {
        // Bellow error code should match what we have in quickfix_bind library.
        match unsafe { Fix_getLastErrorCode() } {
            -10 => Self::DataDictionaryNotFound(last_quickfix_error_message_or_default()),
            -11 => Self::FieldNotFound(last_quickfix_error_message_or_default()),
            -12 => Self::FieldConvertError(last_quickfix_error_message_or_default()),
            -13 => Self::MessageParseError(last_quickfix_error_message_or_default()),
            -14 => Self::InvalidMessage(last_quickfix_error_message_or_default()),
            -15 => Self::ConfigError(last_quickfix_error_message_or_default()),
            -16 => Self::RuntimeError(last_quickfix_error_message_or_default()),
            -17 => Self::InvalidTagNumber(last_quickfix_error_message_or_default()),
            -18 => Self::RequiredTagMissing(last_quickfix_error_message_or_default()),
            -19 => Self::TagNotDefinedForMessage(last_quickfix_error_message_or_default()),
            -20 => Self::NoTagValue(last_quickfix_error_message_or_default()),
            -21 => Self::IncorrectTagValue(last_quickfix_error_message_or_default()),
            -22 => Self::IncorrectDataFormat(last_quickfix_error_message_or_default()),
            -23 => Self::IncorrectMessageStructure(last_quickfix_error_message_or_default()),
            -24 => Self::DuplicateFieldNumber(last_quickfix_error_message_or_default()),
            -25 => Self::InvalidMessageType(last_quickfix_error_message_or_default()),
            -26 => Self::UnsupportedMessageType(last_quickfix_error_message_or_default()),
            -27 => Self::UnsupportedVersion(last_quickfix_error_message_or_default()),
            -28 => Self::TagOutOfOrder(last_quickfix_error_message_or_default()),
            -29 => Self::RepeatedTag(last_quickfix_error_message_or_default()),
            -30 => Self::RepeatingGroupCountMismatch(last_quickfix_error_message_or_default()),
            -31 => Self::DoNotSend(last_quickfix_error_message_or_default()),
            -32 => Self::RejectLogon(last_quickfix_error_message_or_default()),
            -33 => Self::SessionNotFound(last_quickfix_error_message_or_default()),
            -34 => Self::IOException(last_quickfix_error_message_or_default()),
            -35 => Self::SocketException(last_quickfix_error_message_or_default()),
            -36 => Self::SocketSendFailed(last_quickfix_error_message_or_default()),
            -37 => Self::SocketRecvFailed(last_quickfix_error_message_or_default()),
            -38 => Self::SocketCloseFailed(last_quickfix_error_message_or_default()),
            value => {
                Self::InvalidFunctionReturnCode(value, last_quickfix_error_message_or_default())
            }
        }
    }
}

impl From<NulError> for QuickFixError {
    fn from(value: NulError) -> Self {
        Self::InvalidArgument(value.to_string())
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
