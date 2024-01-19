use quickfix::*;

#[test]
fn test_from_file() {
    assert_eq!(
        SessionSettings::try_from_path("invalid_file.ini").unwrap_err(),
        QuickFixError::NullFunctionReturn
    );
    assert_eq!(
        SessionSettings::try_from_path("invalid_\0file.ini").unwrap_err(),
        QuickFixError::invalid_argument("nul byte found in provided data at position: 8")
    );
    let _settings1 = SessionSettings::new();
    let _settings2 = SessionSettings::try_from_path("../examples/configs/settings.ini").unwrap();
}

#[test]
fn test_getter_and_setter() {
    let mut dict_global = Dictionary::with_name("DEFAULT").unwrap();
    dict_global
        .set("ConnectionType", "initiator".to_string())
        .unwrap();
    dict_global.set("foo", 60).unwrap();

    let session_id = SessionId::try_new("FIX.4.4", "CLIENT1", "SERVER1", "").unwrap();
    let mut dict_session = Dictionary::with_name("SESSION").unwrap();
    dict_session.set("foo", 45).unwrap();

    let mut settings = SessionSettings::new();

    // Read settings without configure it
    assert!(settings
        .with_dictionary(Some(session_id.clone()), |dict| dict
            .get::<String>("foo")
            .unwrap())
        .is_none(),);

    // Configure settings
    settings.set(None, dict_global).unwrap();
    settings
        .set(Some(session_id.clone()), dict_session)
        .unwrap();

    // Read settings back
    assert_eq!(
        settings
            .with_dictionary(None, |dict| dict.get::<String>("foo").unwrap())
            .unwrap(),
        "60"
    );
    assert_eq!(
        settings
            .with_dictionary(None, |dict| dict.get::<String>("bar").ok())
            .unwrap(),
        None
    );
    assert_eq!(
        settings
            .with_dictionary(Some(session_id.clone()), |dict| dict
                .get::<String>("foo")
                .unwrap())
            .unwrap(),
        "45"
    );
    assert_eq!(
        settings
            .with_dictionary(Some(session_id.clone()), |dict| dict
                .get::<String>("bar ")
                .ok())
            .unwrap(),
        None,
    );
}
