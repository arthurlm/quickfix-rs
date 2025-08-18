use thiserror::Error;

//TODO: should later be matched against QuickFixErros
#[derive(Error, Debug)]
pub enum NativeError {
    #[error("Field conversion error: {0}")]
    FieldConvertError(String),

    #[error("Field not found: {0}")]
    FieldNotFound(String),

    #[error("Invalid message: {0}")]
    InvalidMessage(String),

    #[error("Session not found: {0}")]
    SessionNotFound(String),

    #[error("Network error: {0}")]
    NetworkError(String),

    //----data dictionary specific errors
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Unsupported version")]
    UnsupportedVersion,

    #[error("Tag out of order: {0}")]
    TagOutOfOrder(u32),

    #[error("Invalid tag number: {0}")]
    InvalidTagNumber(u32),

    #[error("Invalid message type")]
    InvalidMessageType,

    #[error("Incorrect data format for field {0}: {1}")]
    IncorrectDataFormat(u32, String),

    #[error("Incorrect tag value for field {0}")]
    IncorrectTagValue(u32),

    #[error("No tag value for field {0}")]
    NoTagValue(u32),

    #[error("Tag not defined for message: {0}")]
    TagNotDefinedforMessage(u32),

    #[error("Repeating group count mismatch for field {0}")]
    RepeatingGroupCountMismatch(u32),

    #[error("Required tag missing: {0}")]
    MissingTag(u32),

    #[error("Repeated tag: {0}")]
    RepeatedTag(u32),

    #[error("Data dictionary not found")]
    DataDictionaryNotFound,

    #[error("Io error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("XML parsing error: {0}")]
    XMLError(String),
}

pub type Result<T> = std::result::Result<T, NativeError>;
