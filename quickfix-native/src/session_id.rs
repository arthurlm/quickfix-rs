use std::fmt::Display;

use crate::fields::{BeginString, SenderCompID, TargetCompID};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SessionID {
    pub begin_string: BeginString,
    pub sender_comp_id: SenderCompID,
    pub target_comp_id: TargetCompID,
    pub session_qualifier: String,
    pub is_fixt: bool,
    pub frozen_string: String,
}

impl SessionID {
    pub fn new(
        begin_string: String,
        sender_comp_id: String,
        target_comp_id: String,
        session_qualifier: String,
    ) -> Self {
        let begin_string = BeginString::new(begin_string);
        let is_fixt = begin_string.get_value().starts_with("FIXT");

        let mut session_id = SessionID {
            begin_string,
            sender_comp_id: SenderCompID::new(sender_comp_id),
            target_comp_id: TargetCompID::new(target_comp_id),
            session_qualifier,
            is_fixt,
            frozen_string: String::new(),
        };

        session_id.update_frozen_string();
        session_id
    }

    pub fn get_begin_string(&self) -> &BeginString {
        &self.begin_string
    }

    pub fn get_sender_comp_id(&self) -> &SenderCompID {
        &self.sender_comp_id
    }
    pub fn get_target_comp_id(&self) -> &TargetCompID {
        &self.target_comp_id
    }
    pub fn get_session_qualifier(&self) -> &str {
        &self.session_qualifier
    }
    pub fn is_fixt(&self) -> bool {
        self.is_fixt
    }

    pub fn to_string_frozen(&self) -> &str {
        &self.frozen_string
    }

    pub fn from_string(s: &str) -> Option<Self> {
        let first = s.find(':')?;
        let second = s.find("->")?;
        let third = s.rfind(':')?;

        let begin_string = s[..first].to_string();
        let sender_comp_id = s[first + 1..second].to_string();

        // session qualifier can be empty which results in third and first ':' to
        // be located on the same position
        let (target_comp_id, session_qualifier) = if first == third {
            (s[second + 2..].to_string(), String::new())
        } else {
            (s[second + 2..third].to_string(), s[third + 1..].to_string())
        };

        Some(Self::new(
            begin_string,
            sender_comp_id,
            target_comp_id,
            session_qualifier,
        ))
    }
    fn update_frozen_string(&mut self) {
        self.frozen_string = format!(
            "{}:{}->{}{}",
            self.begin_string.get_value(),
            self.sender_comp_id.get_value(),
            self.target_comp_id.get_value(),
            if self.session_qualifier.is_empty() {
                String::new()
            } else {
                format!(":{}", self.session_qualifier)
            }
        );
    }
}

impl TryFrom<&str> for SessionID {
    type Error = &'static str;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_string(value).ok_or("Invalid SessionID format")
    }
}
impl TryFrom<String> for SessionID {
    type Error = &'static str;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        Self::from_string(&value).ok_or("Invalid SessionID format")
    }
}

impl Display for SessionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.frozen_string)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_id_from_string_without_qualifier() {
        let session_id = SessionID::from_string("FIX.4.4:SENDER->TARGET").unwrap();
        assert_eq!(session_id.get_begin_string().get_value(), "FIX.4.4");
        assert_eq!(session_id.get_sender_comp_id().get_value(), "SENDER");
        assert_eq!(session_id.get_target_comp_id().get_value(), "TARGET");
        assert_eq!(session_id.get_session_qualifier(), "");
        assert_eq!(session_id.to_string_frozen(), "FIX.4.4:SENDER->TARGET");
    }

    #[test]
    fn test_session_id_from_string_with_qualifier() {
        let session_id = SessionID::from_string("FIX.4.4:SENDER->TARGET:QUALIFIER").unwrap();
        assert_eq!(session_id.get_begin_string().get_value(), "FIX.4.4");
        assert_eq!(session_id.get_sender_comp_id().get_value(), "SENDER");
        assert_eq!(session_id.get_target_comp_id().get_value(), "TARGET");
        assert_eq!(session_id.get_session_qualifier(), "QUALIFIER");
        assert_eq!(
            session_id.to_string_frozen(),
            "FIX.4.4:SENDER->TARGET:QUALIFIER"
        );
    }

    #[test]
    fn test_session_id_is_fixt() {
        let session_id = SessionID::from_string("FIXT.1.1:SENDER->TARGET").unwrap();
        assert!(session_id.is_fixt());

        let session_id2 = SessionID::from_string("FIX.4.4:SENDER->TARGET").unwrap();
        assert!(!session_id2.is_fixt());
    }

    #[test]
    fn test_invalid_session_id_string() {
        assert!(SessionID::from_string("invalid").is_none());
        assert!(SessionID::from_string("FIX.4.4:SENDER").is_none()); // Missing ->
        assert!(SessionID::from_string("SENDER->TARGET").is_none()); // Missing first :
    }
}
