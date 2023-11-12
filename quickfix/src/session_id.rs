use std::ffi::CStr;

use quickfix_ffi::{
    FixSessionID_getBeginString, FixSessionID_getSenderCompID, FixSessionID_getSessionQualifier,
    FixSessionID_getTargetCompID, FixSessionID_isFIXT,
};

#[derive(Debug)]
pub struct SessionId(pub(crate) quickfix_ffi::FixSessionID_t);

impl SessionId {
    pub fn get_begin_string(&self) -> Option<String> {
        match unsafe { FixSessionID_getBeginString(self.0) } {
            Some(val) => {
                let cstr = unsafe { CStr::from_ptr(val.as_ptr()) };
                Some(String::from_utf8_lossy(cstr.to_bytes()).to_string())
            }
            None => None,
        }
    }

    pub fn get_sender_comp_id(&self) -> Option<String> {
        match unsafe { FixSessionID_getSenderCompID(self.0) } {
            Some(val) => {
                let cstr = unsafe { CStr::from_ptr(val.as_ptr()) };
                Some(String::from_utf8_lossy(cstr.to_bytes()).to_string())
            }
            None => None,
        }
    }

    pub fn get_target_comp_id(&self) -> Option<String> {
        match unsafe { FixSessionID_getTargetCompID(self.0) } {
            Some(val) => {
                let cstr = unsafe { CStr::from_ptr(val.as_ptr()) };
                Some(String::from_utf8_lossy(cstr.to_bytes()).to_string())
            }
            None => None,
        }
    }

    pub fn get_session_qualifier(&self) -> Option<String> {
        match unsafe { FixSessionID_getSessionQualifier(self.0) } {
            Some(val) => {
                let cstr = unsafe { CStr::from_ptr(val.as_ptr()) };
                Some(String::from_utf8_lossy(cstr.to_bytes()).to_string())
            }
            None => None,
        }
    }

    pub fn is_fixt(&self) -> bool {
        let val = unsafe { FixSessionID_isFIXT(self.0) };
        val != 0
    }
}
