use quickfix_ffi::{
    FixApplicationFactoryMessageStore_new, FixFactoryMemoryStoreCallbacks_t,
    FixFileMessageStoreFactory_new, FixMemoryMessageStoreFactory_new, FixMemoryStore_t,
    FixMessageStoreCallbacks_t, FixMessageStoreFactory_delete, FixMessageStoreFactory_t,
    FixMessageStore_new, FixMessageStore_t, FixNullMessageStoreFactory_new, FixSessionID_t,
    FixUtcTimeStamp_delete, FixUtcTimeStamp_new, FixUtcTimeStamp_t,
};
use std::alloc::{alloc, Layout};
use std::ffi;
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::os::raw::c_char;
use std::panic::catch_unwind;

use crate::utils::from_ffi_str;
use crate::{Message, QuickFixError, SessionId, SessionSettings};

#[cfg(feature = "build-with-mysql")]
pub mod mysql;

#[cfg(feature = "build-with-postgres")]
pub mod postgres;

///  Object can be converted as a foreign object representing a `MessageStore`.
pub trait FfiMessageStoreFactory {
    /// Get a representation of the message store as a FFI pointer.
    fn as_ffi_ptr(&self) -> FixMessageStoreFactory_t;
}

pub trait FfiMessageStore {
    fn as_ffi_ptr(&self) -> FixMessageStore_t;
}

/// File based implementation of `MessageStore`.
#[derive(Debug)]
pub struct FileMessageStoreFactory(FixMessageStoreFactory_t);

impl FileMessageStoreFactory {
    /// Try to create new struct from settings.
    pub fn try_new(settings: &SessionSettings) -> Result<Self, QuickFixError> {
        unsafe { FixFileMessageStoreFactory_new(settings.0) }
            .map(Self)
            .ok_or_else(QuickFixError::from_last_error)
    }
}

impl FfiMessageStoreFactory for FileMessageStoreFactory {
    fn as_ffi_ptr(&self) -> FixMessageStoreFactory_t {
        self.0
    }
}

impl Drop for FileMessageStoreFactory {
    fn drop(&mut self) {
        unsafe { FixMessageStoreFactory_delete(self.0) }
    }
}

/// In memory implementation of `MessageStore`.
#[derive(Debug)]
pub struct MemoryMessageStoreFactory(FixMessageStoreFactory_t);

impl MemoryMessageStoreFactory {
    /// Create new struct.
    pub fn new() -> Self {
        Self::default()
    }
}

impl FfiMessageStoreFactory for MemoryMessageStoreFactory {
    fn as_ffi_ptr(&self) -> FixMessageStoreFactory_t {
        self.0
    }
}

impl Default for MemoryMessageStoreFactory {
    fn default() -> Self {
        unsafe { FixMemoryMessageStoreFactory_new() }
            .map(Self)
            .expect("Fail to allocate MemoryMessageStore")
    }
}

impl Drop for MemoryMessageStoreFactory {
    fn drop(&mut self) {
        unsafe { FixMessageStoreFactory_delete(self.0) }
    }
}

/// Null implementation of MessageStore.
///
/// Will not actually store messages. Useful for admin-only or market data-only applications.
#[derive(Debug)]
pub struct NullMessageStoreFactory(FixMessageStoreFactory_t);

impl NullMessageStoreFactory {
    /// Create new struct.
    pub fn new() -> Self {
        Self::default()
    }
}

impl FfiMessageStoreFactory for NullMessageStoreFactory {
    fn as_ffi_ptr(&self) -> FixMessageStoreFactory_t {
        self.0
    }
}

impl Default for NullMessageStoreFactory {
    fn default() -> Self {
        unsafe { FixNullMessageStoreFactory_new() }
            .map(Self)
            .expect("Fail to allocate NullMessageStore")
    }
}

impl Drop for NullMessageStoreFactory {
    fn drop(&mut self) {
        unsafe { FixMessageStoreFactory_delete(self.0) }
    }
}

pub trait MessageStoreTrait {
    fn set(&mut self, seq_num: i32, message: &str) -> i8;
    fn get(&self, begin: i32, end: i32) -> Vec<String>;
    fn get_next_sender_seq_num(&self) -> i32;
    fn get_next_target_seq_num(&self) -> i32;
    fn set_next_sender_seq_num(&mut self, seq_num: i32);
    fn set_next_target_seq_num(&mut self, seq_num: i32);
    fn increment_next_sender_seq_num(&mut self);
    fn increment_next_target_seq_num(&mut self);
    fn get_creation_time(&self) -> FixUtcTimeStamp;
    fn reset(&mut self, now: &FixUtcTimeStamp);
    fn refresh(&mut self);
}

