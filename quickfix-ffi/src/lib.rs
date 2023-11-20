#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{ffi, ptr::NonNull};

pub type NullableCStr = Option<NonNull<ffi::c_char>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixSessionSettings_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixDataDictionary_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixFileStoreFactory_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixLogFactory_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixApplication_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixSessionID_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixMessage_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixSocketAcceptor_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixSocketInitiator_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixHeader_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixTrailer_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixGroup_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct FixApplicationCallbacks_t {
    pub onCreate: extern "C" fn(*const ffi::c_void, FixSessionID_t),
    pub onLogon: extern "C" fn(*const ffi::c_void, FixSessionID_t),
    pub onLogout: extern "C" fn(*const ffi::c_void, FixSessionID_t),
    pub toAdmin: extern "C" fn(*const ffi::c_void, FixMessage_t, FixSessionID_t),
    pub toApp: extern "C" fn(*const ffi::c_void, FixMessage_t, FixSessionID_t),
    pub fromAdmin: extern "C" fn(*const ffi::c_void, FixMessage_t, FixSessionID_t),
    pub fromApp: extern "C" fn(*const ffi::c_void, FixMessage_t, FixSessionID_t),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct FixLogCallbacks_t {
    pub onIncoming: extern "C" fn(
        data: *const ffi::c_void,
        sessionId: Option<FixSessionID_t>,
        msg: *const ffi::c_char,
    ),
    pub onOutgoing: extern "C" fn(
        data: *const ffi::c_void,
        sessionId: Option<FixSessionID_t>,
        msg: *const ffi::c_char,
    ),
    pub onEvent: extern "C" fn(
        data: *const ffi::c_void,
        sessionId: Option<FixSessionID_t>,
        msg: *const ffi::c_char,
    ),
}

#[link(name = "quickfixbind")]
extern "C" {
    pub fn FixSessionSettings_new() -> Option<FixSessionSettings_t>;
    pub fn FixSessionSettings_fromPath(
        configPath: *const ffi::c_char,
    ) -> Option<FixSessionSettings_t>;
    pub fn FixSessionSettings_delete(obj: FixSessionSettings_t);

    pub fn FixDataDictionary_new() -> Option<FixDataDictionary_t>;
    pub fn FixDataDictionary_fromPath(
        configPath: *const ffi::c_char,
    ) -> Option<FixDataDictionary_t>;
    pub fn FixDataDictionary_delete(obj: FixDataDictionary_t);

    pub fn FixFileStoreFactory_new(settings: FixSessionSettings_t)
        -> Option<FixFileStoreFactory_t>;
    pub fn FixFileStoreFactory_delete(obj: FixFileStoreFactory_t);

    pub fn FixLogFactory_new(
        data: *const ffi::c_void,
        callbacks: *const FixLogCallbacks_t,
    ) -> Option<FixLogFactory_t>;
    pub fn FixLogFactory_delete(obj: FixLogFactory_t);

    pub fn FixApplication_new(
        data: *const ffi::c_void,
        callbacks: *const FixApplicationCallbacks_t,
    ) -> Option<FixApplication_t>;
    pub fn FixApplication_delete(obj: FixApplication_t);

    pub fn FixSocketAcceptor_new(
        application: FixApplication_t,
        storeFactory: FixFileStoreFactory_t,
        settings: FixSessionSettings_t,
        logFactory: FixLogFactory_t,
    ) -> Option<FixSocketAcceptor_t>;
    #[must_use]
    pub fn FixSocketAcceptor_start(obj: FixSocketAcceptor_t) -> i8;
    #[must_use]
    pub fn FixSocketAcceptor_block(obj: FixSocketAcceptor_t) -> i8;
    #[must_use]
    pub fn FixSocketAcceptor_poll(obj: FixSocketAcceptor_t) -> i8;
    #[must_use]
    pub fn FixSocketAcceptor_stop(obj: FixSocketAcceptor_t) -> i8;
    #[must_use]
    pub fn FixSocketAcceptor_isLoggedOn(obj: FixSocketAcceptor_t) -> i8;
    #[must_use]
    pub fn FixSocketAcceptor_isStopped(obj: FixSocketAcceptor_t) -> i8;
    pub fn FixSocketAcceptor_delete(obj: FixSocketAcceptor_t);

    pub fn FixSocketInitiator_new(
        application: FixApplication_t,
        storeFactory: FixFileStoreFactory_t,
        settings: FixSessionSettings_t,
        logFactory: FixLogFactory_t,
    ) -> Option<FixSocketInitiator_t>;
    #[must_use]
    pub fn FixSocketInitiator_start(obj: FixSocketInitiator_t) -> i8;
    #[must_use]
    pub fn FixSocketInitiator_block(obj: FixSocketInitiator_t) -> i8;
    #[must_use]
    pub fn FixSocketInitiator_poll(obj: FixSocketInitiator_t) -> i8;
    #[must_use]
    pub fn FixSocketInitiator_stop(obj: FixSocketInitiator_t) -> i8;
    #[must_use]
    pub fn FixSocketInitiator_isLoggedOn(obj: FixSocketInitiator_t) -> i8;
    #[must_use]
    pub fn FixSocketInitiator_isStopped(obj: FixSocketInitiator_t) -> i8;
    pub fn FixSocketInitiator_delete(obj: FixSocketInitiator_t);

    pub fn FixSessionID_new(
        beginString: *const ffi::c_char,
        senderCompID: *const ffi::c_char,
        targetCompID: *const ffi::c_char,
        sessionQualifier: *const ffi::c_char,
    ) -> Option<FixSessionID_t>;
    pub fn FixSessionID_copy(src: FixSessionID_t) -> Option<FixSessionID_t>;
    pub fn FixSessionID_getBeginString(obj: FixSessionID_t) -> NullableCStr;
    pub fn FixSessionID_getSenderCompID(obj: FixSessionID_t) -> NullableCStr;
    pub fn FixSessionID_getTargetCompID(obj: FixSessionID_t) -> NullableCStr;
    pub fn FixSessionID_getSessionQualifier(obj: FixSessionID_t) -> NullableCStr;
    pub fn FixSessionID_isFIXT(obj: FixSessionID_t) -> i8;
    pub fn FixSessionID_toString(obj: FixSessionID_t) -> NullableCStr;
    pub fn FixSessionID_delete(obj: FixSessionID_t);

    pub fn FixMessage_new() -> Option<FixMessage_t>;
    pub fn FixMessage_fromString(text: *const ffi::c_char) -> Option<FixMessage_t>;
    pub fn FixMessage_fromStringAndDictionary(
        text: *const ffi::c_char,
        dictionary: FixDataDictionary_t,
    ) -> Option<FixMessage_t>;
    #[must_use]
    pub fn FixMessage_setField(obj: FixMessage_t, tag: i32, value: *const ffi::c_char) -> i8;
    #[must_use]
    pub fn FixMessage_getField(obj: FixMessage_t, tag: i32) -> NullableCStr;
    #[must_use]
    pub fn FixMessage_removeField(obj: FixMessage_t, tag: i32) -> i8;
    #[must_use]
    pub fn FixMessage_toBuffer(
        obj: FixMessage_t,
        buffer: *mut ffi::c_char,
        length: ffi::c_long,
    ) -> i8;
    pub fn FixMessage_delete(obj: FixMessage_t);

    pub fn FixMessage_copyHeader(obj: FixMessage_t) -> Option<FixHeader_t>;
    pub fn FixMessage_getHeaderRef(obj: FixMessage_t) -> Option<FixHeader_t>;
    pub fn FixHeader_getField(obj: FixHeader_t, tag: i32) -> NullableCStr;
    pub fn FixHeader_setField(obj: FixHeader_t, tag: i32, value: *const ffi::c_char) -> i8;
    pub fn FixHeader_removeField(obj: FixHeader_t, tag: i32) -> i8;
    pub fn FixHeader_delete(obj: FixHeader_t);

    pub fn FixMessage_copyTrailer(obj: FixMessage_t) -> Option<FixTrailer_t>;
    pub fn FixMessage_getTrailerRef(obj: FixMessage_t) -> Option<FixTrailer_t>;
    pub fn FixTrailer_getField(obj: FixTrailer_t, tag: i32) -> NullableCStr;
    pub fn FixTrailer_setField(obj: FixTrailer_t, tag: i32, value: *const ffi::c_char) -> i8;
    pub fn FixTrailer_removeField(obj: FixTrailer_t, tag: i32) -> i8;
    pub fn FixTrailer_delete(obj: FixTrailer_t);

    pub fn FixMessage_copyGroup(obj: FixMessage_t, num: i32, tag: i32) -> Option<FixGroup_t>;
    pub fn FixMessage_getGroupRef(obj: FixMessage_t, num: i32, tag: i32) -> Option<FixGroup_t>;
    pub fn FixGroup_getField(obj: FixGroup_t, tag: i32) -> NullableCStr;
    pub fn FixGroup_setField(obj: FixGroup_t, tag: i32, value: *const ffi::c_char) -> i8;
    pub fn FixGroup_removeField(obj: FixGroup_t, tag: i32) -> i8;
    pub fn FixGroup_delete(obj: FixGroup_t);

    pub fn FixSession_sendToTarget(msg: FixMessage_t, session_id: FixSessionID_t) -> i8;
}
