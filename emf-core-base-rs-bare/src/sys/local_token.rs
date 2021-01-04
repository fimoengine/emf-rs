use super::{SyncHandlerWrapper, SysToken};
use crate::ffi::Bool;
use crate::{ffi, BaseInterfaceFn, FnId};
use std::ffi::CStr;
use std::mem::ManuallyDrop;
use std::ptr::null;

/// Access token to a local interface.
pub struct LocalToken<'a, T: Sized + ffi::InterfaceBinding> {
    interface: &'a T,
}

impl<'a, T: Sized + ffi::InterfaceBinding> LocalToken<'a, T> {
    /// Creates a new `SysToken` by locking the interface.
    #[inline]
    #[must_use]
    pub fn new(interface: &'a T) -> Self {
        unsafe {
            interface.sys_lock();
            Self { interface }
        }
    }

    /// Tries to acquire a `SysToken` without blocking the tread.
    #[inline]
    #[must_use]
    pub fn try_new(interface: &'a T) -> Option<Self> {
        unsafe {
            match interface.sys_try_lock() {
                Bool::False => None,
                Bool::True => Some(Self { interface }),
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
    pub unsafe fn new_unchecked(interface: &'a T) -> ManuallyDrop<Self> {
        ManuallyDrop::new(Self { interface })
    }

    /// Retrieves a reference to the interface.
    #[inline]
    pub fn interface(&self) -> &'a T {
        self.interface
    }
}

impl<'a, T: Sized + ffi::InterfaceBinding> SysToken<'a> for LocalToken<'a, T> {
    #[inline]
    fn shutdown(&self) -> ! {
        unsafe { self.interface.sys_shutdown() }
    }

    #[inline]
    fn panic<S: AsRef<CStr>>(&self, err: Option<&S>) -> ! {
        unsafe {
            match err {
                Some(e) => self.interface.sys_panic(e.as_ref().as_ptr()),
                None => self.interface.sys_panic(null()),
            }
        }
    }

    #[inline]
    #[must_use]
    fn has_function<const ID: FnId, F: BaseInterfaceFn<{ ID }>>(&self) -> bool {
        unsafe { self.interface.sys_has_function(ID).into() }
    }

    #[inline]
    #[must_use]
    fn get_function<const ID: FnId, F: BaseInterfaceFn<{ ID }>>(&self) -> Option<F::Type> {
        unsafe {
            self.interface
                .sys_get_function(ID)
                .to_native()
                .map(|f| F::cast(f))
        }
    }

    #[inline]
    #[must_use]
    fn get_sync_handler<S: SyncHandlerWrapper<'a>>(&self) -> S {
        unsafe {
            let handler = self.interface.sys_get_sync_handler();
            S::from(&*handler.as_ptr())
        }
    }

    #[inline]
    unsafe fn set_sync_handler<S: SyncHandlerWrapper<'static>>(&self, handler: Option<S>) {
        match handler {
            None => self.interface.sys_set_sync_handler(null()),
            Some(handler) => self
                .interface
                .sys_set_sync_handler(handler.as_ref() as *const _),
        }
    }
}

impl<'a, T: Sized + ffi::InterfaceBinding> Drop for LocalToken<'a, T> {
    fn drop(&mut self) {
        unsafe {
            self.interface.sys_unlock();
        }
    }
}
