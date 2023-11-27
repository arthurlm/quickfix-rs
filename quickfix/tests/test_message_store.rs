use quickfix::*;

#[test]
fn test_file() {
    let settings = SessionSettings::new();
    let _message_store_factory = FileMessageStoreFactory::try_new(&settings).unwrap();
}

#[test]
#[cfg(feature = "build-with-mysql")]
fn test_mysql() {
    let settings = SessionSettings::new();
    let _message_store_factory = MySqlMessageStoreFactory::try_new(&settings).unwrap();
}

#[test]
#[cfg(feature = "build-with-postgres")]
fn test_postgres() {
    let settings = SessionSettings::new();
    let _message_store_factory = PostgresMessageStoreFactory::try_new(&settings).unwrap();
}

#[test]
fn test_memory() {
    let _message_store_factory = MemoryMessageStoreFactory::new();
}
