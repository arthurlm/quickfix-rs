use std::marker::PhantomData;

use quickfix_ffi::{
    FixSocketInitiator_block, FixSocketInitiator_delete, FixSocketInitiator_isLoggedOn,
    FixSocketInitiator_isStopped, FixSocketInitiator_new, FixSocketInitiator_poll,
    FixSocketInitiator_start, FixSocketInitiator_stop, FixSocketInitiator_t,
};

use crate::{
    utils::{ffi_code_to_bool, ffi_code_to_result},
    Application, ApplicationCallback, FileLogFactory, FileStoreFactory, QuickFixError,
    SessionSettings,
};

#[derive(Debug)]
pub struct SocketInitiator<'a, C: ApplicationCallback> {
    pub(crate) inner: FixSocketInitiator_t,
    phantom: PhantomData<&'a C>,
}

impl<'a, C: ApplicationCallback> SocketInitiator<'a, C> {
    pub fn try_new(
        settings: &SessionSettings,
        application: &'a Application<C>,
        store_factory: &'a FileStoreFactory,
        log_factory: &'a FileLogFactory,
    ) -> Result<Self, QuickFixError> {
        match unsafe {
            FixSocketInitiator_new(application.0, store_factory.0, settings.0, log_factory.0)
        } {
            Some(inner) => Ok(Self {
                inner,
                phantom: PhantomData,
            }),
            None => Err(QuickFixError::InvalidFunctionReturn),
        }
    }

    pub fn start(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketInitiator_start(self.inner) })
    }

    pub fn block(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketInitiator_block(self.inner) })
    }

    pub fn poll(&mut self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketInitiator_poll(self.inner) })
    }

    pub fn stop(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketInitiator_stop(self.inner) })
    }

    pub fn is_logged_on(&self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketInitiator_isLoggedOn(self.inner) })
    }

    pub fn is_stopped(&self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketInitiator_isStopped(self.inner) })
    }
}

impl<C: ApplicationCallback> Drop for SocketInitiator<'_, C> {
    fn drop(&mut self) {
        unsafe { FixSocketInitiator_delete(self.inner) }
    }
}
