//! Definition of the `sys` api.

use crate::containers::Optional;
use crate::fn_ptr::BaseFn;
use crate::{Bool, FnId};
use std::os::raw::c_char;

/// An opaque structure representing a synchronisation handler.
#[repr(C)]
pub struct SyncHandler {
    _private: [u8; 0],
}

/// A function pointer to a `lock` function.
///
/// The lock function must have the following properties:
///
/// - The `emf-core-base` interface must be locked exclusively.
/// - In case the interface is already locked, the thread must be stalled until a lock can occur.
/// - The `emf-core-base` interface must be unlockable with the associated unlock function.
pub type SyncHandlerLockFn = extern "C" fn(sync_handler: *mut SyncHandler);

/// A function pointer to a `try-lock` function.
///
/// The try-lock function must have the following properties:
///
/// - The function must succeed if the interface is in an unlocked state and fail otherwise.
/// - On success, the `emf-core-base` interface must be locked exclusively and `emf_cbase_bool_true` must be returned.
/// - On failure, `emf_cbase_bool_false` must be returned.
/// - In case of failure, the calling thread may not be stalled.
/// - The `emf-core-base` interface must be unlockable with the associated unlock function.
pub type SyncHandlerTryLockFn = extern "C" fn(sync_handler: *mut SyncHandler) -> Bool;

/// A function pointer to an `unlock` function.
///
/// The unlock function must have the following properties:
///
/// - The `emf-core-base` interface must be unlocked if, at the time of calling, it is an a locked state.
pub type SyncHandlerUnlockFn = extern "C" fn(sync_handler: *mut SyncHandler);

/// The interface of a synchronisation handler.
///
/// A synchronisation handler manages the concurrent access of the `emf-core-base` interface.
///
/// # Default handler
///
/// The default synchronisation handler models a non-recursive, unique lock.
/// Once locked, the caller has safe access to the whole interface.
/// Locking twice will result in a deadlock whereas unlocking twice is undefined behaviour.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct SyncHandlerInterface {
    pub sync_handler: *mut SyncHandler,
    pub lock_fn: SyncHandlerLockFn,
    pub try_lock_fn: SyncHandlerTryLockFn,
    pub unlock_fn: SyncHandlerUnlockFn,
}

extern "C" {

    /// Locks the interface.
    ///
    /// The calling thread is stalled until the lock can be acquired.
    /// Only one thread can hold the lock at a time.
    ///
    /// # Deadlock
    ///
    /// Calling this function while the calling thread holds a lock may result in a deadlock.
    pub fn emf_cbase_sys_lock();

    /// Tries to lock the interface.
    ///
    /// The function fails if another thread already holds the lock.
    /// The result is `emf_cbase_bool_true` on success and `emf_cbase_bool_false` otherwise.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock (See [emf_cbase_sys_lock()]).
    #[must_use]
    pub fn emf_cbase_sys_try_lock() -> Bool;

    /// Unlocks the interface.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock (See [emf_cbase_sys_lock()]).
    pub fn emf_cbase_sys_unlock();

    /// Sends a termination signal.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock (See [emf_cbase_sys_lock()]).
    pub fn emf_cbase_sys_shutdown() -> !;

    /// Panics.
    ///
    /// Execution of the program is stopped abruptly after the error is logged.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock (See [emf_cbase_sys_lock()]).
    pub fn emf_cbase_sys_panic(error: *const c_char) -> !;

    /// Checks if a function is implemented.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock (See [emf_cbase_sys_lock()]).
    #[must_use]
    pub fn emf_cbase_sys_has_function(fn_id: FnId) -> Bool;

    /// Retrieves the function pointer to the function.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock (See [emf_cbase_sys_lock()]).
    #[must_use]
    pub fn emf_cbase_sys_get_function(fn_id: FnId) -> Optional<BaseFn>;

    /// Fetches the current synchronisation handler.
    ///
    /// The result of this call will never be `NULL`.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock (See [emf_cbase_sys_lock()]).
    #[must_use]
    pub fn emf_cbase_sys_get_sync_handler() -> *const SyncHandlerInterface;

    /// Sets a new synchronisation handler.
    ///
    /// The default synchronisation handler is used, if `sync_handler` is `NULL`.
    ///
    /// # Uses
    ///
    /// This function can be used by modules, that want to provide a more complex
    /// synchronisation mechanism than the one presented by the default handler.
    ///
    /// # Swapping
    ///
    /// The swapping occurs in three steps:
    ///
    /// 1. The new synchronisation handler is locked.
    /// 2. The new synchronisation handler is set as the main synchronisation handler.
    /// 3. The old synchronisation handler is unlocked.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock (See [emf_cbase_sys_lock()]).
    ///
    /// # Warning
    ///
    /// Changing the synchronisation handler may break some modules, if
    /// they depend on a specific synchronisation handler.
    pub fn emf_cbase_sys_set_sync_handler(sync_handler: *const SyncHandlerInterface);
}
