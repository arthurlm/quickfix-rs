#![allow(missing_debug_implementations)]

use crate::{DayOfWeek, Dictionary, QuickFixError};

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

macro_rules! impl_dictionary_item {
    ($item:tt) => {
        impl DictionaryItem for $item {
            fn apply_param(&self, dict: &mut Dictionary) -> Result<(), QuickFixError> {
                dict.set(stringify!($item), self.0)
            }
        }
    };

    ($item:tt as String) => {
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

/// Session connection type.
pub enum ConnectionType {
    /// Session is acceptor.
    Acceptor,

    /// Session is initiator.
    Initiator,
}

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

/// Acceptor port.
pub struct SocketAcceptPort(pub u16);
impl_dictionary_item!(SocketAcceptPort as i32);

/// Initiator port to connect to.
pub struct SocketConnectPort(pub u16);
impl_dictionary_item!(SocketConnectPort as i32);

/// Initiator host to connect to.
pub struct SocketConnectHost<'a>(pub &'a str);
impl_dictionary_item!(SocketConnectHost as String);

/// Initiator port to connect from.
pub struct SocketConnectSourcePort(pub u16);
impl_dictionary_item!(SocketConnectSourcePort as i32);

/// Initiator host to connect from.
pub struct SocketConnectSourceHost<'a>(pub &'a str);
impl_dictionary_item!(SocketConnectSourceHost as String);

/// Set SO_REUSEADDR flag when creating socket.
pub struct SocketReuseAddress(pub bool);
impl_dictionary_item!(SocketReuseAddress);

/// Set TCP_NODELAY flag when creating socket.
pub struct SocketNodelay(pub bool);
impl_dictionary_item!(SocketNodelay);

/// Configure SO_SNDBUF socket parameter.
pub struct SocketSendBufferSize(pub u16);
impl_dictionary_item!(SocketSendBufferSize as i32);

/// Configure SO_RCVBUF socket parameter.
pub struct SocketReceiveBufferSize(pub u16);
impl_dictionary_item!(SocketReceiveBufferSize as i32);

/// Time in second to wait before reconnect.
pub struct ReconnectInterval(pub u16);
impl_dictionary_item!(ReconnectInterval as i32);

/// FIX heartbeat time in seconds.
pub struct HeartBtInt(pub u16);
impl_dictionary_item!(HeartBtInt as i32);

/// Send redundant resend requests.
pub struct SendRedundantResendRequests(pub bool);
impl_dictionary_item!(SendRedundantResendRequests);

/// Send next expected message sequence number.
pub struct SendNextExpectedMsgSeqNum(pub bool);
impl_dictionary_item!(SendNextExpectedMsgSeqNum);

/// Use local time.
pub struct UseLocalTime(pub bool);
impl_dictionary_item!(UseLocalTime);

/// Session start time.
pub struct StartTime<'a>(pub &'a str);
impl_dictionary_item!(StartTime as String);

/// Session start day.
pub struct StartDay(pub DayOfWeek);
impl_dictionary_item!(StartDay);

/// Session end time.
pub struct EndTime<'a>(pub &'a str);
impl_dictionary_item!(EndTime as String);

/// Session end day.
pub struct EndDay(pub DayOfWeek);
impl_dictionary_item!(EndDay);

/// Logon time.
pub struct LogonTime<'a>(pub &'a str);
impl_dictionary_item!(LogonTime as String);

/// Logon day.
pub struct LogonDay(pub DayOfWeek);
impl_dictionary_item!(LogonDay);

/// Logon timeout.
pub struct LogonTimeout(pub u16);
impl_dictionary_item!(LogonTimeout as i32);

/// Logout time.
pub struct LogoutTime<'a>(pub &'a str);
impl_dictionary_item!(LogoutTime as String);

/// Logout day.
pub struct LogoutDay(pub DayOfWeek);
impl_dictionary_item!(LogoutDay);

/// Logout timeout.
pub struct LogoutTimeout(pub u16);
impl_dictionary_item!(LogoutTimeout as i32);

/// Application version ID.
pub struct DefaultApplVerID<'a>(pub &'a str);
impl_dictionary_item!(DefaultApplVerID as String);

/// Parse and use data dictionary.
pub struct UseDataDictionary(pub bool);
impl_dictionary_item!(UseDataDictionary);

/// Data dictionary XML spec path.
pub struct DataDictionary<'a>(pub &'a str);
impl_dictionary_item!(DataDictionary as String);

/// Transport data dictionary XML spec path.
pub struct TransportDataDictionary<'a>(pub &'a str);
impl_dictionary_item!(TransportDataDictionary as String);

/// Message store path.
pub struct FileStorePath<'a>(pub &'a str);
impl_dictionary_item!(FileStorePath as String);

