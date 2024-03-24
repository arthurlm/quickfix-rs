use quickfix::*;

#[test]
fn test_file() {
    let settings = SessionSettings::new();
    let message_store_factory = FileMessageStoreFactory::try_new(&settings).unwrap();
    let _ptr = message_store_factory.as_ffi_ptr();
}

#[test]
#[cfg(feature = "build-with-mysql")]
fn test_mysql() {
    let settings = SessionSettings::new();
    let message_store_factory = MySqlMessageStoreFactory::try_new(&settings).unwrap();
    let _ptr = message_store_factory.as_ffi_ptr();
}

#[test]
#[cfg(feature = "build-with-postgres")]
fn test_postgres() {
    let settings = SessionSettings::new();
    let message_store_factory = PostgresMessageStoreFactory::try_new(&settings).unwrap();
    let _ptr = message_store_factory.as_ffi_ptr();
}

#[test]
fn test_memory() {
    let message_store_factory = MemoryMessageStoreFactory::new();
    let _ptr = message_store_factory.as_ffi_ptr();
}

#[test]
fn test_null() {
    let message_store_factory = NullMessageStoreFactory::new();
    let _ptr = message_store_factory.as_ffi_ptr();
}
