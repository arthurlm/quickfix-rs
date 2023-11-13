use quickfix::Message;

#[test]
fn test_read_empy_message() {
    let msg = Message::try_new().unwrap();
    assert_eq!(msg.as_string().as_deref(), Ok("9=0\u{1}10=167\u{1}"));
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
