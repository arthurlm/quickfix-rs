use quickfix_ffi::{FixFileLogFactory_delete, FixFileLogFactory_new, FixFileLogFactory_t};

use crate::{QuickFixError, SessionSettings};

#[derive(Debug)]
pub struct FileLogFactory(pub(crate) FixFileLogFactory_t);

impl FileLogFactory {
    pub fn try_new(settings: &SessionSettings) -> Result<Self, QuickFixError> {
        unsafe { FixFileLogFactory_new(settings.0) }
            .map(Self)
            .ok_or(QuickFixError::InvalidFunctionReturn)
    }
}

impl Drop for FileLogFactory {
    fn drop(&mut self) {
        unsafe { FixFileLogFactory_delete(self.0) }
    }
}
