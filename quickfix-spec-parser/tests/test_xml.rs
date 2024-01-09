use quick_xml::events::BytesStart;
use quickfix_spec_parser::{read_attribute, FixSpecError};

#[test]
fn test_read_attributes() {
    let div =
        BytesStart::new("div").with_attributes([(b"class".as_slice(), b"test-me".as_slice())]);

    // Check valid.
    assert_eq!(read_attribute(&div, "class").as_deref(), Ok("test-me"));

    // Check not found.
    assert_eq!(
        read_attribute(&div, "style"),
        Err(FixSpecError::InvalidAttribute("style".to_string()))
    );
}
