use std::fmt;

use quickfix_ffi::{
    FixTrailer_addGroup, FixTrailer_copy, FixTrailer_copyGroup, FixTrailer_delete,
    FixTrailer_getField, FixTrailer_new, FixTrailer_removeField, FixTrailer_setField, FixTrailer_t,
};

use crate::{
    utils::{ffi_code_to_result, read_checked_cstr},
    FieldMap, Group, IntoFixValue, QuickFixError,
};

/// Trailer part of a FIX message.
pub struct Trailer(pub(crate) FixTrailer_t);

impl Trailer {
    /// Create new empty struct.
    pub fn new() -> Self {
        Self::default()
    }
}

impl FieldMap for Trailer {
    fn get_field(&self, tag: i32) -> Option<String> {
        unsafe { FixTrailer_getField(self.0, tag) }.map(read_checked_cstr)
    }

    fn set_field<V: IntoFixValue>(&mut self, tag: i32, value: V) -> Result<(), QuickFixError> {
        let fix_value = value.into_fix_value()?;
        ffi_code_to_result(unsafe { FixTrailer_setField(self.0, tag, fix_value.as_ptr()) })
    }

    fn remove_field(&mut self, tag: i32) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixTrailer_removeField(self.0, tag) })
    }

    fn add_group(&mut self, group: &Group) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixTrailer_addGroup(self.0, group.0) })?;
        Ok(())
    }

    fn clone_group(&self, index: i32, tag: i32) -> Option<Group> {
        unsafe { FixTrailer_copyGroup(self.0, index, tag) }.map(Group)
    }
}

impl Clone for Trailer {
    fn clone(&self) -> Self {
        Self(unsafe { FixTrailer_copy(self.0) }.expect("Fail to clone Trailer"))
    }
}

impl fmt::Debug for Trailer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Trailer").finish()
    }
}

impl Default for Trailer {
    fn default() -> Self {
        unsafe { FixTrailer_new() }
            .map(Self)
            .expect("Fail to allocate new Trailer")
    }
}

impl Drop for Trailer {
    fn drop(&mut self) {
        unsafe { FixTrailer_delete(self.0) }
    }
}
