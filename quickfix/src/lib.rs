#![warn(missing_docs, missing_debug_implementations)]

/*! This project is an **unofficial** binding between quickfix library and Rust projects.

![CI workflow](https://github.com/arthurlm/quickfix-rs/actions/workflows/ci.yml/badge.svg)
![MSRV](https://img.shields.io/badge/MSRV-1.70.0-blue)
[![codecov](https://codecov.io/gh/arthurlm/quickfix-rs/graph/badge.svg?token=WVEWW996GO)](https://codecov.io/gh/arthurlm/quickfix-rs)
[![dependency status](https://deps.rs/repo/github/arthurlm/quickfix-rs/status.svg)](https://deps.rs/repo/github/arthurlm/quickfix-rs)

## Features

- Provide basic and safe API wrapper above [quickfix](https://github.com/quickfix/quickfix) library.
- Run on any hardware and operating system supported by Rust Tier 1 (Windows 7+, MacOS 10.12+ & Linux).
- Message decoding / encoding including run-time validation.
- Session state storage options: SQL, File, In Memory.
- Logging options: stdout, stderr, [log](https://crates.io/crates/log) or any other crate if you implement your own trait.

## Build requirements

Following package must be install to build the library:

- `cmake`
- a C++ compiler (with C++17 support)
- `rustup` / `rustc` / `cargo` (obviously 😉)

## Example usage

Here some minimal code sample to getting started:

```rust
# use quickfix::*;
// Configure FIX engine.
let mut settings = SessionSettings::new();

settings.set(None, {
    let mut params = Dictionary::new();
    params.set("ConnectionType", "acceptor".to_string())?;
    params
})?;

settings.set(Some(SessionId::try_new("FIX.4.4", "ME", "THEIR", "")?), {
    let mut params = Dictionary::new();
    params.set("StartTime", "12:30:00".to_string())?;
    params.set("EndTime", "23:30:00".to_string())?;
    params.set("SocketAcceptPort", 4000)?;
    params.set("DataDictionary", "../quickfix-ffi/libquickfix/spec/FIX41.xml".to_string())?;
    params
})?;

// Configure FIX callbacks.
pub struct MyApplication;

impl ApplicationCallback for MyApplication {
    // Implement whatever callback you need

    fn on_create(&self, _session: &SessionId) {
        // Do whatever you want here 😁
    }
}

// Create FIX objects.
let store_factory = MemoryMessageStoreFactory::new();
let log_factory = LogFactory::try_new(&StdLogger::Stdout)?;
let app = Application::try_new(&MyApplication)?;

let mut acceptor = SocketAcceptor::try_new(&settings, &app, &store_factory, &log_factory)?;

// Start session.
acceptor.start()?;

loop {
    // Do whatever you want here ...
    # break;
}

// End application
acceptor.stop()?;
# Ok::<(), QuickFixError>(())
```

*/

mod application;
mod data_dictionary;
mod days;
mod dictionary;
mod error;
mod group;
mod header;
mod log_factory;
mod message;
mod message_store_factory;
mod session;
mod session_id;
mod session_settings;
mod socket_acceptor;
mod socket_initiator;
mod trailer;

mod utils;

use std::ffi::CString;

pub use application::{Application, ApplicationCallback};
pub use data_dictionary::DataDictionary;
pub use days::DayOfWeek;
pub use dictionary::Dictionary;
pub use error::QuickFixError;
pub use group::Group;
pub use header::Header;
pub use log_factory::{LogCallback, LogFactory, NullLogger, StdLogger};
pub use message::Message;
pub use message_store_factory::{
    FfiMessageStoreFactory, FileMessageStoreFactory, MemoryMessageStoreFactory,
};
pub use session::send_to_target;
pub use session_id::SessionId;
pub use session_settings::SessionSettings;
pub use socket_acceptor::SocketAcceptor;
pub use socket_initiator::SocketInitiator;
pub use trailer::Trailer;

#[cfg(feature = "log")]
pub use log_factory::RustLogger;
#[cfg(feature = "build-with-mysql")]
pub use message_store_factory::mysql::MySqlMessageStoreFactory;
#[cfg(feature = "build-with-postgres")]
pub use message_store_factory::postgres::PostgresMessageStoreFactory;

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
    fn set_field<V: AsRef<str>>(&mut self, tag: i32, value: V) -> Result<(), QuickFixError>;

    /// Remove a field from  collection.
    fn remove_field(&mut self, tag: i32) -> Result<(), QuickFixError>;
}

/// Allow reading / writing value (aka property) from an object.
pub trait PropertyContainer<T> {
    /// Read value from object.
    fn ffi_get(&self, key: CString) -> Result<T, QuickFixError>;

    /// Write value into object.
    fn ffi_set(&mut self, key: CString, value: T) -> Result<(), QuickFixError>;
}
