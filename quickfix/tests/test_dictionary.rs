use quickfix::*;

#[test]
fn test_text() {
    let mut dict = Dictionary::try_new("HELLO").unwrap();

    // Test with valid kay / value
    dict.set("str_1", "foo".to_string()).unwrap();
    assert_eq!(dict.get::<String>("str_1").unwrap(), "foo");

    dict.set("str_2", "".to_string()).unwrap();
    assert_eq!(dict.get::<String>("str_2").unwrap(), "");

    // Test with invalid key
    assert!(dict.get::<String>("invalid").is_err());
}

#[test]
fn test_int() {
    let mut dict = Dictionary::try_new("HELLO").unwrap();

    // Test with valid kay / value
    dict.set("int_1", 42).unwrap();
    assert_eq!(dict.get::<i32>("int_1").unwrap(), 42);

    dict.set("int_2", -48).unwrap();
    assert_eq!(dict.get::<i32>("int_2").unwrap(), -48);

    dict.set("int_3", 0).unwrap();
    assert_eq!(dict.get::<i32>("int_3").unwrap(), 0);

    // Test with invalid key
    assert_eq!(dict.get::<i32>("invalid").unwrap(), 0);
}

#[test]
fn test_double() {
    let mut dict = Dictionary::try_new("HELLO").unwrap();

    // Test with valid kay / value
    dict.set("double_1", 56.8).unwrap();
    assert_eq!(dict.get::<f64>("double_1").unwrap(), 56.8);

    dict.set("double_2", f64::NAN).unwrap();
    assert_eq!(dict.get::<f64>("double_2").unwrap(), 0.0);

    dict.set("double_3", 0.0).unwrap();
    assert_eq!(dict.get::<f64>("double_3").unwrap(), 0.0);

    dict.set("double_4", f64::NEG_INFINITY).unwrap();
    assert_eq!(dict.get::<f64>("double_4").unwrap(), 0.0);

    dict.set("double_5", f64::INFINITY).unwrap();
    assert_eq!(dict.get::<f64>("double_5").unwrap(), 0.0);

    dict.set("double_6", -51.23).unwrap();
    assert_eq!(dict.get::<f64>("double_6").unwrap(), -51.23);

    // Test with invalid key
    assert_eq!(dict.get::<f64>("invalid").unwrap(), 0.0);
}

#[test]
fn test_bool() {
    let mut dict = Dictionary::try_new("HELLO").unwrap();

    // Test with valid kay / value
    dict.set("bool_1", false).unwrap();
    assert_eq!(dict.get::<bool>("bool_1").unwrap(), false);

    dict.set("bool_2", true).unwrap();
    assert_eq!(dict.get::<bool>("bool_2").unwrap(), true);

    // Test with invalid key
    assert!(dict.get::<bool>("invalid").is_err());
}

#[test]
fn test_day() {
    let mut dict = Dictionary::try_new("HELLO").unwrap();

    // Test with valid kay / value
    dict.set("day_1", 0).unwrap();
    assert!(dict.get::<DayOfWeek>("day_1").is_err());

    dict.set("day_2", "TH".to_string()).unwrap();
    assert_eq!(dict.get::<DayOfWeek>("day_2").unwrap(), DayOfWeek::Thursday);

    dict.set("day_3", -4).unwrap();
    assert!(dict.get::<DayOfWeek>("day_3").is_err());

    dict.set("day_4", DayOfWeek::Friday).unwrap();
    assert_eq!(dict.get::<DayOfWeek>("day_4").unwrap(), DayOfWeek::Friday);
    assert_eq!(dict.get::<String>("day_4").unwrap(), "FR");

    // Test with invalid key
    assert!(dict.get::<DayOfWeek>("invalid").is_err());
}