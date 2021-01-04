use super::{SyncHandlerWrapper, SysToken};
use crate::ffi::Bool;
use crate::{ffi, BaseInterfaceFn, FnId};
use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem::ManuallyDrop;
use std::ptr::null;

/// Access token to the global interface.
pub struct GlobalToken<'a> {
    _phantom: PhantomData<&'a ()>,
}

impl<'a> GlobalToken<'a> {
    /// Creates a new `SysToken` by locking the interface.
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        unsafe {
            ffi::sys::emf_cbase_sys_lock();
            Self {
                _phantom: PhantomData,
            }
        }
    }

    /// Tries to acquire a `SysToken` without blocking the tread.
    #[inline]
    #[must_use]
    pub fn try_new() -> Option<Self> {
        unsafe {
            match ffi::sys::emf_cbase_sys_try_lock() {
                Bool::False => None,
                Bool::True => Some(Self {
                    _phantom: PhantomData,
                }),
            }
        }
    }

    /// Constructs a new `SysToken` without locking the interface.
    ///
    /// # Safety
    ///
    /// The developer must be certain that the interface is already locked
    /// before calling this function.
    ///
    /// # Uses
    ///
    /// This function is useful for creating a `SysToken` in a callback, when
    /// it is guaranteed that the system is already locked. This is the case with
    /// the module callbacks from the module api.
    #[inline]
    #[must_use]
    pub unsafe fn new_unchecked() -> ManuallyDrop<Self> {
        ManuallyDrop::new(Self {
            _phantom: PhantomData,
        })
    }
}

impl<'a> SysToken<'a> for GlobalToken<'a> {
    #[inline]
    fn shutdown(&self) -> ! {
        unsafe { ffi::sys::emf_cbase_sys_shutdown() }
    }

    #[inline]
    fn panic<T: AsRef<CStr>>(&self, err: Option<&T>) -> ! {
        unsafe {
            match err {
                Some(e) => ffi::sys::emf_cbase_sys_panic(e.as_ref().as_ptr()),
                None => ffi::sys::emf_cbase_sys_panic(null()),
            }
        }
    }

    #[inline]
    #[must_use]
    fn has_function<const ID: FnId, T: BaseInterfaceFn<{ ID }>>(&self) -> bool {
        unsafe { ffi::sys::emf_cbase_sys_has_function(ID).into() }
    }

    #[inline]
    #[must_use]
    fn get_function<const ID: FnId, T: BaseInterfaceFn<{ ID }>>(&self) -> Option<T::Type> {
        unsafe {
            ffi::sys::emf_cbase_sys_get_function(ID)
                .to_native()
                .map(|f| T::cast(f))
        }
    }

    #[inline]
    #[must_use]
    fn get_sync_handler<T: SyncHandlerWrapper<'a>>(&self) -> T {
        unsafe {
            let handler = ffi::sys::emf_cbase_sys_get_sync_handler();
            T::from(&*handler.as_ptr())
        }
    }

    #[inline]
    unsafe fn set_sync_handler<T: SyncHandlerWrapper<'static>>(&self, handler: Option<T>) {
        match handler {
            None => ffi::sys::emf_cbase_sys_set_sync_handler(null()),
            Some(handler) => ffi::sys::emf_cbase_sys_set_sync_handler(handler.as_ref() as *const _),
        }
    }
}

impl Default for GlobalToken<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for GlobalToken<'_> {
    fn drop(&mut self) {
        unsafe { ffi::sys::emf_cbase_sys_unlock() }
    }
}
