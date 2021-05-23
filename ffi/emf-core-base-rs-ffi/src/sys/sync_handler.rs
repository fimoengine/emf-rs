//! Interface of a sync handler
//!
//! Any object that can be wrapped into a [SyncHandlerInterface] can be used as a sync handler.
use crate::collections::NonNullConst;
use crate::{Bool, TypeWrapper};
use std::ptr::NonNull;

/// Opaque structure representing a sync handler.
#[repr(C)]
pub struct SyncHandler {
    _dummy: [u8; 0],
}

pub type LockFn = TypeWrapper<unsafe extern "C-unwind" fn(handler: Option<NonNull<SyncHandler>>)>;
pub type TryLockFn =
    TypeWrapper<unsafe extern "C-unwind" fn(handler: Option<NonNull<SyncHandler>>) -> Bool>;
pub type UnlockFn = TypeWrapper<unsafe extern "C-unwind" fn(handler: Option<NonNull<SyncHandler>>)>;

/// VTable of a sync handler.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SyncHandlerVTable {
    pub lock_fn: LockFn,
    pub try_lock_fn: TryLockFn,
    pub unlock_fn: UnlockFn,
}

/// Interface of a sync handler.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SyncHandlerInterface {
    pub handler: Option<NonNull<SyncHandler>>,
    pub vtable: NonNullConst<SyncHandlerVTable>,
}

unsafe impl Send for SyncHandlerInterface {}
unsafe impl Sync for SyncHandlerInterface {}

/// Helper trait for using a sync handler.
pub trait SyncHandlerBinding {
    /// Locks the synchronisation handler.
    ///
    /// The calling thread is stalled until the lock can be acquired.
    /// Only one thread can hold the lock at a time.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [SyncHandlerBinding] may break some invariants
    /// of the sys api, if not handled with care.
    unsafe fn lock(&self);

    /// Tries to lock the synchronisation handler.
    ///
    /// The function fails if another thread already holds the lock.
    ///
    /// # Return
    ///
    /// [Bool::True] on success and [Bool::False] otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [SyncHandlerBinding] may break some invariants
    /// of the sys api, if not handled with care.
    unsafe fn try_lock(&self) -> Bool;

    /// Unlocks the synchronisation handler.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Trying to call this function without prior locking is undefined behaviour.
    /// Direct usage of a [SyncHandlerBinding] may break some invariants
    /// of the sys api, if not handled with care.
    unsafe fn unlock(&self);
}

impl SyncHandlerBinding for SyncHandlerInterface {
    #[inline]
    unsafe fn lock(&self) {
        (self.vtable.as_ref().lock_fn)(self.handler)
    }

    #[inline]
    unsafe fn try_lock(&self) -> Bool {
        (self.vtable.as_ref().try_lock_fn)(self.handler)
    }

    #[inline]
    unsafe fn unlock(&self) {
        (self.vtable.as_ref().unlock_fn)(self.handler)
    }
}
