use quickfix::Message;

#[test]
fn test_read_empy_message() {
    let msg = Message::try_new().unwrap();
    assert_eq!(msg.as_string().as_deref(), Ok("9=0\u{1}10=167\u{1}"));
}

#[test]
fn test_from_text() {
    // Check compute len + checksum
    {
        let msg = Message::try_from_text("").unwrap();
        assert_eq!(msg.as_string().as_deref(), Ok("9=0\u{1}10=167\u{1}"));
    }
    // Check recompute len + checksum
    {
        let msg = Message::try_from_text("9=0\u{1}10=000\u{1}").unwrap();
        assert_eq!(msg.as_string().as_deref(), Ok("9=0\u{1}10=167\u{1}"));
    }
    // Check compute len + checksum
    {
        let msg = Message::try_from_text("42=foo\u{1}56=bar\u{1}").unwrap();
        assert_eq!(
            msg.as_string().as_deref(),
            Ok("9=14\u{1}56=bar\u{1}42=foo\u{1}10=162\u{1}")
        );
    }
    // Check recompute len + checksum
    {
        let msg = Message::try_from_text("9=14\u{1}56=bar\u{1}42=foo\u{1}10=162\u{1}").unwrap();
        assert_eq!(
            msg.as_string().as_deref(),
            Ok("9=14\u{1}56=bar\u{1}42=foo\u{1}10=162\u{1}")
        );
    }
}

#[test]
fn test_set_field() {
    let mut msg = Message::try_new().unwrap();
    msg.set_field(42, "foo").unwrap();
    msg.set_field(56, "bar").unwrap();
    assert_eq!(
        msg.as_string().as_deref(),
        Ok("9=14\u{1}42=foo\u{1}56=bar\u{1}10=162\u{1}")
    );
}

#[test]
fn test_set_field_twice() {
    let mut msg = Message::try_new().unwrap();

    msg.set_field(42, "foo").unwrap();
    assert_eq!(
        msg.as_string().as_deref(),
        Ok("9=7\u{1}42=foo\u{1}10=150\u{1}")
    );

    msg.set_field(42, "bar").unwrap();
    assert_eq!(
        msg.as_string().as_deref(),
        Ok("9=7\u{1}42=bar\u{1}10=135\u{1}")
    );
}

#[test]
fn test_get_field() {
    let mut msg = Message::try_new().unwrap();
    assert_eq!(msg.get_field(42), None);

    msg.set_field(42, "hello world").unwrap();
    assert_eq!(msg.get_field(42).as_deref(), Some("hello world"));
}

#[test]
fn test_remove_field() {
    let mut msg = Message::try_new().unwrap();
    assert_eq!(msg.get_field(42), None);

    msg.remove_field(42).unwrap();
    assert_eq!(msg.get_field(42), None);

    msg.set_field(42, "hello world").unwrap();
    assert_eq!(msg.get_field(42).as_deref(), Some("hello world"));

    msg.remove_field(42).unwrap();
    assert_eq!(msg.get_field(42), None);
}
