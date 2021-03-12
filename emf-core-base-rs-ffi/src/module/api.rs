//! Module api.
//!
//! The library api is exposed be the [ModuleBinding] trait.
use crate::collections::{ConstSpan, MutSpan, NonNullConst, Result};
use crate::library::OSPathChar;
use crate::module::module_loader::ModuleLoaderInterface;
use crate::module::{
    Error, Interface, InterfaceDescriptor, InternalHandle, LoaderHandle, ModuleHandle, ModuleInfo,
    ModuleStatus, ModuleType,
};
use crate::{Bool, CBase, CBaseInterface, TypeWrapper};
use std::ptr::NonNull;

pub type RegisterLoaderFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        loader: NonNullConst<ModuleLoaderInterface>,
        mod_type: NonNullConst<ModuleType>,
    ) -> Result<LoaderHandle, Error>,
>;

pub type UnregisterLoaderFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        loader: LoaderHandle,
    ) -> Result<i8, Error>,
>;

pub type GetLoaderInterfaceFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        loader: LoaderHandle,
    ) -> Result<NonNullConst<ModuleLoaderInterface>, Error>,
>;

pub type GetLoaderHandleFromTypeFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        mod_type: NonNullConst<ModuleType>,
    ) -> Result<LoaderHandle, Error>,
>;

pub type GetLoaderHandleFromModuleFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
    ) -> Result<LoaderHandle, Error>,
>;

pub type GetNumModulesFn =
    TypeWrapper<unsafe extern "C-unwind" fn(base_module: Option<NonNull<CBase>>) -> usize>;

pub type GetNumLoadersFn =
    TypeWrapper<unsafe extern "C-unwind" fn(base_module: Option<NonNull<CBase>>) -> usize>;

pub type GetNumExportedInterfacesFn =
    TypeWrapper<unsafe extern "C-unwind" fn(base_module: Option<NonNull<CBase>>) -> usize>;

pub type ModuleExistsFn = TypeWrapper<
    unsafe extern "C-unwind" fn(base_module: Option<NonNull<CBase>>, handle: ModuleHandle) -> Bool,
>;

pub type TypeExistsFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        mod_type: NonNullConst<ModuleType>,
    ) -> Bool,
>;

pub type ExportedInterfaceExistsFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Bool,
>;

pub type GetModulesFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        buffer: NonNull<MutSpan<ModuleInfo>>,
    ) -> Result<usize, Error>,
>;

pub type GetModuleTypesFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        buffer: NonNull<MutSpan<ModuleType>>,
    ) -> Result<usize, Error>,
>;

pub type GetExportedInterfacesFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        buffer: NonNull<MutSpan<InterfaceDescriptor>>,
    ) -> Result<usize, Error>,
>;

pub type GetExportedInterfaceHandleFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<ModuleHandle, Error>,
>;

pub type CreateModuleHandleFn =
    TypeWrapper<unsafe extern "C-unwind" fn(base_module: Option<NonNull<CBase>>) -> ModuleHandle>;

pub type RemoveModuleHandleFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
    ) -> Result<i8, Error>,
>;

pub type LinkModuleFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
        loader: LoaderHandle,
        internal: InternalHandle,
    ) -> Result<i8, Error>,
>;

pub type GetInternalModuleHandleFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
    ) -> Result<InternalHandle, Error>,
>;

pub type AddModuleFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        loader: LoaderHandle,
        path: NonNullConst<OSPathChar>,
    ) -> Result<ModuleHandle, Error>,
>;

pub type RemoveModuleFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
    ) -> Result<i8, Error>,
>;

pub type LoadFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
    ) -> Result<i8, Error>,
>;

pub type UnloadFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
    ) -> Result<i8, Error>,
>;

pub type InitializeFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
    ) -> Result<i8, Error>,
>;

pub type TerminateFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
    ) -> Result<i8, Error>,
>;

pub type AddDependencyFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<i8, Error>,
>;

pub type RemoveDependencyFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<i8, Error>,
>;

pub type ExportInterfaceFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<i8, Error>,
>;

pub type GetLoadDependenciesFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error>,
>;

pub type GetRuntimeDependenciesFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error>,
>;

pub type GetExportableInterfacesFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error>,
>;

pub type FetchStatusFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
    ) -> Result<ModuleStatus, Error>,
>;

pub type GetModulePathFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
    ) -> Result<NonNullConst<OSPathChar>, Error>,
>;

pub type GetModuleInfoFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
    ) -> Result<NonNullConst<ModuleInfo>, Error>,
>;

pub type GetInterfaceFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: ModuleHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<Interface, Error>,
>;

