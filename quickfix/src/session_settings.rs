use std::{ffi::CString, path::Path};

use crate::QuickFixError;

#[derive(Debug)]
pub struct SessionSettings(pub(crate) quickfix_ffi::FixSessionSettings_t);

impl SessionSettings {
    pub fn try_new() -> Result<Self, QuickFixError> {
        match unsafe { quickfix_ffi::FixSessionSettings_new() } {
            Some(val) => Ok(Self(val)),
            None => Err(QuickFixError::InvalidFunctionReturn),
        }
    }

    pub fn try_from_path<P: AsRef<Path>>(path: P) -> Result<Self, QuickFixError> {
        let safe_path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| QuickFixError::invalid_argument("Cannot convert path to C path"))?;
        let ffi_path = CString::new(safe_path)?;

        match unsafe { quickfix_ffi::FixSessionSettings_fromPath(ffi_path.as_ptr()) } {
            Some(val) => Ok(Self(val)),
            None => Err(QuickFixError::InvalidFunctionReturn),
        }
    }
}

impl Drop for SessionSettings {
    fn drop(&mut self) {
        unsafe { quickfix_ffi::FixSessionSettings_delete(self.0) }
    }
}
