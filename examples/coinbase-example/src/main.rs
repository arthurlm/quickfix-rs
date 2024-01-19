use std::{
    io::{stdin, Read},
    thread,
    time::Duration,
};

use coinbase_example::*;
use coinbase_fix42::{field_types::MsgType, *};
use coinbase_fix_utils::{config::CoinbaseConfig, logon_utils};
use quickfix::*;

pub struct MyApplication {
    config: CoinbaseConfig,
}

impl MyApplication {
    fn from_env() -> Self {
        Self {
            config: CoinbaseConfig::from_env(),
        }
    }
}

impl ApplicationCallback for MyApplication {
    fn on_msg_to_admin(&self, msg: &mut Message, _session: &SessionId) {
        // Intercept login message automatically sent by quickfix library
        if msg.with_header(|h| h.get_field(field_id::MSG_TYPE))
            == Some(MsgType::Logon.as_fix_value())
        {
            // Complete missing required fields.
            logon_utils::fill_message(msg, &self.config).expect("Fail to complete logon message");

            // Sign message.
            logon_utils::sign(msg, &self.config).expect("Fail to sign logon message");
        }
    }
}

fn main() -> anyhow::Result<()> {
    // Init our callbacks.
    let my_app = MyApplication::from_env();

    // Init FIX engine.
    let settings = build_session_settings(&my_app.config)?;
    let store_factory = MemoryMessageStoreFactory::new(); // Coinbase do not have FIX replay enabled.
    let log_factory = LogFactory::try_new(&StdLogger::Stdout)?;
    let app = Application::try_new(&my_app)?;

    let mut acceptor = SocketInitiator::try_new(&settings, &app, &store_factory, &log_factory)?;

    // Start the engine.
    acceptor.start()?;

    // Wait for login sequence completion
    while !acceptor.is_logged_on()? {
        thread::sleep(Duration::from_millis(250));
    }
    println!("We are now logged on. Let's trade 🎇");

    // Start main app loop.
    println!(">> App running, press 'q' to quit");
    let mut stdin = stdin().lock();
    let mut stdin_buf = [0];
    loop {
        let _ = stdin.read_exact(&mut stdin_buf);
        if stdin_buf[0] == b'q' {
            break;
        }
    }

    // Stop and close everything.
    acceptor.stop()?;
    Ok(())
}
