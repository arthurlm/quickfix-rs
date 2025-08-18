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
            ServerType::Receiver => SessionId::try_new("FIX.4.4", "RECEIVER", "SENDER", ""),
            ServerType::Sender => SessionId::try_new("FIX.4.4", "SENDER", "RECEIVER", ""),
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

pub fn build_ssl_settings(
    server_type: ServerType,
    port: u16,
) -> Result<SessionSettings, QuickFixError> {
    let mut settings = build_settings(server_type, port)?;

    settings.set(
        None,
        Dictionary::try_from_items(&[
            &ServerCertificateFile("../quickfix/tests/certs/serverCert.pem"),
            &ServerCertificateKeyFile("../quickfix/tests/certs/serverKey.pem"),
            &ClientCertificateFile("../quickfix/tests/certs/clientCert.pem"),
            &ClientCertificateKeyFile("../quickfix/tests/certs/clientKey.pem"),
            &SSLProtocol::TLSv1_2,
        ])?,
    )?;

    Ok(settings)
}
