//! Sys api.
//!
//! The sys api is exposed by the [SysBinding] trait.
use crate::collections::{NonNullConst, Optional};
use crate::sys::sync_handler::SyncHandlerInterface;
use crate::{Bool, CBase, CBaseFn, CBaseInterface, FnId, TypeWrapper};
use std::ptr::NonNull;

pub type ShutdownFn =
    TypeWrapper<unsafe extern "C-unwind" fn(base_module: Option<NonNull<CBase>>) -> !>;

pub type PanicFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        error: Option<NonNullConst<u8>>,
    ) -> !,
>;

pub type HasFunctionFn =
    TypeWrapper<unsafe extern "C-unwind" fn(base_module: Option<NonNull<CBase>>, id: FnId) -> Bool>;

pub type GetFunctionFn = TypeWrapper<
    unsafe extern "C-unwind" fn(base_module: Option<NonNull<CBase>>, id: FnId) -> Optional<CBaseFn>,
>;

pub type LockFn = TypeWrapper<unsafe extern "C-unwind" fn(base_module: Option<NonNull<CBase>>)>;

pub type TryLockFn =
    TypeWrapper<unsafe extern "C-unwind" fn(base_module: Option<NonNull<CBase>>) -> Bool>;

pub type UnlockFn = TypeWrapper<unsafe extern "C-unwind" fn(base_module: Option<NonNull<CBase>>)>;

pub type GetSyncHandlerFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
    ) -> NonNullConst<SyncHandlerInterface>,
>;

pub type SetSyncHandlerFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handler: Option<NonNullConst<SyncHandlerInterface>>,
    ),
>;

/// Helper trait for using the sys api.
pub trait SysBinding {
    /// Sends a termination signal.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn shutdown(&mut self) -> !;

    /// Execution of the program is stopped abruptly. The error may be logged.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn panic(&self, error: Option<NonNullConst<u8>>) -> !;

    /// Checks if a function is implemented.
    ///
    /// # Return
    ///
    /// [Bool::True] if the function exists, [Bool::False] otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn has_function(&self, id: FnId) -> Bool;

    /// Fetches a function from the interface.
    ///
    /// # Return
    ///
    /// Function pointer to the requested function, if it exists.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn get_function(&self, id: FnId) -> Optional<CBaseFn>;

    /// Locks the interface.
    ///
    /// The calling thread is stalled until the lock can be acquired.
    /// Only one thread can hold the lock at a time.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn lock(&self);

    /// Tries to lock the interface.
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
    unsafe fn try_lock(&self) -> Bool;

    /// Unlocks the interface.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Trying to call this function without prior locking is undefined behaviour.
    unsafe fn unlock(&self);

    /// Fetches the active synchronization handler.
    ///
    /// # Return
    ///
    /// Pointer to the active synchronization handler.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_sync_handler(&self) -> NonNullConst<SyncHandlerInterface>;

    /// Sets a new synchronization handler.
    ///
    /// The default synchronization handler is used, if `handler` is [Option::None].
    ///
    /// # Uses
    ///
    /// This function can be used by modules, that want to provide a more complex
    /// synchronization mechanism than the one presented by the default handler.
    ///
    /// # Swapping
    ///
    /// The swapping occurs in three steps:
    ///
    /// 1. The new synchronization handler is locked.
    /// 2. The new synchronization handler is set as the active synchronization handler.
    /// 3. The old synchronization handler is unlocked.
    ///
    /// # Note
    ///
    /// Changing the synchronization handler may break some modules,
    /// if they depend on a specific synchronization handler.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn set_sync_handler(&mut self, handler: Option<NonNullConst<SyncHandlerInterface>>);
}

impl SysBinding for CBaseInterface {
    #[inline]
    unsafe fn shutdown(&mut self) -> ! {
        (self.sys_shutdown_fn)(self.base_module)
    }

    #[inline]
    unsafe fn panic(&self, error: Option<NonNullConst<u8>>) -> ! {
        (self.sys_panic_fn)(self.base_module, error)
    }

    #[inline]
    unsafe fn has_function(&self, id: FnId) -> Bool {
        (self.sys_has_function_fn)(self.base_module, id)
    }

    #[inline]
    unsafe fn get_function(&self, id: FnId) -> Optional<CBaseFn> {
        (self.sys_get_function_fn)(self.base_module, id)
    }

    #[inline]
    unsafe fn lock(&self) {
        (self.sys_lock_fn)(self.base_module)
    }

    #[inline]
    unsafe fn try_lock(&self) -> Bool {
        (self.sys_try_lock_fn)(self.base_module)
    }

    #[inline]
    unsafe fn unlock(&self) {
        (self.sys_unlock_fn)(self.base_module)
    }

    #[inline]
    unsafe fn get_sync_handler(&self) -> NonNullConst<SyncHandlerInterface> {
        (self.sys_get_sync_handler_fn)(self.base_module)
    }

    #[inline]
    unsafe fn set_sync_handler(&mut self, handler: Option<NonNullConst<SyncHandlerInterface>>) {
        (self.sys_set_sync_handler_fn)(self.base_module, handler)
    }
}
