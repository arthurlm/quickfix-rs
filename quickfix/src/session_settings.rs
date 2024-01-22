use std::{ffi::CString, fmt, mem::ManuallyDrop, path::Path};

use quickfix_ffi::{
    FixSessionSettings_delete, FixSessionSettings_fromPath, FixSessionSettings_getGlobalRef,
    FixSessionSettings_getSessionRef, FixSessionSettings_new, FixSessionSettings_setGlobal,
    FixSessionSettings_setSession, FixSessionSettings_t,
};

use crate::{utils::ffi_code_to_result, Dictionary, QuickFixError, SessionId};

/// Container for setting dictionaries mapped to sessions.
pub struct SessionSettings(pub(crate) FixSessionSettings_t);

impl SessionSettings {
    /// Create new empty struct.
    pub fn new() -> Self {
        Self::default()
    }

    /// Try to load struct data from Path.
    pub fn try_from_path<P: AsRef<Path>>(path: P) -> Result<Self, QuickFixError> {
        let safe_path = path
            .as_ref()
            .to_str()
            .ok_or_else(|| QuickFixError::invalid_argument("Cannot convert path to C path"))?;
        let ffi_path = CString::new(safe_path)?;

        unsafe { FixSessionSettings_fromPath(ffi_path.as_ptr()) }
            .map(Self)
            .ok_or_else(QuickFixError::null)
    }

    /// Borrow inner dictionary for session or global configuration.
    pub fn with_dictionary<T, F>(&self, session_id: Option<SessionId>, f: F) -> Option<T>
    where
        F: FnOnce(&Dictionary) -> T,
    {
        // Get dict ptr.
        let res = unsafe {
            match session_id {
                None => FixSessionSettings_getGlobalRef(self.0),
                Some(session_id) => FixSessionSettings_getSessionRef(self.0, session_id.0),
            }
        };

        // Check ptr and pass it to callback.
        if let Some(ptr) = res {
            let obj = ManuallyDrop::new(Dictionary(ptr));
            Some(f(&obj))
        } else {
            None
        }
    }

    /// Set dictionary parameter for session or global configuration.
    pub fn set(
        &mut self,
        session_id: Option<SessionId>,
        value: Dictionary,
    ) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe {
            match session_id {
                None => FixSessionSettings_setGlobal(self.0, value.0),
                Some(session_id) => FixSessionSettings_setSession(self.0, session_id.0, value.0),
            }
        })
    }
}

impl fmt::Debug for SessionSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("SessionSettings").finish()
    }
}

impl Default for SessionSettings {
    fn default() -> Self {
        unsafe { FixSessionSettings_new() }
            .map(Self)
            .expect("Fail to allocate new SessionSettings")
    }
}

impl Drop for SessionSettings {
    fn drop(&mut self) {
        unsafe { FixSessionSettings_delete(self.0) }
    }
}
