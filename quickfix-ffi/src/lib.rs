#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::{ffi, ptr::NonNull};

pub type NullableCStr = Option<NonNull<ffi::c_char>>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixSessionSettings_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixDictionary_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixDataDictionary_t(NonNull<ffi::c_void>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct FixMessageStoreFactory_t(NonNull<ffi::c_void>);

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

    // Error management

    pub fn Fix_getLastErrorMessage() -> Option<NonNull<ffi::c_char>>;

    pub fn Fix_getLastErrorCode() -> i8;

    pub fn Fix_clearLastErrorMessage();

    // Session settings

    pub fn FixSessionSettings_new() -> Option<FixSessionSettings_t>;

    pub fn FixSessionSettings_fromPath(
        configPath: *const ffi::c_char,
    ) -> Option<FixSessionSettings_t>;

    pub fn FixSessionSettings_getGlobalRef(obj: FixSessionSettings_t) -> Option<FixDictionary_t>;

    pub fn FixSessionSettings_getSessionRef(
        obj: FixSessionSettings_t,
        id: FixSessionID_t,
    ) -> Option<FixDictionary_t>;

    #[must_use]
    pub fn FixSessionSettings_setGlobal(obj: FixSessionSettings_t, value: FixDictionary_t) -> i8;

    #[must_use]
    pub fn FixSessionSettings_setSession(
        obj: FixSessionSettings_t,
        id: FixSessionID_t,
        value: FixDictionary_t,
    ) -> i8;

    pub fn FixSessionSettings_delete(obj: FixSessionSettings_t);

    // Dictionary

    pub fn FixDictionary_new(name: *const ffi::c_char) -> Option<FixDictionary_t>;

    #[must_use]
    pub fn FixDictionary_setString(
        obj: FixDictionary_t,
        key: *const ffi::c_char,
        value: *const ffi::c_char,
    ) -> i8;

    #[must_use]
    pub fn FixDictionary_setInt(obj: FixDictionary_t, key: *const ffi::c_char, value: i32) -> i8;

    #[must_use]
    pub fn FixDictionary_setDouble(obj: FixDictionary_t, key: *const ffi::c_char, value: f64)
        -> i8;

    #[must_use]
    pub fn FixDictionary_setBool(obj: FixDictionary_t, key: *const ffi::c_char, value: i8) -> i8;

    #[must_use]
    pub fn FixDictionary_setDay(obj: FixDictionary_t, key: *const ffi::c_char, value: i32) -> i8;

    pub fn FixDictionary_getStringLen(obj: FixDictionary_t, key: *const ffi::c_char) -> i64;

    #[must_use]
    pub fn FixDictionary_readString(
        obj: FixDictionary_t,
        key: *const ffi::c_char,
        buffer: *mut ffi::c_char,
        buffer_len: i64,
    ) -> i8;

    pub fn FixDictionary_getInt(obj: FixDictionary_t, key: *const ffi::c_char) -> i32;

    pub fn FixDictionary_getDouble(obj: FixDictionary_t, key: *const ffi::c_char) -> f64;

    pub fn FixDictionary_getBool(obj: FixDictionary_t, key: *const ffi::c_char) -> i8;

    pub fn FixDictionary_getDay(obj: FixDictionary_t, key: *const ffi::c_char) -> i32;

    pub fn FixDictionary_hasKey(obj: FixDictionary_t, key: *const ffi::c_char) -> i8;

    pub fn FixDictionary_delete(obj: FixDictionary_t);

    // Data dictionary

    pub fn FixDataDictionary_new() -> Option<FixDataDictionary_t>;

    pub fn FixDataDictionary_fromPath(
        configPath: *const ffi::c_char,
    ) -> Option<FixDataDictionary_t>;

    pub fn FixDataDictionary_delete(obj: FixDataDictionary_t);

    // Message store factory

    pub fn FixFileMessageStoreFactory_new(
        settings: FixSessionSettings_t,
    ) -> Option<FixMessageStoreFactory_t>;

    pub fn FixMemoryMessageStoreFactory_new() -> Option<FixMessageStoreFactory_t>;

    #[cfg(feature = "build-with-mysql")]
    pub fn FixMysqlMessageStoreFactory_new(
        settings: FixSessionSettings_t,
    ) -> Option<FixMessageStoreFactory_t>;

    #[cfg(feature = "build-with-postgres")]
    pub fn FixPostgresMessageStoreFactory_new(
        settings: FixSessionSettings_t,
    ) -> Option<FixMessageStoreFactory_t>;

    pub fn FixMessageStoreFactory_delete(obj: FixMessageStoreFactory_t);

    // Log factory

    pub fn FixLogFactory_new(
        data: *const ffi::c_void,
        callbacks: *const FixLogCallbacks_t,
    ) -> Option<FixLogFactory_t>;

    pub fn FixLogFactory_delete(obj: FixLogFactory_t);

    // Application

    pub fn FixApplication_new(
        data: *const ffi::c_void,
        callbacks: *const FixApplicationCallbacks_t,
    ) -> Option<FixApplication_t>;

    pub fn FixApplication_delete(obj: FixApplication_t);

    // Socket acceptor

    pub fn FixSocketAcceptor_new(
        application: FixApplication_t,
        storeFactory: FixMessageStoreFactory_t,
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

    // Socket initiator

    pub fn FixSocketInitiator_new(
        application: FixApplication_t,
        storeFactory: FixMessageStoreFactory_t,
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

    // Session ID

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

    // Message

    pub fn FixMessage_new() -> Option<FixMessage_t>;

    pub fn FixMessage_copy(src: FixMessage_t) -> Option<FixMessage_t>;

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
    pub fn FixMessage_addGroup(obj: FixMessage_t, group: FixGroup_t) -> i8;

    pub fn FixMessage_getStringLen(obj: FixMessage_t) -> i64;

    #[must_use]
    pub fn FixMessage_readString(
        obj: FixMessage_t,
        buffer: *mut ffi::c_char,
        buffer_len: i64,
    ) -> i8;

    pub fn FixMessage_delete(obj: FixMessage_t);

    // Header

    pub fn FixHeader_new() -> Option<FixHeader_t>;

    pub fn FixHeader_copy(src: FixHeader_t) -> Option<FixHeader_t>;

    pub fn FixMessage_copyHeader(obj: FixMessage_t) -> Option<FixHeader_t>;

    pub fn FixMessage_getHeaderRef(obj: FixMessage_t) -> Option<FixHeader_t>;

    pub fn FixHeader_getField(obj: FixHeader_t, tag: i32) -> NullableCStr;

    #[must_use]
    pub fn FixHeader_setField(obj: FixHeader_t, tag: i32, value: *const ffi::c_char) -> i8;

    #[must_use]
    pub fn FixHeader_removeField(obj: FixHeader_t, tag: i32) -> i8;

    #[must_use]
    pub fn FixHeader_addGroup(obj: FixHeader_t, group: FixGroup_t) -> i8;

    pub fn FixHeader_delete(obj: FixHeader_t);

    // Trailer

    pub fn FixTrailer_new() -> Option<FixTrailer_t>;

    pub fn FixTrailer_copy(src: FixTrailer_t) -> Option<FixTrailer_t>;

    pub fn FixMessage_copyTrailer(obj: FixMessage_t) -> Option<FixTrailer_t>;

    pub fn FixMessage_getTrailerRef(obj: FixMessage_t) -> Option<FixTrailer_t>;

    pub fn FixTrailer_getField(obj: FixTrailer_t, tag: i32) -> NullableCStr;

    #[must_use]
    pub fn FixTrailer_setField(obj: FixTrailer_t, tag: i32, value: *const ffi::c_char) -> i8;

    #[must_use]
    pub fn FixTrailer_removeField(obj: FixTrailer_t, tag: i32) -> i8;

    #[must_use]
    pub fn FixTrailer_addGroup(obj: FixTrailer_t, group: FixGroup_t) -> i8;

    pub fn FixTrailer_delete(obj: FixTrailer_t);

    // Group

    pub fn FixGroup_new(fieldId: i32, delim: i32, order: *const i32) -> Option<FixGroup_t>;

    pub fn FixGroup_copy(src: FixGroup_t) -> Option<FixGroup_t>;

    pub fn FixMessage_copyGroup(obj: FixMessage_t, num: i32, tag: i32) -> Option<FixGroup_t>;

    pub fn FixHeader_copyGroup(obj: FixHeader_t, num: i32, tag: i32) -> Option<FixGroup_t>;

    pub fn FixTrailer_copyGroup(obj: FixTrailer_t, num: i32, tag: i32) -> Option<FixGroup_t>;

    pub fn FixGroup_copyGroup(obj: FixGroup_t, num: i32, tag: i32) -> Option<FixGroup_t>;

    pub fn FixMessage_getGroupRef(obj: FixMessage_t, num: i32, tag: i32) -> Option<FixGroup_t>;

    pub fn FixGroup_getFieldId(obj: FixGroup_t) -> i32;

    pub fn FixGroup_getDelim(obj: FixGroup_t) -> i32;

    pub fn FixGroup_getField(obj: FixGroup_t, tag: i32) -> NullableCStr;

    #[must_use]
    pub fn FixGroup_setField(obj: FixGroup_t, tag: i32, value: *const ffi::c_char) -> i8;

    #[must_use]
    pub fn FixGroup_removeField(obj: FixGroup_t, tag: i32) -> i8;

    #[must_use]
    pub fn FixGroup_addGroup(obj: FixGroup_t, group: FixGroup_t) -> i8;

    pub fn FixGroup_delete(obj: FixGroup_t);

    // Session

    pub fn FixSession_sendToTarget(msg: FixMessage_t, session_id: FixSessionID_t) -> i8;
}
