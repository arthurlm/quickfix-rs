use std::{thread, time::Duration};

use quickfix::*;
use utils::*;

mod utils;

#[test]
fn test_session_login_logout() -> Result<(), QuickFixError> {
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

    // Start the app
    socket_receiver.start()?;
    socket_sender.start()?;

    // Wait for login completion
    while !sender.is_logged_in() && !receiver.is_logged_in() {
        thread::sleep(Duration::from_millis(50));
    }

    // Lookup session
    let mut session = unsafe { Session::lookup(&ServerType::Receiver.session_id()) }.unwrap();

    // Play with session state
    assert!(session.is_logged_on().unwrap());

    // Logout
    session.logout().unwrap();
    while session.is_logged_on().unwrap() {
        thread::sleep(Duration::from_millis(50));
    }

    // Stop everything
    socket_receiver.stop()?;
    socket_sender.stop()?;

    Ok(())
}
