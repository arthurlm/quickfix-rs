use quickfix::*;

#[test]
fn test_new() {
    let session = SessionId::try_new("FIX.4.1", "FOO", "BAR", "").unwrap();
    assert_eq!(session.get_begin_string().as_deref(), Some("FIX.4.1"));
    assert_eq!(session.get_sender_comp_id().as_deref(), Some("FOO"));
    assert_eq!(session.get_target_comp_id().as_deref(), Some("BAR"));
    assert_eq!(session.get_session_qualifier().as_deref(), Some(""));
    assert_eq!(session.as_string(), "FIX.4.1:FOO->BAR");
}

#[test]
fn test_new_invalid_string() {
    let expected =
        QuickFixError::invalid_argument("nul byte found in provided data at position: 3");

    let err = SessionId::try_new("Bad\0 FIX.4.1", "FOO", "BAR", "").unwrap_err();
    assert_eq!(err, expected);
    let err = SessionId::try_new("FIX.4.1", "Bad\0 FOO", "BAR", "").unwrap_err();
    assert_eq!(err, expected);
    let err = SessionId::try_new("FIX.4.1", "FOO", "Bad\0 BAR", "").unwrap_err();
    assert_eq!(err, expected);
    let err = SessionId::try_new("FIX.4.1", "FOO", "BAR", "Bad\0").unwrap_err();
    assert_eq!(err, expected);
}

#[test]
fn test_fixt() {
    let session1 = SessionId::try_new("FIX.4.1", "FOO", "BAR", "").unwrap();
    let session2 = SessionId::try_new("FIXT", "FOO", "BAR", "").unwrap();

    assert!(!session1.is_fixt());
    assert!(session2.is_fixt());
}

#[test]
fn test_clone() {
    let session1 = SessionId::try_new("FIX.4.1", "FOO", "BAR", "").unwrap();
    assert_eq!(session1.as_string(), "FIX.4.1:FOO->BAR");

    let session2 = session1.clone();
    assert_eq!(session2.as_string(), "FIX.4.1:FOO->BAR");

    // Test do not crash after drop
    drop(session1);
    assert_eq!(session2.as_string(), "FIX.4.1:FOO->BAR");
}
