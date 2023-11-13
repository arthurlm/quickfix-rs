use std::marker::PhantomData;

use quickfix_ffi::{
    FixSocketAcceptor_block, FixSocketAcceptor_delete, FixSocketAcceptor_isLoggedOn,
    FixSocketAcceptor_isStopped, FixSocketAcceptor_new, FixSocketAcceptor_poll,
    FixSocketAcceptor_start, FixSocketAcceptor_stop, FixSocketAcceptor_t,
};

use crate::{
    utils::{ffi_code_to_bool, ffi_code_to_result},
    Application, ApplicationCallback, FileLogFactory, FileStoreFactory, QuickFixError,
    SessionSettings,
};

#[derive(Debug)]
pub struct SocketAcceptor<'a, C: ApplicationCallback> {
    pub(crate) inner: FixSocketAcceptor_t,
    phantom: PhantomData<&'a C>,
}

impl<'a, C: ApplicationCallback> SocketAcceptor<'a, C> {
    pub fn try_new(
        settings: &SessionSettings,
        application: &'a Application<C>,
        store_factory: &'a FileStoreFactory,
        log_factory: &'a FileLogFactory,
    ) -> Result<Self, QuickFixError> {
        match unsafe {
            FixSocketAcceptor_new(application.0, store_factory.0, settings.0, log_factory.0)
        } {
            Some(inner) => Ok(Self {
                inner,
                phantom: PhantomData,
            }),
            None => Err(QuickFixError::InvalidFunctionReturn),
        }
    }

    pub fn start(&self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketAcceptor_start(self.inner) })
    }

    pub fn block(&self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketAcceptor_block(self.inner) })
    }

    pub fn poll(&self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketAcceptor_poll(self.inner) })
    }

    pub fn stop(&self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketAcceptor_stop(self.inner) })
    }

    pub fn is_logged_on(&self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketAcceptor_isLoggedOn(self.inner) })
    }

    pub fn is_stopped(&self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketAcceptor_isStopped(self.inner) })
    }
}

impl<C: ApplicationCallback> Drop for SocketAcceptor<'_, C> {
    fn drop(&mut self) {
        unsafe { FixSocketAcceptor_delete(self.inner) }
    }
}
