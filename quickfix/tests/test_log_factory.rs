use quickfix::*;

#[test]
fn test_log_factory() {
    let _log_factory = LogFactory::try_new(&StdLogger::Stdout).unwrap();
    let _log_factory = LogFactory::try_new(&StdLogger::Stderr).unwrap();
}

#[test]
#[cfg(feature = "log")]
fn test_extra_log_factory() {
    use quickfix::RustLogger;

    let _log_factory = LogFactory::try_new(&RustLogger).unwrap();
}
