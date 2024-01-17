use std::str::FromStr;

use crate::FixSpecError;

#[derive(Debug, Clone, Copy)]
pub enum MessageCategory {
    App,
    Admin,
}

impl MessageCategory {
    pub const fn as_static_str(&self) -> &'static str {
        match self {
            MessageCategory::App => "app",
            MessageCategory::Admin => "admin",
        }
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
