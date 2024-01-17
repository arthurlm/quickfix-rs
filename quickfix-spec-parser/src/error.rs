use std::{num::ParseIntError, string::FromUtf8Error};

use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq, Clone)]
pub enum FixSpecError {
    #[error("invalid document: {0}")]
    InvalidDocument(&'static str),

    #[error("invalid attribute: {0}")]
    InvalidAttribute(String),

    #[error("invalid content: {0}")]
    InvalidContent(String),

    #[error("xml error: {0}")]
    Xml(String),
}

impl From<FromUtf8Error> for FixSpecError {
    fn from(err: FromUtf8Error) -> Self {
        Self::InvalidContent(err.to_string())
    }
}

impl From<ParseIntError> for FixSpecError {
    fn from(err: ParseIntError) -> Self {
        Self::InvalidContent(err.to_string())
    }
}

impl From<quick_xml::Error> for FixSpecError {
    fn from(err: quick_xml::Error) -> Self {
        Self::Xml(err.to_string())
    }
}
