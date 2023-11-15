mod application;
mod data_dictionary;
mod error;
mod file_log_factory;
mod file_store_factory;
mod group;
mod header;
mod message;
mod session_id;
mod session_settings;
mod socket_acceptor;
mod socket_initiator;
mod trailer;

mod utils;

pub use application::{Application, ApplicationCallback};
pub use data_dictionary::DataDictionary;
pub use error::QuickFixError;
pub use file_log_factory::FileLogFactory;
pub use file_store_factory::FileStoreFactory;
pub use group::Group;
pub use header::Header;
pub use message::Message;
pub use session_id::SessionId;
pub use session_settings::SessionSettings;
pub use socket_acceptor::SocketAcceptor;
pub use socket_initiator::SocketInitiator;
pub use trailer::Trailer;

pub trait ConnectionHandler {
    fn start(&mut self) -> Result<(), QuickFixError>;
    fn block(&mut self) -> Result<(), QuickFixError>;
    fn poll(&mut self) -> Result<bool, QuickFixError>;
    fn stop(&mut self) -> Result<(), QuickFixError>;
    fn is_logged_on(&self) -> Result<bool, QuickFixError>;
    fn is_stopped(&self) -> Result<bool, QuickFixError>;
}

pub trait FieldMap {
    fn get_field(&self, tag: i32) -> Option<String>;
    fn set_field(&mut self, tag: i32, value: &str) -> Result<(), QuickFixError>;
    fn remove_field(&mut self, tag: i32) -> Result<(), QuickFixError>;
}
