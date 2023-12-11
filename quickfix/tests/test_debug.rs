use quickfix::*;

#[test]
fn test_data_dictionary() {
    let obj = DataDictionary::new();
    assert_eq!(format!("{obj:?}"), "DataDictionary");
}

#[test]
fn test_dictionary() {
    let obj = Dictionary::new();
    assert_eq!(format!("{obj:?}"), "Dictionary");
}

#[test]
fn test_header() {
    let obj = Header::new();
    assert_eq!(format!("{obj:?}"), "Header");
}

#[test]
fn test_trailer() {
    let obj = Trailer::new();
    assert_eq!(format!("{obj:?}"), "Trailer");
}

#[test]
fn test_group() {
    let obj = Group::try_new(42, 10).unwrap();
    assert_eq!(format!("{obj:?}"), "Group { id: 42, delim: 10 }");
}
