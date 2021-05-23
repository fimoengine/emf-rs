//! Interface of a module loader.
//!
//! Any object that can be wrapped into a [ModuleLoaderInterface] can be used as a library loader.
use crate::collections::{ConstSpan, NonNullConst, Result};
use crate::errors::Error;
use crate::library::OSPathString;
use crate::module::native_module::{NativeModule, NativeModuleInterface};
use crate::module::{Interface, InterfaceDescriptor, InternalHandle, ModuleInfo, ModuleStatus};
use crate::TypeWrapper;
use std::ffi::c_void;
use std::ptr::NonNull;

/// Opaque structure representing a module loader.
#[repr(C)]
pub struct ModuleLoader {
    _dummy: [u8; 0],
}

pub type AddModuleFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        path: OSPathString,
    ) -> Result<InternalHandle, Error>,
>;

pub type RemoveModuleFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        handle: InternalHandle,
    ) -> Result<i8, Error>,
>;

pub type LoadFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        handle: InternalHandle,
    ) -> Result<i8, Error>,
>;

pub type UnloadFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        handle: InternalHandle,
    ) -> Result<i8, Error>,
>;

pub type InitializeFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        handle: InternalHandle,
    ) -> Result<i8, Error>,
>;

pub type TerminateFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        handle: InternalHandle,
    ) -> Result<i8, Error>,
>;

pub type FetchStatusFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        handle: InternalHandle,
    ) -> Result<ModuleStatus, Error>,
>;

pub type GetInterfaceFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        handle: InternalHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<Interface, Error>,
>;

pub type GetModuleInfoFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        handle: InternalHandle,
    ) -> Result<NonNullConst<ModuleInfo>, Error>,
>;

pub type GetModulePathFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        handle: InternalHandle,
    ) -> Result<OSPathString, Error>,
>;

pub type GetLoadDependenciesFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        handle: InternalHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error>,
>;

pub type GetRuntimeDependenciesFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        handle: InternalHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error>,
>;

pub type GetExportableInterfacesFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        handle: InternalHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error>,
>;

pub type GetExtendedVTableFn = TypeWrapper<
    unsafe extern "C-unwind" fn(loader: Option<NonNull<ModuleLoader>>) -> NonNullConst<c_void>,
>;

/// Interface of a module loader.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ModuleLoaderInterfaceVTable {
    pub add_module_fn: AddModuleFn,
    pub remove_module_fn: RemoveModuleFn,
    pub load_fn: LoadFn,
    pub unload_fn: UnloadFn,
    pub initialize_fn: InitializeFn,
    pub terminate_fn: TerminateFn,
    pub fetch_status_fn: FetchStatusFn,
    pub get_interface_fn: GetInterfaceFn,
    pub get_module_info_fn: GetModuleInfoFn,
    pub get_module_path_fn: GetModulePathFn,
    pub get_load_dependencies_fn: GetLoadDependenciesFn,
    pub get_runtime_dependencies_fn: GetRuntimeDependenciesFn,
    pub get_exportable_interfaces_fn: GetExportableInterfacesFn,
    pub get_extended_vtable_fn: GetExtendedVTableFn,
}

/// Interface of a module loader.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ModuleLoaderInterface {
    pub loader: Option<NonNull<ModuleLoader>>,
    pub vtable: NonNullConst<ModuleLoaderInterfaceVTable>,
}

unsafe impl Send for ModuleLoaderInterface {}
unsafe impl Sync for ModuleLoaderInterface {}

/// Helper trait for using a module loader.
pub trait ModuleLoaderBinding {
    /// Adds a new module.
    ///
    /// # Failure
    ///
    /// Fails if `path` is invalid or the type of the
    /// module can not be loaded with the loader.
    ///
    /// # Return
    ///
    /// Module handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn add_module(&mut self, path: OSPathString) -> Result<InternalHandle, Error>;

    /// Removes a module.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid or the module is not in an unloaded state.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn remove_module(&mut self, handle: InternalHandle) -> Result<i8, Error>;

    /// Loads a module.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid, the load dependencies of the module are
    /// not exported or the module is not in an unloaded state.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn load(&mut self, handle: InternalHandle) -> Result<i8, Error>;

    /// Unloads a module.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid or the module is in an unloaded or ready state.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn unload(&mut self, handle: InternalHandle) -> Result<i8, Error>;

    /// Initializes a module.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid, the runtime dependencies of the
    /// module are not exported or the module is not in a loaded state.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn initialize(&mut self, handle: InternalHandle) -> Result<i8, Error>;

