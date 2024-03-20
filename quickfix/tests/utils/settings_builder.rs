use quickfix::{dictionary_item::*, *};

pub enum ServerType {
    Receiver,
    Sender,
}

impl ServerType {
    pub fn connection_type(&self) -> ConnectionType {
        match self {
            ServerType::Receiver => ConnectionType::Acceptor,
            ServerType::Sender => ConnectionType::Initiator,
        }
    }

    pub fn session_id(&self) -> SessionId {
        match self {
            ServerType::Receiver => SessionId::try_new("FIX.4.4", "ME", "THEIR", ""),
            ServerType::Sender => SessionId::try_new("FIX.4.4", "THEIR", "ME", ""),
        }
        .expect("Fail to build session ID")
    }
}

pub fn build_settings(
    server_type: ServerType,
    port: u16,
) -> Result<SessionSettings, QuickFixError> {
    let mut settings = SessionSettings::new();

    settings.set(
        None,
        Dictionary::try_from_items(&[&server_type.connection_type(), &ReconnectInterval(60)])?,
    )?;

    settings.set(
        Some(&server_type.session_id()),
        Dictionary::try_from_items(&[
            &StartTime("00:00:00"),
            &EndTime("23:59:59"),
            &HeartBtInt(20),
            &DataDictionary("../quickfix-ffi/libquickfix/spec/FIX44.xml"),
            &SocketAcceptPort(port),
            &SocketConnectPort(port),
            &SocketConnectHost("127.0.0.1"),
        ])?,
    )?;

    Ok(settings)
}
