use quickfix_ffi::FixSession_sendToTarget;

use crate::{utils::ffi_code_to_result, Message, QuickFixError, SessionId};

/// Send message to target design in session ID.
pub fn send_to_target(msg: Message, session_id: &SessionId) -> Result<(), QuickFixError> {
    // NOTE: Message may be changed by real library. Just consume it so nothing will leak to rust code.
    ffi_code_to_result(unsafe { FixSession_sendToTarget(msg.0, session_id.0) })
}
