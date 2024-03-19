use quickfix::{dictionary_item::*, *};

#[test]

fn test_build() {
    let dict = Dictionary::try_from_items(&[
        &ConnectionType::Acceptor,
        &SocketAcceptPort(42),
        &SocketConnectPort(56),
        &SocketConnectHost("10.8.0.3"),
        &ReconnectInterval(20),
        &HeartBtInt(30),
        &StartTime("00:00:05"),
        &EndTime("23:59:55"),
        &DefaultApplVerID("8"),
        &DataDictionary("foo/FIX50.xml"),
        &TransportDataDictionary("bar/FIXT11.xml"),
        &FileStorePath("my_store"),
    ])
    .unwrap();

    // Check unset value.
    assert_eq!(
        dict.get::<String>("whatever"),
        Err(QuickFixError::ConfigError(
            "Configuration failed: whatever not defined".to_string()
        ))
    );

    // Check set values.
    assert_eq!(
        dict.get::<String>("ConnectionType").as_deref(),
        Ok("acceptor")
    );
    assert_eq!(dict.get::<i32>("SocketAcceptPort"), Ok(42));
    assert_eq!(dict.get::<i32>("SocketConnectPort"), Ok(56));
    assert_eq!(
        dict.get::<String>("SocketConnectHost").as_deref(),
        Ok("10.8.0.3")
    );
    assert_eq!(dict.get::<i32>("ReconnectInterval"), Ok(20));
    assert_eq!(dict.get::<i32>("HeartBtInt"), Ok(30));
    assert_eq!(dict.get::<String>("StartTime").as_deref(), Ok("00:00:05"));
    assert_eq!(dict.get::<String>("EndTime").as_deref(), Ok("23:59:55"));
    assert_eq!(dict.get::<String>("DefaultApplVerID").as_deref(), Ok("8"));
    assert_eq!(
        dict.get::<String>("DataDictionary").as_deref(),
        Ok("foo/FIX50.xml")
    );
    assert_eq!(
        dict.get::<String>("TransportDataDictionary").as_deref(),
        Ok("bar/FIXT11.xml")
    );
    assert_eq!(
        dict.get::<String>("FileStorePath").as_deref(),
        Ok("my_store")
    );
}
