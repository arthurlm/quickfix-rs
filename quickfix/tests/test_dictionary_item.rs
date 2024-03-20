use quickfix::{dictionary_item::*, *};

#[test]

fn test_build() {
    let dict = Dictionary::try_from_items(&[
        &ConnectionType::Acceptor,
        &SocketAcceptPort(42),
        &SocketConnectPort(56),
        &SocketConnectHost("10.8.0.3"),
        &SocketConnectSourcePort(69),
        &SocketConnectSourceHost("10.8.0.4"),
        &SocketReuseAddress(true),
        &SocketNodelay(false),
        &SocketSendBufferSize(4096),
        &SocketReceiveBufferSize(8192),
        &ReconnectInterval(20),
        &HeartBtInt(30),
        &SendRedundantResendRequests(true),
        &SendNextExpectedMsgSeqNum(false),
        &UseLocalTime(true),
        &StartTime("00:00:05"),
        &StartDay(DayOfWeek::Monday),
        &EndTime("23:59:55"),
        &EndDay(DayOfWeek::Saturday),
        &LogonTime("12:00:00"),
        &LogonDay(DayOfWeek::Tuesday),
        &LogonTimeout(90),
        &LogoutTime("18:00:00"),
        &LogoutDay(DayOfWeek::Friday),
        &LogoutTimeout(15),
        &DefaultApplVerID("8"),
        &UseDataDictionary(true),
        &DataDictionary("foo/FIX50.xml"),
        &TransportDataDictionary("bar/FIXT11.xml"),
        &FileStorePath("my_store"),
        &CheckCompID(true),
        &CheckLatency(false),
        &MaxLatency(-20),
        &ValidateLengthAndChecksum(false),
        &ValidateFieldsOutOfOrder(false),
        &ValidateFieldsHaveValues(true),
        &ValidateUserDefinedFields(true),
        &AllowUnknownMsgFields(false),
        &PreserveMessageFieldsOrder(true),
        &ResetOnLogon(true),
        &ResetOnLogout(true),
        &ResetOnDisconnect(false),
        &RefreshOnLogon(true),
        &HttpAcceptPort(9090),
        &PersistMessages(false),
        &("foo", "bar"),
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
    assert_eq!(dict.get("SocketAcceptPort"), Ok(42));
    assert_eq!(dict.get("SocketConnectPort"), Ok(56));
    assert_eq!(
        dict.get::<String>("SocketConnectHost").as_deref(),
        Ok("10.8.0.3")
    );
    assert_eq!(dict.get("SocketConnectSourcePort"), Ok(69));
    assert_eq!(
        dict.get::<String>("SocketConnectSourceHost").as_deref(),
        Ok("10.8.0.4")
    );
    assert_eq!(dict.get("SocketReuseAddress"), Ok(true));
    assert_eq!(dict.get("SocketNodelay"), Ok(false));
    assert_eq!(dict.get("SocketSendBufferSize"), Ok(4096));
    assert_eq!(dict.get("SocketReceiveBufferSize"), Ok(8192));

    assert_eq!(dict.get("ReconnectInterval"), Ok(20));
    assert_eq!(dict.get("HeartBtInt"), Ok(30));

    assert_eq!(dict.get("SendRedundantResendRequests"), Ok(true));
    assert_eq!(dict.get("SendNextExpectedMsgSeqNum"), Ok(false));

    assert_eq!(dict.get("UseLocalTime"), Ok(true));
    assert_eq!(dict.get::<String>("StartTime").as_deref(), Ok("00:00:05"));
    assert_eq!(dict.get::<String>("StartDay").as_deref(), Ok("MO"));
    assert_eq!(dict.get::<String>("EndTime").as_deref(), Ok("23:59:55"));
    assert_eq!(dict.get::<String>("EndDay").as_deref(), Ok("SA"));

    assert_eq!(dict.get::<String>("LogonTime").as_deref(), Ok("12:00:00"));
    assert_eq!(dict.get::<String>("LogonDay").as_deref(), Ok("TU"));
    assert_eq!(dict.get("LogonTimeout"), Ok(90));
    assert_eq!(dict.get::<String>("LogoutTime").as_deref(), Ok("18:00:00"));
    assert_eq!(dict.get::<String>("LogoutDay").as_deref(), Ok("FR"));
    assert_eq!(dict.get("LogoutTimeout"), Ok(15));

    assert_eq!(dict.get::<String>("DefaultApplVerID").as_deref(), Ok("8"));
    assert_eq!(dict.get("UseDataDictionary"), Ok(true));
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
    assert_eq!(dict.get::<String>("foo").as_deref(), Ok("bar"));

    assert_eq!(dict.get("CheckCompID"), Ok(true));
    assert_eq!(dict.get("CheckLatency"), Ok(false));
    assert_eq!(dict.get("MaxLatency"), Ok(-20));
    assert_eq!(dict.get("ValidateLengthAndChecksum"), Ok(false));
    assert_eq!(dict.get("ValidateFieldsOutOfOrder"), Ok(false));
    assert_eq!(dict.get("ValidateFieldsHaveValues"), Ok(true));
    assert_eq!(dict.get("ValidateUserDefinedFields"), Ok(true));
    assert_eq!(dict.get("AllowUnknownMsgFields"), Ok(false));
    assert_eq!(dict.get("PreserveMessageFieldsOrder"), Ok(true));

    assert_eq!(dict.get("ResetOnLogon"), Ok(true));
    assert_eq!(dict.get("ResetOnLogout"), Ok(true));
    assert_eq!(dict.get("ResetOnDisconnect"), Ok(false));

    assert_eq!(dict.get("RefreshOnLogon"), Ok(true));

    assert_eq!(dict.get("HttpAcceptPort"), Ok(9090));

    assert_eq!(dict.get("PersistMessages"), Ok(false));
}
