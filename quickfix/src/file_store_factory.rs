use quickfix_ffi::{FixFileStoreFactory_delete, FixFileStoreFactory_new, FixFileStoreFactory_t};

use crate::{QuickFixError, SessionSettings};

/// File based implementation of MessageStore.
#[derive(Debug)]
pub struct FileStoreFactory(pub(crate) FixFileStoreFactory_t);

impl FileStoreFactory {
    /// Try to create new struct from settings.
    pub fn try_new(settings: &SessionSettings) -> Result<Self, QuickFixError> {
        unsafe { FixFileStoreFactory_new(settings.0) }
            .map(Self)
            .ok_or(QuickFixError::NullFunctionReturn)
    }
}

impl Drop for FileStoreFactory {
    fn drop(&mut self) {
        unsafe { FixFileStoreFactory_delete(self.0) }
    }
}
