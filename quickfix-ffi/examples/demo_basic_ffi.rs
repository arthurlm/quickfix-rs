use std::{
    env,
    ffi::{self, CString},
    io::{stdin, Read},
    process::exit,
};

use quickfix_ffi::*;

extern "C" fn custom_on_create(data: *const ffi::c_void, session: FixSessionID_t) {
    println!("customOnCreate: {data:?} {session:?}");
}

extern "C" fn custom_on_logon(data: *const ffi::c_void, session: FixSessionID_t) {
    println!("customOnLogon: {data:?} {session:?}");
}

extern "C" fn custom_on_logout(data: *const ffi::c_void, session: FixSessionID_t) {
    println!("customOnLogout: {data:?} {session:?}");
}

extern "C" fn custom_to_admin(
    data: *const ffi::c_void,
    msg: FixMessage_t,
    session: FixSessionID_t,
) {
    println!("customToAdmin: {data:?} {msg:?} {session:?}");
}

extern "C" fn custom_to_app(data: *const ffi::c_void, msg: FixMessage_t, session: FixSessionID_t) {
    println!("customToApp: {data:?} {msg:?} {session:?}");
}

extern "C" fn custom_from_admin(
    data: *const ffi::c_void,
    msg: FixMessage_t,
    session: FixSessionID_t,
) {
    println!("customFromAdmin: {data:?} {msg:?} {session:?}");
}

extern "C" fn custom_from_app(
    data: *const ffi::c_void,
    msg: FixMessage_t,
    session: FixSessionID_t,
) {
    println!("customFromApp: {data:?} {msg:?} {session:?}");
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
        let settings = FixSessionSettings_new(config_path.as_ptr()).expect("Fail to load settings");
        let store_factory = FixFileStoreFactory_new(settings).expect("Fail to build store factory");
        let log_factory = FixFileLogFactory_new(settings).expect("Fail to build log factory");
        let application =
            FixApplication_new(0xFEED as *const ffi::c_void, std::ptr::addr_of!(callbacks))
                .expect("Fail to build application");
        let acceptor = FixSocketAcceptor_new(application, store_factory, settings, log_factory)
            .expect("Fail to build acceptor");

        println!(">> Acceptor START");
        assert_eq!(FixSocketAcceptor_start(acceptor), 0);

        println!(">> Press Q to exit");
        loop {
            let _ = stdin.read_exact(&mut stdin_buf);
            if stdin_buf[0] == b'q' {
                break;
            }
        }

        println!(">> Acceptor STOP");
        assert_eq!(FixSocketAcceptor_stop(acceptor), 0);

        println!(">> Cleaning resources");
        // FixSocketAcceptor_delete(acceptor); // FIXME
        FixApplication_delete(application);
        FixFileLogFactory_delete(log_factory);
        FixFileStoreFactory_delete(store_factory);
        FixSessionSettings_delete(settings);
    }
}
