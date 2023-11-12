use crate::{QuickFixError, SessionSettings};

#[derive(Debug)]
pub struct FileLogFactory(pub(crate) quickfix_ffi::FixFileLogFactory_t);

impl FileLogFactory {
    pub fn try_new(settings: &SessionSettings) -> Result<Self, QuickFixError> {
        match unsafe { quickfix_ffi::FixFileLogFactory_new(settings.0) } {
            Some(val) => Ok(Self(val)),
            None => Err(QuickFixError::InvalidFunctionReturn),
        }
    }
}

impl Drop for FileLogFactory {
    fn drop(&mut self) {
        unsafe { quickfix_ffi::FixFileLogFactory_delete(self.0) }
    }
}
