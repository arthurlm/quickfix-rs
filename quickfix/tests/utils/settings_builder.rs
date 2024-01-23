use quickfix::*;

pub enum ServerType {
    Receiver,
    Sender,
}

impl ServerType {
    pub fn name(&self) -> &'static str {
        match self {
            ServerType::Receiver => "acceptor",
            ServerType::Sender => "initiator",
        }
    }

    pub fn session_id(&self) -> SessionId {
        match self {
            ServerType::Receiver => SessionId::try_new("FIX.4.4", "ME", "THEIR", ""),
            ServerType::Sender => SessionId::try_new("FIX.4.4", "THEIR", "ME", ""),
        }
        .expect("Fail to build session ID")
    }

    pub fn fill_settings(&self, params: &mut Dictionary, port: u16) -> Result<(), QuickFixError> {
        match self {
            ServerType::Receiver => {
                params.set("SocketAcceptPort", port as i32)?;
            }
            ServerType::Sender => {
                params.set("SocketConnectPort", port as i32)?;
                params.set("SocketConnectHost", "127.0.0.1")?;
            }
        }
        Ok(())
    }
}

pub fn build_settings(
    server_type: ServerType,
    port: u16,
) -> Result<SessionSettings, QuickFixError> {
    let mut settings = SessionSettings::new();

    settings.set(None, {
        let mut params = Dictionary::new();
        params.set("ConnectionType", server_type.name())?;
        params.set("ReconnectInterval", 60)?;
        params
    })?;

    settings.set(Some(&server_type.session_id()), {
        let mut params = Dictionary::new();
        params.set("StartTime", "00:00:00")?;
        params.set("EndTime", "23:59:59")?;
        params.set("HeartBtInt", 20)?;
        params.set(
            "DataDictionary",
            "../quickfix-ffi/libquickfix/spec/FIX44.xml",
        )?;
        server_type.fill_settings(&mut params, port)?;
        params
    })?;

    Ok(settings)
}
