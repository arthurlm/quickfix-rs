use std::net::TcpListener;

mod msg_const;
mod recorder;
mod settings_builder;

use quickfix::ApplicationCallback;

pub use msg_const::*;
pub use recorder::*;
pub use settings_builder::*;

pub fn find_available_port() -> u16 {
    (8000..9000)
        .find(|port| TcpListener::bind(("127.0.0.1", *port)).is_ok())
        .expect("No port available in test range")
}

pub struct NullFixApplication;

impl ApplicationCallback for NullFixApplication {}
