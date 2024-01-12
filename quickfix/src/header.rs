use std::{ffi::CString, fmt};

use quickfix_ffi::{
    FixHeader_delete, FixHeader_getField, FixHeader_new, FixHeader_removeField, FixHeader_setField,
    FixHeader_t,
};

use crate::{
    utils::{ffi_code_to_result, read_checked_cstr},
    FieldMap, QuickFixError,
};

/// Header part of a FIX message.
pub struct Header(pub(crate) FixHeader_t);

impl Header {
    /// Create new empty struct.
    pub fn new() -> Self {
        Self::default()
    }
}

impl FieldMap for Header {
    fn get_field(&self, tag: i32) -> Option<String> {
        unsafe { FixHeader_getField(self.0, tag) }.map(read_checked_cstr)
    }

    fn set_field<V: AsRef<str>>(&mut self, tag: i32, value: V) -> Result<(), QuickFixError> {
        let ffi_value = CString::new(value.as_ref())?;
        ffi_code_to_result(unsafe { FixHeader_setField(self.0, tag, ffi_value.as_ptr()) })
    }

    fn remove_field(&mut self, tag: i32) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixHeader_removeField(self.0, tag) })
    }
}

impl fmt::Debug for Header {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Header").finish()
    }
}

impl Default for Header {
    fn default() -> Self {
        unsafe { FixHeader_new() }
            .map(Self)
            .expect("Fail to allocate new Header")
    }
}

impl Drop for Header {
    fn drop(&mut self) {
        unsafe { FixHeader_delete(self.0) }
    }
}
