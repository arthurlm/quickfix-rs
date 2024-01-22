use std::{
    env,
    ffi::{self, CString},
    io::{stdin, Read},
    process::exit,
};

use quickfix_ffi::*;

extern "C" fn custom_on_create(data: *const ffi::c_void, session: FixSessionID_t) {
    println!("custom_on_create: {data:?} {session:?}");
}

extern "C" fn custom_on_logon(data: *const ffi::c_void, session: FixSessionID_t) {
    println!("custom_on_logon: {data:?} {session:?}");
}

extern "C" fn custom_on_logout(data: *const ffi::c_void, session: FixSessionID_t) {
    println!("custom_on_logout: {data:?} {session:?}");
}

extern "C" fn custom_to_admin(
    data: *const ffi::c_void,
    msg: FixMessage_t,
    session: FixSessionID_t,
) {
    println!("custom_to_admin: {data:?} {msg:?} {session:?}");
}

extern "C" fn custom_to_app(
    data: *const ffi::c_void,
    msg: FixMessage_t,
    session: FixSessionID_t,
) -> i8 {
    println!("custom_to_app: {data:?} {msg:?} {session:?}");
    CALLBACK_OK
}

extern "C" fn custom_from_admin(
    data: *const ffi::c_void,
    msg: FixMessage_t,
    session: FixSessionID_t,
) -> i8 {
    println!("custom_from_admin: {data:?} {msg:?} {session:?}");
    CALLBACK_OK
}

extern "C" fn custom_from_app(
    data: *const ffi::c_void,
    msg: FixMessage_t,
    session: FixSessionID_t,
) -> i8 {
    println!("custom_from_app: {data:?} {msg:?} {session:?}");
    CALLBACK_OK
}

const APP_CALLBACKS: FixApplicationCallbacks_t = FixApplicationCallbacks_t {
    onCreate: custom_on_create,
    onLogon: custom_on_logon,
    onLogout: custom_on_logout,
    toAdmin: custom_to_admin,
    toApp: custom_to_app,
    fromAdmin: custom_from_admin,
    fromApp: custom_from_app,
};

extern "C" fn custom_on_incoming(
    data: *const ffi::c_void,
    session_id: Option<FixSessionID_t>,
    msg: *const ffi::c_char,
) {
    println!("custom_on_incoming: {data:?} {session_id:?} {msg:?}");
}

extern "C" fn custom_on_outgoing(
    data: *const ffi::c_void,
    session_id: Option<FixSessionID_t>,
    msg: *const ffi::c_char,
) {
    println!("custom_on_outgoing: {data:?} {session_id:?} {msg:?}");
}

extern "C" fn custom_on_event(
    data: *const ffi::c_void,
    session_id: Option<FixSessionID_t>,
    msg: *const ffi::c_char,
) {
    println!("custom_on_event: {data:?} {session_id:?} {msg:?}");
}

const LOG_CALLBACKS: FixLogCallbacks_t = FixLogCallbacks_t {
    onIncoming: custom_on_incoming,
    onOutgoing: custom_on_outgoing,
    onEvent: custom_on_event,
};

fn main() {
    let args: Vec<_> = env::args().collect();
    let Some(config_file) = args.get(1) else {
        eprintln!("Bad program usage: {} <config_file>", args[0]);
        exit(1);
    };

    let config_path =
        CString::new(config_file.as_str()).expect("Fail to convert config_file to CString");

    let mut stdin = stdin().lock();
    let mut stdin_buf = [0];

    unsafe {
        println!(">> Creating resources");
        let settings =
            FixSessionSettings_fromPath(config_path.as_ptr()).expect("Fail to load settings");
        let store_factory =
            FixFileMessageStoreFactory_new(settings).expect("Fail to build store factory");
        let log_factory = FixLogFactory_new(0xBEEF as *const ffi::c_void, &LOG_CALLBACKS)
            .expect("Fail to build log factory");
        let application = FixApplication_new(0xFEED as *const ffi::c_void, &APP_CALLBACKS)
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
        FixSocketAcceptor_delete(acceptor);
        FixApplication_delete(application);
        FixLogFactory_delete(log_factory);
        FixMessageStoreFactory_delete(store_factory);
        FixSessionSettings_delete(settings);
    }
}
