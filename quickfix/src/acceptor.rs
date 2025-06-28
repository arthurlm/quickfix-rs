use std::marker::PhantomData;

use quickfix_ffi::{
    FixAcceptor_block, FixAcceptor_delete, FixAcceptor_getSession, FixAcceptor_isLoggedOn,
    FixAcceptor_isStopped, FixAcceptor_new, FixAcceptor_poll, FixAcceptor_start, FixAcceptor_stop,
    FixAcceptor_t,
};

use crate::{
    utils::{ffi_code_to_bool, ffi_code_to_result},
    Application, ApplicationCallback, ConnectionHandler, ConnectionMode, FfiMessageStoreFactory,
    LogCallback, LogFactory, QuickFixError, Session, SessionContainer, SessionId, SessionSettings,
};

/// Socket implementation of incoming connections handler.
#[derive(Debug)]
pub struct Acceptor<'a, A, L, S>
where
    A: ApplicationCallback,
    S: FfiMessageStoreFactory,
    L: LogCallback,
{
    inner: FixAcceptor_t,
    phantom_application: PhantomData<&'a A>,
    phantom_message_store_factory: PhantomData<&'a S>,
    phantom_log_factory: PhantomData<&'a L>,
}

impl<'a, A, L, S> Acceptor<'a, A, L, S>
where
    A: ApplicationCallback,
    S: FfiMessageStoreFactory,
    L: LogCallback,
{
    /// Try create new struct from its mandatory components.
    pub fn try_new(
        settings: &SessionSettings,
        application: &'a Application<A>,
        store_factory: &'a S,
        log_factory: &'a LogFactory<L>,
        server_mode: ConnectionMode,
    ) -> Result<Self, QuickFixError> {
        match unsafe {
            FixAcceptor_new(
                application.0,
                store_factory.as_ffi_ptr(),
                settings.0,
                log_factory.0,
                server_mode.is_single_threaded() as i8,
            )
        } {
            Some(inner) => Ok(Self {
                inner,
                phantom_application: PhantomData,
                phantom_message_store_factory: PhantomData,
                phantom_log_factory: PhantomData,
            }),
            None => Err(QuickFixError::from_last_error()),
        }
    }
}

impl<A, L, S> ConnectionHandler for Acceptor<'_, A, L, S>
where
    A: ApplicationCallback,
    S: FfiMessageStoreFactory,
    L: LogCallback,
{
    fn start(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixAcceptor_start(self.inner) })
    }

    fn block(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixAcceptor_block(self.inner) })
    }

    fn poll(&mut self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixAcceptor_poll(self.inner) })
    }

    fn stop(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixAcceptor_stop(self.inner) })
    }

    fn is_logged_on(&self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixAcceptor_isLoggedOn(self.inner) })
    }

    fn is_stopped(&self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixAcceptor_isStopped(self.inner) })
    }
}

impl<A, L, S> SessionContainer for Acceptor<'_, A, L, S>
where
    A: ApplicationCallback,
    S: FfiMessageStoreFactory,
    L: LogCallback,
{
    fn session(&self, session_id: SessionId) -> Result<Session<'_>, QuickFixError> {
        unsafe {
            FixAcceptor_getSession(self.inner, session_id.0)
                .map(|inner| Session {
                    inner,
                    phantom_container: PhantomData,
                })
                .ok_or_else(|| {
                    QuickFixError::SessionNotFound(format!("No session found: {session_id:?}"))
                })
        }
    }
}

impl<A, L, S> Drop for Acceptor<'_, A, L, S>
where
    A: ApplicationCallback,
    S: FfiMessageStoreFactory,
    L: LogCallback,
{
    fn drop(&mut self) {
        let _ = self.stop();
        unsafe { FixAcceptor_delete(self.inner) }
    }
}
