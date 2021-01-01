use crate::bindings::closure_wrapper::{wrap_closure_0, wrap_closure_1};
use crate::bindings::sys::{
    emf_cbase_sys_get_function, emf_cbase_sys_get_sync_handler, emf_cbase_sys_has_function,
    emf_cbase_sys_lock, emf_cbase_sys_panic, emf_cbase_sys_shutdown, emf_cbase_sys_try_lock,
    emf_cbase_sys_unlock,
};
use crate::bindings::BASE_INTERFACE;
use crate::containers::{NonNullConst, Optional};
use crate::fn_ptr::BaseFn;
use crate::sys::{emf_cbase_sys_set_sync_handler, SyncHandlerInterface};
use crate::{BaseInterface, Bool, FnId};
use emf_core_base_rs_ffi_bare::BaseT;
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::panic::catch_unwind;
use std::ptr::null;

#[test]
fn sys_lock() {
    let mut val = false;
    let mut closure = || val = true;
    let wrapper = wrap_closure_0(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.sys_lock_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        emf_cbase_sys_lock();
        assert_eq!(val, true);
    }
}

#[test]
fn sys_try_lock() {
    let ret = Bool::False;
    let mut val = false;
    let mut closure = || {
        val = true;
        ret
    };
    let wrapper = wrap_closure_0(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.sys_try_lock_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        assert_eq!(emf_cbase_sys_try_lock(), ret);
        assert_eq!(val, true);
    }
}

#[test]
fn sys_unlock() {
    let mut val = false;
    let mut closure = || val = true;
    let wrapper = wrap_closure_0(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.sys_unlock_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        emf_cbase_sys_unlock();
        assert_eq!(val, true);
    }
}

#[test]
fn sys_shutdown() {
    let mut val = false;
    let mut closure = || {
        val = true;
        panic!()
    };
    let wrapper = wrap_closure_0(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.sys_shutdown_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = catch_unwind(|| emf_cbase_sys_shutdown());
        assert_eq!(result.is_err(), true);
        assert_eq!(val, true);
    }
}

#[test]
fn sys_panic() {
    let mut val = false;
    let mut closure = |_err: *const c_char| {
        val = true;
        panic!()
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.sys_panic_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = catch_unwind(|| emf_cbase_sys_panic(null()));
        assert_eq!(result.is_err(), true);
        assert_eq!(val, true);
    }
}

#[test]
fn sys_has_function() {
    let mut val = false;
    let mut closure = |_fn_id: FnId| {
        val = true;
        Bool::False
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.sys_has_function_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_sys_has_function(FnId::LibraryGetDataSymbol);
        assert_eq!(result, Bool::False);
        assert_eq!(val, true);
    }
}

#[test]
fn sys_get_function() {
    let mut val = false;
    let mut closure = |_fn_id: FnId| {
        val = true;
        Optional::<BaseFn>::none()
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.sys_get_function_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_sys_get_function(FnId::LibraryGetDataSymbol);
        assert_eq!(result, Optional::none());
        assert_eq!(val, true);
    }
}

#[test]
fn sys_get_sync_handler() {
    let mut val = false;
    let mut closure = || {
        val = true;
        NonNullConst::<SyncHandlerInterface>::dangling()
    };
    let wrapper = wrap_closure_0(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.sys_get_sync_handler_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_sys_get_sync_handler();
        assert_eq!(result.as_ptr(), NonNullConst::dangling().as_ptr());
        assert_eq!(val, true);
    }
}

#[test]
fn sys_set_sync_handler() {
    let mut val = false;
    let mut closure = |_: *const SyncHandlerInterface| {
        val = true;
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.sys_set_sync_handler_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        emf_cbase_sys_set_sync_handler(null());
        assert_eq!(val, true);
    }
}
