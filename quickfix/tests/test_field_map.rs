use quickfix::*;

#[test]
fn test_header() {
    check_field_map(Header::new());
}

#[test]
fn test_trailer() {
    check_field_map(Trailer::new());
}

#[test]
fn test_group() {
    check_field_map(Group::try_new(42, 10).unwrap());
}

fn check_field_map<T: FieldMap>(mut item: T) {
    const FIELD_ID: i32 = 89;

    // Check not set
    assert_eq!(item.get_field(FIELD_ID), None);

    // Set and check
    item.set_field(FIELD_ID, "foo").unwrap();
    assert_eq!(item.get_field(FIELD_ID).as_deref(), Some("foo"));

    // Update and set
    item.set_field(FIELD_ID, "bar").unwrap();
    assert_eq!(item.get_field(FIELD_ID).as_deref(), Some("bar"));

    // Set with invalid value and check
    assert_eq!(
        item.set_field(FIELD_ID, "\0 haha"),
        Err(QuickFixError::invalid_argument(
            "nul byte found in provided data at position: 0"
        ))
    );
    assert_eq!(item.get_field(FIELD_ID).as_deref(), Some("bar"));

    // Remove and check
    item.remove_field(FIELD_ID).unwrap();
    assert_eq!(item.get_field(FIELD_ID), None);
}
