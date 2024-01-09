use quickfix_spec_parser::*;

#[test]
fn test_invalid_field_type() {
    assert_eq!(
        "foo".parse::<FieldType>().unwrap_err(),
        FixSpecError::InvalidContent("unknown field type: foo".to_string())
    );
}

#[test]
fn test_invalid_message_category() {
    assert_eq!(
        "foo".parse::<MessageCategory>().unwrap_err(),
        FixSpecError::InvalidContent("invalid msgcat: foo".to_string())
    );
}
