use std::{ffi, marker::PhantomData, mem::ManuallyDrop};

use quickfix_ffi::{
    FixApplicationCallbacks_t, FixApplication_delete, FixApplication_t, FixMessage_t,
    FixSessionID_t,
};

use crate::{Message, QuickFixError, SessionId};

#[allow(unused_variables)]
pub trait ApplicationCallback {
    fn on_create(&self, session: &SessionId) {}
    fn on_logon(&self, session: &SessionId) {}
    fn on_logout(&self, session: &SessionId) {}
    fn on_msg_to_admin(&self, msg: &mut Message, session: &SessionId) {}
    fn on_msg_to_app(&self, msg: &mut Message, session: &SessionId) {}
    fn on_msg_from_admin(&self, msg: &Message, session: &SessionId) {}
    fn on_msg_from_app(&self, msg: &Message, session: &SessionId) {}
}

#[derive(Debug)]
pub struct Application<'a, C: ApplicationCallback> {
    pub(crate) fix_application: FixApplication_t,
    #[allow(dead_code)]
    fix_callbacks: Box<FixApplicationCallbacks_t>,
    phantom: PhantomData<&'a C>,
}

impl<'a, C: ApplicationCallback> Application<'a, C> {
    pub fn try_new(callbacks: &'a C) -> Result<Self, QuickFixError> {
        let fix_callbacks = Box::new(FixApplicationCallbacks_t {
            onCreate: Self::on_create,
            onLogon: Self::on_logon,
            onLogout: Self::on_logout,
            toAdmin: Self::to_admin,
            toApp: Self::to_app,
            fromAdmin: Self::from_admin,
            fromApp: Self::from_app,
        });

        match unsafe {
            quickfix_ffi::FixApplication_new(
                callbacks as *const C as *const ffi::c_void,
                &*fix_callbacks,
            )
        } {
            Some(fix_application) => Ok(Self {
                fix_application,
                fix_callbacks,
                phantom: PhantomData,
            }),
            None => todo!(),
        }
    }

    extern "C" fn on_create(data: *const ffi::c_void, session: FixSessionID_t) {
        let this = unsafe { &*(data as *const C) };
        let session_id = ManuallyDrop::new(SessionId(session));
        this.on_create(&session_id)
    }

    extern "C" fn on_logon(data: *const ffi::c_void, session: FixSessionID_t) {
        let this = unsafe { &*(data as *const C) };
        let session_id = ManuallyDrop::new(SessionId(session));
        this.on_logon(&session_id)
    }

    extern "C" fn on_logout(data: *const ffi::c_void, session: FixSessionID_t) {
        let this = unsafe { &*(data as *const C) };
        let session_id = ManuallyDrop::new(SessionId(session));
        this.on_logout(&session_id)
    }

    extern "C" fn to_admin(data: *const ffi::c_void, msg: FixMessage_t, session: FixSessionID_t) {
        let this = unsafe { &*(data as *const C) };
        let mut msg = ManuallyDrop::new(Message(msg));
        let session_id = ManuallyDrop::new(SessionId(session));
        this.on_msg_to_admin(&mut msg, &session_id)
    }

    extern "C" fn to_app(data: *const ffi::c_void, msg: FixMessage_t, session: FixSessionID_t) {
        let this = unsafe { &*(data as *const C) };
        let mut msg = ManuallyDrop::new(Message(msg));
        let session_id = ManuallyDrop::new(SessionId(session));
        this.on_msg_to_app(&mut msg, &session_id)
    }

    extern "C" fn from_admin(data: *const ffi::c_void, msg: FixMessage_t, session: FixSessionID_t) {
        let this = unsafe { &*(data as *const C) };
        let msg = ManuallyDrop::new(Message(msg));
        let session_id = ManuallyDrop::new(SessionId(session));
        this.on_msg_from_admin(&msg, &session_id)
    }

    extern "C" fn from_app(data: *const ffi::c_void, msg: FixMessage_t, session: FixSessionID_t) {
        let this = unsafe { &*(data as *const C) };
        let msg = ManuallyDrop::new(Message(msg));
        let session_id = ManuallyDrop::new(SessionId(session));
        this.on_msg_from_app(&msg, &session_id)
    }
}

impl<C: ApplicationCallback> Drop for Application<'_, C> {
    fn drop(&mut self) {
        unsafe { FixApplication_delete(self.fix_application) };
    }
}
