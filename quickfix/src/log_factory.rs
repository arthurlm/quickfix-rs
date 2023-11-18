use std::{
    ffi,
    io::{self, Write},
    marker::PhantomData,
    mem::ManuallyDrop,
};

use quickfix_ffi::{
    FixLogCallbacks_t, FixLogFactory_delete, FixLogFactory_new, FixLogFactory_t, FixSessionID_t,
    NullableCStr,
};

use crate::{utils::read_checked_cstr, QuickFixError, SessionId};

#[allow(unused_variables)]
pub trait LogCallback {
    fn on_incoming(&self, session_id: Option<&SessionId>, msg: &str) {}
    fn on_outgoing(&self, session_id: Option<&SessionId>, msg: &str) {}
    fn on_event(&self, session_id: Option<&SessionId>, msg: &str) {}
}

#[derive(Debug)]
pub struct LogFactory<'a, C: LogCallback>(pub(crate) FixLogFactory_t, PhantomData<&'a C>);

impl<'a, C> LogFactory<'a, C>
where
    C: LogCallback + 'static,
{
    pub fn try_new(callbacks: &'a C) -> Result<Self, QuickFixError> {
        match unsafe {
            FixLogFactory_new(
                callbacks as *const C as *const ffi::c_void,
                &Self::CALLBACKS,
            )
        } {
            Some(fix_log_factory) => Ok(Self(fix_log_factory, PhantomData)),
            None => Err(QuickFixError::InvalidFunctionReturn),
        }
    }

    const CALLBACKS: FixLogCallbacks_t = FixLogCallbacks_t {
        onIncoming: Self::on_incoming,
        onOutgoing: Self::on_outgoing,
        onEvent: Self::on_event,
    };

    extern "C" fn on_incoming(
        data: *const ffi::c_void,
        session_id_ptr: Option<FixSessionID_t>,
        msg_ptr: NullableCStr,
    ) {
        let Some(msg_ptr) = msg_ptr else { return };
        let this = unsafe { &*(data as *const C) };
        let msg = read_checked_cstr(msg_ptr);
        let session_id = session_id_ptr.map(|ptr| ManuallyDrop::new(SessionId(ptr)));
        this.on_incoming(session_id.as_deref(), &msg);
    }

    extern "C" fn on_outgoing(
        data: *const ffi::c_void,
        session_id_ptr: Option<FixSessionID_t>,
        msg_ptr: NullableCStr,
    ) {
        let Some(msg_ptr) = msg_ptr else { return };
        let this = unsafe { &*(data as *const C) };
        let msg = read_checked_cstr(msg_ptr);
        let session_id = session_id_ptr.map(|ptr| ManuallyDrop::new(SessionId(ptr)));
        this.on_outgoing(session_id.as_deref(), &msg);
    }

    extern "C" fn on_event(
        data: *const ffi::c_void,
        session_id_ptr: Option<FixSessionID_t>,
        msg_ptr: NullableCStr,
    ) {
        let Some(msg_ptr) = msg_ptr else { return };
        let this = unsafe { &*(data as *const C) };
        let msg = read_checked_cstr(msg_ptr);
        let session_id = session_id_ptr.map(|ptr| ManuallyDrop::new(SessionId(ptr)));
        this.on_event(session_id.as_deref(), &msg);
    }
}

impl<C: LogCallback> Drop for LogFactory<'_, C> {
    fn drop(&mut self) {
        unsafe { FixLogFactory_delete(self.0) }
    }
}

#[derive(Debug)]
pub enum StdLogger {
    Stdout,
    Stderr,
}

impl StdLogger {
    fn print(&self, text: &str) {
        let _ = match self {
            StdLogger::Stdout => writeln!(io::stdout(), "{text}"),
            StdLogger::Stderr => writeln!(io::stdout(), "{text}"),
        };
    }
}

impl LogCallback for StdLogger {
    fn on_incoming(&self, session_id: Option<&SessionId>, msg: &str) {
        self.print(&format!("FIX incoming: {session_id:?}: {msg}"));
    }

    fn on_outgoing(&self, session_id: Option<&SessionId>, msg: &str) {
        self.print(&format!("FIX outgoing: {session_id:?}: {msg}"));
    }

    fn on_event(&self, session_id: Option<&SessionId>, msg: &str) {
        self.print(&format!("FIX event: {session_id:?}: {msg}"));
    }
}

pub struct RustLogger;

impl LogCallback for RustLogger {
    fn on_incoming(&self, session_id: Option<&SessionId>, msg: &str) {
        log::info!("FIX: Incoming: {session_id:?}: {msg}");
    }

    fn on_outgoing(&self, session_id: Option<&SessionId>, msg: &str) {
        log::info!("FIX: Outcoming: {session_id:?}: {msg}");
    }

    fn on_event(&self, session_id: Option<&SessionId>, msg: &str) {
        log::info!("FIX: Event: {session_id:?}: {msg}");
    }
}
