use std::ffi::{CStr, CString};

use quickfix_ffi::{
    FixDictionary_delete, FixDictionary_getBool, FixDictionary_getDay, FixDictionary_getDouble,
    FixDictionary_getInt, FixDictionary_getStringLen, FixDictionary_new, FixDictionary_readString,
    FixDictionary_setBool, FixDictionary_setDay, FixDictionary_setDouble, FixDictionary_setInt,
    FixDictionary_setString, FixDictionary_t,
};

use crate::{
    utils::{ffi_code_to_bool, ffi_code_to_result},
    DayOfWeek, PropertyContainer, QuickFixError,
};

/// For storage and retrieval of key/value pairs.
#[derive(Debug)]
pub struct Dictionary(pub(crate) FixDictionary_t);

impl Dictionary {
    /// Try to create new empty struct with a given name.
    pub fn try_new(name: &str) -> Result<Self, QuickFixError> {
        let c_name = CString::new(name)?;
        unsafe { FixDictionary_new(c_name.as_ptr()) }
            .map(Self)
            .ok_or(QuickFixError::NullFunctionReturn)
    }

    /// Read value from dictionary for a given key.
    pub fn get<T>(&self, key: &str) -> Result<T, QuickFixError>
    where
        Self: PropertyContainer<T>,
    {
        let c_key = CString::new(key)?;
        self.ffi_get(c_key)
    }

    /// Write value into dictionary for a given key.
    pub fn set<T>(&mut self, key: &str, value: T) -> Result<(), QuickFixError>
    where
        Self: PropertyContainer<T>,
    {
        let c_key = CString::new(key)?;
        self.ffi_set(c_key, value)
    }
}

impl PropertyContainer<String> for Dictionary {
    fn ffi_get(&self, key: CString) -> Result<String, QuickFixError> {
        unsafe {
            // Prepare output buffer
            let mut buffer_len = FixDictionary_getStringLen(self.0, key.as_ptr());
            if buffer_len < 0 {
                return Err(QuickFixError::InvalidFunctionReturnCode(buffer_len as i8));
            }

            buffer_len += 1; // Add null end char

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
            let text = CStr::from_bytes_with_nul(&buffer)?.to_str()?.to_string();
            Ok(text)
        }
    }

    fn ffi_set(&mut self, key: CString, value: String) -> Result<(), QuickFixError> {
        let c_value = CString::new(value)?;
        ffi_code_to_result(unsafe {
            FixDictionary_setString(self.0, key.as_ptr(), c_value.as_ptr())
        })
    }
}

impl PropertyContainer<i32> for Dictionary {
    fn ffi_get(&self, key: CString) -> Result<i32, QuickFixError> {
        Ok(unsafe { FixDictionary_getInt(self.0, key.as_ptr()) })
    }

    fn ffi_set(&mut self, key: CString, value: i32) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixDictionary_setInt(self.0, key.as_ptr(), value) })
    }
}

impl PropertyContainer<f64> for Dictionary {
    fn ffi_get(&self, key: CString) -> Result<f64, QuickFixError> {
        Ok(unsafe { FixDictionary_getDouble(self.0, key.as_ptr()) })
    }

    fn ffi_set(&mut self, key: CString, value: f64) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixDictionary_setDouble(self.0, key.as_ptr(), value) })
    }
}

impl PropertyContainer<bool> for Dictionary {
    fn ffi_get(&self, key: CString) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixDictionary_getBool(self.0, key.as_ptr()) })
    }

    fn ffi_set(&mut self, key: CString, value: bool) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixDictionary_setBool(self.0, key.as_ptr(), value as i8) })
    }
}

impl PropertyContainer<DayOfWeek> for Dictionary {
    fn ffi_get(&self, key: CString) -> Result<DayOfWeek, QuickFixError> {
        DayOfWeek::try_from(unsafe { FixDictionary_getDay(self.0, key.as_ptr()) })
    }

    fn ffi_set(&mut self, key: CString, value: DayOfWeek) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixDictionary_setDay(self.0, key.as_ptr(), value as i32) })
    }
}

impl Drop for Dictionary {
    fn drop(&mut self) {
        unsafe { FixDictionary_delete(self.0) }
    }
}