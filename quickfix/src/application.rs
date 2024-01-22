use std::{any::Any, ffi, marker::PhantomData, mem::ManuallyDrop, panic::catch_unwind};

use quickfix_ffi::{
    FixApplicationCallbacks_t, FixApplication_delete, FixApplication_new, FixApplication_t,
    FixMessage_t, FixSessionID_t,
};

use crate::{Message, QuickFixError, SessionId};

/// Error result that can occurs from a `on_msg_to_app` callback.
#[derive(Debug)]
pub enum MsgToAppError {
    /// Indicates user does not want to send a message
    DoNotSend,
}

/// Error result that can occurs from a `on_msg_from_admin` callback.
#[derive(Debug)]
pub enum MsgFromAdminError {
    /// Field not found inside a message.
    FieldNotFound,
    /// Field has a badly formatted value.
    IncorrectDataFormat,
    /// Field has a value that is out of range.
    IncorrectTagValue,
    /// User wants to reject permission to logon.
    RejectLogon,
}

/// Error result that can occurs from a `on_msg_from_app` callback.
#[derive(Debug)]
pub enum MsgFromAppError {
    /// Field not found inside a message.
    FieldNotFound,
    /// Field has a badly formatted value.
    IncorrectDataFormat,
    /// Field has a value that is out of range.
    IncorrectTagValue,
    /// Message type not supported by application.
    UnsupportedMessageType,
}

trait AsFixCallbackCode {
    fn as_callback_code(&self) -> i8;
}

fn callback_to_code<T: AsFixCallbackCode>(input: Result<Result<(), T>, Box<dyn Any + Send>>) -> i8 {
    match input {
        Err(_) => 0, // Just ignore the panic from rust and let FIX engine continue its workflow.
        Ok(Ok(())) => 0, // Everything goes right 🎇!
        Ok(Err(x)) => x.as_callback_code(), // Use as deliberately change the control flow.
    }
}

impl AsFixCallbackCode for MsgToAppError {
    fn as_callback_code(&self) -> i8 {
        quickfix_ffi::CALLBACK_RESULT_DO_NOT_SEND
    }
}

impl AsFixCallbackCode for MsgFromAdminError {
    fn as_callback_code(&self) -> i8 {
        match self {
            Self::FieldNotFound => quickfix_ffi::CALLBACK_RESULT_FIELD_NOT_FOUND,
            Self::IncorrectDataFormat => quickfix_ffi::CALLBACK_RESULT_INCORRECT_DATA_FORMAT,
            Self::IncorrectTagValue => quickfix_ffi::CALLBACK_RESULT_INCORRECT_TAG_VALUE,
            Self::RejectLogon => quickfix_ffi::CALLBACK_RESULT_REJECT_LOGON,
        }
    }
}

impl AsFixCallbackCode for MsgFromAppError {
    fn as_callback_code(&self) -> i8 {
        match self {
            Self::FieldNotFound => quickfix_ffi::CALLBACK_RESULT_FIELD_NOT_FOUND,
            Self::IncorrectDataFormat => quickfix_ffi::CALLBACK_RESULT_INCORRECT_DATA_FORMAT,
            Self::IncorrectTagValue => quickfix_ffi::CALLBACK_RESULT_INCORRECT_TAG_VALUE,
            Self::UnsupportedMessageType => quickfix_ffi::CALLBACK_RESULT_UNSUPPORTED_MESSAGE_TYPE,
        }
    }
}

/// These methods notify your application about events that happen on active FIX sessions.
///
/// There is no guarantee how many threads will be calling these functions.
#[allow(unused_variables)]
pub trait ApplicationCallback {
    /// On session created.
    fn on_create(&self, session: &SessionId) {}

    /// On session logon.
    fn on_logon(&self, session: &SessionId) {}

    /// On session logout.
    fn on_logout(&self, session: &SessionId) {}

    /// Called before sending message to admin level.
    ///
    /// Message can be updated at this stage.
    fn on_msg_to_admin(&self, msg: &mut Message, session: &SessionId) {}

    /// Called before sending message to application level.
    ///
    /// Message can be updated at this stage.
    fn on_msg_to_app(&self, msg: &mut Message, session: &SessionId) -> Result<(), MsgToAppError> {
        Ok(())
    }

