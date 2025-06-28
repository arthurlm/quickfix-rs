use std::{
    env,
    io::{stdin, Read},
    process::exit,
};

use quickfix::*;

#[derive(Default)]
pub struct MyApplication;

impl ApplicationCallback for MyApplication {
    // Implement whatever callback you need

    fn on_create(&self, _session: &SessionId) {
        // Do whatever you want here 😁
    }
}

fn main() -> Result<(), QuickFixError> {
    let args: Vec<_> = env::args().collect();
    let Some(config_file) = args.get(1) else {
        eprintln!("Bad program usage: {} <config_file>", args[0]);
        exit(1);
    };

    println!(">> Creating resources");
    let settings = SessionSettings::try_from_path(config_file)?;
    let store_factory = FileMessageStoreFactory::try_new(&settings)?;
    let log_factory = LogFactory::try_new(&StdLogger::Stdout)?;
    let callbacks = MyApplication;
    let app = Application::try_new(&callbacks)?;

    let mut acceptor = Acceptor::try_new(&settings, &app, &store_factory, &log_factory)?;

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
