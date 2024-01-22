use quickfix::*;

#[test]
fn test_derive() {
    // Debug + Display
    assert_eq!(
        format!(
            "{:?}",
            QuickFixError::InvalidArgument("hello world".to_string())
        ),
        "InvalidArgument(\"hello world\")"
    );
    assert_eq!(
        format!("{}", QuickFixError::from_last_error()),
        "invalid function return code: code=0, msg=Cannot get last error message from quickfix library"
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

#[test]
fn test_null() {
    assert_eq!(
        QuickFixError::from_last_error(),
        QuickFixError::InvalidFunctionReturnCode(
            0,
            "Cannot get last error message from quickfix library".to_string()
        )
    );
}
