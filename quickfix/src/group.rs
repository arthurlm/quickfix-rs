use std::ffi::CString;

use quickfix_ffi::{
    FixGroup_delete, FixGroup_getField, FixGroup_removeField, FixGroup_setField, FixGroup_t,
};

use crate::{
    utils::{ffi_code_to_result, read_checked_cstr},
    FieldMap, QuickFixError,
};

#[derive(Debug)]
pub struct Group(pub(crate) FixGroup_t);

impl FieldMap for Group {
    fn get_field(&self, tag: i32) -> Option<String> {
        unsafe { FixGroup_getField(self.0, tag) }.map(read_checked_cstr)
    }

    fn set_field(&mut self, tag: i32, value: &str) -> Result<(), QuickFixError> {
        let ffi_value = CString::new(value)?;
        ffi_code_to_result(unsafe { FixGroup_setField(self.0, tag, ffi_value.as_ptr()) })
    }

    fn remove_field(&mut self, tag: i32) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixGroup_removeField(self.0, tag) })
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        unsafe { FixGroup_delete(self.0) }
    }
}
