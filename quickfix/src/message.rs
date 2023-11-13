use std::ffi::CString;

use quickfix_ffi::{
    FixMessage_delete, FixMessage_getField, FixMessage_new, FixMessage_removeField,
    FixMessage_setField, FixMessage_toBuffer,
};

use crate::{
    utils::{read_buffer_to_string, read_checked_cstr},
    QuickFixError,
};

#[derive(Debug)]
pub struct Message(pub(crate) quickfix_ffi::FixMessage_t);

impl Message {
    pub fn try_new() -> Result<Self, QuickFixError> {
        match unsafe { FixMessage_new() } {
            Some(val) => Ok(Self(val)),
            None => Err(QuickFixError::InvalidFunctionReturn),
        }
    }

    pub fn set_field(&mut self, tag: i32, value: &str) -> Result<(), QuickFixError> {
        let ffi_value = CString::new(value)?;

        match unsafe { FixMessage_setField(self.0, tag, ffi_value.as_ptr()) } {
            0 => Ok(()),
            code => Err(QuickFixError::InvalidFunctionReturnCode(code)),
        }
    }

    pub fn get_field(&self, tag: i32) -> Option<String> {
        unsafe { FixMessage_getField(self.0, tag) }.map(read_checked_cstr)
    }

    pub fn remove_field(&self, tag: i32) -> Result<(), QuickFixError> {
        match unsafe { FixMessage_removeField(self.0, tag) } {
            0 => Ok(()),
            code => Err(QuickFixError::InvalidFunctionReturnCode(code)),
        }
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
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe { FixMessage_delete(self.0) }
    }
}
