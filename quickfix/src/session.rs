use std::fmt;

use quickfix_ffi::{
    FixSession_isLoggedOn, FixSession_logout, FixSession_lookup, FixSession_sendToTarget,
    FixSession_t,
};

use crate::{
    utils::{ffi_code_to_bool, ffi_code_to_result},
    Message, QuickFixError, SessionId,
};

/// Send message to target design in session ID.
pub fn send_to_target(msg: Message, session_id: &SessionId) -> Result<(), QuickFixError> {
    // NOTE: Message may be changed by real library. Just consume it so nothing will leak to rust code.
    ffi_code_to_result(unsafe { FixSession_sendToTarget(msg.0, session_id.0) })
}

/// FIX Session.
pub struct Session(pub(crate) FixSession_t);

impl Session {
    /// Find a session by its ID.
    ///
    /// # Safety
    ///
    /// Function is unsafe because there is no way to bind FIX session lifetime
    /// to rust session lifetime.
    ///
    /// Maybe Oren Miller as a better idea / solution to solve this issue.
    pub unsafe fn lookup(session_id: &SessionId) -> Result<Self, QuickFixError> {
        match unsafe { FixSession_lookup(session_id.0) } {
            Some(session) => Ok(Self(session)),
            None => Err(QuickFixError::from_last_error()),
        }
    }

    /// Force session logout.
    pub fn logout(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSession_logout(self.0) })
    }

    /// Check if session is logged on.
    pub fn is_logged_on(&mut self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSession_isLoggedOn(self.0) })
    }
}

impl fmt::Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Session").finish()
    }
}
