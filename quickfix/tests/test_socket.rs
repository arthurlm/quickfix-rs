use quickfix::{dictionary_item::*, *};

struct MyApplication;

impl ApplicationCallback for MyApplication {}

fn build_settings(connection_type: ConnectionType) -> Result<SessionSettings, QuickFixError> {
    let mut settings = SessionSettings::new();

    settings.set(
        None,
        Dictionary::try_from_items(&[&connection_type, &FileStorePath("store")])?,
    )?;

    settings.set(
        Some(&SessionId::try_new("FIX.4.4", "ME", "THEIR", "")?),
        Dictionary::try_from_items(&[
            &StartTime("12:30:00"),
            &EndTime("23:30:00"),
            &HeartBtInt(20),
            &SocketAcceptPort(4000),
            &DataDictionary("../quickfix-ffi/libquickfix/spec/FIX41.xml"),
        ])?,
    )?;

    Ok(settings)
}

#[test]
fn test_handler() {
    {
        let settings = build_settings(ConnectionType::Acceptor).unwrap();
        let app = Application::try_new(&MyApplication).unwrap();
        let message_store = MemoryMessageStoreFactory::new();
        let logger = LogFactory::try_new(&StdLogger::Stdout).unwrap();

        check_connection_handler(
            Acceptor::try_new(&settings, &app, &message_store, &logger).unwrap(),
        );
    }

    {
        let settings = build_settings(ConnectionType::Initiator).unwrap();
        let app = Application::try_new(&MyApplication).unwrap();
        let message_store = MemoryMessageStoreFactory::new();
        let logger = LogFactory::try_new(&StdLogger::Stdout).unwrap();

        check_connection_handler(
            Initiator::try_new(&settings, &app, &message_store, &logger).unwrap(),
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
