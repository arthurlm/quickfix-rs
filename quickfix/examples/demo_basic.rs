use std::{
    env,
    io::{stdin, Read},
    process::exit,
    sync::atomic::{AtomicU32, Ordering},
};

use quickfix::{
    Application, ApplicationCallback, FileLogFactory, FileStoreFactory, Message, SessionId,
    SessionSettings, SocketAcceptor,
};

fn main() {
    let args: Vec<_> = env::args().collect();
    let Some(config_file) = args.get(1) else {
        eprintln!("Bad program usage: {} <config_file>", args[0]);
        exit(1);
    };

    println!(">> Creating resources");
    let settings = SessionSettings::try_new(config_file).expect("Fail to load settings");
    let store_factory = FileStoreFactory::try_new(&settings).expect("Fail to build store factory");
    let log_factory = FileLogFactory::try_new(&settings).expect("Fail to build log factory");
    let app = Application::try_new(MyApplication::new("hello_FIX")).expect("Fail to init app");

    let acceptor = SocketAcceptor::try_new(&settings, &app, &store_factory, &log_factory)
        .expect("Fail to build acceptor");

    println!(">> Acceptor START");
    acceptor.start().expect("Fail to start acceptor");

    println!(">> Press Q to exit");
    let mut stdin = stdin().lock();
    let mut stdin_buf = [0];
    loop {
        let _ = stdin.read_exact(&mut stdin_buf);
        if stdin_buf[0] == b'q' {
            break;
        }
    }

    println!(">> Acceptor STOP");
    acceptor.stop().expect("Fail to start acceptor");
}

pub struct MyApplication<'a> {
    name: &'a str,
    active_session: AtomicU32,
}

impl<'a> MyApplication<'a> {
    pub fn new(name: &'a str) -> Self {
        Self {
            name,
            active_session: AtomicU32::new(0),
        }
    }
}

impl ApplicationCallback for MyApplication<'_> {
    fn on_create(&self, session: &SessionId) {
        println!(
            "on_create: {session:?}: {} {}",
            self.name,
            self.active_session.load(Ordering::Relaxed)
        );
        println!(
            "Session info: {:?}, {:?}->{:?}, qual={:?} FIXT={}",
            session.get_begin_string(),
            session.get_sender_comp_id(),
            session.get_target_comp_id(),
            session.get_session_qualifier(),
            session.is_fixt()
        )
    }

    fn on_logon(&self, session: &SessionId) {
        self.active_session.fetch_add(1, Ordering::Relaxed);

        println!(
            "on_logon: {session:?}: {} {}",
            self.name,
            self.active_session.load(Ordering::Relaxed)
        );
    }

    fn on_logout(&self, session: &SessionId) {
        self.active_session.fetch_sub(1, Ordering::Relaxed);

        println!(
            "on_logout: {session:?}: {} {}",
            self.name,
            self.active_session.load(Ordering::Relaxed)
        );
    }

    fn on_msg_to_admin(&self, msg: &mut Message, session: &SessionId) {
        println!(
            "to_admin: {msg:?} {session:?}: {} {}",
            self.name,
            self.active_session.load(Ordering::Relaxed)
        );
    }

    fn on_msg_to_app(&self, msg: &mut Message, session: &SessionId) {
        println!(
            "to_app: {msg:?} {session:?}: {} {}",
            self.name,
            self.active_session.load(Ordering::Relaxed)
        );
    }

    fn on_msg_from_admin(&self, msg: &Message, session: &SessionId) {
        println!(
            "from_admin: {msg:?} {session:?}: {} {}",
            self.name,
            self.active_session.load(Ordering::Relaxed)
        );
    }

    fn on_msg_from_app(&self, msg: &Message, session: &SessionId) {
        println!(
            "from_app: {msg:?} {session:?}: {} {}",
            self.name,
            self.active_session.load(Ordering::Relaxed)
        );
    }
}
