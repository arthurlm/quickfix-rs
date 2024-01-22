use quickfix_ffi::{
    FixMessageStoreFactory_delete, FixMessageStoreFactory_t, FixPostgresMessageStoreFactory_new,
};

use crate::{FfiMessageStoreFactory, QuickFixError, SessionSettings};

/// PostgreSQL based implementation of `MessageStore`.
#[derive(Debug)]
pub struct PostgresMessageStoreFactory(FixMessageStoreFactory_t);

impl PostgresMessageStoreFactory {
    /// Try to create new struct from settings.
    pub fn try_new(settings: &SessionSettings) -> Result<Self, QuickFixError> {
        unsafe { FixPostgresMessageStoreFactory_new(settings.0) }
            .map(Self)
            .ok_or_else(QuickFixError::from_last_error)
    }
}

impl FfiMessageStoreFactory for PostgresMessageStoreFactory {
    fn as_ffi_ptr(&self) -> FixMessageStoreFactory_t {
        self.0
    }
}

impl Drop for PostgresMessageStoreFactory {
    fn drop(&mut self) {
        unsafe { FixMessageStoreFactory_delete(self.0) }
    }
}
