//! API of a sync handler.
use crate::ffi::sys::sync_handler::{SyncHandlerBinding, SyncHandlerInterface};
use crate::ffi::Bool;
use std::marker::PhantomData;

/// A borrowed sync handler.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct SyncHandler<'a> {
    handler: SyncHandlerInterface,
    phantom: PhantomData<&'a ()>,
}

/// The API of a sync handler.
pub trait SyncHandlerAPI<'a> {
    /// Type of the sync handler.
    type Handler;

    /// Fetches a pointer that can be used with the interface.
    fn to_raw(&self) -> SyncHandlerInterface;

    /// Construct a new instance with the pointer.
    ///
    /// # Safety
    ///
    /// This function should not be used directly.
    unsafe fn from_raw(handler: SyncHandlerInterface) -> Self::Handler;

    /// Locks the synchronisation handler.
    ///
    /// The calling thread is stalled until the lock can be acquired.
    /// Only one thread can hold the lock at a time.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [SyncHandlerAPI] may break some invariants
    /// of the sys api, if not handled with care.
    unsafe fn lock(&self);

    /// Tries to lock the synchronisation handler.
    ///
    /// The function fails if another thread already holds the lock.
    ///
    /// # Return
    ///
    /// [true] on success and [false] otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [SyncHandlerAPI] may break some invariants
    /// of the sys api, if not handled with care.
    unsafe fn try_lock(&self) -> bool;

    /// Unlocks the synchronisation handler.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Trying to call this function without prior locking is undefined behaviour.
    /// Direct usage of a [SyncHandlerAPI] may break some invariants
    /// of the sys api, if not handled with care.
    unsafe fn unlock(&self);
}

impl SyncHandlerAPI<'_> for SyncHandler<'_> {
    type Handler = Self;

    #[inline]
    fn to_raw(&self) -> SyncHandlerInterface {
        self.handler
    }

    #[inline]
    unsafe fn from_raw(handler: SyncHandlerInterface) -> Self::Handler {
        Self {
            handler,
            phantom: PhantomData,
        }
    }

    #[inline]
    unsafe fn lock(&self) {
        self.handler.lock()
    }

    #[inline]
    unsafe fn try_lock(&self) -> bool {
        self.handler.try_lock() == Bool::True
    }

    #[inline]
    unsafe fn unlock(&self) {
        self.handler.unlock()
    }
}
