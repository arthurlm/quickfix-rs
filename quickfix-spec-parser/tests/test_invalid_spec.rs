use quickfix_spec_parser::*;

fn check_text(expected: &[u8]) {
    assert!(parse_spec(expected).is_err(),);
}

#[test]
fn test_invalid_parse() {
    // Bad root node.
    check_text(b"<root></root>");

    // Bad version.
    check_text(r#"<fix major="8"></fix>"#.as_bytes());
    check_text(r#"<fix major="8" minor="4"></fix>"#.as_bytes());
    check_text(r#"<fix major="hello"></fix>"#.as_bytes());
    check_text(r#"<fix major="8" minor="hello"></fix>"#.as_bytes());

    // Missing fix type.
    check_text(r#"<fix major="8" minor="4" servicepack="3"></fix>"#.as_bytes());

    // Invalid XML.
    check_text(b"<fix type='FIXT' major='1' minor='1' servicepack='0'><head");

    // Invalid UTF8 attribute.
    check_text(b"<fix type='\xFF\xFF' major='1' minor='1' servicepack='0'><head");
}
