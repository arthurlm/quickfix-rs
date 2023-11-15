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
