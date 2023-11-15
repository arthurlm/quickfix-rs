use std::{
    io::{stdout, Write},
    sync::atomic::{AtomicU32, Ordering},
};

use colored::Colorize;
use quickfix::{ApplicationCallback, Message, SessionId};

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
        let _ = write!(stdout, "{}", callback_name.blue().bold());
        let _ = write!(stdout, "({}={}) ", "id".green(), msg_count);
        let _ = write!(stdout, "{}={:?}", "session".green(), session);
        if let Some(msg) = msg {
            let _ = write!(stdout, " {}={:?}", "msg".green(), msg);
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

    fn on_msg_to_app(&self, msg: &mut Message, session: &SessionId) {
        self.inc_message_index();
        self.print_callback("to_app", session, Some(msg));
    }

    fn on_msg_from_admin(&self, msg: &Message, session: &SessionId) {
        self.inc_message_index();
        self.print_callback("from_admin", session, Some(msg));
    }

    fn on_msg_from_app(&self, msg: &Message, session: &SessionId) {
        self.inc_message_index();
        self.print_callback("from_app", session, Some(msg));
    }
}
