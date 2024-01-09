use std::fs;

use quickfix_spec_parser::{parse_spec, write_spec};

fn main() {
    // Parse XML spec.
    let spec = parse_spec(include_bytes!(
        "../../quickfix-ffi/libquickfix/spec/FIXT11.xml"
    ))
    .unwrap();

    // Print it.
    println!("spec: {spec:#?}");

    // Rewrite it as XML and apply change to make it match with original spec format.
    let out = write_spec(&spec).unwrap();
    let txt = String::from_utf8(out.to_vec())
        .unwrap()
        .replace('\"', "'")
        .replace("/>", " />");

    fs::write("out.xml", txt).unwrap();
}
