use std::{
    ffi::{self, CStr},
    ptr::NonNull,
};

pub fn read_buffer_to_string(buffer: &[u8]) -> String {
    let null_index = buffer.iter().position(|x| *x == 0).unwrap_or(buffer.len());
    String::from_utf8_lossy(&buffer[..null_index]).to_string()
}

pub fn read_checked_cstr(val: NonNull<ffi::c_char>) -> String {
    let cstr = unsafe { CStr::from_ptr(val.as_ptr()) };
    String::from_utf8_lossy(cstr.to_bytes()).to_string()
}
