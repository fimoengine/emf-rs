//! Definition of the `InterfaceLoader` trait.

use crate::fn_ptr::SysGetFunctionFn;
use crate::{BaseT, InterfaceBinding};

/// A trait to initialize a binding object.
pub trait InterfaceLoader {
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
    ) -> &'static Self::BindingT;
}
