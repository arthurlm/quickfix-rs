use quickfix::*;

#[test]
fn test_derive() {
    // Debug + Display
    assert_eq!(
        format!(
            "{:?}",
            QuickFixError::NullFunctionReturn("hello world".to_string())
        ),
        "NullFunctionReturn(\"hello world\")"
    );
    assert_eq!(
        format!("{}", QuickFixError::null()),
        "null function return: Cannot get last error message from quickfix library"
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
        QuickFixError::null(),
        QuickFixError::NullFunctionReturn(
            "Cannot get last error message from quickfix library".to_string()
        )
    );
}