    /// Terminates a module.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid or the module is not in a ready state.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn terminate(&mut self, handle: InternalHandle) -> Result<i8, Error>;

    /// Fetches the load status of a module.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid.
    ///
    /// # Return
    ///
    /// Module status on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn fetch_status(&self, handle: InternalHandle) -> Result<ModuleStatus, Error>;

    /// Fetches an interface from a module.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid, the module is not in a ready
    /// state or the interface is not contained in the module.
    ///
    /// # Return
    ///
    /// Interface on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_interface(
        &self,
        handle: InternalHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<Interface, Error>;

    /// Fetches the module info from a module.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid or the module is not yet loaded.
    ///
    /// # Return
    ///
    /// Module info on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_module_info(
        &self,
        handle: InternalHandle,
    ) -> Result<NonNullConst<ModuleInfo>, Error>;

    /// Fetches the path a module was loaded from.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid or the module is not yet loaded.
    ///
    /// # Return
    ///
    /// Module path on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_module_path(&self, handle: InternalHandle) -> Result<OSPathString, Error>;

    /// Fetches the load dependencies of a module.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid.
    ///
    /// # Return
    ///
    /// Load dependencies on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_load_dependencies(
        &self,
        handle: InternalHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error>;

    /// Fetches the runtime dependencies of a module.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid or the module is not yet loaded.
    ///
    /// # Return
    ///
    /// Runtime dependencies on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_runtime_dependencies(
        &self,
        handle: InternalHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error>;

    /// Fetches the exportable interfaces of a module.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid or the module is not yet loaded.
    ///
    /// # Return
    ///
    /// Exportable interfaces on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_exportable_interfaces(
        &self,
        handle: InternalHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error>;

    /// Fetches a pointer to the extended loader vtable.
    ///
    /// # Return
    ///
    /// Pointer to the loader vtable.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_extended_vtable(&self) -> NonNullConst<c_void>;
}

impl ModuleLoaderBinding for ModuleLoaderInterface {
    #[inline]
    unsafe fn add_module(&mut self, path: OSPathString) -> Result<InternalHandle, Error> {
        (self.vtable.as_ref().add_module_fn)(self.loader, path)
    }