#[derive(Debug)]
pub struct MessageStore<'a, C: MessageStoreTrait>(pub(crate) FixMessageStore_t, PhantomData<&'a C>);

impl<'a, C> FfiMessageStore for MessageStore<'a, C>
where
    C: MessageStoreTrait,
{
    fn as_ffi_ptr(&self) -> FixMessageStore_t {
        self.0
    }
}

impl<'a, C> MessageStore<'a, C>
where
    C: MessageStoreTrait + 'static,
{
    /// Try create new struct from its underlying components.
    pub fn try_new(callbacks: &C) -> Result<Self, QuickFixError> {
        match unsafe {
            FixMessageStore_new(
                callbacks as *const C as *const ffi::c_void,
                &Self::CALLBACKS,
            )
        } {
            Some(fix_memory_store) => Ok(Self(fix_memory_store, PhantomData)),
            None => Err(QuickFixError::from_last_error()),
        }
    }

    const CALLBACKS: FixMessageStoreCallbacks_t = FixMessageStoreCallbacks_t {
        set: Self::set,
        get: Self::get,
        getNextSenderMsgSeqNum: Self::get_next_sender_seq_num,
        getNextTargetMsgSeqNum: Self::get_next_target_seq_num,
        setNextSenderMsgSeqNum: Self::set_next_sender_seq_num,
        setNextTargetMsgSeqNum: Self::set_next_target_seq_num,
        incrNextSenderMsgSeqNum: Self::increment_sender_seq_num,
        incrNextTargetMsgSeqNum: Self::increment_target_seq_num,
        getCreationTime: Self::creation_time,
        reset: Self::reset,
        refresh: Self::refresh,
    };

    extern "C" fn set(data: *const ffi::c_void, seq_num: i32, msg_ptr: *const ffi::c_char) -> i8 {
        catch_unwind(|| {
            let msg = unsafe { from_ffi_str(msg_ptr) };
            let this = unsafe { &mut *(data as *mut C) };
            this.set(seq_num, msg)
        })
        .unwrap_or_else(|_| 0i8)
    }

    pub extern "C" fn get(
        data: *const ffi::c_void,
        begin: i32,
        end: i32,
    ) -> *const *const ffi::c_char {
        let messages: Vec<String> = catch_unwind(|| {
            let this = unsafe { &*(data as *const C) };
            this.get(begin, end)
        })
        .unwrap_or_else(|_| vec![]);
        // Fill the vector with C-style strings
        unsafe {
            let layout =
                Layout::array::<*const c_char>(messages.len() + 1).expect("Unable to allocate");
            let strings = alloc(layout) as *mut *const c_char;
            for (i, msg) in messages.iter().enumerate() {
                let c_string = CString::new(msg.as_str()).expect("CString::new failed");
                *strings.wrapping_add(i) = c_string.into_raw();
            }
            *strings.wrapping_add(messages.len()) = std::ptr::null();
            std::mem::forget(strings);
            strings
        }
    }
    pub extern "C" fn get_next_sender_seq_num(data: *const ffi::c_void) -> i32 {
        catch_unwind(|| {
            let this = unsafe { &*(data as *const C) };
            this.get_next_sender_seq_num()
        })
        .unwrap_or_else(|_| 0i32)
    }
    pub extern "C" fn get_next_target_seq_num(data: *const ffi::c_void) -> i32 {
        catch_unwind(|| {
            let this = unsafe { &*(data as *const C) };
            this.get_next_target_seq_num()
        })
        .unwrap_or_else(|_| 0i32)
    }
    pub extern "C" fn set_next_sender_seq_num(data: *const ffi::c_void, seq_num: i32) {
        let _ = catch_unwind(|| {
            let this = unsafe { &mut *(data as *mut C) };
            this.set_next_sender_seq_num(seq_num)
        })
        .unwrap_or_else(|_| {});
    }
    pub extern "C" fn set_next_target_seq_num(data: *const ffi::c_void, seq_num: i32) {
        let _ = catch_unwind(|| {
            let this = unsafe { &mut *(data as *mut C) };
            this.set_next_target_seq_num(seq_num)
        })
        .unwrap_or_else(|_| {});
    }
    pub extern "C" fn increment_sender_seq_num(data: *const ffi::c_void) {
        let _ = catch_unwind(|| {
            let this = unsafe { &mut *(data as *mut C) };
            this.increment_next_sender_seq_num()
        })
        .unwrap_or_else(|_| {});
    }
    pub extern "C" fn increment_target_seq_num(data: *const ffi::c_void) {
        let _ = catch_unwind(|| {
            let this = unsafe { &mut *(data as *mut C) };
            this.increment_next_target_seq_num()
        })
        .unwrap_or_else(|_| {});
    }
    pub extern "C" fn creation_time(data: *const ffi::c_void) -> FixUtcTimeStamp_t {
        match catch_unwind(|| {
            let this = unsafe { &*(data as *const C) };
            this.get_creation_time()
        }) {
            Ok(res) => res.0,
            Err(_) => panic!("Unable to get creation time"),
        }
    }
    pub extern "C" fn refresh(data: *const ffi::c_void) {
        let _ = catch_unwind(|| {
            let this = unsafe { &mut *(data as *mut C) };
            this.refresh()
        })
        .unwrap_or_else(|_| {});
    }
    pub extern "C" fn reset(data: *const ffi::c_void, now: FixUtcTimeStamp_t) {
        match catch_unwind(|| {
            let this = unsafe { &mut *(data as *mut C) };
            let val = ManuallyDrop::new(FixUtcTimeStamp(now));
            this.reset(&val)
        }) {
            Ok(res) => res,
            Err(_) => panic!("Unable to get reset"),
        };
    }
}

