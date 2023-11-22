use std::io::{stdin, Read};

use quickfix::{
    Application, ApplicationCallback, ConnectionHandler, Dictionary, FileStoreFactory, LogFactory,
    QuickFixError, SessionId, SessionSettings, SocketAcceptor, StdLogger,
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
    let mut global_params = Dictionary::default();
    global_params.set("ConnectionType", "acceptor".to_string())?;
    global_params.set("ReconnectInterval", 60)?;
    global_params.set("FileStorePath", "store".to_string())?;

    let mut session1_params = Dictionary::default();
    session1_params.set("StartTime", "12:30:00".to_string())?;
    session1_params.set("EndTime", "23:30:00".to_string())?;
    session1_params.set("HeartBtInt", 20)?;
    session1_params.set("SocketAcceptPort", 4000)?;
    session1_params.set("DataDictionary", "libquickfix/spec/FIX41.xml".to_string())?;

    let mut settings = SessionSettings::new();
    settings.set(None, global_params)?;
    settings.set(
        Some(SessionId::try_new("FIX.4.4", "ME", "THEIR", "")?),
        session1_params,
    )?;

    Ok(settings)
}

fn main() -> Result<(), QuickFixError> {
    println!(">> Configuring application");
    let settings = build_settings()?;

    println!(">> Creating resources");
    let store_factory = FileStoreFactory::try_new(&settings)?;
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
