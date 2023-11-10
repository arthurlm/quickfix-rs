use crate::{error::QuickFixError, session_settings::SessionSettings};

#[derive(Debug)]
pub struct FileStoreFactory(pub(crate) quickfix_ffi::FixFileStoreFactory_t);

impl FileStoreFactory {
    pub fn try_new(settings: &SessionSettings) -> Result<Self, QuickFixError> {
        match unsafe { quickfix_ffi::FixFileStoreFactory_new(settings.0) } {
            Some(val) => Ok(Self(val)),
            None => Err(QuickFixError::InvalidFunctionReturn),
        }
    }
}

impl Drop for FileStoreFactory {
    fn drop(&mut self) {
        unsafe { quickfix_ffi::FixFileStoreFactory_delete(self.0) }
    }
}
