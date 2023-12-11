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
