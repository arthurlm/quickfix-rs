use std::marker::PhantomData;

use quickfix_ffi::{
    FixSocketAcceptor_block, FixSocketAcceptor_delete, FixSocketAcceptor_isLoggedOn,
    FixSocketAcceptor_isStopped, FixSocketAcceptor_new, FixSocketAcceptor_poll,
    FixSocketAcceptor_start, FixSocketAcceptor_stop, FixSocketAcceptor_t,
};

use crate::{
    utils::{ffi_code_to_bool, ffi_code_to_result},
    Application, ApplicationCallback, ConnectionHandler, FileStoreFactory, LogCallback, LogFactory,
    QuickFixError, SessionSettings,
};

#[derive(Debug)]
pub struct SocketAcceptor<'a, A, L>
where
    A: ApplicationCallback,
    L: LogCallback,
{
    pub(crate) inner: FixSocketAcceptor_t,
    phantom1: PhantomData<&'a A>,
    phantom2: PhantomData<&'a L>,
}

impl<'a, A, L> SocketAcceptor<'a, A, L>
where
    A: ApplicationCallback,
    L: LogCallback,
{
    pub fn try_new(
        settings: &SessionSettings,
        application: &'a Application<A>,
        store_factory: &'a FileStoreFactory,
        log_factory: &'a LogFactory<L>,
    ) -> Result<Self, QuickFixError> {
        match unsafe {
            FixSocketAcceptor_new(application.0, store_factory.0, settings.0, log_factory.0)
        } {
            Some(inner) => Ok(Self {
                inner,
                phantom1: PhantomData,
                phantom2: PhantomData,
            }),
            None => Err(QuickFixError::InvalidFunctionReturn),
        }
    }
}

impl<A, L> ConnectionHandler for SocketAcceptor<'_, A, L>
where
    A: ApplicationCallback,
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

impl<A, L> Drop for SocketAcceptor<'_, A, L>
where
    A: ApplicationCallback,
    L: LogCallback,
{
    fn drop(&mut self) {
        unsafe { FixSocketAcceptor_delete(self.inner) }
    }
}
