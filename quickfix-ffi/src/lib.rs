#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::ffi;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FixSessionSettings_t(*const ffi::c_void);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FixFileStoreFactory_t(*const ffi::c_void);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FixFileLogFactory_t(*const ffi::c_void);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FixApplication_t(*const ffi::c_void);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FixSessionID_t(*const ffi::c_void);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FixMessage_t(*const ffi::c_void);

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct FixSocketAcceptor_t(*const ffi::c_void);

#[repr(C)]
pub struct FixApplicationCallbacks_t {
    pub onCreate: extern "C" fn(session: FixSessionID_t),
    pub onLogon: extern "C" fn(session: FixSessionID_t),
    pub onLogout: extern "C" fn(session: FixSessionID_t),
    pub toAdmin: extern "C" fn(msg: FixMessage_t, session: FixSessionID_t),
    pub toApp: extern "C" fn(msg: FixMessage_t, session: FixSessionID_t),
    pub fromAdmin: extern "C" fn(msg: FixMessage_t, session: FixSessionID_t),
    pub fromApp: extern "C" fn(msg: FixMessage_t, session: FixSessionID_t),
}

#[link(name = "quickfixbind")]
extern "C" {
    pub fn FixSessionSettings_new(configPath: *const ffi::c_char) -> FixSessionSettings_t;
    pub fn FixSessionSettings_delete(obj: FixSessionSettings_t);

    pub fn FixFileStoreFactory_new(settings: FixSessionSettings_t) -> FixFileStoreFactory_t;
    pub fn FixFileStoreFactory_delete(obj: FixFileStoreFactory_t);

    pub fn FixFileLogFactory_new(settings: FixSessionSettings_t) -> FixFileLogFactory_t;
    pub fn FixFileLogFactory_delete(obj: FixFileLogFactory_t);

    pub fn FixApplication_new(callbacks: *const FixApplicationCallbacks_t) -> FixApplication_t;
    pub fn FixApplication_delete(obj: FixApplication_t);

    pub fn FixSocketAcceptor_new(
        application: FixApplication_t,
        storeFactory: FixFileStoreFactory_t,
        settings: FixSessionSettings_t,
        logFactory: FixFileLogFactory_t,
    ) -> FixSocketAcceptor_t;
    pub fn FixSocketAcceptor_start(obj: FixSocketAcceptor_t) -> ffi::c_int;
    pub fn FixSocketAcceptor_stop(obj: FixSocketAcceptor_t) -> ffi::c_int;
    pub fn FixSocketAcceptor_delete(obj: FixSocketAcceptor_t);

}
