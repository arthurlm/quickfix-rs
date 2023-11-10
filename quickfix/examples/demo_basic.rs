use std::{env, process::exit};

use quickfix::{
    files_log_factory::FileLogFactory, files_store_factory::FileStoreFactory,
    session_settings::SessionSettings,
};

fn main() {
    let args: Vec<_> = env::args().collect();
    let Some(config_file) = args.get(1) else {
        eprintln!("Bad program usage: {} <config_file>", args[0]);
        exit(1);
    };

    let settings = SessionSettings::try_new(config_file).expect("Fail to load settings");
    let _store_factory = FileStoreFactory::try_new(&settings).expect("Fail to build store factory");
    let _log_factory = FileLogFactory::try_new(&settings).expect("Fail to build log factory");
}
