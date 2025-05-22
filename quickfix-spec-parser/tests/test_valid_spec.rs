use quickfix_spec_parser::*;

fn check(expected: &[u8]) {
    let spec = parse_spec(expected).unwrap();

    let out = write_spec(&spec).unwrap();
    let out_txt = String::from_utf8(out)
        .unwrap()
        .replace('\"', "'")
        .replace("/>", " />")
        .replace("\r\n", "\n");

    let expected_txt = String::from_utf8(expected.to_vec())
        .unwrap()
        .replace("\r\n", "\n");

    assert_eq!(out_txt, expected_txt);
}

#[test]
fn test_fix40() {
    check(include_bytes!(
        "../../quickfix-ffi/libquickfix/spec/FIX40.xml"
    ));
}

#[test]
fn test_fix41() {
    check(include_bytes!(
        "../../quickfix-ffi/libquickfix/spec/FIX41.xml"
    ));
}

#[test]
fn test_fix42() {
    check(include_bytes!(
        "../../quickfix-ffi/libquickfix/spec/FIX42.xml"
    ));
}

#[test]
fn test_fix43() {
    check(include_bytes!(
        "../../quickfix-ffi/libquickfix/spec/FIX43.xml"
    ));
}

#[test]
fn test_fix44() {
    check(include_bytes!(
        "../../quickfix-ffi/libquickfix/spec/FIX44.xml"
    ));
}

#[test]
fn test_fix50() {
    check(include_bytes!(
        "../../quickfix-ffi/libquickfix/spec/FIX50.xml"
    ));
}

#[test]
fn test_fix50sp1() {
    check(include_bytes!(
        "../../quickfix-ffi/libquickfix/spec/FIX50SP1.xml"
    ));
}

#[test]
fn test_fix50sp2() {
    check(include_bytes!(
        "../../quickfix-ffi/libquickfix/spec/FIX50SP2.xml"
    ));
}

#[test]
fn test_fixt11() {
    check(include_bytes!(
        "../../quickfix-ffi/libquickfix/spec/FIXT11.xml"
    ));
}

#[test]
fn test_coinbase_order_entry() {
    // Cannot check parsed content, since comments are dropped.
    parse_spec(include_bytes!(
        "../../examples/coinbase-fix42-order-entry/src/cb-FIX42-prod-sand.xml"
    ))
    .unwrap();
}

#[test]
fn test_coinbase_market_data() {
    check(include_bytes!(
        "../../examples/coinbase-fix50-market-data/src/cb-FIX50-prod-sand.xml"
    ));
}

#[test]
fn test_parse_with_comment() {
    // This test has multiple purposes:
    // 1. Increase code coverage
    // 2. Check parser do not crash if there is unhandled node
    parse_spec(include_bytes!("data/commented_file.xml")).unwrap();
}
