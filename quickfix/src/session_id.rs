use std::{ffi::CString, fmt};

use quickfix_ffi::{
    FixSessionID_delete, FixSessionID_getBeginString, FixSessionID_getSenderCompID,
    FixSessionID_getSessionQualifier, FixSessionID_getTargetCompID, FixSessionID_isFIXT,
    FixSessionID_toString,
};

use crate::{utils::read_checked_cstr, QuickFixError};

pub struct SessionId(pub(crate) quickfix_ffi::FixSessionID_t);

impl SessionId {
    pub fn try_new(
        begin_string: &str,
        sender_comp_id: &str,
        target_comp_id: &str,
        session_qualifier: &str,
    ) -> Result<Self, QuickFixError> {
        let ffi_begin_string = CString::new(begin_string)?;
        let ffi_sender_comp_id = CString::new(sender_comp_id)?;
        let ffi_target_comp_id = CString::new(target_comp_id)?;
        let ffi_session_qualifier = CString::new(session_qualifier)?;

        match unsafe {
            quickfix_ffi::FixSessionID_new(
                ffi_begin_string.as_ptr(),
                ffi_sender_comp_id.as_ptr(),
                ffi_target_comp_id.as_ptr(),
                ffi_session_qualifier.as_ptr(),
            )
        } {
            Some(val) => Ok(Self(val)),
            None => Err(QuickFixError::InvalidFunctionReturn),
        }
    }

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

    pub fn as_string(&self) -> String {
        unsafe { FixSessionID_toString(self.0) }
            .map(read_checked_cstr)
            .unwrap_or_default()
    }
}

impl fmt::Debug for SessionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SessionId").field(&self.as_string()).finish()
    }
}

impl Drop for SessionId {
    fn drop(&mut self) {
        unsafe { FixSessionID_delete(self.0) }
    }
}
