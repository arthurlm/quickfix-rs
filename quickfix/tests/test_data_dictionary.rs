use quickfix::{DataDictionary, QuickFixError};

#[test]
fn test_create_and_drop() {
    DataDictionary::try_new().unwrap();
}

#[test]
fn test_open_invalid_data_dictionary() {
    assert_eq!(
        DataDictionary::try_from_path("/invalid/path").unwrap_err(),
        QuickFixError::InvalidFunctionReturn
    );
}

#[test]
fn test_open_valid_data_dictionary() {
    DataDictionary::try_from_path("../libquickfix/spec/FIX44.xml").unwrap();
}

#[test]
fn test_build_message() {
    let dd = DataDictionary::try_from_path("../libquickfix/spec/FIX44.xml").unwrap();
    let msg = dd.try_build_message("8=FIX.4.1\u{1}9=65\u{1}35=A\u{1}34=1\u{1}49=SERVER1\u{1}52=20231115-14:02:24\u{1}56=CLIENT1\u{1}98=0\u{1}108=20\u{1}10=035\u{1}").unwrap();
    assert_eq!(msg.as_string().as_deref(), Ok("8=FIX.4.1\u{1}9=65\u{1}35=A\u{1}34=1\u{1}49=SERVER1\u{1}52=20231115-14:02:24\u{1}56=CLIENT1\u{1}98=0\u{1}108=20\u{1}10=035\u{1}"))
}
