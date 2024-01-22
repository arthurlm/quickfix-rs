use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

use quickfix::*;

#[derive(Debug)]
pub struct FixRecorder {
    expected_session_id: SessionId,
    session_created: AtomicUsize,
    is_logged_in: AtomicBool,
    sent_admin: AtomicUsize,
    sent_user: AtomicUsize,
    recv_admin: AtomicUsize,
    recv_user: AtomicUsize,
}

impl FixRecorder {
    pub fn new(expected_session_id: SessionId) -> Self {
        Self {
            expected_session_id,
            session_created: AtomicUsize::new(0),
            is_logged_in: AtomicBool::new(false),
            sent_admin: AtomicUsize::new(0),
            sent_user: AtomicUsize::new(0),
            recv_admin: AtomicUsize::new(0),
            recv_user: AtomicUsize::new(0),
        }
    }

    pub fn session_created(&self) -> usize {
        self.session_created.load(Ordering::Relaxed)
    }

    pub fn is_logged_in(&self) -> bool {
        self.is_logged_in.load(Ordering::Relaxed)
    }

    pub fn admin_msg_count(&self) -> MsgCounter {
        MsgCounter {
            sent: self.sent_admin.load(Ordering::Relaxed),
            recv: self.recv_admin.load(Ordering::Relaxed),
        }
    }

    pub fn user_msg_count(&self) -> MsgCounter {
        MsgCounter {
            sent: self.sent_user.load(Ordering::Relaxed),
            recv: self.recv_user.load(Ordering::Relaxed),
        }
    }
}

impl ApplicationCallback for FixRecorder {
    fn on_create(&self, session_id: &SessionId) {
        self.session_created.fetch_add(1, Ordering::Relaxed);
        assert_session_id_equals(&self.expected_session_id, &session_id);
    }

    fn on_logon(&self, session_id: &SessionId) {
        self.is_logged_in.store(true, Ordering::Relaxed);
        assert_session_id_equals(&self.expected_session_id, &session_id);
    }

    fn on_logout(&self, session_id: &SessionId) {
        self.is_logged_in.store(false, Ordering::Relaxed);
        assert_session_id_equals(&self.expected_session_id, &session_id);
    }

    fn on_msg_to_admin(&self, _msg: &mut Message, session_id: &SessionId) {
        self.sent_admin.fetch_add(1, Ordering::Relaxed);
        assert_session_id_equals(&self.expected_session_id, &session_id);
    }

    fn on_msg_to_app(
        &self,
        _msg: &mut Message,
        session_id: &SessionId,
    ) -> Result<(), MsgToAppError> {
        self.sent_user.fetch_add(1, Ordering::Relaxed);
        assert_session_id_equals(&self.expected_session_id, &session_id);
        Ok(())
    }

    fn on_msg_from_admin(
        &self,
        _msg: &Message,
        session_id: &SessionId,
    ) -> Result<(), MsgFromAdminError> {
        self.recv_admin.fetch_add(1, Ordering::Relaxed);
        assert_session_id_equals(&self.expected_session_id, &session_id);
        Ok(())
    }

    fn on_msg_from_app(
        &self,
        _msg: &Message,
        session_id: &SessionId,
    ) -> Result<(), MsgFromAppError> {
        self.recv_user.fetch_add(1, Ordering::Relaxed);
        assert_session_id_equals(&self.expected_session_id, &session_id);
        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
pub struct MsgCounter {
    pub sent: usize,
    pub recv: usize,
}

fn assert_session_id_equals(a: &SessionId, b: &SessionId) {
    assert_eq!(a.as_string(), b.as_string());
}
