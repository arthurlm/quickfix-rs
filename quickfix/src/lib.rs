#![warn(missing_docs, missing_debug_implementations)]

/*! This project is an **unofficial** binding between quickfix library and Rust projects.

![CI workflow](https://github.com/arthurlm/quickfix-rs/actions/workflows/ci.yml/badge.svg)
![MSRV](https://img.shields.io/badge/MSRV-1.70.0-blue)
[![codecov](https://codecov.io/gh/arthurlm/quickfix-rs/graph/badge.svg?token=WVEWW996GO)](https://codecov.io/gh/arthurlm/quickfix-rs)
[![dependency status](https://deps.rs/repo/github/arthurlm/quickfix-rs/status.svg)](https://deps.rs/repo/github/arthurlm/quickfix-rs)

## Features

- Provide basic and safe API wrapper above [quickfix](https://github.com/quickfix/quickfix) library.
- Run on any hardware and operating system supported by Rust Tier 1 (Windows 7+, MacOS 10.12+ & Linux).
- Only include and compile what you need since project is split into minimal crates.
- Message decoding / encoding including run-time validation.
- Supports FIX versions 4x (version 5x can be build locally from XML spec file).
- Spec driven run-time message validation.
- Spec driven code generation of type-safe FIX messages, fields, and repeating groups.
- Session state storage options: SQL, File, In Memory.
- Logging options: stdout, stderr, [log](https://crates.io/crates/log) or any other crate if you implement your own trait.

## Project status

Project is working and most of the source library features are available.

It is ready for production and real application use cases.

**API MAY CHANGE IN FUTURE VERSION**\
Crate is still in the [reviewing process](https://github.com/quickfix/quickfix/issues/533).
Feel free to participate and share your point of view on this github issue.

## Build requirements

Following package must be install to build the library:

- `cmake`
- a C++ compiler (with C++17 support)
- `rustup` / `rustc` / `cargo` (obviously 😉)
- `rustfmt` for auto generated messages from spec.

## Example usage

Here some minimal code sample to getting started:

```rust
# use quickfix::*;
# use quickfix::dictionary_item::*;
// Configure FIX engine.
let mut settings = SessionSettings::new();

settings.set(
    None,
    Dictionary::try_from_items(&[
        &ConnectionType::Acceptor,
    ])?
)?;

settings.set(
    Some(&SessionId::try_new("FIX.4.4", "ME", "THEIR", "")?),
    Dictionary::try_from_items(&[
        &StartTime("12:30:00"),
        &EndTime("23:30:00"),
        &SocketAcceptPort(4000),
        &DataDictionary("../quickfix-ffi/libquickfix/spec/FIX41.xml"),
    ])?
)?;

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

let mut acceptor = Acceptor::try_new(&settings, &app, &store_factory, &log_factory)?;

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

mod acceptor;
mod application;
mod data_dictionary;
mod days;
mod dictionary;
/// Common dictionary configuration parameters.
pub mod dictionary_item;
mod error;
mod group;
mod header;
mod initiator;
mod log_factory;
mod message;
mod message_store_factory;
mod session;
mod session_id;
mod session_settings;
mod trailer;

mod utils;

use std::ffi::{CString, NulError};

pub use acceptor::Acceptor;
pub use application::{
    Application, ApplicationCallback, MsgFromAdminError, MsgFromAppError, MsgToAppError,
};
pub use data_dictionary::DataDictionary;
pub use days::DayOfWeek;
pub use dictionary::Dictionary;
pub use error::QuickFixError;
pub use group::Group;
pub use header::Header;
pub use initiator::Initiator;
pub use log_factory::{LogCallback, LogFactory, NullLogger, StdLogger};
pub use message::Message;
pub use message_store_factory::{
    FfiMessageStoreFactory, FileMessageStoreFactory, MemoryMessageStoreFactory,
    NullMessageStoreFactory,
};
pub use session::{send_to_target, Session};
pub use session_id::SessionId;
pub use session_settings::SessionSettings;
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

/// Define a container of session
///
/// May be a socket acceptor / initiator.
pub trait SessionContainer {
    /// Borrow mutable session to the container.
    ///
    /// Session is lookup using its ID.
    fn session(&self, session_id: SessionId) -> Result<Session<'_>, QuickFixError>;
}

/// Convert object to FIX value.
///
/// This trait is like `std::fmt::Display` but it has a different meaning.
/// Using `AsRef<str>` feel also like mixing different thing together that does not match.
///
/// This is why it exists.
pub trait IntoFixValue {
    /// Convert implementer to a printable null terminated FIX value.
    fn into_fix_value(self) -> Result<CString, NulError>;
}

macro_rules! impl_into_fix_value {
    ($t:ty) => {
        impl IntoFixValue for $t {
            fn into_fix_value(self) -> Result<CString, NulError> {
                CString::new(self.to_string())
            }
        }
    };
}

impl_into_fix_value!(u8);
impl_into_fix_value!(u16);
impl_into_fix_value!(u32);
impl_into_fix_value!(u64);
impl_into_fix_value!(u128);
impl_into_fix_value!(usize);
impl_into_fix_value!(i8);
impl_into_fix_value!(i16);
impl_into_fix_value!(i32);
impl_into_fix_value!(i64);
impl_into_fix_value!(i128);
impl_into_fix_value!(isize);
impl_into_fix_value!(f32);
impl_into_fix_value!(f64);

impl IntoFixValue for String {
    fn into_fix_value(self) -> Result<CString, NulError> {
        CString::new(self)
    }
}

impl IntoFixValue for &str {
    fn into_fix_value(self) -> Result<CString, NulError> {
        CString::new(self)
    }
}

impl IntoFixValue for bool {
    fn into_fix_value(self) -> Result<CString, NulError> {
        // Check reference here: https://www.onixs.biz/fix-dictionary/4.3/tagNum_575.html
        CString::new(if self { "Y" } else { "N" })
    }
}

/// Stores and organizes a collection of Fields.
///
/// This is the basis for a message, header, and trailer.  This collection
/// class uses a sorter to keep the fields in a particular order.
pub trait FieldMap {
    /// Get field value from its tag number.
    fn get_field(&self, tag: i32) -> Option<String>;

    /// Set field value for a given tag number.
    fn set_field<V: IntoFixValue>(&mut self, tag: i32, value: V) -> Result<(), QuickFixError>;

    /// Remove a field from  collection.
    fn remove_field(&mut self, tag: i32) -> Result<(), QuickFixError>;

    /// Add a new group to storage and update count.
    fn add_group(&mut self, group: &Group) -> Result<(), QuickFixError>;

    /// Clone group part for a given tag and group index.
    fn clone_group(&self, index: i32, tag: i32) -> Option<Group>;
}

/// Allow reading value (aka property) from a foreign (C++) object.
pub trait ForeignPropertyGetter<T> {
    /// Read foreign value from object.
    fn ffi_get(&self, key: CString) -> Result<T, QuickFixError>;
}

/// Allow writing value (aka property) into a foreign (C++) object.
pub trait ForeignPropertySetter<T> {
    /// Write foreign value into object.
    fn ffi_set(&mut self, key: CString, value: T) -> Result<(), QuickFixError>;
}
