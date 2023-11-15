use std::{ffi::CString, fmt, mem::ManuallyDrop};

use quickfix_ffi::{
    FixMessage_delete, FixMessage_fromString, FixMessage_getField, FixMessage_getGroupRef,
    FixMessage_getHeaderRef, FixMessage_getTrailerRef, FixMessage_new, FixMessage_removeField,
    FixMessage_setField, FixMessage_t, FixMessage_toBuffer,
};

use crate::{
    group::Group,
    header::Header,
    trailer::Trailer,
    utils::{ffi_code_to_result, read_buffer_to_string, read_checked_cstr},
    FieldMap, QuickFixError,
};

pub struct Message(pub(crate) FixMessage_t);

impl Message {
    pub fn try_new() -> Result<Self, QuickFixError> {
        unsafe { FixMessage_new() }
            .map(Self)
            .ok_or(QuickFixError::InvalidFunctionReturn)
    }

    pub fn try_from_text(text: &str) -> Result<Self, QuickFixError> {
        let ffi_text = CString::new(text)?;
        unsafe { FixMessage_fromString(ffi_text.as_ptr()) }
            .map(Self)
            .ok_or(QuickFixError::InvalidFunctionReturn)
    }

    pub fn as_string(&self) -> Result<String, QuickFixError> {
        self.as_string_with_len(4096 /* 1 page */)
    }

    pub fn as_string_with_len(&self, max_len: usize) -> Result<String, QuickFixError> {
        let mut buffer = vec![0_u8; max_len];
        let buffer_ptr = buffer.as_mut_ptr() as *mut i8;
        match unsafe { FixMessage_toBuffer(self.0, buffer_ptr, max_len as i64) } {
            0 => Ok(read_buffer_to_string(&buffer)),
            code => Err(QuickFixError::InvalidFunctionReturnCode(code)),
        }
    }

    pub fn with_header<T, F>(&self, f: F) -> Result<T, QuickFixError>
    where
        F: FnOnce(&Header) -> T,
    {
        if let Some(ptr) = unsafe { FixMessage_getHeaderRef(self.0) } {
            let obj = ManuallyDrop::new(Header(ptr));
            Ok(f(&obj))
        } else {
            Err(QuickFixError::InvalidFunctionReturn)
        }
    }

    pub fn with_header_mut<T, F>(&mut self, f: F) -> Result<T, QuickFixError>
    where
        F: FnOnce(&mut Header) -> T,
    {
        if let Some(ptr) = unsafe { FixMessage_getHeaderRef(self.0) } {
            let mut obj = ManuallyDrop::new(Header(ptr));
            Ok(f(&mut obj))
        } else {
            Err(QuickFixError::InvalidFunctionReturn)
        }
    }

    pub fn with_trailer<T, F>(&self, f: F) -> Result<T, QuickFixError>
    where
        F: FnOnce(&Trailer) -> T,
    {
        if let Some(ptr) = unsafe { FixMessage_getTrailerRef(self.0) } {
            let obj = ManuallyDrop::new(Trailer(ptr));
            Ok(f(&obj))
        } else {
            Err(QuickFixError::InvalidFunctionReturn)
        }
    }

    pub fn with_trailer_mut<T, F>(&mut self, f: F) -> Result<T, QuickFixError>
    where
        F: FnOnce(&mut Trailer) -> T,
    {
        if let Some(ptr) = unsafe { FixMessage_getTrailerRef(self.0) } {
            let mut obj = ManuallyDrop::new(Trailer(ptr));
            Ok(f(&mut obj))
        } else {
            Err(QuickFixError::InvalidFunctionReturn)
        }
    }

    pub fn with_group<T, F>(&self, index: i32, tag: i32, f: F) -> Result<T, QuickFixError>
    where
        F: FnOnce(&Group) -> T,
    {
        if let Some(ptr) = unsafe { FixMessage_getGroupRef(self.0, index, tag) } {
            let obj = ManuallyDrop::new(Group(ptr));
            Ok(f(&obj))
        } else {
            Err(QuickFixError::InvalidFunctionReturn)
        }
    }

    pub fn with_group_mut<T, F>(&mut self, index: i32, tag: i32, f: F) -> Result<T, QuickFixError>
    where
        F: FnOnce(&mut Group) -> T,
    {
        if let Some(ptr) = unsafe { FixMessage_getGroupRef(self.0, index, tag) } {
            let mut obj = ManuallyDrop::new(Group(ptr));
            Ok(f(&mut obj))
        } else {
            Err(QuickFixError::InvalidFunctionReturn)
        }
    }
}

impl FieldMap for Message {
    fn get_field(&self, tag: i32) -> Option<String> {
        unsafe { FixMessage_getField(self.0, tag) }.map(read_checked_cstr)
    }

    fn set_field(&mut self, tag: i32, value: &str) -> Result<(), QuickFixError> {
        let ffi_value = CString::new(value)?;
        ffi_code_to_result(unsafe { FixMessage_setField(self.0, tag, ffi_value.as_ptr()) })
    }

    fn remove_field(&mut self, tag: i32) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixMessage_removeField(self.0, tag) })
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Message").field(&self.as_string()).finish()
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe { FixMessage_delete(self.0) }
    }
}
