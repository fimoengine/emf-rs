//! The `unwind_internal` extension.
use crate::collections::NonNullConst;
use crate::version::ReleaseType;
use crate::{CBase, CBaseBinding};
use std::ptr::NonNull;

/// Name of the extension.
pub const UNWIND_INTERNAL_INTERFACE_NAME: &str = "emf::cbase::unwind_internal";

/// Major version.
pub const UNWIND_INTERNAL_VERSION_MAJOR: i32 = 0;

/// Minor version.
pub const UNWIND_INTERNAL_VERSION_MINOR: i32 = 1;

/// Patch version.
pub const UNWIND_INTERNAL_VERSION_PATCH: i32 = 0;

/// Release type.
pub const UNWIND_INTERNAL_VERSION_RELEASE_TYPE: ReleaseType = ReleaseType::Stable;

/// Release number.
pub const UNWIND_INTERNAL_VERSION_RELEASE_NUMBER: i8 = 0;

/// Build number.
pub const UNWIND_INTERNAL_VERSION_BUILD: i64 = 0;

/// Version string.
pub const UNWIND_INTERNAL_VERSION_STRING: &str = "0.1.0";

/// Opaque structure representing a context.
#[repr(C)]
pub struct Context {
    _dummy: [u8; 0],
}

pub type ShutdownFn = unsafe extern "C" fn(context: Option<NonNull<Context>>) -> !;
pub type PanicFn =
    unsafe extern "C" fn(context: Option<NonNull<Context>>, error: Option<NonNullConst<u8>>) -> !;

pub type SetContextFn =
    unsafe extern "C" fn(base_module: Option<NonNull<CBase>>, context: Option<NonNull<Context>>);
pub type GetContextFn =
    unsafe extern "C" fn(base_module: Option<NonNull<CBase>>) -> Option<NonNull<Context>>;
pub type SetShutdownFn =
    unsafe extern "C" fn(base_module: Option<NonNull<CBase>>, shutdown_fn: Option<ShutdownFn>);
pub type GetShutdownFn =
    unsafe extern "C" fn(base_module: Option<NonNull<CBase>>) -> Option<ShutdownFn>;
pub type SetPanicFn =
    unsafe extern "C" fn(base_module: Option<NonNull<CBase>>, shutdown_fn: Option<PanicFn>);
pub type GetPanicFn = unsafe extern "C" fn(base_module: Option<NonNull<CBase>>) -> Option<PanicFn>;

/// Extension interface.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct UnwindInternalInterface {
    pub set_context_fn: SetContextFn,
    pub get_context_fn: GetContextFn,
    pub set_shutdown_fn_fn: SetShutdownFn,
    pub get_shutdown_fn_fn: GetShutdownFn,
    pub set_panic_fn_fn: SetPanicFn,
    pub get_panic_fn_fn: GetPanicFn,
}

unsafe impl Send for UnwindInternalInterface {}
unsafe impl Sync for UnwindInternalInterface {}

/// Helper trait for using the extension.
pub trait UnwindInternalBinding<T>
where
    T: CBaseBinding,
{
    /// Sets a new unwinding context.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn set_context(&mut self, base_module: &T, context: Option<NonNull<Context>>);

    /// Fetches the current unwinding context.
    ///
    /// # Return
    ///
    /// Unwinding context.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_context(&self, base_module: &T) -> Option<NonNull<Context>>;

    /// Sets a new shutdown function.
    ///
    /// # Note
    ///
    /// Passing [Option::None] uses the default shutdown function.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn set_shutdown_fn(&mut self, base_module: &T, shutdown_fn: Option<ShutdownFn>);

    /// Fetches the current shutdown function.
    ///
    /// # Return
    ///
    /// Shutdown function.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_shutdown_fn(&self, base_module: &T) -> Option<ShutdownFn>;

    /// Sets a new panic function.
    ///
    /// # Note
    ///
    /// Passing [Option::None] uses the default panic function.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn set_panic_fn(&mut self, base_module: &T, panic_fn: Option<PanicFn>);

    /// Fetches the current panic function.
    ///
    /// # Return
    ///
    /// Shutdown function.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_panic_fn(&self, base_module: &T) -> Option<PanicFn>;
}

impl<T> UnwindInternalBinding<T> for UnwindInternalInterface
where
    T: CBaseBinding,
{
    #[inline]
    unsafe fn set_context(&mut self, base_module: &T, context: Option<NonNull<Context>>) {
        (self.set_context_fn)(base_module.base_module(), context)
    }

    #[inline]
    unsafe fn get_context(&self, base_module: &T) -> Option<NonNull<Context>> {
        (self.get_context_fn)(base_module.base_module())
    }

    #[inline]
    unsafe fn set_shutdown_fn(&mut self, base_module: &T, shutdown_fn: Option<ShutdownFn>) {
        (self.set_shutdown_fn_fn)(base_module.base_module(), shutdown_fn)
    }

    #[inline]
    unsafe fn get_shutdown_fn(&self, base_module: &T) -> Option<ShutdownFn> {
        (self.get_shutdown_fn_fn)(base_module.base_module())
    }

    #[inline]
    unsafe fn set_panic_fn(&mut self, base_module: &T, panic_fn: Option<PanicFn>) {
        (self.set_panic_fn_fn)(base_module.base_module(), panic_fn)
    }

    #[inline]
    unsafe fn get_panic_fn(&self, base_module: &T) -> Option<PanicFn> {
        (self.get_panic_fn_fn)(base_module.base_module())
    }
}

pub type GetUnwindInternalInterfaceFn =
    unsafe extern "C" fn(
        base_module: Option<NonNull<CBase>>,
    ) -> NonNullConst<UnwindInternalInterface>;
