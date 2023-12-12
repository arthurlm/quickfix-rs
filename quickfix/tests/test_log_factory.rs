use quickfix::*;
use utils::*;

mod utils;

const TEST_PORT: u16 = 8001;

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

#[test]
fn test_null_logger() {
    checker::run(
        TEST_PORT,
        NullLogger,
        // Sender
        NullFixApplication,
        MemoryMessageStoreFactory::new(),
        // Receiver
        NullFixApplication,
        MemoryMessageStoreFactory::new(),
    )
    .unwrap();
}

#[test]
fn test_stdout_logger() {
    checker::run(
        TEST_PORT,
        StdLogger::Stdout,
        // Sender
        NullFixApplication,
        MemoryMessageStoreFactory::new(),
        // Receiver
        NullFixApplication,
        MemoryMessageStoreFactory::new(),
    )
    .unwrap();
}

#[test]
fn test_stderr_logger() {
    checker::run(
        TEST_PORT,
        StdLogger::Stderr,
        // Sender
        NullFixApplication,
        MemoryMessageStoreFactory::new(),
        // Receiver
        NullFixApplication,
        MemoryMessageStoreFactory::new(),
    )
    .unwrap();
}

#[test]
#[cfg(feature = "log")]
fn test_rust_logger() {
    use quickfix::RustLogger;

    checker::run(
        TEST_PORT,
        RustLogger,
        // Sender
        NullFixApplication,
        MemoryMessageStoreFactory::new(),
        // Receiver
        NullFixApplication,
        MemoryMessageStoreFactory::new(),
    )
    .unwrap();
}
