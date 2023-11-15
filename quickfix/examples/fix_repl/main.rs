use std::{env, process::exit};

use quickfix::{
    Application, ConnectionHandler, FileLogFactory, FileStoreFactory, SessionSettings,
    SocketAcceptor, SocketInitiator,
};

use crate::{command_exec::FixShell, fix_app::MyApplication};

mod command_exec;
mod command_parser;
mod fix_app;

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
            let connection_handler =
                SocketInitiator::try_new(&settings, &app, &store_factory, &log_factory)
                    .expect("Fail to build connection handler");

            server_loop(connection_handler);
        }
        "acceptor" => {
            let connection_handler =
                SocketAcceptor::try_new(&settings, &app, &store_factory, &log_factory)
                    .expect("Fail to build connection handler");

            server_loop(connection_handler);
        }
        _ => {
            eprintln!("Invalid connection mode");
            exit(1);
        }
    }

    println!(">> All cleared. Bye !");
}

fn server_loop<C: ConnectionHandler>(mut connection_handler: C) {
    println!(">> connection handler START");
    connection_handler
        .start()
        .expect("Fail to start connection handler");

    let mut shell = FixShell::new();
    shell.repl(&mut connection_handler);

    println!(">> connection handler STOP");
    connection_handler
        .stop()
        .expect("Fail to start connection handler");
}
