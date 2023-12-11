use std::{
    net::TcpListener,
    sync::atomic::{AtomicBool, AtomicUsize, Ordering},
    thread,
    time::Duration,
};

use msg_const::*;
use quickfix::*;

mod msg_const;

// FIX Application logic ===============================================================================================

#[derive(Debug)]
struct FixRecorder {
    expected_session_id: SessionId,
    session_created: AtomicUsize,
    is_logged_in: AtomicBool,
    sent_admin: AtomicUsize,
    sent_user: AtomicUsize,
    recv_admin: AtomicUsize,
    recv_user: AtomicUsize,
}

impl FixRecorder {
    fn new(expected_session_id: SessionId) -> Self {
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

    fn session_created(&self) -> usize {
        self.session_created.load(Ordering::Relaxed)
    }

    fn is_logged_in(&self) -> bool {
        self.is_logged_in.load(Ordering::Relaxed)
    }

    fn admin_msg_count(&self) -> MsgCounter {
        MsgCounter {
            sent: self.sent_admin.load(Ordering::Relaxed),
            recv: self.recv_admin.load(Ordering::Relaxed),
        }
    }

    fn user_msg_count(&self) -> MsgCounter {
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

    fn on_msg_to_app(&self, _msg: &mut Message, session_id: &SessionId) {
        self.sent_user.fetch_add(1, Ordering::Relaxed);
        assert_session_id_equals(&self.expected_session_id, &session_id);
    }

    fn on_msg_from_admin(&self, _msg: &Message, session_id: &SessionId) {
        self.recv_admin.fetch_add(1, Ordering::Relaxed);
        assert_session_id_equals(&self.expected_session_id, &session_id);
    }

    fn on_msg_from_app(&self, _msg: &Message, session_id: &SessionId) {
        self.recv_user.fetch_add(1, Ordering::Relaxed);
        assert_session_id_equals(&self.expected_session_id, &session_id);
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy)]
struct MsgCounter {
    sent: usize,
    recv: usize,
}

// Main test code 😎 ===================================================================================================

#[test]
fn test_full_fix_application() -> Result<(), QuickFixError> {
    let sender = FixRecorder::new(ServerType::Sender.session_id());
    let receiver = FixRecorder::new(ServerType::Receiver.session_id());

    // Init settings with same server port.
    let communication_port = find_available_port();
    let settings_sender = build_settings(ServerType::Sender, communication_port)?;
    let settings_receiver = build_settings(ServerType::Receiver, communication_port)?;

    let log_factory = LogFactory::try_new(&StdLogger::Stdout)?;

    let app_sender = Application::try_new(&sender)?;
    let app_receiver = Application::try_new(&receiver)?;

    let message_store_factory_sender = MemoryMessageStoreFactory::new();
    let message_store_factory_receiver = MemoryMessageStoreFactory::new();

    // Check every counter are null
    assert_eq!(sender.session_created(), 0);
    assert_eq!(receiver.session_created(), 0);
    assert_eq!(sender.admin_msg_count(), MsgCounter::default());
    assert_eq!(sender.user_msg_count(), MsgCounter::default());
    assert_eq!(receiver.admin_msg_count(), MsgCounter::default());
    assert_eq!(receiver.user_msg_count(), MsgCounter::default());

    // Init socket acceptor / initiator.
    let mut socket_sender = SocketInitiator::try_new(
        &settings_sender,
        &app_sender,
        &message_store_factory_sender,
        &log_factory,
    )?;
    let mut socket_receiver = SocketAcceptor::try_new(
        &settings_receiver,
        &app_receiver,
        &message_store_factory_receiver,
        &log_factory,
    )?;

    // Check session have been configured
    assert_eq!(sender.session_created(), 1);
    assert_eq!(receiver.session_created(), 1);

    // Check connection state = OFF
    assert!(!sender.is_logged_in());
    assert!(!socket_sender.is_logged_on().unwrap());
    assert!(!receiver.is_logged_in());
    assert!(!socket_receiver.is_logged_on().unwrap());

    // Start the app
    socket_receiver.start()?;
    socket_sender.start()?;

    // Wait for login completion
    while !sender.is_logged_in() && !receiver.is_logged_in() {
        thread::sleep(Duration::from_millis(50));
    }

    // Check connection state = ON
    assert!(sender.is_logged_in());
    assert!(socket_sender.is_logged_on().unwrap());
    assert!(receiver.is_logged_in());
    assert!(socket_receiver.is_logged_on().unwrap());

    // Check counter
    assert_eq!(sender.admin_msg_count(), MsgCounter { recv: 1, sent: 1 });

    assert_eq!(receiver.admin_msg_count(), MsgCounter { recv: 1, sent: 1 });

    // Send a message from one app to the other
    assert_eq!(sender.user_msg_count(), MsgCounter::default());
    assert_eq!(receiver.user_msg_count(), MsgCounter::default());

    let news = build_news("Hello", &[])?;
    send_to_target(news, &ServerType::Sender.session_id())?;
    thread::sleep(Duration::from_millis(50));

    assert_eq!(sender.user_msg_count(), MsgCounter { sent: 1, recv: 0 });
    assert_eq!(receiver.user_msg_count(), MsgCounter { sent: 0, recv: 1 });

    let news = build_news(
        "Anyone here",
        &["This news have", "some content", "that is very interesting"],
    )?;
    send_to_target(news, &ServerType::Receiver.session_id())?;
    thread::sleep(Duration::from_millis(50));

    assert_eq!(sender.user_msg_count(), MsgCounter { sent: 1, recv: 1 });
    assert_eq!(receiver.user_msg_count(), MsgCounter { sent: 1, recv: 1 });

    // Stop everything
    socket_receiver.stop()?;
    socket_sender.stop()?;

    // Check connection state = OFF
    assert!(!sender.is_logged_in());
    assert!(!socket_sender.is_logged_on().unwrap());
    assert!(!receiver.is_logged_in());
    assert!(!socket_receiver.is_logged_on().unwrap());

    // Check counter
    assert_eq!(
        sender.admin_msg_count(),
        MsgCounter {
            sent: 3, /* ??? */
            recv: 2
        }
    );
    assert_eq!(receiver.admin_msg_count(), MsgCounter { recv: 2, sent: 2 });

    Ok(())
}

// Settings builder ====================================================================================================

enum ServerType {
    Receiver,
    Sender,
}

impl ServerType {
    fn name(&self) -> &'static str {
        match self {
            ServerType::Receiver => "acceptor",
            ServerType::Sender => "initiator",
        }
    }

    fn session_id(&self) -> SessionId {
        match self {
            ServerType::Receiver => SessionId::try_new("FIX.4.4", "ME", "THEIR", ""),
            ServerType::Sender => SessionId::try_new("FIX.4.4", "THEIR", "ME", ""),
        }
        .expect("Fail to build session ID")
    }

    fn fill_settings(&self, params: &mut Dictionary, port: u16) -> Result<(), QuickFixError> {
        match self {
            ServerType::Receiver => {
                params.set("SocketAcceptPort", port as i32)?;
            }
            ServerType::Sender => {
                params.set("SocketConnectPort", port as i32)?;
                params.set("SocketConnectHost", "127.0.0.1".to_string())?;
            }
        }
        Ok(())
    }
}

fn build_settings(server_type: ServerType, port: u16) -> Result<SessionSettings, QuickFixError> {
    let mut settings = SessionSettings::new();

    settings.set(None, {
        let mut params = Dictionary::new();
        params.set("ConnectionType", server_type.name().to_string())?;
        params.set("ReconnectInterval", 60)?;
        params
    })?;

    settings.set(Some(server_type.session_id()), {
        let mut params = Dictionary::new();
        params.set("StartTime", "00:00:00".to_string())?;
        params.set("EndTime", "23:59:59".to_string())?;
        params.set("HeartBtInt", 20)?;
        params.set(
            "DataDictionary",
            "../quickfix-ffi/libquickfix/spec/FIX44.xml".to_string(),
        )?;
        server_type.fill_settings(&mut params, port)?;
        params
    })?;

    Ok(settings)
}

// Utils ===============================================================================================================

fn find_available_port() -> u16 {
    (8000..9000)
        .find(|port| TcpListener::bind(("127.0.0.1", *port)).is_ok())
        .expect("No port available in test range")
}

fn assert_session_id_equals(a: &SessionId, b: &SessionId) {
    assert_eq!(a.as_string(), b.as_string());
}