/// Helper trait for using the module api.
pub trait ModuleBinding {
    /// Registers a new module loader.
    ///
    /// Module types starting with `__` are reserved for future use.
    ///
    /// # Failure
    ///
    /// The function fails if `mod_type` already exists.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn register_loader(
        &mut self,
        loader: NonNullConst<ModuleLoaderInterface>,
        mod_type: NonNullConst<ModuleType>,
    ) -> Result<LoaderHandle, Error>;

    /// Unregisters an existing module loader.
    ///
    /// Unregistering a module loader also unloads the modules it loaded.
    ///
    /// # Failure
    ///
    /// The function fails if `loader` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn unregister_loader(&mut self, loader: LoaderHandle) -> Result<i8, Error>;

    /// Fetches the interface of a module loader.
    ///
    /// # Failure
    ///
    /// The function fails if `loader` is invalid.
    ///
    /// # Return
    ///
    /// Interface on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_loader_interface(
        &mut self,
        loader: LoaderHandle,
    ) -> Result<NonNullConst<ModuleLoaderInterface>, Error>;

    /// Fetches the handle of the loader associated with a module type.
    ///
    /// # Failure
    ///
    /// The function fails if `mod_type` does not exist.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_loader_handle_from_type(
        &self,
        mod_type: NonNullConst<ModuleType>,
    ) -> Result<LoaderHandle, Error>;

    /// Fetches the handle of the loader linked with the module handle.
    ///
    /// # Failure
    ///
    /// The function fails if `handle` is invalid.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_loader_handle_from_module(
        &self,
        handle: ModuleHandle,
    ) -> Result<LoaderHandle, Error>;

    /// Fetches the number of loaded modules.
    ///
    /// # Return
    ///
    /// Number of modules.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_num_modules(&self) -> usize;

    /// Fetches the number of loaders.
    ///
    /// # Return
    ///
    /// Number of module loaders.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_num_loaders(&self) -> usize;

    /// Fetches the number of exported interfaces.
    ///
    /// # Return
    ///
    /// Number of exported interfaces.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_num_exported_interfaces(&self) -> usize;

    /// Checks if a module exists.
    ///
    /// # Return
    ///
    /// [Bool::True] if it exists, [Bool::False] otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn module_exists(&self, handle: ModuleHandle) -> Bool;

    /// Checks if a module type exists.
    ///
    /// # Return
    ///
    /// [Bool::True] if it exists, [Bool::False] otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn type_exists(&self, mod_type: NonNullConst<ModuleType>) -> Bool;

    /// Checks whether an exported interface exists.
    ///
    /// # Return
    ///
    /// [Bool::True] if it exists, [Bool::False] otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn exported_interface_exists(
        &self,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Bool;

    /// Copies the available module info into a buffer.
    ///
    /// # Failure
    ///
    /// Fails if `buffer.as_ref().len() < get_num_modules()`.
    ///
    /// # Return
    ///
    /// Number if written module info on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_modules(&self, buffer: NonNull<MutSpan<ModuleInfo>>) -> Result<usize, Error>;

    /// Copies the available module types into a buffer.
    ///
    /// # Failure
    ///
    /// Fails if `buffer.as_ref().len() < get_num_loaders()`.
    ///
    /// # Return
    ///
    /// Number if written module types on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_module_types(&self, buffer: NonNull<MutSpan<ModuleType>>)
        -> Result<usize, Error>;

    /// Copies the descriptors of the exported interfaces into a buffer.
    ///
    /// # Failure
    ///
    /// Fails if `buffer.as_ref().len() < get_num_exported_interfaces()`.
    ///
    /// # Return
    ///
    /// Number if written descriptors on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_exported_interfaces(
        &self,
        buffer: NonNull<MutSpan<InterfaceDescriptor>>,
    ) -> Result<usize, Error>;

    /// Fetches the module handle of the exported interface.
    ///
    /// # Failure
    ///
    /// Fails if `interface` does not exist.
    ///
    /// # Return
    ///
    /// Module handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_exported_interface_handle(
        &self,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<ModuleHandle, Error>;

    /// Creates a new unlinked module handle.
    ///
    /// # Note
    ///
    /// The handle must be linked before use.
    ///
    /// # Return
    ///
    /// Module handle.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn create_module_handle(&mut self) -> ModuleHandle;

    /// Removes an existing module handle.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid.
    ///
    /// # Note
    ///
    /// Removing the handle does not unload the module.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn remove_module_handle(&mut self, handle: ModuleHandle) -> Result<i8, Error>;

    /// Links a module handle to an internal module handle.
    ///
    /// # Failure
    ///
    /// Fails if `handle` or`loader` are invalid.
    ///
    /// # Note
    ///
    /// Incorrect usage can lead to dangling handles or use-after-free errors.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn link_module(
        &mut self,
        handle: ModuleHandle,
        loader: LoaderHandle,
        internal: InternalHandle,
    ) -> Result<i8, Error>;

    /// Fetches the internal handle linked with the module handle.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid.
    ///
    /// # Return
    ///
    /// Internal handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_internal_module_handle(
        &self,
        handle: ModuleHandle,
    ) -> Result<InternalHandle, Error>;

    /// Adds a new module.
    ///
    /// # Failure
    ///
    /// Fails if `loader` or `path` is invalid or the type
    /// of the module can not be loaded with the loader.
    ///
    /// # Return
    ///
    /// Module handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn add_module(
        &mut self,
        loader: LoaderHandle,
        path: NonNullConst<OSPathChar>,
    ) -> Result<ModuleHandle, Error>;

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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn remove_module(&mut self, handle: ModuleHandle) -> Result<i8, Error>;

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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn load(&mut self, handle: ModuleHandle) -> Result<i8, Error>;

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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn unload(&mut self, handle: ModuleHandle) -> Result<i8, Error>;

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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn initialize(&mut self, handle: ModuleHandle) -> Result<i8, Error>;

    /// Terminates a module.
    ///
    /// Terminating a module also removes the interfaces it exported.
    /// The modules that depend on the module are terminated.
    /// If they list the module as a load dependency, they are also unloaded.
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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn terminate(&mut self, handle: ModuleHandle) -> Result<i8, Error>;

    /// Registers a new runtime dependency of the module.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn add_dependency(
        &mut self,
        handle: ModuleHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<i8, Error>;

    /// Removes an existing runtime dependency from the module.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn remove_dependency(
        &mut self,
        handle: ModuleHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<i8, Error>;

    /// Exports an interface of a module.
    ///
    /// # Failure
    ///
    /// Fails if `handle` is invalid, `interface` is already exported,
    /// `interface` is not contained in the module or the module is not yet initialized.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn export_interface(
        &mut self,
        handle: ModuleHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<i8, Error>;

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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_load_dependencies(
        &self,
        handle: ModuleHandle,
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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_runtime_dependencies(
        &self,
        handle: ModuleHandle,
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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_exportable_interfaces(
        &self,
        handle: ModuleHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error>;

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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn fetch_status(&self, handle: ModuleHandle) -> Result<ModuleStatus, Error>;

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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_module_path(
        &self,
        handle: ModuleHandle,
    ) -> Result<NonNullConst<OSPathChar>, Error>;

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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_module_info(
        &self,
        handle: ModuleHandle,
    ) -> Result<NonNullConst<ModuleInfo>, Error>;

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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_interface(
        &self,
        handle: ModuleHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<Interface, Error>;
}

impl ModuleBinding for CBaseInterface {
    #[inline]
    unsafe fn register_loader(
        &mut self,
        loader: NonNullConst<ModuleLoaderInterface>,
        mod_type: NonNullConst<ModuleType>,
    ) -> Result<LoaderHandle, Error> {
        (self.module_register_loader_fn)(self.base_module, loader, mod_type)
    }

    #[inline]
    unsafe fn unregister_loader(&mut self, loader: LoaderHandle) -> Result<i8, Error> {
        (self.module_unregister_loader_fn)(self.base_module, loader)
    }

    #[inline]
    unsafe fn get_loader_interface(
        &mut self,
        loader: LoaderHandle,
    ) -> Result<NonNullConst<ModuleLoaderInterface>, Error> {
        (self.module_get_loader_interface_fn)(self.base_module, loader)
    }

    #[inline]
    unsafe fn get_loader_handle_from_type(
        &self,
        mod_type: NonNullConst<ModuleType>,
    ) -> Result<LoaderHandle, Error> {
        (self.module_get_loader_handle_from_type_fn)(self.base_module, mod_type)
    }

    #[inline]
    unsafe fn get_loader_handle_from_module(
        &self,
        handle: ModuleHandle,
    ) -> Result<LoaderHandle, Error> {
        (self.module_get_loader_handle_from_module_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn get_num_modules(&self) -> usize {
        (self.module_get_num_modules_fn)(self.base_module)
    }

    #[inline]
    unsafe fn get_num_loaders(&self) -> usize {
        (self.module_get_num_loaders_fn)(self.base_module)
    }

    #[inline]
    unsafe fn get_num_exported_interfaces(&self) -> usize {
        (self.module_get_num_exported_interfaces_fn)(self.base_module)
    }

    #[inline]
    unsafe fn module_exists(&self, handle: ModuleHandle) -> Bool {
        (self.module_module_exists_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn type_exists(&self, mod_type: NonNullConst<ModuleType>) -> Bool {
        (self.module_type_exists_fn)(self.base_module, mod_type)
    }

    #[inline]
    unsafe fn exported_interface_exists(
        &self,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Bool {
        (self.module_exported_interface_exists_fn)(self.base_module, interface)
    }

    #[inline]
    unsafe fn get_modules(&self, buffer: NonNull<MutSpan<ModuleInfo>>) -> Result<usize, Error> {
        (self.module_get_modules_fn)(self.base_module, buffer)
    }

    #[inline]
    unsafe fn get_module_types(
        &self,
        buffer: NonNull<MutSpan<ModuleType>>,
    ) -> Result<usize, Error> {
        (self.module_get_module_types_fn)(self.base_module, buffer)
    }

    #[inline]
    unsafe fn get_exported_interfaces(
        &self,
        buffer: NonNull<MutSpan<InterfaceDescriptor>>,
    ) -> Result<usize, Error> {
        (self.module_get_exported_interfaces_fn)(self.base_module, buffer)
    }

    #[inline]
    unsafe fn get_exported_interface_handle(
        &self,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<ModuleHandle, Error> {
        (self.module_get_exported_interface_handle_fn)(self.base_module, interface)
    }

    #[inline]
    unsafe fn create_module_handle(&mut self) -> ModuleHandle {
        (self.module_create_module_handle_fn)(self.base_module)
    }

    #[inline]
    unsafe fn remove_module_handle(&mut self, handle: ModuleHandle) -> Result<i8, Error> {
        (self.module_remove_module_handle_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn link_module(
        &mut self,
        handle: ModuleHandle,
        loader: LoaderHandle,
        internal: InternalHandle,
    ) -> Result<i8, Error> {
        (self.module_link_module_fn)(self.base_module, handle, loader, internal)
    }

    #[inline]
    unsafe fn get_internal_module_handle(
        &self,
        handle: ModuleHandle,
    ) -> Result<InternalHandle, Error> {
        (self.module_get_internal_module_handle_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn add_module(
        &mut self,
        loader: LoaderHandle,
        path: NonNullConst<OSPathChar>,
    ) -> Result<ModuleHandle, Error> {
        (self.module_add_module_fn)(self.base_module, loader, path)
    }

    #[inline]
    unsafe fn remove_module(&mut self, handle: ModuleHandle) -> Result<i8, Error> {
        (self.module_remove_module_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn load(&mut self, handle: ModuleHandle) -> Result<i8, Error> {
        (self.module_load_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn unload(&mut self, handle: ModuleHandle) -> Result<i8, Error> {
        (self.module_unload_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn initialize(&mut self, handle: ModuleHandle) -> Result<i8, Error> {
        (self.module_initialize_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn terminate(&mut self, handle: ModuleHandle) -> Result<i8, Error> {
        (self.module_terminate_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn add_dependency(
        &mut self,
        handle: ModuleHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<i8, Error> {
        (self.module_add_dependency_fn)(self.base_module, handle, interface)
    }

    #[inline]
    unsafe fn remove_dependency(
        &mut self,
        handle: ModuleHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<i8, Error> {
        (self.module_remove_dependency_fn)(self.base_module, handle, interface)
    }

    #[inline]
    unsafe fn export_interface(
        &mut self,
        handle: ModuleHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<i8, Error> {
        (self.module_export_interface_fn)(self.base_module, handle, interface)
    }

    #[inline]
    unsafe fn get_load_dependencies(
        &self,
        handle: ModuleHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error> {
        (self.module_get_load_dependencies_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn get_runtime_dependencies(
        &self,
        handle: ModuleHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error> {
        (self.module_get_runtime_dependencies_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn get_exportable_interfaces(
        &self,
        handle: ModuleHandle,
    ) -> Result<ConstSpan<InterfaceDescriptor>, Error> {
        (self.module_get_exportable_interfaces_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn fetch_status(&self, handle: ModuleHandle) -> Result<ModuleStatus, Error> {
        (self.module_fetch_status_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn get_module_path(
        &self,
        handle: ModuleHandle,
    ) -> Result<NonNullConst<OSPathChar>, Error> {
        (self.module_get_module_path_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn get_module_info(
        &self,
        handle: ModuleHandle,
    ) -> Result<NonNullConst<ModuleInfo>, Error> {
        (self.module_get_module_info_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn get_interface(
        &self,
        handle: ModuleHandle,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<Interface, Error> {
        (self.module_get_interface_fn)(self.base_module, handle, interface)
    }
}
