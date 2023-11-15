use std::{ffi::CString, path::Path};

use quickfix_ffi::{
    FixSessionSettings_delete, FixSessionSettings_fromPath, FixSessionSettings_new,
    FixSessionSettings_t,
};

use crate::QuickFixError;

#[derive(Debug)]
pub struct SessionSettings(pub(crate) FixSessionSettings_t);

impl SessionSettings {
    pub fn try_new() -> Result<Self, QuickFixError> {
        unsafe { FixSessionSettings_new() }
            .map(Self)
            .ok_or(QuickFixError::InvalidFunctionReturn)
    }

    pub fn try_from_path<P: AsRef<Path>>(path: P) -> Result<Self, QuickFixError> {
        let safe_path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| QuickFixError::invalid_argument("Cannot convert path to C path"))?;
        let ffi_path = CString::new(safe_path)?;

        unsafe { FixSessionSettings_fromPath(ffi_path.as_ptr()) }
            .map(Self)
            .ok_or(QuickFixError::InvalidFunctionReturn)
    }
}

impl Drop for SessionSettings {
    fn drop(&mut self) {
        unsafe { FixSessionSettings_delete(self.0) }
    }
}
