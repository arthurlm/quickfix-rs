use std::{
    ffi::{self, CStr},
    ptr::NonNull,
};

use crate::QuickFixError;

#[inline(always)]
pub fn read_buffer_to_string(buffer: &[u8]) -> String {
    let null_index = buffer.iter().position(|x| *x == 0).unwrap_or(buffer.len());
    String::from_utf8_lossy(&buffer[..null_index]).to_string()
}

#[inline(always)]
pub fn read_checked_cstr(val: NonNull<ffi::c_char>) -> String {
    let cstr = unsafe { CStr::from_ptr(val.as_ptr()) };
    String::from_utf8_lossy(cstr.to_bytes()).to_string()
}

#[inline(always)]
pub unsafe fn from_ffi_str<'a>(ptr: *const ffi::c_char) -> &'a str {
    assert!(!ptr.is_null(), "null ptr given from `c_str()`");
    let cstr = CStr::from_ptr(ptr);
    cstr.to_str().unwrap_or("invalid `c_str()` received")
}

#[inline(always)]
pub fn ffi_code_to_result(code: i8) -> Result<(), QuickFixError> {
    match code {
        0 => Ok(()),
        code => Err(QuickFixError::InvalidFunctionReturnCode(code)),
    }
}

#[inline(always)]
pub fn ffi_code_to_bool(code: i8) -> Result<bool, QuickFixError> {
    match code {
        1 => Ok(true),
        0 => Ok(false),
        code => Err(QuickFixError::InvalidFunctionReturnCode(code)),
    }
}
