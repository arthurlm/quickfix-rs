#![allow(missing_debug_implementations)]

use crate::{Dictionary, QuickFixError};

/// Trait that represent quickfix configuration parameter.
///
/// Object that implement this trait can be apply to a dictionary.
///
/// # Dev note
///
/// Parameter key is constant, but value type can be different from one object to the other.
/// For example, `ConnectionType` is string, `SocketConnectPort` is integer, etc.
///
/// We cannot use generic in trait. So we need to "apply" inner value to the dictionary instead of
/// returning a generic value that the dictionary may use.
pub trait DictionaryItem {
    /// Apply inner value to dictionary.
    fn apply_param(&self, dict: &mut Dictionary) -> Result<(), QuickFixError>;
}

/// Session connection type.
pub enum ConnectionType {
    /// Session is acceptor.
    Acceptor,

    /// Session is initiator.
    Initiator,
}

/// Acceptor port.
pub struct SocketAcceptPort(pub u32);

/// Initiator port to connect to.
pub struct SocketConnectPort(pub u32);

/// Initiator host to connect to.
pub struct SocketConnectHost<'a>(pub &'a str);

/// Time in second to wait before reconnect.
pub struct ReconnectInterval(pub u16);

/// FIX heartbeat time in seconds.
pub struct HeartBtInt(pub u16);

/// Session start time.
pub struct StartTime<'a>(pub &'a str);

/// Session end time.
pub struct EndTime<'a>(pub &'a str);

/// Application version ID.
pub struct DefaultApplVerID<'a>(pub &'a str);

/// Data dictionary XML spec path.
pub struct DataDictionary<'a>(pub &'a str);

/// Transport data dictionary XML spec path.
pub struct TransportDataDictionary<'a>(pub &'a str);

/// Message store path.
pub struct FileStorePath<'a>(pub &'a str);

impl DictionaryItem for ConnectionType {
    fn apply_param(&self, dict: &mut Dictionary) -> Result<(), QuickFixError> {
        dict.set(
            "ConnectionType",
            match self {
                Self::Acceptor => "acceptor",
                Self::Initiator => "initiator",
            },
        )
    }
}

macro_rules! impl_dictionary_item {
    ($item:tt) => {
        impl DictionaryItem for $item<'_> {
            fn apply_param(&self, dict: &mut Dictionary) -> Result<(), QuickFixError> {
                dict.set(stringify!($item), self.0.to_string())
            }
        }
    };

    ($item:ty as $target_type:ty) => {
        impl DictionaryItem for $item {
            fn apply_param(&self, dict: &mut Dictionary) -> Result<(), QuickFixError> {
                dict.set(stringify!($item), self.0 as $target_type)
            }
        }
    };
}

impl_dictionary_item!(SocketAcceptPort as i32);
impl_dictionary_item!(SocketConnectPort as i32);
impl_dictionary_item!(SocketConnectHost);
impl_dictionary_item!(ReconnectInterval as i32);
impl_dictionary_item!(HeartBtInt as i32);
impl_dictionary_item!(StartTime);
impl_dictionary_item!(EndTime);
impl_dictionary_item!(DefaultApplVerID);
impl_dictionary_item!(DataDictionary);
impl_dictionary_item!(TransportDataDictionary);
impl_dictionary_item!(FileStorePath);

impl Dictionary {
    /// Build dictionary parameters from multiple items.
    pub fn try_from_items(items: &[&dyn DictionaryItem]) -> Result<Self, QuickFixError> {
        let mut output = Self::new();
        for item in items {
            item.apply_param(&mut output)?;
        }
        Ok(output)
    }
}
