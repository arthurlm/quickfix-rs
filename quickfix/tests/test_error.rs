use quickfix::*;

#[test]
fn test_derive() {
    // Debug + Display
    assert_eq!(
        format!("{:?}", QuickFixError::NullFunctionReturn),
        "NullFunctionReturn"
    );
    assert_eq!(
        format!("{}", QuickFixError::NullFunctionReturn),
        "null function return"
    );

    // PartialEq + Eq
    assert_eq!(
        QuickFixError::invalid_argument("Hello"),
        QuickFixError::InvalidArgument("Hello".to_string())
    );
    assert_ne!(
        QuickFixError::invalid_argument("Hello"),
        QuickFixError::InvalidArgument("World".to_string())
    );

    // Clone
    assert_eq!(
        QuickFixError::invalid_argument("Hello").clone(),
        QuickFixError::InvalidArgument("Hello".to_string())
    );
}
