use quickfix::*;

struct MyApplication;

impl ApplicationCallback for MyApplication {}

fn build_settings(connection_type: &str) -> Result<SessionSettings, QuickFixError> {
    let mut global_params = Dictionary::default();
    global_params.set("ConnectionType", connection_type.to_string())?;
    global_params.set("FileStorePath", "store".to_string())?;

    let mut session1_params = Dictionary::default();
    session1_params.set("StartTime", "12:30:00".to_string())?;
    session1_params.set("EndTime", "23:30:00".to_string())?;
    session1_params.set("HeartBtInt", 20)?;
    session1_params.set("SocketAcceptPort", 4000)?;
    session1_params.set(
        "DataDictionary",
        "../quickfix-ffi/libquickfix/spec/FIX41.xml".to_string(),
    )?;

    let mut settings = SessionSettings::new();
    settings.set(None, global_params)?;
    settings.set(
        Some(SessionId::try_new("FIX.4.4", "ME", "THEIR", "")?),
        session1_params,
    )?;

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