/// Validate message comp ID.
pub struct CheckCompID(pub bool);
impl_dictionary_item!(CheckCompID);

/// Validate message latency.
pub struct CheckLatency(pub bool);
impl_dictionary_item!(CheckLatency);

/// Max allowed latency.
pub struct MaxLatency(pub i32);
impl_dictionary_item!(MaxLatency);

/// Validate message header length and checksum.
pub struct ValidateLengthAndChecksum(pub bool);
impl_dictionary_item!(ValidateLengthAndChecksum);

/// Validate message fields sent in wrong order.
pub struct ValidateFieldsOutOfOrder(pub bool);
impl_dictionary_item!(ValidateFieldsOutOfOrder);

/// Validate message fields inner values.
pub struct ValidateFieldsHaveValues(pub bool);
impl_dictionary_item!(ValidateFieldsHaveValues);

/// Validate user custom fields.
pub struct ValidateUserDefinedFields(pub bool);
impl_dictionary_item!(ValidateUserDefinedFields);

/// Allow unknown message fields.
pub struct AllowUnknownMsgFields(pub bool);
impl_dictionary_item!(AllowUnknownMsgFields);

/// Preserve message fields order.
pub struct PreserveMessageFieldsOrder(pub bool);
impl_dictionary_item!(PreserveMessageFieldsOrder);

/// Reset FIX session on logon.
pub struct ResetOnLogon(pub bool);
impl_dictionary_item!(ResetOnLogon);

/// Reset FIX session on logout.
pub struct ResetOnLogout(pub bool);
impl_dictionary_item!(ResetOnLogout);

/// Reset FIX session on disconnection.
pub struct ResetOnDisconnect(pub bool);
impl_dictionary_item!(ResetOnDisconnect);

/// Refresh FIX session on logon.
pub struct RefreshOnLogon(pub bool);
impl_dictionary_item!(RefreshOnLogon);

/// Built-in HTTP server main port.
pub struct HttpAcceptPort(pub u16);
impl_dictionary_item!(HttpAcceptPort as i32);

/// Persist FIX messages.
pub struct PersistMessages(pub bool);
impl_dictionary_item!(PersistMessages);

/// Reset sequence number as soon as session is initialized.
pub struct SendResetSeqNumFlag(pub bool);
impl_dictionary_item!(SendResetSeqNumFlag);

/// Path where SSL certificate file can be found.
pub struct ServerCertificateFile<'a>(pub &'a str);
impl_dictionary_item!(ServerCertificateFile as String);

/// Path where SSL certificate key file can be found.
pub struct ServerCertificateKeyFile<'a>(pub &'a str);
impl_dictionary_item!(ServerCertificateKeyFile as String);

/// Path where SSL certificate file can be found.
pub struct ClientCertificateFile<'a>(pub &'a str);
impl_dictionary_item!(ClientCertificateFile as String);

/// Path where SSL certificate key file can be found.
pub struct ClientCertificateKeyFile<'a>(pub &'a str);
impl_dictionary_item!(ClientCertificateKeyFile as String);

/// Enabled and active SSL protocol.
pub enum SSLProtocol {
    /// This is the Secure Sockets Layer (SSL) protocol, version 2.0. It is the
    /// original SSL protocol as designed by Netscape Corporation.
    SSLv2,
    /// This is the Secure Sockets Layer (SSL) protocol, version 3.0. It is the
    /// successor to SSLv2 and the currently (as of February 1999) de-facto
    /// standardized SSL protocol from Netscape Corporation. It's supported by
    /// almost all popular browsers.
    SSLv3,
    /// This is the Transport Layer Security (TLS) protocol, version 1.0.
    TLSv1,
    /// This is the Transport Layer Security (TLS) protocol, version 1.1.
    TLSv1_1,
    /// This is the Transport Layer Security (TLS) protocol, version 1.2.
    TLSv1_2,
    /// This is a shortcut for `+SSLv2 +SSLv3 +TLSv1 +TLSv1_1 +TLSv1_2' and a convenient way for
    /// enabling all protocols except one when used in combination with the minus
    /// sign on a protocol as the example above shows.
    All,
}

impl DictionaryItem for SSLProtocol {
    fn apply_param(&self, dict: &mut Dictionary) -> Result<(), QuickFixError> {
        dict.set(
            "SSLProtocol",
            match self {
                Self::SSLv2 => "SSLv2",
                Self::SSLv3 => "SSLv3",
                Self::TLSv1 => "TLSv1",
                Self::TLSv1_1 => "TLSv1_1",
                Self::TLSv1_2 => "TLSv1_2",
                Self::All => "all",
            },
        )
    }
}

impl DictionaryItem for (&'static str, &str) {
    fn apply_param(&self, dict: &mut Dictionary) -> Result<(), QuickFixError> {
        dict.set(self.0, self.1.to_string())
    }
}

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
