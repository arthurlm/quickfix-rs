use std::{
    env,
    io::{stdin, Read},
    process::exit,
};

use quickfix::{
    Application, ApplicationCallback, ConnectionHandler, FileStoreFactory, LogFactory, SessionId,
    SessionSettings, SocketAcceptor, StdLogger,
};

#[derive(Default)]
pub struct MyApplication;

impl ApplicationCallback for MyApplication {
    // Implement whatever callback you need

    fn on_create(&self, _session: &SessionId) {
        // Do whatever you want here 😁
    }
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let Some(config_file) = args.get(1) else {
        eprintln!("Bad program usage: {} <config_file>", args[0]);
        exit(1);
    };

    println!(">> Creating resources");
    let settings = SessionSettings::try_from_path(config_file).expect("Fail to load settings");
    let store_factory = FileStoreFactory::try_new(&settings).expect("Fail to build store factory");
    let log_factory = LogFactory::try_new(&StdLogger::Stdout).expect("Fail to build log factory");
    let callbacks = MyApplication;
    let app = Application::try_new(&callbacks).expect("Fail to init app");

    let mut acceptor = SocketAcceptor::try_new(&settings, &app, &store_factory, &log_factory)
        .expect("Fail to build connection handler");

    println!(">> connection handler START");
    acceptor.start().expect("Fail to start connection handler");

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
    acceptor.stop().expect("Fail to start connection handler");

    println!(">> All cleared. Bye !");
}
