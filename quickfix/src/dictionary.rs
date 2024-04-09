use std::{ffi::CString, fmt};

use quickfix_ffi::{
    FixDictionary_delete, FixDictionary_getBool, FixDictionary_getDay, FixDictionary_getDouble,
    FixDictionary_getInt, FixDictionary_getStringLen, FixDictionary_hasKey, FixDictionary_new,
    FixDictionary_readString, FixDictionary_setBool, FixDictionary_setDay, FixDictionary_setDouble,
    FixDictionary_setInt, FixDictionary_setString, FixDictionary_t,
};

use crate::{
    utils::{ffi_code_to_bool, ffi_code_to_result},
    DayOfWeek, ForeignPropertyGetter, ForeignPropertySetter, QuickFixError,
};

/// For storage and retrieval of key/value pairs.
pub struct Dictionary(pub(crate) FixDictionary_t);

impl Dictionary {
    /// Create a new empty struct.
    pub fn new() -> Self {
        Self::default()
    }

    /// Try to create new empty struct with a given name.
    pub fn with_name(name: &str) -> Result<Self, QuickFixError> {
        let c_name = CString::new(name)?;
        unsafe { FixDictionary_new(c_name.as_ptr()) }
            .map(Self)
            .ok_or_else(QuickFixError::from_last_error)
    }

    /// Check if dictionary contains key.
    pub fn contains(&self, key: &str) -> Result<bool, QuickFixError> {
        let c_key = CString::new(key)?;
        ffi_code_to_bool(unsafe { FixDictionary_hasKey(self.0, c_key.as_ptr()) })
    }

    /// Read value from dictionary for a given key.
    pub fn get<T>(&self, key: &str) -> Result<T, QuickFixError>
    where
        Self: ForeignPropertyGetter<T>,
    {
        let c_key = CString::new(key)?;
        self.ffi_get(c_key)
    }

    /// Write value into dictionary for a given key.
    pub fn set<T>(&mut self, key: &str, value: T) -> Result<(), QuickFixError>
    where
        Self: ForeignPropertySetter<T>,
    {
        let c_key = CString::new(key)?;
        self.ffi_set(c_key, value)
    }
}

impl ForeignPropertyGetter<String> for Dictionary {
    fn ffi_get(&self, key: CString) -> Result<String, QuickFixError> {
        unsafe {
            // Prepare output buffer
            let buffer_len = FixDictionary_getStringLen(self.0, key.as_ptr())
                .try_into()
                .map_err(|_err| QuickFixError::from_last_error())?;

            // Allocate buffer on rust side
            let mut buffer = vec![0_u8; buffer_len as usize];
            assert_eq!(buffer.len(), buffer_len as usize);

            // Read text
            ffi_code_to_result(FixDictionary_readString(
                self.0,
                key.as_ptr(),
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
}

impl ForeignPropertySetter<String> for Dictionary {
    fn ffi_set(&mut self, key: CString, value: String) -> Result<(), QuickFixError> {
        self.ffi_set(key, value.as_str())
    }
}

impl ForeignPropertySetter<&str> for Dictionary {
    fn ffi_set(&mut self, key: CString, value: &str) -> Result<(), QuickFixError> {
        let c_value = CString::new(value)?;
        ffi_code_to_result(unsafe {
            FixDictionary_setString(self.0, key.as_ptr(), c_value.as_ptr())
        })
    }
}

impl ForeignPropertyGetter<i32> for Dictionary {
    fn ffi_get(&self, key: CString) -> Result<i32, QuickFixError> {
        Ok(unsafe { FixDictionary_getInt(self.0, key.as_ptr()) })
    }
}

impl ForeignPropertySetter<i32> for Dictionary {
    fn ffi_set(&mut self, key: CString, value: i32) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixDictionary_setInt(self.0, key.as_ptr(), value) })
    }
}

impl ForeignPropertyGetter<f64> for Dictionary {
    fn ffi_get(&self, key: CString) -> Result<f64, QuickFixError> {
        Ok(unsafe { FixDictionary_getDouble(self.0, key.as_ptr()) })
    }
}

impl ForeignPropertySetter<f64> for Dictionary {
    fn ffi_set(&mut self, key: CString, value: f64) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixDictionary_setDouble(self.0, key.as_ptr(), value) })
    }
}

impl ForeignPropertyGetter<bool> for Dictionary {
    fn ffi_get(&self, key: CString) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixDictionary_getBool(self.0, key.as_ptr()) })
    }
}

impl ForeignPropertySetter<bool> for Dictionary {
    fn ffi_set(&mut self, key: CString, value: bool) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixDictionary_setBool(self.0, key.as_ptr(), value as i8) })
    }
}

impl ForeignPropertyGetter<DayOfWeek> for Dictionary {
    fn ffi_get(&self, key: CString) -> Result<DayOfWeek, QuickFixError> {
        DayOfWeek::try_from(unsafe { FixDictionary_getDay(self.0, key.as_ptr()) })
    }
}

impl ForeignPropertySetter<DayOfWeek> for Dictionary {
    fn ffi_set(&mut self, key: CString, value: DayOfWeek) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixDictionary_setDay(self.0, key.as_ptr(), value as i32) })
    }
}

impl fmt::Debug for Dictionary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Dictionary").finish()
    }
}

impl Default for Dictionary {
    fn default() -> Self {
        Self::with_name("").expect("Fail to allocate Dictionary")
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        unsafe { FixDictionary_delete(self.0) }
    }
}
