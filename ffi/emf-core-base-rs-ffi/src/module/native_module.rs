//! Interface of a native module.
//!
//! A native module is any library that exposes a symbol of the type [NativeModuleInterface].
use crate::collections::{ConstSpan, NonNullConst, Result};
use crate::module::{Error, Interface, InterfaceDescriptor, ModuleHandle, ModuleInfo};
use crate::sys::api::{GetFunctionFn, HasFunctionFn};
use crate::{CBase, TypeWrapper};
use std::ptr::NonNull;

/// Opaque structure representing a native module.
#[repr(C)]
pub struct NativeModule {
    _dummy: [u8; 0],
}

pub type LoadFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        handle: ModuleHandle,
        base_module: Option<NonNull<CBase>>,
        has_function_fn: HasFunctionFn,
        get_function_fn: GetFunctionFn,
    ) -> Result<Option<NonNull<NativeModule>>, Error>,
>;

pub type UnloadFn = TypeWrapper<
    unsafe extern "C-unwind" fn(module: Option<NonNull<NativeModule>>) -> Result<i8, Error>,
>;

pub type InitializeFn = TypeWrapper<
    unsafe extern "C-unwind" fn(module: Option<NonNull<NativeModule>>) -> Result<i8, Error>,
>;

pub type TerminateFn = TypeWrapper<
    unsafe extern "C-unwind" fn(module: Option<NonNull<NativeModule>>) -> Result<i8, Error>,
>;

pub type GetInterfaceFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        module: Option<NonNull<NativeModule>>,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<Interface, Error>,
>;

pub type GetModuleInfoFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        module: Option<NonNull<NativeModule>>,
    ) -> Result<NonNullConst<ModuleInfo>, Error>,
>;

pub type GetLoadDependenciesFn =
    TypeWrapper<unsafe extern "C-unwind" fn() -> ConstSpan<InterfaceDescriptor>>;

pub type GetRuntimeDependenciesFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        module: Option<NonNull<NativeModule>>,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error>,
>;

pub type GetExportableInterfacesFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        module: Option<NonNull<NativeModule>>,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error>,
>;

/// Interface of a native module.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NativeModuleInterface {
    pub load_fn: LoadFn,
    pub unload_fn: UnloadFn,
    pub initialize_fn: InitializeFn,
    pub terminate_fn: TerminateFn,
    pub get_interface_fn: GetInterfaceFn,
    pub get_module_info_fn: GetModuleInfoFn,
    pub get_load_dependencies_fn: GetLoadDependenciesFn,
    pub get_runtime_dependencies_fn: GetRuntimeDependenciesFn,
    pub get_exportable_interfaces_fn: GetExportableInterfacesFn,
}

unsafe impl Send for NativeModuleInterface {}
unsafe impl Sync for NativeModuleInterface {}

/// Helper trait for using a native module.
pub trait NativeModuleBinding {
    /// Loads the module.
    ///
    /// # Failure
    ///
    /// The function can fail if some module invariant is not met.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeModuleBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn load(
        &mut self,
        handle: ModuleHandle,
        base_module: Option<NonNull<CBase>>,
        has_function_fn: HasFunctionFn,
        get_function_fn: GetFunctionFn,
    ) -> Result<Option<NonNull<NativeModule>>, Error>;

    /// Unloads the module.
    ///
    /// # Failure
    ///
    /// The function can fail if some module invariant is not met or `module` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeModuleBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn unload(&mut self, module: Option<NonNull<NativeModule>>) -> Result<i8, Error>;

    /// Initializes the module.
    ///
    /// # Failure
    ///
    /// The function can fail if some module invariant is not met or `module` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeModuleBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn initialize(&mut self, module: Option<NonNull<NativeModule>>) -> Result<i8, Error>;

    /// Terminates the module.
    ///
    /// # Failure
    ///
    /// The function can fail if some module invariant is not met or `module` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeModuleBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn terminate(&mut self, module: Option<NonNull<NativeModule>>) -> Result<i8, Error>;

    /// Fetches an interface from the module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid.
    ///
    /// # Return
    ///
    /// Interface on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    /// Direct usage of a [NativeModuleBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_interface(
        &self,
        module: Option<NonNull<NativeModule>>,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<Interface, Error>;

    /// Fetches the module info of the module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid.
    ///
    /// # Return
    ///
    /// Module info on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    /// Direct usage of a [NativeModuleBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_module_info(
        &self,
        module: Option<NonNull<NativeModule>>,
    ) -> Result<NonNullConst<ModuleInfo>, Error>;

    /// Fetches the load dependencies of the module.
    ///
    /// # Return
    ///
    /// Load dependencies.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeModuleBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_load_dependencies(&self) -> ConstSpan<InterfaceDescriptor>;

    /// Fetches the runtime dependencies of the module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid.
    ///
    /// # Return
    ///
    /// Runtime dependencies on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    /// Direct usage of a [NativeModuleBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_runtime_dependencies(
        &self,
        module: Option<NonNull<NativeModule>>,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error>;

    /// Fetches the exportable interfaces of the module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid.
    ///
    /// # Return
    ///
    /// Exportable interfaces on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    /// Direct usage of a [NativeModuleBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_exportable_interfaces(
        &self,
        module: Option<NonNull<NativeModule>>,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error>;
}

impl NativeModuleBinding for NativeModuleInterface {
    #[inline]
    unsafe fn load(
        &mut self,
        handle: ModuleHandle,
        base_module: Option<NonNull<CBase>>,
        has_function_fn: HasFunctionFn,
        get_function_fn: GetFunctionFn,
    ) -> Result<Option<NonNull<NativeModule>>, Error> {
        (self.load_fn)(handle, base_module, has_function_fn, get_function_fn)
    }

    #[inline]
    unsafe fn unload(&mut self, module: Option<NonNull<NativeModule>>) -> Result<i8, Error> {
        (self.unload_fn)(module)
    }

    #[inline]
    unsafe fn initialize(&mut self, module: Option<NonNull<NativeModule>>) -> Result<i8, Error> {
        (self.initialize_fn)(module)
    }

    #[inline]
    unsafe fn terminate(&mut self, module: Option<NonNull<NativeModule>>) -> Result<i8, Error> {
        (self.terminate_fn)(module)
    }

    #[inline]
    unsafe fn get_interface(
        &self,
        module: Option<NonNull<NativeModule>>,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<Interface, Error> {
        (self.get_interface_fn)(module, interface)
    }

    #[inline]
    unsafe fn get_module_info(
        &self,
        module: Option<NonNull<NativeModule>>,
    ) -> Result<NonNullConst<ModuleInfo>, Error> {
        (self.get_module_info_fn)(module)
    }

    #[inline]
    unsafe fn get_load_dependencies(&self) -> ConstSpan<InterfaceDescriptor> {
        (self.get_load_dependencies_fn)()
    }

    #[inline]
    unsafe fn get_runtime_dependencies(
        &self,
        module: Option<NonNull<NativeModule>>,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error> {
        (self.get_runtime_dependencies_fn)(module)
    }

    #[inline]
    unsafe fn get_exportable_interfaces(
        &self,
        module: Option<NonNull<NativeModule>>,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error> {
        (self.get_exportable_interfaces_fn)(module)
    }
}
