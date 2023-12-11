use std::io::{stdin, Read};

use quickfix::{
    Application, ApplicationCallback, ConnectionHandler, Dictionary, LogFactory,
    MemoryMessageStoreFactory, QuickFixError, SessionId, SessionSettings, SocketAcceptor,
    StdLogger,
};

#[derive(Default)]
pub struct MyApplication;

impl ApplicationCallback for MyApplication {
    // Implement whatever callback you need

    fn on_create(&self, _session: &SessionId) {
        // Do whatever you want here 😁
    }
}

fn build_settings() -> Result<SessionSettings, QuickFixError> {
    let mut settings = SessionSettings::new();

    settings.set(None, {
        let mut params = Dictionary::new();
        params.set("ConnectionType", "acceptor".to_string())?;
        params.set("ReconnectInterval", 60)?;
        params.set("FileStorePath", "store".to_string())?;
        params
    })?;

    settings.set(Some(SessionId::try_new("FIX.4.4", "ME", "THEIR", "")?), {
        let mut params = Dictionary::new();
        params.set("StartTime", "12:30:00".to_string())?;
        params.set("EndTime", "23:30:00".to_string())?;
        params.set("HeartBtInt", 20)?;
        params.set("SocketAcceptPort", 4000)?;
        params.set(
            "DataDictionary",
            "quickfix-ffi/libquickfix/spec/FIX41.xml".to_string(),
        )?;
        params
    })?;

    Ok(settings)
}

fn main() -> Result<(), QuickFixError> {
    println!(">> Configuring application");
    let settings = build_settings()?;

    println!(">> Creating resources");
    let store_factory = MemoryMessageStoreFactory::new();
    let log_factory = LogFactory::try_new(&StdLogger::Stdout)?;
    let callbacks = MyApplication;
    let app = Application::try_new(&callbacks)?;

    let mut acceptor = SocketAcceptor::try_new(&settings, &app, &store_factory, &log_factory)?;

    println!(">> connection handler START");
    acceptor.start()?;

    println!(">> App running, press 'q' to quit");
    let mut stdin = stdin().lock();
    let mut stdin_buf = [0];
    loop {
        let _ = stdin.read_exact(&mut stdin_buf);
        if stdin_buf[0] == b'q' {
            break;
        }
    }

    println!(">> connection handler STOP");
    acceptor.stop()?;

    println!(">> All cleared. Bye !");
    Ok(())
}
