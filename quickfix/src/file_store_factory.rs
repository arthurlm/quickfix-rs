use quickfix_ffi::{FixFileStoreFactory_delete, FixFileStoreFactory_new, FixFileStoreFactory_t};

use crate::{QuickFixError, SessionSettings};

#[derive(Debug)]
pub struct FileStoreFactory(pub(crate) FixFileStoreFactory_t);

impl FileStoreFactory {
    pub fn try_new(settings: &SessionSettings) -> Result<Self, QuickFixError> {
        unsafe { FixFileStoreFactory_new(settings.0) }
            .map(Self)
            .ok_or(QuickFixError::InvalidFunctionReturn)
    }
}

impl Drop for FileStoreFactory {
    fn drop(&mut self) {
        unsafe { FixFileStoreFactory_delete(self.0) }
    }
}
