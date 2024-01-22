use quickfix_ffi::{
    FixMessageStoreFactory_delete, FixMessageStoreFactory_t, FixMysqlMessageStoreFactory_new,
};

use crate::{FfiMessageStoreFactory, QuickFixError, SessionSettings};

/// MySQL based implementation of `MessageStore`.
#[derive(Debug)]
pub struct MySqlMessageStoreFactory(FixMessageStoreFactory_t);

impl MySqlMessageStoreFactory {
    /// Try to create new struct from settings.
    pub fn try_new(settings: &SessionSettings) -> Result<Self, QuickFixError> {
        unsafe { FixMysqlMessageStoreFactory_new(settings.0) }
            .map(Self)
            .ok_or_else(QuickFixError::from_last_error)
    }
}

impl FfiMessageStoreFactory for MySqlMessageStoreFactory {
    fn as_ffi_ptr(&self) -> FixMessageStoreFactory_t {
        self.0
    }
}

impl Drop for MySqlMessageStoreFactory {
    fn drop(&mut self) {
        unsafe { FixMessageStoreFactory_delete(self.0) }
    }
}
