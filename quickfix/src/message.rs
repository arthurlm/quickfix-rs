use std::{ffi::CString, fmt, mem::ManuallyDrop};

use quickfix_ffi::{
    FixMessage_addGroup, FixMessage_copy, FixMessage_copyGroup, FixMessage_copyHeader,
    FixMessage_copyTrailer, FixMessage_delete, FixMessage_fromString, FixMessage_getField,
    FixMessage_getGroupRef, FixMessage_getHeaderRef, FixMessage_getStringLen,
    FixMessage_getTrailerRef, FixMessage_new, FixMessage_readString, FixMessage_removeField,
    FixMessage_setField, FixMessage_t,
};

use crate::{
    group::Group,
    header::Header,
    trailer::Trailer,
    utils::{ffi_code_to_result, read_checked_cstr},
    FieldMap, IntoFixValue, QuickFixError,
};

/// Base class for all FIX messages.
pub struct Message(pub(crate) FixMessage_t);

impl Message {
    /// Create new empty struct.
    pub fn new() -> Self {
        Self::default()
    }

    /// Try create new struct from raw text message.
    pub fn try_from_text(text: &str) -> Result<Self, QuickFixError> {
        let ffi_text = CString::new(text)?;
        unsafe { FixMessage_fromString(ffi_text.as_ptr()) }
            .map(Self)
            .ok_or_else(QuickFixError::from_last_error)
    }

    /// Try reading underlying struct buffer as a FIX string.
    ///
    /// # Performances
    ///
    /// Do not use this method in latency sensitive code.
    ///
    /// String will be generated twice in C++ code:
    /// - Once for getting a safe buffer length.
    /// - Then to copy buffer to rust "memory".
    pub fn to_fix_string(&self) -> Result<String, QuickFixError> {
        unsafe {
            // Prepare output buffer
            let buffer_len = FixMessage_getStringLen(self.0);
            if buffer_len < 0 {
                return Err(QuickFixError::InvalidBufferLen);
            }

            // Allocate buffer on rust side
            let mut buffer = vec![0_u8; buffer_len as usize];
            assert_eq!(buffer.len(), buffer_len as usize);

            // Read text
            ffi_code_to_result(FixMessage_readString(
                self.0,
                buffer.as_mut_ptr().cast(),
                buffer_len,
            ))?;

            // Convert to String
            //
            // NOTE: Here, I deliberately made the choice to drop C weird string / invalid UTF8 string
            //       content. If this happen, there is not so much we can do about ...
            //       Returning no error is sometime nicer, than an incomprehensible error.
            let text = CString::from_vec_with_nul(buffer).unwrap_or_default();
            Ok(text.to_string_lossy().to_string())
        }
    }

    /// Clone struct header part.
    ///
    /// # Panic
    ///
    /// When memory allocation fail in C++ library.
    pub fn clone_header(&self) -> Header {
        unsafe { FixMessage_copyHeader(self.0) }
            .map(Header)
            .expect("Fail to allocate new Header")
    }