pub struct FixUtcTimeStamp(FixUtcTimeStamp_t);
impl FixUtcTimeStamp {
    pub fn try_from(
        hour: i32,
        minute: i32,
        second: i32,
        millisecond: i32,
        day: i32,
        month: i32,
        year: i32,
    ) -> FixUtcTimeStamp {
        unsafe { FixUtcTimeStamp_new(hour, minute, second, millisecond, day, month, year) }
            .map(Self)
            .expect("Fail to allocate FixUtcTimeStamp")
    }
}

#[allow(unused_variables)]
pub trait MessageFactoryTrait<Store : MessageStoreTrait>
{
    /// New outgoing message will be sent.
    fn create(&mut self, session_id: &SessionId) -> MessageStore<Store>;

    fn on_delete(&self, store: &MessageStore<Store>);
}

#[derive(Debug)]
pub struct MessageFactory<'a, Store: MessageStoreTrait, Callbacks: MessageFactoryTrait<Store>>(
    pub(crate) FixMessageStoreFactory_t,
    PhantomData<&'a Callbacks>,
    PhantomData<&'a Store,>
);

impl<'a, Store, Callbacks> FfiMessageStoreFactory for MessageFactory<'a, Store, Callbacks>
where
    Store: MessageStoreTrait,
    Callbacks: MessageFactoryTrait<Store> + 'static,
{
    fn as_ffi_ptr(&self) -> FixMessageStoreFactory_t {
        self.0
    }
}

impl<'a, Store, Callbacks> MessageFactory<'a, Store, Callbacks>
where
    Store: MessageStoreTrait,
    Callbacks: MessageFactoryTrait<Store> + 'static,
{
    /// Try create new struct from its underlying components.
    pub fn try_new(callbacks: &'a Callbacks) -> Result<Self, QuickFixError> {
        match unsafe {
            FixApplicationFactoryMessageStore_new(
                callbacks as *const Callbacks as *const ffi::c_void,
                &Self::CALLBACKS,
            )
        } {
            Some(fix_memory_store) => Ok(Self(fix_memory_store, PhantomData, PhantomData)),
            None => Err(QuickFixError::from_last_error()),
        }
    }

    const CALLBACKS: FixFactoryMemoryStoreCallbacks_t = FixFactoryMemoryStoreCallbacks_t {
        onCreate: Self::on_create,
        onDelete: Self::on_delete,
    };

    extern "C" fn on_create(
        data: *const ffi::c_void,
        session: FixSessionID_t,
    ) -> Option<FixMessageStore_t> {
        let session_id = ManuallyDrop::new(SessionId(session));

        match catch_unwind(|| {
            let this = unsafe { &mut *(data as *mut Callbacks) };
            this.create(&session_id)
        }) {
            Ok(store) => {
                let store = ManuallyDrop::new(store);
                Some(store.as_ffi_ptr())
            }
            Err(_) => None,
        }
    }

    extern "C" fn on_delete(data: *const ffi::c_void, store: Option<FixMessageStore_t>) {
        match catch_unwind(|| {
            let this = unsafe { &*(data as *const Callbacks) };
            match store {
                Some(store) => {
                    let store = ManuallyDrop::new(store);
                    this.on_delete(&(MessageStore(*store, PhantomData)))
                }
                None => {}
            }
        }) {
            Ok(_) => {}
            Err(_) => {}
        }
    }
}
