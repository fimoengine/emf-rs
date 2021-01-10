//! Utilities the manage the binding to the interface.

use crate::{ffi, BaseInterface, FFIObject};
use ffi::fn_ptr::SysGetFunctionFn;
use ffi::{BaseT, InterfaceBinding};

/// Initialize ffi bindings.
pub mod ffi_bindings {
    #[cfg(feature = "global_api")]
    pub use emf_core_base_rs_ffi::initialize_base_binding as initialize_ffi_base_binding;
    pub use emf_core_base_rs_ffi::InterfaceLoader as FFIInterfaceLoader;
}

/// A trait to initialize a binding object.
pub trait InterfaceLoader<'a> {
    /// Type of the binding object.
    type BindingT: InterfaceBinding;

    /// Initializes the binding object.
    ///
    /// # Safety
    ///
    /// The parameter `get_function_fn` must be able to accept `base_module`.
    ///
    /// # Panics
    ///
    /// This function panics if it can not initialize the binding
    unsafe fn initialize(
        base_module: *mut BaseT,
        get_function_fn: SysGetFunctionFn,
    ) -> Self::BindingT;
}

/// Initializes the binding to the `emf-core-base` interface.
///
/// Calling this is necessary if the user wishes to use a function defined
/// by the `emf-core-base` interface. Alternatively, a local object implementing the
/// [InterfaceLoader] trait, such as [BaseInterface], can be used.
///
/// # Safety
///
/// The parameter `get_function_fn` must be able to accept `base_module`.
///
/// # Panics
///
/// This function panics if it can not initialize the binding
#[cfg(feature = "global_api")]
pub unsafe fn initialize_base_binding(
    base_module: *mut BaseT,
    get_function_fn: SysGetFunctionFn,
) -> BaseInterface<'static> {
    let bindings = ffi_bindings::initialize_ffi_base_binding(base_module, get_function_fn);
    BaseInterface::from_native(bindings)
}

impl<'a> InterfaceLoader<'a> for BaseInterface<'a> {
    type BindingT = Self;

    unsafe fn initialize(
        base_module: *mut BaseT,
        get_function_fn: SysGetFunctionFn,
    ) -> Self::BindingT {
        use ffi_bindings::FFIInterfaceLoader;

        let bindings = ffi::BaseInterface::initialize(base_module, get_function_fn);
        BaseInterface::from_native(bindings)
    }
}