    /// Read struct header part.
    ///
    /// # Panic
    ///
    /// When struct pointer cannot be read from `FIX::Message`. This is
    /// something that could not be theoretically possible.
    pub fn with_header<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&Header) -> T,
    {
        let ptr =
            unsafe { FixMessage_getHeaderRef(self.0) }.expect("Fail to get ptr on message header");

        let obj = ManuallyDrop::new(Header(ptr));
        f(&obj)
    }

    /// Read or write struct header part.
    ///
    /// # Panic
    ///
    /// When struct pointer cannot be read from `FIX::Message`. This is
    /// something that could not be theoretically possible.
    pub fn with_header_mut<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Header) -> T,
    {
        let ptr =
            unsafe { FixMessage_getHeaderRef(self.0) }.expect("Fail to get ptr on message header");

        let mut obj = ManuallyDrop::new(Header(ptr));
        f(&mut obj)
    }

    /// Clone struct trailer part.
    ///
    /// # Panic
    ///
    /// When memory allocation fail in C++ library.
    pub fn clone_trailer(&self) -> Trailer {
        unsafe { FixMessage_copyTrailer(self.0) }
            .map(Trailer)
            .expect("Fail to allocate new Trailer")
    }

    /// Read struct trailer part.
    ///
    /// # Panic
    ///
    /// When struct pointer cannot be read from `FIX::Message`. This is
    /// something that could not be theoretically possible.
    pub fn with_trailer<T, F>(&self, f: F) -> T
    where
        F: FnOnce(&Trailer) -> T,
    {
        let ptr = unsafe { FixMessage_getTrailerRef(self.0) }
            .expect("Fail to get ptr on message trailer");

        let obj = ManuallyDrop::new(Trailer(ptr));
        f(&obj)
    }

    /// Read or write struct trailer part.
    ///
    /// # Panic
    ///
    /// When struct pointer cannot be read from `FIX::Message`. This is
    /// something that could not be theoretically possible.
    pub fn with_trailer_mut<T, F>(&mut self, f: F) -> T
    where
        F: FnOnce(&mut Trailer) -> T,
    {
        let ptr = unsafe { FixMessage_getTrailerRef(self.0) }
            .expect("Fail to get ptr on message trailer");

        let mut obj = ManuallyDrop::new(Trailer(ptr));
        f(&mut obj)
    }

    /// Read struct group part for a given tag and group index.
    pub fn with_group<T, F>(&self, index: i32, tag: i32, f: F) -> Option<T>
    where
        F: FnOnce(&Group) -> T,
    {
        if let Some(ptr) = unsafe { FixMessage_getGroupRef(self.0, index, tag) } {
            let obj = ManuallyDrop::new(Group(ptr));
            Some(f(&obj))
        } else {
            None
        }
    }

    /// Read or write struct group part for a given tag and group index.
    pub fn with_group_mut<T, F>(&mut self, index: i32, tag: i32, f: F) -> Option<T>
    where
        F: FnOnce(&mut Group) -> T,
    {
        if let Some(ptr) = unsafe { FixMessage_getGroupRef(self.0, index, tag) } {
            let mut obj = ManuallyDrop::new(Group(ptr));
            Some(f(&mut obj))
        } else {
            None
        }
    }
}

impl FieldMap for Message {
    fn get_field(&self, tag: i32) -> Option<String> {
        unsafe { FixMessage_getField(self.0, tag) }.map(read_checked_cstr)
    }

    fn set_field<V: IntoFixValue>(&mut self, tag: i32, value: V) -> Result<(), QuickFixError> {
        let fix_value = value.into_fix_value()?;
        ffi_code_to_result(unsafe { FixMessage_setField(self.0, tag, fix_value.as_ptr()) })
    }

    fn remove_field(&mut self, tag: i32) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixMessage_removeField(self.0, tag) })
    }

    fn add_group(&mut self, group: &Group) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixMessage_addGroup(self.0, group.0) })?;
        Ok(())
    }

    fn clone_group(&self, index: i32, tag: i32) -> Option<Group> {
        unsafe { FixMessage_copyGroup(self.0, index, tag) }.map(Group)
    }
}

impl Clone for Message {
    fn clone(&self) -> Self {
        Self(unsafe { FixMessage_copy(self.0) }.expect("Fail to clone Message"))
    }
}

impl fmt::Debug for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut printer = f.debug_tuple("Message");

        if let Ok(txt) = self.to_fix_string() {
            printer.field(&txt.replace(1 as char, "|"));
        }

        printer.finish()
    }
}

impl Default for Message {
    fn default() -> Self {
        unsafe { FixMessage_new() }
            .map(Self)
            .expect("Fail to allocate new Message")
    }
}

impl Drop for Message {
    fn drop(&mut self) {
        unsafe { FixMessage_delete(self.0) }
    }
}
