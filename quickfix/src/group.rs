use std::{ffi::CString, fmt};

use quickfix_ffi::{
    FixGroup_delete, FixGroup_getDelim, FixGroup_getField, FixGroup_getFieldId, FixGroup_new,
    FixGroup_removeField, FixGroup_setField, FixGroup_t,
};

use crate::{
    utils::{ffi_code_to_result, read_checked_cstr},
    FieldMap, QuickFixError,
};

/// Base class for all FIX repeating groups.
pub struct Group(pub(crate) FixGroup_t);

impl Group {
    /// Create new empty struct.
    pub fn try_new(field_id: i32, delim: i32) -> Option<Self> {
        unsafe { FixGroup_new(field_id, delim) }.map(Self)
    }

    /// Get field ID.
    pub fn field_id(&self) -> i32 {
        unsafe { FixGroup_getFieldId(self.0) }
    }

    /// Get delimiter.
    pub fn delim(&self) -> i32 {
        unsafe { FixGroup_getDelim(self.0) }
    }
}

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

impl fmt::Debug for Group {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Group")
            .field("id", &self.field_id())
            .field("delim", &self.delim())
            .finish()
    }
}

impl Drop for Group {
    fn drop(&mut self) {
        unsafe { FixGroup_delete(self.0) }
    }
}
