use std::ffi::CString;

use quickfix_ffi::{
    FixTrailer_delete, FixTrailer_getField, FixTrailer_removeField, FixTrailer_setField,
    FixTrailer_t,
};

use crate::{
    utils::{ffi_code_to_result, read_checked_cstr},
    FieldMap, QuickFixError,
};

/// Trailer part of a FIX message.
#[derive(Debug)]
pub struct Trailer(pub(crate) FixTrailer_t);

impl FieldMap for Trailer {
    fn get_field(&self, tag: i32) -> Option<String> {
        unsafe { FixTrailer_getField(self.0, tag) }.map(read_checked_cstr)
    }

    fn set_field(&mut self, tag: i32, value: &str) -> Result<(), QuickFixError> {
        let ffi_value = CString::new(value)?;
        ffi_code_to_result(unsafe { FixTrailer_setField(self.0, tag, ffi_value.as_ptr()) })
    }

    fn remove_field(&mut self, tag: i32) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixTrailer_removeField(self.0, tag) })
    }
}

impl Drop for Trailer {
    fn drop(&mut self) {
        unsafe { FixTrailer_delete(self.0) }
    }
}
