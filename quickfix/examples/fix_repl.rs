use std::{
    env,
    io::{self, stdin, stdout, BufRead, StdinLock, Write},
    process::exit,
    sync::atomic::{AtomicU32, Ordering},
};

use colored::Colorize;
use quickfix::{
    Application, ApplicationCallback, ConnectionHandler, FileLogFactory, FileStoreFactory, Message,
    SessionId, SessionSettings, SocketAcceptor, SocketInitiator,
};

fn main() {
    let args: Vec<_> = env::args().collect();
    let (Some(connect_mode), Some(config_file)) = (args.get(1), args.get(2)) else {
        eprintln!(
            "Bad program usage: {} [acceptor|initiator] <config_file>",
            args[0]
        );
        exit(1);
    };

    println!(">> Creating resources");
    let settings = SessionSettings::try_from_path(config_file).expect("Fail to load settings");
    let store_factory = FileStoreFactory::try_new(&settings).expect("Fail to build store factory");
    let log_factory = FileLogFactory::try_new(&settings).expect("Fail to build log factory");
    let callbacks = MyApplication::new();
    let app = Application::try_new(&callbacks).expect("Fail to init app");

    match connect_mode.as_str() {
        "initiator" => {
            let mut connection_handler =
                SocketInitiator::try_new(&settings, &app, &store_factory, &log_factory)
                    .expect("Fail to build connection handler");

            server_loop(&mut connection_handler);
        }
        "acceptor" => {
            let mut connection_handler =
                SocketAcceptor::try_new(&settings, &app, &store_factory, &log_factory)
                    .expect("Fail to build connection handler");

            server_loop(&mut connection_handler);
        }
        _ => {
            eprintln!("Invalid connection mode");
            exit(1);
        }
    }
}

fn server_loop<C: ConnectionHandler>(connection_handler: &mut C) {
    println!(">> connection handler START");
    connection_handler
        .start()
        .expect("Fail to start connection handler");

    let mut shell = FixShell::new();
    shell.repl(connection_handler);

    println!(">> connection handler STOP");
    connection_handler
        .stop()
        .expect("Fail to start connection handler");
}

#[derive(Default)]
pub struct MyApplication {
    message_index: AtomicU32,
}

impl MyApplication {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn inc_message_index(&self) {
        self.message_index.fetch_add(1, Ordering::Relaxed);
    }

    pub fn print_callback(&self, callback_name: &str, session: &SessionId, msg: Option<&Message>) {
        let msg_count = self.message_index.load(Ordering::Relaxed);

        let mut stdout = stdout().lock();
        let _ = write!(stdout, "{}", callback_name.blue().bold());
        let _ = write!(stdout, "({}={}) ", "id".green(), msg_count);
        let _ = write!(stdout, "{}={:?}", "session".green(), session);
        if let Some(msg) = msg {
            let _ = write!(stdout, " {}={:?}", "msg".green(), msg);
        }
        let _ = writeln!(stdout);
    }
}

impl ApplicationCallback for MyApplication {
    fn on_create(&self, session: &SessionId) {
        self.print_callback("on_create", session, None);
    }

    fn on_logon(&self, session: &SessionId) {
        self.print_callback("on_logon", session, None);
    }

    fn on_logout(&self, session: &SessionId) {
        self.print_callback("on_logout", session, None);
    }

    fn on_msg_to_admin(&self, msg: &mut Message, session: &SessionId) {
        self.inc_message_index();
        self.print_callback("to_admin", session, Some(msg));
    }

    fn on_msg_to_app(&self, msg: &mut Message, session: &SessionId) {
        self.inc_message_index();
        self.print_callback("to_app", session, Some(msg));
    }

    fn on_msg_from_admin(&self, msg: &Message, session: &SessionId) {
        self.inc_message_index();
        self.print_callback("from_admin", session, Some(msg));
    }

    fn on_msg_from_app(&self, msg: &Message, session: &SessionId) {
        self.inc_message_index();
        self.print_callback("from_app", session, Some(msg));
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
        write!(stdout, "{}> ", "FIX".blue())?;
        stdout.flush()?;
        drop(stdout);

        self.last_command.clear();
        self.stdin.read_line(&mut self.last_command)?;

        Ok(())
    }

    fn repl<C: ConnectionHandler>(&mut self, connection_handler: &mut C) {
        println!(">> Type 'help' or '?' for more information, 'quit' or 'q' to exit.");

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
                "help" | "?" => {
                    println!("Available commands:");
                    println!("- status              : Print connection handler status");
                    println!("- start               : Start connection handler");
                    println!("- block               : Block connection handler");
                    println!("- poll                : Poll connection handler");
                    println!("- stop                : Stop connection handler");
                    println!("- msg K1=V1 K2=V2 ... : Create new FIX message");
                }
                "status" => {
                    println!(
                        "Connection handler status: logged_on={:?}, stopped={:?}",
                        connection_handler.is_logged_on(),
                        connection_handler.is_stopped(),
                    );
                }
                "stop" => {
                    println!("RESULT: {:?}", connection_handler.stop());
                }
                "start" => {
                    println!("RESULT: {:?}", connection_handler.start());
                }
                "block" => {
                    println!("RESULT: {:?}", connection_handler.block());
                }
                "poll" => {
                    println!("RESULT: {:?}", connection_handler.poll());
                }
                "" => {}
                cmd if cmd.starts_with("msg ") => {
                    let mut msg = Message::try_new().expect("Fail to create FIX message");

                    for token in cmd.split_whitespace() {
                        let mut token_iter = token.trim().splitn(2, '=');
                        let fix_tag: i32 = token_iter.next().unwrap_or("0").parse().unwrap_or(0);
                        let fix_val = token_iter.next().unwrap_or("N/A");

                        msg.set_field(fix_tag, fix_val)
                            .expect("Fail to set FIX value");
                    }

                    println!("Message: {msg:?}");
                }
                cmd => {
                    eprintln!("UNKNOWN COMMAND: {cmd}")
                }
            };
        }
    }
}
