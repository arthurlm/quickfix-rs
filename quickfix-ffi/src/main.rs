use std::{
    env,
    ffi::CString,
    io::{stdin, Read},
    process::exit,
};

use quickfix_ffi::*;

extern "C" fn custom_on_create(session: FixSessionID_t) {
    println!("customOnCreate: {session:?}");
}

extern "C" fn custom_on_logon(session: FixSessionID_t) {
    println!("customOnLogon:{session:?}");
}

extern "C" fn custom_on_logout(session: FixSessionID_t) {
    println!("customOnLogout:{session:?}");
}

extern "C" fn custom_to_admin(msg: FixMessage_t, session: FixSessionID_t) {
    println!("customToAdmin: {msg:?} {session:?}");
}

extern "C" fn custom_to_app(msg: FixMessage_t, session: FixSessionID_t) {
    println!("customToApp: {msg:?} {session:?}");
}

extern "C" fn custom_from_admin(msg: FixMessage_t, session: FixSessionID_t) {
    println!("customFromAdmin: {msg:?} {session:?}");
}

extern "C" fn custom_from_app(msg: FixMessage_t, session: FixSessionID_t) {
    println!("customFromApp: {msg:?} {session:?}");
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let Some(config_file) = args.get(1) else {
        eprintln!("Bad program usage: {} <config_file>", args[0]);
        exit(1);
    };

    let callbacks = FixApplicationCallbacks_t {
        onCreate: custom_on_create,
        onLogon: custom_on_logon,
        onLogout: custom_on_logout,
        toAdmin: custom_to_admin,
        toApp: custom_to_app,
        fromAdmin: custom_from_admin,
        fromApp: custom_from_app,
    };

    let config_path =
        CString::new(config_file.as_str()).expect("Fail to convert config_file to CString");

    let mut stdin = stdin().lock();
    let mut stdin_buf = [0];

    unsafe {
        println!(">> Creating resources");
        let settings = FixSessionSettings_new(config_path.as_ptr());
        let store_factory = FixFileStoreFactory_new(settings);
        let log_factory = FixFileLogFactory_new(settings);
        let application = FixApplication_new(std::ptr::addr_of!(callbacks));
        let acceptor = FixSocketAcceptor_new(application, store_factory, settings, log_factory);

        println!(">> Acceptor START");
        FixSocketAcceptor_start(acceptor);

        println!(">> Press Q to exit");
        loop {
            let _ = stdin.read_exact(&mut stdin_buf);
            if stdin_buf[0] == b'q' {
                break;
            }
        }

        println!(">> Acceptor STOP");
        FixSocketAcceptor_stop(acceptor);

        println!(">> Cleaning resources");
        // FixSocketAcceptor_delete(acceptor); // FIXME
        FixApplication_delete(application);
        FixFileLogFactory_delete(log_factory);
        FixFileStoreFactory_delete(store_factory);
        FixSessionSettings_delete(settings);
    }
}
