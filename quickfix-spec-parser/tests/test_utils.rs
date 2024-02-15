use quickfix_spec_parser::FixSpec;

#[test]
fn test_new_fixt() {
    let spec = FixSpec::new_fixt();
    assert_eq!(spec.version, (1, 1, 0));
    assert!(spec.is_fixt);
}
