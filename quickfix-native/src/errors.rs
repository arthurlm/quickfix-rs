use thiserror::Error;

//TODO: should later be matched against QuickFixErros
#[derive(Error, Debug, Clone, PartialEq)]
pub enum NativeError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Field conversion error: {0}")]
    FieldConvertError(String),

    #[error("Field not found: {0}")]
    FieldNotFound(String),

    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Io error: {0}")]
    IoError(String),

    #[error("Network error: {0}")]
    NetworkError(String),
}

pub type Result<T> = std::result::Result<T, NativeError>;
