use quickfix::*;

#[test]
fn test_read_empy_message() {
    let msg = Message::new();
    assert_eq!(msg.as_string().as_deref(), Ok("9=0\u{1}10=167\u{1}"));
}

#[test]
fn test_as_string() {
    let msg = Message::try_from_text("9=0\u{1}10=000\u{1}").unwrap();
    assert_eq!(
        msg.as_string_with_len(512).as_deref(),
        Ok("9=0\u{1}10=167\u{1}")
    );
    assert_eq!(
        msg.as_string_with_len(6),
        Err(QuickFixError::InvalidFunctionReturnCode(-3))
    );
}

#[test]
fn test_from_text() {
    // Check with invalid C string
    {
        assert_eq!(
            Message::try_from_text("\050=18").unwrap_err(),
            QuickFixError::invalid_argument("nul byte found in provided data at position: 0")
        );
    }
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
    let mut msg = Message::new();
    msg.set_field(42, "foo").unwrap();
    msg.set_field(56, "bar").unwrap();
    assert_eq!(
        msg.as_string().as_deref(),
        Ok("9=14\u{1}42=foo\u{1}56=bar\u{1}10=162\u{1}")
    );
}

#[test]
fn test_set_field_twice() {
    let mut msg = Message::new();

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
    let mut msg = Message::new();
    assert_eq!(msg.get_field(42), None);

    msg.set_field(42, "hello world").unwrap();
    assert_eq!(msg.get_field(42).as_deref(), Some("hello world"));
}

#[test]
fn test_remove_field() {
    let mut msg = Message::new();
    assert_eq!(msg.get_field(42), None);

    msg.remove_field(42).unwrap();
    assert_eq!(msg.get_field(42), None);

    msg.set_field(42, "hello world").unwrap();
    assert_eq!(msg.get_field(42).as_deref(), Some("hello world"));

    msg.remove_field(42).unwrap();
    assert_eq!(msg.get_field(42), None);
}

#[test]
fn test_get_header() {
    let mut msg = Message::new();
    msg.set_field(40000, "foo").unwrap();

    // Set some header value.
    assert_eq!(msg.with_header(|x| x.get_field(50000)), None);

    msg.with_header_mut(|x| {
        x.set_field(50000, "hello").unwrap();
    });

    assert_eq!(
        msg.with_header(|x| x.get_field(50000)).as_deref(),
        Some("hello")
    );

    // Check full message
    assert_eq!(
        msg.as_string().as_deref(),
        Ok("9=22\u{1}50000=hello\u{1}40000=foo\u{1}10=152\u{1}")
    )
}

#[test]
fn test_copy_header() {
    let mut msg = Message::new();
    msg.with_header_mut(|x| x.set_field(5000, "hello")).unwrap();

    // Check val
    let cpy = msg.clone_header();
    assert_eq!(cpy.get_field(5000).as_deref(), Some("hello"));

    // Update val and check diffs
    msg.with_header_mut(|x| x.set_field(5000, "world")).unwrap();

    assert_eq!(
        msg.with_header(|x| x.get_field(5000)).as_deref(),
        Some("world")
    );
    assert_eq!(cpy.get_field(5000).as_deref(), Some("hello"));
}

#[test]
fn test_get_trailer() {
    let mut msg = Message::new();
    msg.set_field(40000, "foo").unwrap();

    // Set some trailer value.
    assert_eq!(msg.with_trailer(|x| x.get_field(50001)), None);

    msg.with_trailer_mut(|x| {
        x.set_field(50001, "bar").unwrap();
    });

    assert_eq!(
        msg.with_trailer(|x| x.get_field(50001)).as_deref(),
        Some("bar")
    );

    // Check full message
    assert_eq!(
        msg.as_string().as_deref(),
        Ok("9=20\u{1}40000=foo\u{1}50001=bar\u{1}10=184\u{1}")
    )
}

#[test]
fn test_copy_trailer() {
    let mut msg = Message::new();
    msg.with_trailer_mut(|x| x.set_field(5000, "hello"))
        .unwrap();

    // Check val
    let cpy = msg.clone_trailer();
    assert_eq!(cpy.get_field(5000).as_deref(), Some("hello"));

    // Update val and check diffs
    msg.with_trailer_mut(|x| x.set_field(5000, "world"))
        .unwrap();

    assert_eq!(
        msg.with_trailer(|x| x.get_field(5000)).as_deref(),
        Some("world")
    );
    assert_eq!(cpy.get_field(5000).as_deref(), Some("hello"));
}
