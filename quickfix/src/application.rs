use std::ffi;

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
pub struct Application<C: ApplicationCallback> {
    pub(crate) fix_application: FixApplication_t,
    #[allow(dead_code)]
    fix_callbacks: Box<FixApplicationCallbacks_t>,
    #[allow(dead_code)]
    callbacks: Box<C>,
}

impl<C: ApplicationCallback> Application<C> {
    pub fn try_new(callbacks: C) -> Result<Self, QuickFixError> {
        let callbacks = Box::new(callbacks);
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
                callbacks.as_ref() as *const C as *const ffi::c_void,
                &*fix_callbacks,
            )
        } {
            Some(fix_application) => Ok(Self {
                fix_application,
                fix_callbacks,
                callbacks,
            }),
            None => todo!(),
        }
    }

    extern "C" fn on_create(data: *const ffi::c_void, session: FixSessionID_t) {
        let this = unsafe { &*(data as *const C) };
        this.on_create(&SessionId(session))
    }

    extern "C" fn on_logon(data: *const ffi::c_void, session: FixSessionID_t) {
        let this = unsafe { &*(data as *const C) };
        this.on_logon(&SessionId(session))
    }

    extern "C" fn on_logout(data: *const ffi::c_void, session: FixSessionID_t) {
        let this = unsafe { &*(data as *const C) };
        this.on_logout(&SessionId(session))
    }

    extern "C" fn to_admin(data: *const ffi::c_void, msg: FixMessage_t, session: FixSessionID_t) {
        let this = unsafe { &*(data as *const C) };
        this.on_msg_to_admin(&mut Message(msg), &SessionId(session))
    }

    extern "C" fn to_app(data: *const ffi::c_void, msg: FixMessage_t, session: FixSessionID_t) {
        let this = unsafe { &*(data as *const C) };
        this.on_msg_to_app(&mut Message(msg), &SessionId(session))
    }

    extern "C" fn from_admin(data: *const ffi::c_void, msg: FixMessage_t, session: FixSessionID_t) {
        let this = unsafe { &*(data as *const C) };
        this.on_msg_from_admin(&Message(msg), &SessionId(session))
    }

    extern "C" fn from_app(data: *const ffi::c_void, msg: FixMessage_t, session: FixSessionID_t) {
        let this = unsafe { &*(data as *const C) };
        this.on_msg_from_app(&Message(msg), &SessionId(session))
    }
}

impl<C: ApplicationCallback> Drop for Application<C> {
    fn drop(&mut self) {
        unsafe { FixApplication_delete(self.fix_application) };
    }
}
