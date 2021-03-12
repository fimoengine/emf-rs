use crate::ffi::{
    sys::api::GetFunctionFn as GetFunctionFnFFI, CBase as CBaseFFI, CBaseInterface, CBaseLoader,
};
use crate::{CBase, CBaseAccess, CBaseRef};
use std::ptr::NonNull;

/// Trait for loading the interface.
pub trait CBaseAPILoader<'interface> {
    /// Type of the interface.
    type Interface: CBaseAccess<'interface>;

    /// Fetches the `emf-core-base` interface.
    ///
    /// # Safety
    ///
    /// The parameter `get_function_fn` must be able to accept `base_module`.
    ///
    /// # Panics
    ///
    /// This function panics if it can not fetch the interface.
    unsafe fn fetch_interface(
        base_module: Option<NonNull<CBaseFFI>>,
        get_function_fn: GetFunctionFnFFI,
    ) -> Self::Interface;
}

impl<'interface> CBaseAPILoader<'interface> for CBase<'interface> {
    type Interface = Self;

    unsafe fn fetch_interface(
        base_module: Option<NonNull<CBaseFFI>>,
        get_function_fn: GetFunctionFnFFI,
    ) -> Self::Interface {
        Self::new(CBaseRef::new(CBaseInterface::fetch_interface(
            base_module,
            get_function_fn,
        )))
    }
}
