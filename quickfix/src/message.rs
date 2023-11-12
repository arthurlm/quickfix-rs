#[derive(Debug)]
pub struct Message(pub(crate) quickfix_ffi::FixMessage_t);
