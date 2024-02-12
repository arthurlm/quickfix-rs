use std::str::FromStr;

use quick_xml::events::BytesStart;

use crate::{read_attribute, FixSpecError};

/// Message dest.
#[derive(Debug, Clone, Copy)]
pub enum MessageCategory {
    /// Message is targeting application level.
    App,
    /// Message is related to protocol / admin / technical task.
    Admin,
}

impl MessageCategory {
    /// Convert value to a static string.
    ///
    /// Mostly useful for debugging / display purpose.
    pub const fn as_static_str(&self) -> &'static str {
        match self {
            MessageCategory::App => "app",
            MessageCategory::Admin => "admin",
        }
    }

    pub(crate) fn parse_xml_element(element: &BytesStart) -> Result<Self, FixSpecError> {
        let item = read_attribute(element, "msgcat")?.parse()?;
        Ok(item)
    }
}

impl FromStr for MessageCategory {
    type Err = FixSpecError;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "admin" => Ok(Self::Admin),
            "app" => Ok(Self::App),
            x => Err(FixSpecError::InvalidContent(format!("invalid msgcat: {x}"))),
        }
    }
}
