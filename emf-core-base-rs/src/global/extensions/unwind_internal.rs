//! The `unwind_internal` extension.
//!
//! # Example
//!
//! ```no_run
//! use emf_core_base_rs::global::extensions::unwind_internal::catch_unwind;
//!
//! let result = catch_unwind(|| {
//!     print!("hello");
//! });
//! assert!(result.is_ok());
//!
//! let result = catch_unwind(|| {
//!     panic!("oh no!");
//! });
//! assert!(result.is_err());
//! ```
use super::super::{
    get_interface as get_base_interface, get_mut_interface as get_mut_base_interface,
};
use crate::extensions::unwind_internal::{
    default_context::DefaultContext, Signal, UnwindInternalAPI, UnwindInternalContextAPI,
    UnwindInternalContextRef, UnwindInternalInterface,
};
use std::mem::MaybeUninit;
use std::panic::UnwindSafe;

static mut INTERFACE: MaybeUninit<UnwindInternalInterface<'static>> = MaybeUninit::uninit();

/// Initializes the interface.
#[inline]
pub fn initialize() {
    unsafe {
        INTERFACE = MaybeUninit::new(UnwindInternalInterface::from_interface(get_base_interface()))
    }
}

/// Fetches a reference to the interface.
#[inline]
pub fn get_interface<'a>() -> &'a UnwindInternalInterface<'static> {
    unsafe { &*INTERFACE.as_ptr() }
}

/// Fetches a mutable reference to the interface.
#[inline]
pub fn get_mut_interface<'a>() -> &'a mut UnwindInternalInterface<'static> {
    unsafe { &mut *INTERFACE.as_mut_ptr() }
}

/// Fetches the active context.
///
/// # Return
///
/// Active context.
#[inline]
pub fn get_context() -> Option<UnwindInternalContextRef> {
    get_interface().get_context(get_base_interface())
}

/// Sets the new active context.
#[inline]
pub fn set_context(context: Option<UnwindInternalContextRef>) {
    get_mut_interface().set_context(get_mut_base_interface(), context)
}

/// Sets up the unwinding for the closure `f`
///
/// Any panic or termination signal, that occurs within `f`, is propagated.
///
/// # Return
///
/// Return value from `f`.
#[inline]
pub fn setup_unwind<T>(f: impl FnOnce() -> T + UnwindSafe) -> T {
    DefaultContext::default().setup_unwind(
        get_mut_interface(),
        get_mut_base_interface(),
        move |_interface| f(),
    )
}

/// Sets up the unwinding for the closure `f`
///
/// Any panic or termination signal, that occurs within `f`, is caught and returned.
///
/// # Return
///
/// Return value from `f` or caught signal.
#[inline]
pub fn catch_unwind<T>(f: impl FnOnce() -> T + UnwindSafe) -> Result<T, Signal> {
    DefaultContext::default().catch_unwind(
        get_mut_interface(),
        get_mut_base_interface(),
        move |_interface| f(),
    )
}
