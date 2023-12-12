use std::{thread, time::Duration};

use quickfix::*;
use utils::*;

mod utils;

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
