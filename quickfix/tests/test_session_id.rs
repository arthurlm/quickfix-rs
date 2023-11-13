use quickfix::SessionId;

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
fn test_fixt() {
    let session1 = SessionId::try_new("FIX.4.1", "FOO", "BAR", "").unwrap();
    let session2 = SessionId::try_new("FIXT", "FOO", "BAR", "").unwrap();

    assert!(!session1.is_fixt());
    assert!(session2.is_fixt());
}
