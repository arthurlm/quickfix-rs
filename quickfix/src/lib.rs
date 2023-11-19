#![warn(missing_docs)]

/*! This project is an **unofficial** binding between [quickfix library](https://github.com/quickfix/quickfix) and rust project. */

mod application;
mod data_dictionary;
mod error;
mod file_store_factory;
mod group;
mod header;
mod log_factory;
mod message;
mod session;
mod session_id;
mod session_settings;
mod socket_acceptor;
mod socket_initiator;
mod trailer;

mod utils;

pub use application::{Application, ApplicationCallback};
pub use data_dictionary::DataDictionary;
pub use error::QuickFixError;
pub use file_store_factory::FileStoreFactory;
pub use group::Group;
pub use header::Header;
pub use log_factory::{LogCallback, LogFactory, StdLogger};
pub use message::Message;
pub use session::send_to_target;
pub use session_id::SessionId;
pub use session_settings::SessionSettings;
pub use socket_acceptor::SocketAcceptor;
pub use socket_initiator::SocketInitiator;
pub use trailer::Trailer;

#[cfg(feature = "log")]
pub use log_factory::RustLogger;

/// Permit control of an underlying socket connection.
pub trait ConnectionHandler {
    /// Start handler.
    fn start(&mut self) -> Result<(), QuickFixError>;

    /// Block handler.
    fn block(&mut self) -> Result<(), QuickFixError>;

    /// Poll handler.
    fn poll(&mut self) -> Result<bool, QuickFixError>;

    /// Stop handler.
    fn stop(&mut self) -> Result<(), QuickFixError>;

    /// Check if handler has sent logging message or not.
    fn is_logged_on(&self) -> Result<bool, QuickFixError>;

    /// Check if handler is currently working or not.
    fn is_stopped(&self) -> Result<bool, QuickFixError>;
}

/// Stores and organizes a collection of Fields.
///
/// This is the basis for a message, header, and trailer.  This collection
/// class uses a sorter to keep the fields in a particular order.
pub trait FieldMap {
    /// Get field value from its tag number.
    fn get_field(&self, tag: i32) -> Option<String>;

    /// Set field value for a given tag number.
    fn set_field(&mut self, tag: i32, value: &str) -> Result<(), QuickFixError>;

    /// Remove a field from  collection.
    fn remove_field(&mut self, tag: i32) -> Result<(), QuickFixError>;
}
