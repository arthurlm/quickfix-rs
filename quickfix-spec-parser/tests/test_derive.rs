use std::fmt::Debug;

use quickfix_spec_parser::*;

macro_rules! s {
    ($x:expr) => {
        $x.to_string()
    };
}

fn check<T: Debug>(obj: T, expected: &str) {
    assert_eq!(format!("{obj:?}"), expected);
}

#[test]
fn test_error() {
    assert_eq!(
        format!("{:?}", FixSpecError::InvalidDocument("bad header").clone()),
        "InvalidDocument(\"bad header\")"
    );
    assert_eq!(
        format!("{}", FixSpecError::InvalidDocument("bad header").clone()),
        "invalid document: bad header"
    );
    assert_ne!(
        FixSpecError::InvalidDocument("Bad header").clone(),
        FixSpecError::Xml(s!("hello"))
    );
}

#[test]
fn test_debug() {
    // This test are here only to make code coverage happy ... 😒

    check(
        Component {
            name: s!("foo"),
            required: false,
        }
        .clone(),
        "Component { name: \"foo\", required: false }",
    );
    check(
        ComponentSpec {
            name: s!("bar"),
            values: vec![],
        }
        .clone(),
        "ComponentSpec { name: \"bar\", values: [] }",
    );
    check(MessageCategory::Admin, "Admin");
    check(FieldType::SequenceNumber, "SequenceNumber");
    check(
        FieldAllowedValue {
            value: s!("hello"),
            description: s!("Some value"),
        }
        .clone(),
        "FieldAllowedValue { value: \"hello\", description: \"Some value\" }",
    );
    check(
        FieldSpec {
            number: 42,
            name: s!("The Ultimate Question of Life"),
            r#type: FieldType::Amount,
            values: vec![],
        }.clone(),
        "FieldSpec { number: 42, name: \"The Ultimate Question of Life\", type: Amount, values: [] }",
    );
    check(
        Field {
            name: s!("X"),
            required: false,
        }
        .clone(),
        "Field { name: \"X\", required: false }",
    );
    check(
        Group {
            name: s!("X"),
            required: true,
            values: vec![],
        }
        .clone(),
        "Group { name: \"X\", required: true, values: [] }",
    );
    check(
        FieldValue::Field(Field {
            name: s!("X"),
            required: false,
        })
        .clone(),
        "Field(Field { name: \"X\", required: false })",
    );
    check(
        Message {
            name: s!("foo"),
            category: MessageCategory::App,
            msg_type: s!("bar"),
            values: vec![],
        }
        .clone(),
        "Message { name: \"foo\", msg_type: \"bar\", category: App, values: [] }",
    );
    check(
        FixSpec {
            version: (4, 8, 3),
            is_fixt: false,
            headers: vec![],
            messages: vec![],
            trailers: vec![],
            component_specs: vec![],
            field_specs: vec![],
        }.clone(),
        "FixSpec { version: (4, 8, 3), is_fixt: false, headers: [], messages: [], trailers: [], component_specs: [], field_specs: [] }",
    );
}
