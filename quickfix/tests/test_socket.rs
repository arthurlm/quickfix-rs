use quickfix::*;

struct MyApplication;

impl ApplicationCallback for MyApplication {}

fn build_settings(connection_type: &str) -> Result<SessionSettings, QuickFixError> {
    let mut settings = SessionSettings::new();

    settings.set(None, {
        let mut params = Dictionary::new();
        params.set("ConnectionType", connection_type)?;
        params.set("FileStorePath", "store")?;
        params
    })?;

    settings.set(Some(&SessionId::try_new("FIX.4.4", "ME", "THEIR", "")?), {
        let mut params = Dictionary::new();
        params.set("StartTime", "12:30:00")?;
        params.set("EndTime", "23:30:00")?;
        params.set("HeartBtInt", 20)?;
        params.set("SocketAcceptPort", 4000)?;
        params.set(
            "DataDictionary",
            "../quickfix-ffi/libquickfix/spec/FIX41.xml",
        )?;
        params
    })?;

    Ok(settings)
}

#[test]
fn test_handler() {
    {
        let settings = build_settings("acceptor").unwrap();
        let app = Application::try_new(&MyApplication).unwrap();
        let message_store = MemoryMessageStoreFactory::new();
        let logger = LogFactory::try_new(&StdLogger::Stdout).unwrap();

        check_connection_handler(
            SocketAcceptor::try_new(&settings, &app, &message_store, &logger).unwrap(),
        );
    }

    {
        let settings = build_settings("initiator").unwrap();
        let app = Application::try_new(&MyApplication).unwrap();
        let message_store = MemoryMessageStoreFactory::new();
        let logger = LogFactory::try_new(&StdLogger::Stdout).unwrap();

        check_connection_handler(
            SocketInitiator::try_new(&settings, &app, &message_store, &logger).unwrap(),
        );
    }
}

fn check_connection_handler<C: ConnectionHandler>(mut conn: C) {
    // Test initial state
    assert_eq!(conn.is_stopped(), Ok(true));

    // Test stop when stopped
    assert!(conn.stop().is_ok());
    assert!(conn.stop().is_ok());
    assert_eq!(conn.is_stopped(), Ok(true));

    // Test start when stopped
    assert!(conn.start().is_ok());
    assert_eq!(conn.is_stopped(), Ok(false));

    // Test start when started
    assert!(conn.start().is_err());

    // Test stop when started
    assert!(conn.stop().is_ok());
}
