use quickfix::*;

#[test]
fn test_data_dictionary() {
    let obj = DataDictionary::default();
    assert_eq!(format!("{obj:?}"), "DataDictionary");
}

#[test]
fn test_dictionary() {
    let obj = Dictionary::default();
    assert_eq!(format!("{obj:?}"), "Dictionary");
}
