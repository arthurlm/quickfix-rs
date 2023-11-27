use std::{ffi::CString, path::Path};

use quickfix_ffi::{
    FixDataDictionary_delete, FixDataDictionary_fromPath, FixDataDictionary_new,
    FixDataDictionary_t, FixMessage_fromStringAndDictionary,
};

use crate::{Message, QuickFixError};

/// Represents a data dictionary for a version of FIX.
#[derive(Debug)]
pub struct DataDictionary(FixDataDictionary_t);

impl DataDictionary {
    /// Create a new empty struct.
    pub fn new() -> Self {
        unsafe { FixDataDictionary_new() }
            .map(Self)
            .expect("Fail to allocate new DataDictionary")
    }

    /// Try to load struct data from path.
    pub fn try_from_path<P: AsRef<Path>>(path: P) -> Result<Self, QuickFixError> {
        let safe_path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| QuickFixError::invalid_argument("Cannot convert path to C path"))?;
        let ffi_path = CString::new(safe_path)?;

        unsafe { FixDataDictionary_fromPath(ffi_path.as_ptr()) }
            .map(Self)
            .ok_or(QuickFixError::NullFunctionReturn)
    }

    /// Create a new FIX messages using current dictionary.
    pub fn try_build_message(&self, text: &str) -> Result<Message, QuickFixError> {
        let ffi_text = CString::new(text)?;
        unsafe { FixMessage_fromStringAndDictionary(ffi_text.as_ptr(), self.0) }
            .map(Message)
            .ok_or(QuickFixError::NullFunctionReturn)
    }
}

impl Default for DataDictionary {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for DataDictionary {
    fn drop(&mut self) {
        unsafe { FixDataDictionary_delete(self.0) }
    }
}
