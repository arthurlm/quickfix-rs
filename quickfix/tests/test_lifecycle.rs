use quickfix::{FileLogFactory, FileStoreFactory, QuickFixError, SessionSettings};

#[test]
fn test_session_settings() {
    assert_eq!(
        SessionSettings::try_from_path("invalid_file.ini").unwrap_err(),
        QuickFixError::InvalidFunctionReturn
    );
    let _settings = SessionSettings::try_from_path("../example/settings.ini").unwrap();
}

#[test]
fn test_file_log_factory() {
    let settings = SessionSettings::try_from_path("../example/settings.ini").unwrap();
    let _file_log_factory = FileLogFactory::try_new(&settings).unwrap();
}

#[test]
fn test_file_store_factory() {
    let settings = SessionSettings::try_from_path("../example/settings.ini").unwrap();
    let _file_store_factory = FileStoreFactory::try_new(&settings).unwrap();
}