    #[inline]
    unsafe fn remove_module(&mut self, handle: InternalHandle) -> Result<i8, Error> {
        (self.vtable.as_ref().remove_module_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn load(&mut self, handle: InternalHandle) -> Result<i8, Error> {
        (self.vtable.as_ref().load_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn unload(&mut self, handle: InternalHandle) -> Result<i8, Error> {
        (self.vtable.as_ref().unload_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn initialize(&mut self, handle: InternalHandle) -> Result<i8, Error> {
        (self.vtable.as_ref().initialize_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn terminate(&mut self, handle: InternalHandle) -> Result<i8, Error> {
        (self.vtable.as_ref().terminate_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn fetch_status(&self, handle: InternalHandle) -> Result<ModuleStatus, Error> {
        (self.vtable.as_ref().fetch_status_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_interface(
        &self,
        handle: InternalHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<Interface, Error> {
        (self.vtable.as_ref().get_interface_fn)(self.loader, handle, interface)
    }

    #[inline]
    unsafe fn get_module_info(
        &self,
        handle: InternalHandle,
    ) -> Result<NonNullConst<ModuleInfo>, Error> {
        (self.vtable.as_ref().get_module_info_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_module_path(&self, handle: InternalHandle) -> Result<OSPathString, Error> {
        (self.vtable.as_ref().get_module_path_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_load_dependencies(
        &self,
        handle: InternalHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error> {
        (self.vtable.as_ref().get_load_dependencies_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_runtime_dependencies(
        &self,
        handle: InternalHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error> {
        (self.vtable.as_ref().get_runtime_dependencies_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_exportable_interfaces(
        &self,
        handle: InternalHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error> {
        (self.vtable.as_ref().get_exportable_interfaces_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_extended_vtable(&self) -> NonNullConst<c_void> {
        (self.vtable.as_ref().get_extended_vtable_fn)(self.loader)
    }
}

pub type GetNativeModuleFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        handle: InternalHandle,
    ) -> Result<Option<NonNull<NativeModule>>, Error>,
>;

pub type GetNativeModuleInterfaceFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<ModuleLoader>>,
        handle: InternalHandle,
    ) -> Result<NonNullConst<NativeModuleInterface>, Error>,
>;

/// Interface of a native module loader.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NativeModuleLoaderVTable {
    pub loader_vtable: NonNullConst<ModuleLoaderInterfaceVTable>,
    pub get_native_module_fn: GetNativeModuleFn,
    pub get_native_module_interface_fn: GetNativeModuleInterfaceFn,
}

/// Interface of a native module loader.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NativeModuleLoaderInterface {
    pub loader: Option<NonNull<ModuleLoader>>,
    pub vtable: NonNullConst<NativeModuleLoaderVTable>,
}

unsafe impl Send for NativeModuleLoaderInterface {}
unsafe impl Sync for NativeModuleLoaderInterface {}

/// Helper trait for using a native module loader.
pub trait NativeModuleLoaderBinding: ModuleLoaderBinding {
    /// Fetches the native module handle.
    ///
    /// # Failure
    ///
    /// The function fails if `handle` is invalid.
    ///
    /// # Return
    ///
    /// Native module handle.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_native_module(
        &self,
        handle: InternalHandle,
    ) -> Result<Option<NonNull<NativeModule>>, Error>;

    /// Fetches the interface of a native module handle.
    ///
    /// # Failure
    ///
    /// The function fails if `handle` is invalid.
    ///
    /// # Return
    ///
    /// Native module interface.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderBinding] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_native_module_interface(
        &self,
        handle: InternalHandle,
    ) -> Result<NonNullConst<NativeModuleInterface>, Error>;
}

impl ModuleLoaderBinding for NativeModuleLoaderInterface {
    #[inline]
    unsafe fn add_module(&mut self, path: OSPathString) -> Result<InternalHandle, Error> {
        (self.vtable.as_ref().loader_vtable.as_ref().add_module_fn)(self.loader, path)
    }

    #[inline]
    unsafe fn remove_module(&mut self, handle: InternalHandle) -> Result<i8, Error> {
        (self.vtable.as_ref().loader_vtable.as_ref().remove_module_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn load(&mut self, handle: InternalHandle) -> Result<i8, Error> {
        (self.vtable.as_ref().loader_vtable.as_ref().load_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn unload(&mut self, handle: InternalHandle) -> Result<i8, Error> {
        (self.vtable.as_ref().loader_vtable.as_ref().unload_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn initialize(&mut self, handle: InternalHandle) -> Result<i8, Error> {
        (self.vtable.as_ref().loader_vtable.as_ref().initialize_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn terminate(&mut self, handle: InternalHandle) -> Result<i8, Error> {
        (self.vtable.as_ref().loader_vtable.as_ref().terminate_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn fetch_status(&self, handle: InternalHandle) -> Result<ModuleStatus, Error> {
        (self.vtable.as_ref().loader_vtable.as_ref().fetch_status_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_interface(
        &self,
        handle: InternalHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<Interface, Error> {
        (self.vtable.as_ref().loader_vtable.as_ref().get_interface_fn)(
            self.loader,
            handle,
            interface,
        )
    }

    #[inline]
    unsafe fn get_module_info(
        &self,
        handle: InternalHandle,
    ) -> Result<NonNullConst<ModuleInfo>, Error> {
        (self
            .vtable
            .as_ref()
            .loader_vtable
            .as_ref()
            .get_module_info_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_module_path(&self, handle: InternalHandle) -> Result<OSPathString, Error> {
        (self
            .vtable
            .as_ref()
            .loader_vtable
            .as_ref()
            .get_module_path_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_load_dependencies(
        &self,
        handle: InternalHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error> {
        (self
            .vtable
            .as_ref()
            .loader_vtable
            .as_ref()
            .get_load_dependencies_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_runtime_dependencies(
        &self,
        handle: InternalHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error> {
        (self
            .vtable
            .as_ref()
            .loader_vtable
            .as_ref()
            .get_runtime_dependencies_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_exportable_interfaces(
        &self,
        handle: InternalHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error> {
        (self
            .vtable
            .as_ref()
            .loader_vtable
            .as_ref()
            .get_exportable_interfaces_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_extended_vtable(&self) -> NonNullConst<c_void> {
        (self
            .vtable
            .as_ref()
            .loader_vtable
            .as_ref()
            .get_extended_vtable_fn)(self.loader)
    }
}

impl NativeModuleLoaderBinding for NativeModuleLoaderInterface {
    #[inline]
    unsafe fn get_native_module(
        &self,
        handle: InternalHandle,
    ) -> Result<Option<NonNull<NativeModule>>, Error> {
        (self.vtable.as_ref().get_native_module_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_native_module_interface(
        &self,
        handle: InternalHandle,
    ) -> Result<NonNullConst<NativeModuleInterface>, Error> {
        (self.vtable.as_ref().get_native_module_interface_fn)(self.loader, handle)
    }
}
