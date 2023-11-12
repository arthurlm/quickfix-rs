use std::marker::PhantomData;

use quickfix_ffi::{
    FixSocketAcceptor_delete, FixSocketAcceptor_new, FixSocketAcceptor_start,
    FixSocketAcceptor_stop, FixSocketAcceptor_t,
};

use crate::{
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
            FixSocketAcceptor_new(
                application.fix_application,
                store_factory.0,
                settings.0,
                log_factory.0,
            )
        } {
            Some(inner) => Ok(Self {
                inner,
                phantom: PhantomData,
            }),
            None => Err(QuickFixError::InvalidFunctionReturn),
        }
    }

    pub fn start(&self) -> Result<(), QuickFixError> {
        match unsafe { FixSocketAcceptor_start(self.inner) } {
            0 => Ok(()),
            code => Err(QuickFixError::InvalidFunctionReturnCode(code)),
        }
    }

    pub fn stop(&self) -> Result<(), QuickFixError> {
        match unsafe { FixSocketAcceptor_stop(self.inner) } {
            0 => Ok(()),
            code => Err(QuickFixError::InvalidFunctionReturnCode(code)),
        }
    }
}

impl<C: ApplicationCallback> Drop for SocketAcceptor<'_, C> {
    fn drop(&mut self) {
        unsafe { FixSocketAcceptor_delete(self.inner) }
    }
}
