use quickfix::{FileStoreFactory, LogFactory, QuickFixError, SessionSettings, StdLogger};

#[test]
fn test_session_settings() {
    assert_eq!(
        SessionSettings::try_from_path("invalid_file.ini").unwrap_err(),
        QuickFixError::NullFunctionReturn
    );
    let _settings1 = SessionSettings::try_new().unwrap();
    let _settings2 = SessionSettings::try_from_path("../example/settings.ini").unwrap();
}

#[test]
fn test_log_factory() {
    let _file_log_factory = LogFactory::try_new(&StdLogger::Stdout).unwrap();
    let _file_log_factory = LogFactory::try_new(&StdLogger::Stderr).unwrap();
}

#[test]
#[cfg(feature = "log")]
fn test_extra_log_factory() {
    use quickfix::RustLogger;

    let _file_log_factory = LogFactory::try_new(&RustLogger).unwrap();
}

#[test]
fn test_file_store_factory() {
    let settings = SessionSettings::try_from_path("../example/settings.ini").unwrap();
    let _file_store_factory = FileStoreFactory::try_new(&settings).unwrap();
}
