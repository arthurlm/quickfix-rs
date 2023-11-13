use quickfix_ffi::{
    FixSessionID_getBeginString, FixSessionID_getSenderCompID, FixSessionID_getSessionQualifier,
    FixSessionID_getTargetCompID, FixSessionID_isFIXT,
};

use crate::utils::read_checked_cstr;

#[derive(Debug)]
pub struct SessionId(pub(crate) quickfix_ffi::FixSessionID_t);

impl SessionId {
    pub fn get_begin_string(&self) -> Option<String> {
        unsafe { FixSessionID_getBeginString(self.0) }.map(read_checked_cstr)
    }

    pub fn get_sender_comp_id(&self) -> Option<String> {
        unsafe { FixSessionID_getSenderCompID(self.0) }.map(read_checked_cstr)
    }

    pub fn get_target_comp_id(&self) -> Option<String> {
        unsafe { FixSessionID_getTargetCompID(self.0) }.map(read_checked_cstr)
    }

    pub fn get_session_qualifier(&self) -> Option<String> {
        unsafe { FixSessionID_getSessionQualifier(self.0) }.map(read_checked_cstr)
    }

    pub fn is_fixt(&self) -> bool {
        let val = unsafe { FixSessionID_isFIXT(self.0) };
        val != 0
    }
}
