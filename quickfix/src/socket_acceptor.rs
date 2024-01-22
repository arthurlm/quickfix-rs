use std::marker::PhantomData;

use quickfix_ffi::{
    FixSocketAcceptor_block, FixSocketAcceptor_delete, FixSocketAcceptor_isLoggedOn,
    FixSocketAcceptor_isStopped, FixSocketAcceptor_new, FixSocketAcceptor_poll,
    FixSocketAcceptor_start, FixSocketAcceptor_stop, FixSocketAcceptor_t,
};

use crate::{
    utils::{ffi_code_to_bool, ffi_code_to_result},
    Application, ApplicationCallback, ConnectionHandler, FfiMessageStoreFactory, LogCallback,
    LogFactory, QuickFixError, SessionSettings,
};

/// Socket implementation of incoming connections handler.
#[derive(Debug)]
pub struct SocketAcceptor<'a, A, L, S>
where
    A: ApplicationCallback,
    S: FfiMessageStoreFactory,
    L: LogCallback,
{
    inner: FixSocketAcceptor_t,
    phantom_application: PhantomData<&'a A>,
    phantom_message_store_factory: PhantomData<&'a S>,
    phantom_log_factory: PhantomData<&'a L>,
}

impl<'a, A, L, S> SocketAcceptor<'a, A, L, S>
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
    ) -> Result<Self, QuickFixError> {
        match unsafe {
            FixSocketAcceptor_new(
                application.0,
                store_factory.as_ffi_ptr(),
                settings.0,
                log_factory.0,
            )
        } {
            Some(inner) => Ok(Self {
                inner,
                phantom_application: PhantomData,
                phantom_message_store_factory: PhantomData,
                phantom_log_factory: PhantomData,
            }),
            None => Err(QuickFixError::null()),
        }
    }
}

impl<A, L, S> ConnectionHandler for SocketAcceptor<'_, A, L, S>
where
    A: ApplicationCallback,
    S: FfiMessageStoreFactory,
    L: LogCallback,
{
    fn start(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketAcceptor_start(self.inner) })
    }

    fn block(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketAcceptor_block(self.inner) })
    }

    fn poll(&mut self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketAcceptor_poll(self.inner) })
    }

    fn stop(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketAcceptor_stop(self.inner) })
    }

    fn is_logged_on(&self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketAcceptor_isLoggedOn(self.inner) })
    }

    fn is_stopped(&self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketAcceptor_isStopped(self.inner) })
    }
}

impl<A, L, S> Drop for SocketAcceptor<'_, A, L, S>
where
    A: ApplicationCallback,
    S: FfiMessageStoreFactory,
    L: LogCallback,
{
    fn drop(&mut self) {
        let _ = self.stop();
        unsafe { FixSocketAcceptor_delete(self.inner) }
    }
}
