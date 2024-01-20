use std::{ffi, marker::PhantomData, mem::ManuallyDrop, panic::catch_unwind};

use quickfix_ffi::{
    FixApplicationCallbacks_t, FixApplication_delete, FixApplication_new, FixApplication_t,
    FixMessage_t, FixSessionID_t,
};

use crate::{Message, QuickFixError, SessionId};

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
    fn on_msg_to_app(&self, msg: &mut Message, session: &SessionId) {}

    /// Called after received a message from admin level.
    fn on_msg_from_admin(&self, msg: &Message, session: &SessionId) {}

    /// Called after received a message from application level.
    fn on_msg_from_app(&self, msg: &Message, session: &SessionId) {}
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
            None => Err(QuickFixError::NullFunctionReturn),
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

    extern "C" fn to_app(data: *const ffi::c_void, msg: FixMessage_t, session: FixSessionID_t) {
        let session_id = ManuallyDrop::new(SessionId(session));

        let _ = catch_unwind(|| {
            let this = unsafe { &*(data as *const C) };
            let mut msg = ManuallyDrop::new(Message(msg));
            this.on_msg_to_app(&mut msg, &session_id);
        });
    }

    extern "C" fn from_admin(data: *const ffi::c_void, msg: FixMessage_t, session: FixSessionID_t) {
        let msg = ManuallyDrop::new(Message(msg));
        let session_id = ManuallyDrop::new(SessionId(session));

        let _ = catch_unwind(|| {
            let this = unsafe { &*(data as *const C) };
            this.on_msg_from_admin(&msg, &session_id);
        });
    }

    extern "C" fn from_app(data: *const ffi::c_void, msg: FixMessage_t, session: FixSessionID_t) {
        let msg = ManuallyDrop::new(Message(msg));
        let session_id = ManuallyDrop::new(SessionId(session));

        let _ = catch_unwind(|| {
            let this = unsafe { &*(data as *const C) };
            this.on_msg_from_app(&msg, &session_id);
        });
    }
}

impl<C: ApplicationCallback> Drop for Application<'_, C> {
    fn drop(&mut self) {
        unsafe { FixApplication_delete(self.0) };
    }
}
