use std::{env, process::exit};

use quickfix::{
    Acceptor, Application, ConnectionHandler, FileMessageStoreFactory, Initiator, LogFactory,
    QuickFixError, ConnectionMode, SessionSettings, StdLogger,
};

use crate::{command_exec::FixShell, fix_app::MyApplication};

mod command_exec;
mod command_parser;
mod fix_app;

fn main() -> Result<(), QuickFixError> {
    let args: Vec<_> = env::args().collect();
    let (Some(connect_mode), Some(config_file)) = (args.get(1), args.get(2)) else {
        eprintln!(
            "Bad program usage: {} [acceptor|initiator] <config_file>",
            args[0]
        );
        exit(1);
    };

    println!(">> Creating resources");
    let settings = SessionSettings::try_from_path(config_file)?;
    let store_factory = FileMessageStoreFactory::try_new(&settings)?;
    let log_factory = LogFactory::try_new(&StdLogger::Stdout)?;
    let callbacks = MyApplication::new();
    let app = Application::try_new(&callbacks)?;

    match connect_mode.as_str() {
        "initiator" => server_loop(Initiator::try_new(
            &settings,
            &app,
            &store_factory,
            &log_factory,
            ConnectionMode::SingleThreaded,
        )?),
        "acceptor" => server_loop(Acceptor::try_new(
            &settings,
            &app,
            &store_factory,
            &log_factory,
            ConnectionMode::SingleThreaded,
        )?),
        _ => {
            eprintln!("Invalid connection mode");
            exit(1);
        }
    }?;

    println!(">> All cleared. Bye !");
    Ok(())
}

fn server_loop<C: ConnectionHandler>(mut connection_handler: C) -> Result<(), QuickFixError> {
    println!(">> connection handler START");
    connection_handler.start()?;

    let mut shell = FixShell::new();
    shell.repl(&mut connection_handler);

    println!(">> connection handler STOP");
    connection_handler.stop()?;

    Ok(())
}
