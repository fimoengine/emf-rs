use crate::bindings::BASE_INTERFACE;
use crate::containers::{NonNullConst, Optional};
use crate::fn_ptr::BaseFn;
use crate::sys::SyncHandlerInterface;
use crate::{Bool, FnId, InterfaceBinding};
use std::os::raw::c_char;

#[cfg(test)]
mod tests;

#[no_mangle]
unsafe extern "C" fn emf_cbase_sys_lock() {
    (*BASE_INTERFACE.as_ptr()).sys_lock()
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_sys_try_lock() -> Bool {
    (*BASE_INTERFACE.as_ptr()).sys_try_lock()
}

#[no_mangle]
unsafe extern "C" fn emf_cbase_sys_unlock() {
    (*BASE_INTERFACE.as_ptr()).sys_unlock()
}

#[no_mangle]
unsafe extern "C" fn emf_cbase_sys_shutdown() -> ! {
    (*BASE_INTERFACE.as_ptr()).sys_shutdown()
}

#[no_mangle]
unsafe extern "C" fn emf_cbase_sys_panic(error: *const c_char) -> ! {
    (*BASE_INTERFACE.as_ptr()).sys_panic(error)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_sys_has_function(fn_id: FnId) -> Bool {
    (*BASE_INTERFACE.as_ptr()).sys_has_function(fn_id)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_sys_get_function(fn_id: FnId) -> Optional<BaseFn> {
    (*BASE_INTERFACE.as_ptr()).sys_get_function(fn_id)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_sys_get_sync_handler() -> NonNullConst<SyncHandlerInterface> {
    (*BASE_INTERFACE.as_ptr()).sys_get_sync_handler()
}

#[no_mangle]
unsafe extern "C" fn emf_cbase_sys_set_sync_handler(sync_handler: *const SyncHandlerInterface) {
    (*BASE_INTERFACE.as_ptr()).sys_set_sync_handler(sync_handler)
}
