use quickfix_ffi::{
    FixFileMessageStoreFactory_new, FixMemoryMessageStoreFactory_new,
    FixMessageStoreFactory_delete, FixMessageStoreFactory_t,
};

use crate::{QuickFixError, SessionSettings};

///  Object can be converted as a foreign object representing a `MessageStore`.
pub trait FfiMessageStoreFactory {
    /// Get a representation of the message store as a FFI pointer.
    fn as_ffi_ptr(&self) -> FixMessageStoreFactory_t;
}

/// File based implementation of `MessageStore`.
#[derive(Debug)]
pub struct FileMessageStoreFactory(FixMessageStoreFactory_t);

impl FileMessageStoreFactory {
    /// Try to create new struct from settings.
    pub fn try_new(settings: &SessionSettings) -> Result<Self, QuickFixError> {
        unsafe { FixFileMessageStoreFactory_new(settings.0) }
            .map(Self)
            .ok_or(QuickFixError::NullFunctionReturn)
    }
}

impl FfiMessageStoreFactory for FileMessageStoreFactory {
    fn as_ffi_ptr(&self) -> FixMessageStoreFactory_t {
        self.0
    }
}

impl Drop for FileMessageStoreFactory {
    fn drop(&mut self) {
        unsafe { FixMessageStoreFactory_delete(self.0) }
    }
}

/// In memory implementation of `MessageStore`.
#[derive(Debug)]
pub struct MemoryMessageStoreFactory(FixMessageStoreFactory_t);

impl MemoryMessageStoreFactory {
    /// Create new struct.
    pub fn new() -> Self {
        unsafe { FixMemoryMessageStoreFactory_new() }
            .map(Self)
            .expect("Fail to allocate MemoryMessageStore")
    }
}

impl FfiMessageStoreFactory for MemoryMessageStoreFactory {
    fn as_ffi_ptr(&self) -> FixMessageStoreFactory_t {
        self.0
    }
}

impl Default for MemoryMessageStoreFactory {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for MemoryMessageStoreFactory {
    fn drop(&mut self) {
        unsafe { FixMessageStoreFactory_delete(self.0) }
    }
}
