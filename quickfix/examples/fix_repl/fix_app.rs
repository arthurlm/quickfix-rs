use std::{
    io::{stdout, Write},
    sync::atomic::{AtomicU32, Ordering},
};

use quickfix::*;

#[derive(Default)]
pub struct MyApplication {
    message_index: AtomicU32,
}

impl MyApplication {
    pub fn new() -> Self {
        Self::default()
    }

    fn inc_message_index(&self) {
        self.message_index.fetch_add(1, Ordering::Relaxed);
    }

    fn print_callback(&self, callback_name: &str, session: &SessionId, msg: Option<&Message>) {
        let msg_count = self.message_index.load(Ordering::Relaxed);

        let mut stdout = stdout().lock();
        let _ = write!(stdout, "{callback_name}");
        let _ = write!(stdout, "(id={msg_count}) ");
        let _ = write!(stdout, "session={session:?}");
        if let Some(msg) = msg {
            let _ = write!(stdout, " msg={msg:?}");
        }
        let _ = writeln!(stdout);
    }
}

impl ApplicationCallback for MyApplication {
    fn on_create(&self, session: &SessionId) {
        self.print_callback("on_create", session, None);
    }

    fn on_logon(&self, session: &SessionId) {
        self.print_callback("on_logon", session, None);
    }

    fn on_logout(&self, session: &SessionId) {
        self.print_callback("on_logout", session, None);
    }

    fn on_msg_to_admin(&self, msg: &mut Message, session: &SessionId) {
        self.inc_message_index();
        self.print_callback("to_admin", session, Some(msg));
    }

    fn on_msg_to_app(&self, msg: &mut Message, session: &SessionId) -> Result<(), MsgToAppError> {
        self.inc_message_index();
        self.print_callback("to_app", session, Some(msg));
        Ok(())
    }

    fn on_msg_from_admin(
        &self,
        msg: &Message,
        session: &SessionId,
    ) -> Result<(), MsgFromAdminError> {
        self.inc_message_index();
        self.print_callback("from_admin", session, Some(msg));
        Ok(())
    }

    fn on_msg_from_app(&self, msg: &Message, session: &SessionId) -> Result<(), MsgFromAppError> {
        self.inc_message_index();
        self.print_callback("from_app", session, Some(msg));
        Ok(())
    }
}