    /// Called after received a message from admin level.
    fn on_msg_from_admin(
        &self,
        msg: &Message,
        session: &SessionId,
    ) -> Result<(), MsgFromAdminError> {
        Ok(())
    }

    /// Called after received a message from application level.
    fn on_msg_from_app(&self, msg: &Message, session: &SessionId) -> Result<(), MsgFromAppError> {
        Ok(())
    }
}

/// Application callback wrapper.
#[derive(Debug)]
pub struct Application<'a, C: ApplicationCallback>(pub(crate) FixApplication_t, PhantomData<&'a C>);

impl<'a, C> Application<'a, C>
where
    C: ApplicationCallback + 'static,
{
    /// Try create new struct from its underlying components.
    pub fn try_new(callbacks: &'a C) -> Result<Self, QuickFixError> {
        match unsafe {
            FixApplication_new(
                callbacks as *const C as *const ffi::c_void,
                &Self::CALLBACKS,
            )
        } {
            Some(fix_application) => Ok(Self(fix_application, PhantomData)),
            None => Err(QuickFixError::from_last_error()),
        }
    }

    const CALLBACKS: FixApplicationCallbacks_t = FixApplicationCallbacks_t {
        onCreate: Self::on_create,
        onLogon: Self::on_logon,
        onLogout: Self::on_logout,
        toAdmin: Self::to_admin,
        toApp: Self::to_app,
        fromAdmin: Self::from_admin,
        fromApp: Self::from_app,
    };

    extern "C" fn on_create(data: *const ffi::c_void, session: FixSessionID_t) {
        let session_id = ManuallyDrop::new(SessionId(session));

        let _ = catch_unwind(|| {
            let this = unsafe { &*(data as *const C) };
            this.on_create(&session_id);
        });
    }

    extern "C" fn on_logon(data: *const ffi::c_void, session: FixSessionID_t) {
        let session_id = ManuallyDrop::new(SessionId(session));

        let _ = catch_unwind(|| {
            let this = unsafe { &*(data as *const C) };
            this.on_logon(&session_id);
        });
    }

    extern "C" fn on_logout(data: *const ffi::c_void, session: FixSessionID_t) {
        let session_id = ManuallyDrop::new(SessionId(session));

        let _ = catch_unwind(|| {
            let this = unsafe { &*(data as *const C) };
            this.on_logout(&session_id);
        });
    }

    extern "C" fn to_admin(data: *const ffi::c_void, msg: FixMessage_t, session: FixSessionID_t) {
        let session_id = ManuallyDrop::new(SessionId(session));

        let _ = catch_unwind(|| {
            let this = unsafe { &*(data as *const C) };
            let mut msg = ManuallyDrop::new(Message(msg));
            this.on_msg_to_admin(&mut msg, &session_id);
        });
    }

    extern "C" fn to_app(
        data: *const ffi::c_void,
        msg: FixMessage_t,
        session: FixSessionID_t,
    ) -> i8 {
        let session_id = ManuallyDrop::new(SessionId(session));

        let output_code = catch_unwind(|| {
            let this = unsafe { &*(data as *const C) };
            let mut msg = ManuallyDrop::new(Message(msg));
            this.on_msg_to_app(&mut msg, &session_id)
        });

        callback_to_code(output_code)
    }

    extern "C" fn from_admin(
        data: *const ffi::c_void,
        msg: FixMessage_t,
        session: FixSessionID_t,
    ) -> i8 {
        let msg = ManuallyDrop::new(Message(msg));
        let session_id = ManuallyDrop::new(SessionId(session));

        let output_code = catch_unwind(|| {
            let this = unsafe { &*(data as *const C) };
            this.on_msg_from_admin(&msg, &session_id)
        });

        callback_to_code(output_code)
    }

    extern "C" fn from_app(
        data: *const ffi::c_void,
        msg: FixMessage_t,
        session: FixSessionID_t,
    ) -> i8 {
        let msg = ManuallyDrop::new(Message(msg));
        let session_id = ManuallyDrop::new(SessionId(session));

        let output_code = catch_unwind(|| {
            let this = unsafe { &*(data as *const C) };
            this.on_msg_from_app(&msg, &session_id)
        });

        callback_to_code(output_code)
    }
}

impl<C: ApplicationCallback> Drop for Application<'_, C> {
    fn drop(&mut self) {
        unsafe { FixApplication_delete(self.0) };
    }
}
