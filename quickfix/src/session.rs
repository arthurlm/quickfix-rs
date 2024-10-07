use std::{fmt, marker::PhantomData};

use quickfix_ffi::{
    FixSession_isLoggedOn, FixSession_logout, FixSession_lookup, FixSession_send,
    FixSession_sendToTarget, FixSession_t,
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
pub struct Session<'a> {
    pub(crate) inner: FixSession_t,
    pub(crate) phantom_container: PhantomData<&'a ()>,
}

impl Session<'static> {
    /// Find a session by its ID.
    ///
    /// # Safety
    ///
    /// Function is unsafe because there is no way to bind FIX session lifetime
    /// to rust session lifetime.
    ///
    /// Use `SessionContainer::session` instead. It will give you a safe scope
    /// where session has been borrowed to the acceptor / initiator.
    pub unsafe fn lookup(session_id: &SessionId) -> Result<Self, QuickFixError> {
        match unsafe { FixSession_lookup(session_id.0) } {
            Some(inner) => Ok(Self {
                inner,
                phantom_container: PhantomData,
            }),
            None => Err(QuickFixError::from_last_error()),
        }
    }
}

impl Session<'_> {
    /// Force session logout.
    pub fn logout(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSession_logout(self.inner) })
    }

    /// Check if session is logged on.
    pub fn is_logged_on(&mut self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSession_isLoggedOn(self.inner) })
    }

    /// Send message using current session.
    pub fn send(&mut self, msg: Message) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSession_send(self.inner, msg.0) })
    }
}

impl fmt::Debug for Session<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Session").finish()
    }
}
