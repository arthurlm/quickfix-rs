use std::marker::PhantomData;

use quickfix_ffi::{
    FixSocketInitiator_block, FixSocketInitiator_delete, FixSocketInitiator_isLoggedOn,
    FixSocketInitiator_isStopped, FixSocketInitiator_new, FixSocketInitiator_poll,
    FixSocketInitiator_start, FixSocketInitiator_stop, FixSocketInitiator_t,
};

use crate::{
    utils::{ffi_code_to_bool, ffi_code_to_result},
    Application, ApplicationCallback, ConnectionHandler, FfiMessageStoreFactory, LogCallback,
    LogFactory, QuickFixError, SessionSettings,
};

/// Socket implementation of establishing connections handler.
#[derive(Debug)]
pub struct SocketInitiator<'a, A, L, S>
where
    A: ApplicationCallback,
    S: FfiMessageStoreFactory,
    L: LogCallback,
{
    inner: FixSocketInitiator_t,
    phantom_application: PhantomData<&'a A>,
    phantom_message_store_factory: PhantomData<&'a S>,
    phantom_log_factory: PhantomData<&'a L>,
}

impl<'a, A, L, S> SocketInitiator<'a, A, L, S>
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
            FixSocketInitiator_new(
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

impl<A, L, S> ConnectionHandler for SocketInitiator<'_, A, L, S>
where
    A: ApplicationCallback,
    S: FfiMessageStoreFactory,
    L: LogCallback,
{
    fn start(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketInitiator_start(self.inner) })
    }

    fn block(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketInitiator_block(self.inner) })
    }

    fn poll(&mut self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketInitiator_poll(self.inner) })
    }

    fn stop(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketInitiator_stop(self.inner) })
    }

    fn is_logged_on(&self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketInitiator_isLoggedOn(self.inner) })
    }

    fn is_stopped(&self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketInitiator_isStopped(self.inner) })
    }
}

impl<A, L, S> Drop for SocketInitiator<'_, A, L, S>
where
    A: ApplicationCallback,
    S: FfiMessageStoreFactory,
    L: LogCallback,
{
    fn drop(&mut self) {
        let _ = self.stop();
        unsafe { FixSocketInitiator_delete(self.inner) }
    }
}
