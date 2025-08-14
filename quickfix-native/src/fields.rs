#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct BeginString(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SenderCompID(String);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TargetCompID(String);

impl BeginString {
    pub fn new<S: Into<String>>(value: S) -> Self {
        Self(value.into())
    }

    pub fn get_value(&self) -> &str {
        &self.0
    }

    pub fn get_string(&self) -> &String {
        &self.0
    }
}

impl SenderCompID {
    pub fn new<S: Into<String>>(value: S) -> Self {
        Self(value.into())
    }

    pub fn get_value(&self) -> &str {
        &self.0
    }

    pub fn get_string(&self) -> &String {
        &self.0
    }
}

impl TargetCompID {
    pub fn new<S: Into<String>>(value: S) -> Self {
        Self(value.into())
    }

    pub fn get_value(&self) -> &str {
        &self.0
    }

    pub fn get_string(&self) -> &String {
        &self.0
    }
}

impl From<String> for BeginString {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<String> for SenderCompID {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<String> for TargetCompID {
    fn from(value: String) -> Self {
        Self(value)
    }
}
