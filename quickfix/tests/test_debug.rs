use quickfix::*;

#[test]
fn test_application() {
    let obj = MsgToAppError::DoNotSend;
    assert_eq!(format!("{obj:?}"), "DoNotSend");
    let obj = MsgFromAdminError::RejectLogon;
    assert_eq!(format!("{obj:?}"), "RejectLogon");
    let obj = MsgFromAppError::UnsupportedMessageType;
    assert_eq!(format!("{obj:?}"), "UnsupportedMessageType");
}

#[test]
fn test_data_dictionary() {
    let obj = DataDictionary::new();
    assert_eq!(format!("{obj:?}"), "DataDictionary");
}

#[test]
fn test_dictionary() {
    let obj = Dictionary::new();
    assert_eq!(format!("{obj:?}"), "Dictionary");
}

#[test]
fn test_header() {
    let obj = Header::new();
    assert_eq!(format!("{obj:?}"), "Header");
}

#[test]
fn test_trailer() {
    let obj = Trailer::new();
    assert_eq!(format!("{obj:?}"), "Trailer");
}

#[test]
fn test_group() {
    let obj = Group::try_new(42, 10).unwrap();
    assert_eq!(format!("{obj:?}"), "Group { id: 42, delim: 10 }");
}

#[test]
fn test_log_factory() {
    let obj = LogFactory::try_new(&NullLogger).unwrap();
    assert_eq!(format!("{obj:?}"), "LogFactory");
}

#[test]
fn test_logger() {
    let obj = NullLogger;
    assert_eq!(format!("{obj:?}"), "NullLogger");

    let obj = StdLogger::Stdout;
    assert_eq!(format!("{obj:?}"), "log_stdout");
    let obj = StdLogger::Stderr;
    assert_eq!(format!("{obj:?}"), "log_stderr");

    #[cfg(feature = "log")]
    {
        let obj = RustLogger;
        assert_eq!(format!("{obj:?}"), "RustLogger");
    }
}

#[test]
fn test_message() {
    let obj = Message::try_from_text("9=0\u{1}10=000\u{1}").unwrap();
    assert_eq!(format!("{obj:?}"), "Message(\"9=0|10=167|\")");
}

#[test]
fn test_session_settings() {
    let obj = SessionSettings::new();
    assert_eq!(format!("{obj:?}"), "SessionSettings");
}
