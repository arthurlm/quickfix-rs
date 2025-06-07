use std::collections::HashMap;

use crate::errors::{NativeError, Result};

/// For storage and retrieval of key/value pairs.
#[derive(Default, Debug, Clone)]
pub struct Dictionary {
    pub name: String,
    pub data: HashMap<String, String>,
}

impl<'a> IntoIterator for &'a Dictionary {
    type Item = (&'a String, &'a String);
    type IntoIter = std::collections::hash_map::Iter<'a, String, String>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl Dictionary {
    pub fn new(name: String) -> Self {
        Dictionary {
            name,
            data: HashMap::new(),
        }
    }

    /// Get the name of the dictionary.
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Return the number of key/value pairs.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Check if the dictionary contains a value for key.
    pub fn has(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }

    /// Get a value as a string.
    pub fn get_string(&self, key: &str, capitalize: bool) -> Result<String> {
        let normalized_key = key.trim().to_uppercase();
        let value = self
            .data
            .get(&normalized_key)
            .ok_or_else(|| NativeError::ConfigError(format!("{} not defined", key)))?;

        if capitalize {
            Ok(value.to_uppercase())
        } else {
            Ok(value.clone())
        }
    }

    /// Get a value as an i32.
    pub fn get_int(&self, key: &str) -> Result<i32> {
        let value = self.get_string(key, false)?;
        value.parse::<i32>().map_err(|_| {
            NativeError::FieldConvertError(format!("Illegal value {} for {} ", value, key))
        })
    }
    /// Get a value as a f64
    pub fn get_double(&self, key: &str) -> Result<f64> {
        let value = self.get_string(key, false)?;
        value.parse::<f64>().map_err(|_| {
            NativeError::FieldConvertError(format!("Illegal value {} for {} ", value, key))
        })
    }
    /// Get a value as a bool
    pub fn get_bool(&self, key: &str) -> Result<bool> {
        let value = self.get_string(key, false)?;
        match Self::convert_bool(&value) {
            Some(true) => Ok(true),
            Some(false) => Err(NativeError::FieldConvertError(format!(
                "Returned value is set to 'NO'"
            ))),
            None => Err(NativeError::FieldConvertError(format!(
                "Illegal value {} for {}",
                value, key
            ))),
        }
    }

    /// Get a value as a day of week
    pub fn get_day(&self, key: &str) -> Result<i32> {
        let value = self.get_string(key, false)?;
        if value.len() < 2 {
            return Err(NativeError::FieldConvertError(format!(
                "Illegal value {} for {}",
                value, key
            )));
        }

        let first_two = value[..2].to_lowercase();
        match first_two.as_str() {
            "su" => Ok(1),
            "mo" => Ok(2),
            "tu" => Ok(3),
            "we" => Ok(4),
            "th" => Ok(5),
            "fr" => Ok(6),
            "sa" => Ok(7),
            _ => Err(NativeError::FieldConvertError(format!(
                "Illegal value {} for {}",
                value, key
            ))),
        }
    }

    /// Set a value from a string.
    pub fn set_string(&mut self, key: &str, value: &str) {
        self.data
            .insert(key.trim().to_uppercase(), value.trim().to_uppercase());
    }

    /// Set a value from a string.
    pub fn set_int(&mut self, key: &str, value: i32) {
        self.set_string(key, &value.to_string());
    }

    ///Set a value from a bool
    pub fn set_bool(&mut self, key: &str, value: bool) {
        let val = if value { "Y" } else { "N" };
        self.set_string(key, val);
    }
    /// set a value from a f64
    pub fn set_double(&mut self, key: &str, value: f64) {
        self.set_string(key, &value.to_string());
    }

    /// Set a value from a day
    pub fn set_day(&mut self, key: &str, value: i32) {
        let val = match value {
            1 => "SU",
            2 => "MO",
            3 => "TU",
            4 => "WE",
            5 => "TH",
            6 => "FR",
            7 => "SA",
            _ => return,
        };
        self.set_string(key, val);
    }

    pub fn merge(&mut self, dict: &Dictionary) -> Result<()> {
        for (k, v) in dict {
            self.data.entry(k.clone()).or_insert_with(|| v.clone());
        }

        Ok(())
    }

    /// convert a value to its corresponding bool
    pub fn convert_bool(value: &str) -> Option<bool> {
        match value.to_uppercase().as_str() {
            "Y" => Some(true),
            "N" => Some(false),
            _ => None,
        }
    }
}
