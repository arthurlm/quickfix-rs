use std::{ffi::CString, fmt};

use quickfix_ffi::{
    FixGroup_delete, FixGroup_getDelim, FixGroup_getField, FixGroup_getFieldId, FixGroup_new,
    FixGroup_removeField, FixGroup_setField, FixGroup_t,
};

use crate::{
    utils::{ffi_code_to_result, read_checked_cstr},
    AsFixValue, FieldMap, QuickFixError,
};

/// Base class for all FIX repeating groups.
pub struct Group(pub(crate) FixGroup_t);

impl Group {
    /// Create new empty struct.
    pub fn try_new(field_id: i32, delim: i32) -> Result<Self, QuickFixError> {
        Self::try_with_orders(field_id, delim, &[])
    }

    /// Create struct with all its sub-components values.
    ///
    /// NOTE: Ending orders with 0 field ID is not required. It will be done in this builder.
    pub fn try_with_orders(
        field_id: i32,
        delim: i32,
        orders: &[i32],
    ) -> Result<Self, QuickFixError> {
        let mut safe_orders = Vec::<i32>::with_capacity(orders.len() + 1);
        safe_orders.extend(orders);
        safe_orders.push(0); // Make sure orders input end with null field ID.

        unsafe { FixGroup_new(field_id, delim, safe_orders.as_ptr()) }
            .map(Self)
            .ok_or(QuickFixError::NullFunctionReturn)
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

    fn set_field<V: AsFixValue>(&mut self, tag: i32, value: V) -> Result<(), QuickFixError> {
        let ffi_value = CString::new(value.as_fix_value())?;
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
