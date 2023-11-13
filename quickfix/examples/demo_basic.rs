use std::{
    env,
    io::{self, stdin, stdout, BufRead, StdinLock, Write},
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
    let settings = SessionSettings::try_from_path(config_file).expect("Fail to load settings");
    let store_factory = FileStoreFactory::try_new(&settings).expect("Fail to build store factory");
    let log_factory = FileLogFactory::try_new(&settings).expect("Fail to build log factory");
    let callbacks = MyApplication::new("hello_FIX");
    let app = Application::try_new(&callbacks).expect("Fail to init app");

    let mut acceptor = SocketAcceptor::try_new(&settings, &app, &store_factory, &log_factory)
        .expect("Fail to build acceptor");

    println!(">> Acceptor START");
    acceptor.start().expect("Fail to start acceptor");

    println!(">> Type 'help', 'quit' for more information.");
    let mut shell = FixShell::new();
    shell.repl(&mut acceptor);

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

struct FixShell<'a> {
    stdin: StdinLock<'a>,
    last_command: String,
}

impl FixShell<'_> {
    fn new() -> Self {
        Self {
            stdin: stdin().lock(),
            last_command: String::with_capacity(1024),
        }
    }

    fn wait_user_input(&mut self) -> io::Result<()> {
        let mut stdout = stdout().lock();
        write!(stdout, "quickfix> ")?;
        stdout.flush()?;

        self.last_command.clear();
        self.stdin.read_line(&mut self.last_command)?;

        Ok(())
    }

    fn repl<C: ApplicationCallback>(&mut self, acceptor: &mut SocketAcceptor<'_, C>) {
        loop {
            self.wait_user_input().expect("I/O error");

            // Handle CTRL-D
            if self.last_command.is_empty() {
                println!("CTRL-D");
                break;
            }
            // Handle other commands
            match self.last_command.trim() {
                "quit" | "q" => break,
                "help" => {
                    println!("Available commands:");
                    println!("- status: Print Socket acceptor status");
                    println!("- start:  Start socket acceptor");
                    println!("- block:  Block socket acceptor");
                    println!("- poll:   Poll socket acceptor");
                    println!("- stop:   Stop socket acceptor");
                }
                "status" => {
                    println!(
                        "SocketAcceptorStatus: logged_on={:?}, stopped={:?}",
                        acceptor.is_logged_on(),
                        acceptor.is_stopped(),
                    );
                }
                "stop" => {
                    println!("RESULT: {:?}", acceptor.stop());
                }
                "start" => {
                    println!("RESULT: {:?}", acceptor.start());
                }
                "block" => {
                    println!("RESULT: {:?}", acceptor.block());
                }
                "poll" => {
                    println!("RESULT: {:?}", acceptor.poll());
                }
                "" => {}
                _ => {
                    eprintln!("UNKNOWN COMMAND: {}", self.last_command)
                }
            };
        }
    }
}
