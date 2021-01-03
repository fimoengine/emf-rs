//! Definition of `BaseInterfaceFn`

use crate::ffi;
use crate::FnId;

/// An utility trait to identify function pointer types.
pub trait BaseInterfaceFn<const ID: FnId> {
    type Type;

    /// Casts the function to the right type.
    ///
    /// # Safety
    ///
    /// Casting a pointer is inherently unsafe.
    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type;
}

impl BaseInterfaceFn<{ FnId::SysLock }> for ffi::fn_ptr::SysLockFn {
    type Type = ffi::fn_ptr::SysLockFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::SysTryLock }> for ffi::fn_ptr::SysTryLockFn {
    type Type = ffi::fn_ptr::SysTryLockFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::SysUnlock }> for ffi::fn_ptr::SysUnlockFn {
    type Type = ffi::fn_ptr::SysUnlockFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}
