use std::{ffi::CString, path::Path};

use quickfix_ffi::{
    FixDataDictionary_delete, FixDataDictionary_fromPath, FixDataDictionary_new,
    FixDataDictionary_t,
};

use crate::QuickFixError;

#[derive(Debug)]
pub struct DataDictionary(pub(crate) FixDataDictionary_t);

impl DataDictionary {
    pub fn try_new() -> Result<Self, QuickFixError> {
        unsafe { FixDataDictionary_new() }
            .map(Self)
            .ok_or(QuickFixError::InvalidFunctionReturn)
    }

    pub fn try_from_path<P: AsRef<Path>>(path: P) -> Result<Self, QuickFixError> {
        let safe_path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| QuickFixError::invalid_argument("Cannot convert path to C path"))?;
        let ffi_path = CString::new(safe_path)?;

        unsafe { FixDataDictionary_fromPath(ffi_path.as_ptr()) }
            .map(Self)
            .ok_or(QuickFixError::InvalidFunctionReturn)
    }
}

impl Drop for DataDictionary {
    fn drop(&mut self) {
        unsafe { FixDataDictionary_delete(self.0) }
    }
}
