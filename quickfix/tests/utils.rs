use std::net::TcpListener;

pub fn find_available_port() -> u16 {
    (8000..9000)
        .find(|port| TcpListener::bind(("127.0.0.1", *port)).is_ok())
        .expect("No port available in test range")
}
