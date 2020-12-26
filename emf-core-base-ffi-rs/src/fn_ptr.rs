//! Function types specified by the `emf-core-base` interface.

use crate::containers::Optional;
use crate::sys::SyncHandlerInterface;
use crate::{BaseT, Bool, FnId};
use std::os::raw::c_char;

/// A type-erased function pointer.
pub type BaseFn = extern "C" fn();

/// Function pointer to the `emf_cbase_sys_lock` function.
pub type SysLockFn = extern "C" fn(base_module: *mut BaseT);

/// Function pointer to the `emf_cbase_sys_try_lock` function.
pub type SysTryLockFn = extern "C" fn(base_module: *mut BaseT) -> Bool;

/// Function pointer to the `emf_cbase_sys_unlock` function.
pub type SysUnlockFn = extern "C" fn(base_module: *mut BaseT);

/// Function pointer to the `emf_cbase_sys_shutdown` function.
pub type SysShutdownFn = extern "C" fn(base_module: *mut BaseT) -> !;

/// Function pointer to the `emf_cbase_sys_panic` function.
pub type SysPanicFn = extern "C" fn(base_module: *mut BaseT, error: *const c_char) -> !;

/// Function pointer to the `emf_cbase_sys_has_function` function.
pub type SysHasFunctionFn = extern "C" fn(base_module: *mut BaseT, fn_id: FnId) -> Bool;

/// Function pointer to the `emf_cbase_sys_get_function` function.
pub type SysGetFunctionFn = extern "C" fn(base_module: *mut BaseT, fn_id: FnId) -> Optional<BaseFn>;

/// Function pointer to the `emf_cbase_sys_get_sync_handler` function.
pub type SysGetSyncHandlerFn =
    extern "C" fn(base_module: *mut BaseT) -> *const SyncHandlerInterface;

/// Function pointer to the `emf_cbase_sys_set_sync_handler` function.
pub type SysSetSyncHandlerFn =
    extern "C" fn(base_module: *mut BaseT, sync_handler: *const SyncHandlerInterface);
