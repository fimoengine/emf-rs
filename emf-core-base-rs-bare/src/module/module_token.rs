use crate::module::{
    InterfaceDescriptor, LoaderModuleHandle, LoaderModuleHandleRef, ModuleError, ModuleHandle,
    ModuleHandleRef, ModuleInfo, ModuleInterface, ModuleLoaderHandle, ModuleLoaderHandleRef,
    ModuleLoaderWrapper, ModuleStatus, ModuleType,
};
use crate::{ffi, FFIObject};
use std::path::{Path, PathBuf};

/// Provides access to the `module` api.
pub trait ModuleToken<'a> {
    /// Registers a new module loader.
    ///
    /// Module names starting with `__` are reserved for future use.
    ///
    /// # Failure
    ///
    /// The function fails if `mod_type` already exists.
    fn register_loader<T: ModuleLoaderWrapper<'static>>(
        &self,
        loader: &T,
        mod_type: &ModuleType,
    ) -> Result<ModuleLoaderHandle<'static>, ModuleError>;

    /// Unregisters an existing module loader.
    ///
    /// Unregistering a module loader also unregisters the modules it loaded.
    ///
    /// # Failure
    ///
    /// The function fails if `loader` is invalid.
    fn unregister_loader(&self, loader: ModuleLoaderHandle) -> Option<ModuleError>;

    /// Fetches the number of loaders.
    fn get_num_loaders(&self) -> usize;

    /// Copies the available module types.
    ///
    /// # Failure
    ///
    /// The function fails if `buffer` too small.
    fn get_module_types(&self, buffer: &mut [ModuleType]) -> Result<usize, ModuleError>;

    /// Fetches the number of loaded modules.
    fn get_num_modules(&self) -> usize;

    /// Copies the available module infos.
    ///
    /// # Failure
    ///
    /// The function fails if `buffer` too small.
    fn get_modules(&self, buffer: &mut [ModuleInfo]) -> Result<usize, ModuleError>;

    /// Fetches the number of exported interfaces.
    fn get_num_exported_interfaces(&self) -> usize;

    /// Copies the descriptors of the exported interfaces.
    ///
    /// # Failure
    ///
    /// The function fails if `buffer` too small.
    fn get_exported_interfaces(
        &self,
        buffer: &mut [InterfaceDescriptor],
    ) -> Result<usize, ModuleError>;

    /// Fetches the handle of the loader associated with a module type.
    ///
    /// # Failure
    ///
    /// The function fails if `mod_type` does not exist.
    fn get_loader_handle(
        &self,
        mod_type: &ModuleType,
    ) -> Result<ModuleLoaderHandleRef<'a>, ModuleError>;

    /// Checks if a module type exists.
    fn module_type_exist(&self, mod_type: &ModuleType) -> bool;

    /// Checks if a module exists.
    fn module_exists(&self, module: &ModuleHandleRef) -> bool;

    /// Fetches the module handle of the exported interface.
    ///
    /// # Failure
    ///
    /// The function fails if `interface` does not exist.
    fn get_exported_interface_handle(
        &self,
        interface: &InterfaceDescriptor,
    ) -> Result<ModuleHandleRef<'a>, ModuleError>;

    /// Checks whether an exported interface exists.
    fn exported_interface_exists(&self, interface: &InterfaceDescriptor) -> bool;

    /// Creates a new unlinked module handle.
    ///
    /// # Safety
    ///
    /// The handle must be linked before use (See [ModuleToken::link_module()]).
    unsafe fn create_module_handle(&self) -> ModuleHandle<'static>;

    /// Removes an existing module handle.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid.
    ///
    /// # Safety
    ///
    /// Removing the handle does not unload the module.
    unsafe fn remove_module_handle(&self, module: ModuleHandle) -> Option<ModuleError>;

    /// Links a module handle to an internal module handle.
    ///
    /// # Failure
    ///
    /// The function fails if `module` or`loader` are invalid.
    ///
    /// # Safety
    ///
    /// Incorrect usage can lead to dangling handles or use-after-free errors.
    unsafe fn link_module<'b, 'c: 'd, 'd: 'b, T: ModuleLoaderWrapper<'d>>(
        &self,
        module: &ModuleHandle,
        loader: &'c ModuleLoaderHandleRef<'c>,
        internal_handle: &'b LoaderModuleHandle<'b, 'd, T>,
    ) -> Option<ModuleError>;

    /// Fetches the internal handle linked with the module handle.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid.
    ///
    /// # Safety
    ///
    /// Care must be taken when dealing with internal handles.
    unsafe fn get_loader_module_handle<'b, T: ModuleLoaderWrapper<'a>>(
        &self,
        module: &'b ModuleHandleRef<'b>,
    ) -> Result<LoaderModuleHandleRef<'b, 'a, T>, ModuleError>;

    /// Fetches the loader handle linked with the module handle.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid.
    ///
    /// # Safety
    ///
    /// Care must be taken when dealing with internal handles.
    unsafe fn get_loader_handle_from_mod(
        &self,
        module: &ModuleHandleRef,
    ) -> Result<ModuleLoaderHandleRef<'a>, ModuleError>;

    /// Fetches the interface of a module loader.
    ///
    /// # Failure
    ///
    /// The function fails if `loader` is invalid.
    ///
    /// # Safety
    ///
    /// The usage of a `ModuleLoader` bypasses the `module` api.
    unsafe fn get_loader_interface<T: ModuleLoaderWrapper<'a>>(
        &self,
        loader: &ModuleLoaderHandleRef,
    ) -> Result<T, ModuleError>;

    /// Adds a new module.
    ///
    /// # Failure
    ///
    /// The function fails if `loader` or `path` is invalid or the type of
    /// the module can not be loaded by the loader.
    fn add_module<'b, 'c: 'b, T: AsRef<Path>>(
        &self,
        loader: &'c ModuleLoaderHandleRef<'c>,
        path: &T,
    ) -> Result<ModuleHandle<'b>, ModuleError>;

    /// Removes a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid or the module is not in an unloaded state.
    fn remove_module(&self, module: ModuleHandle) -> Option<ModuleError>;

    /// Loads a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid, the load dependencies
    /// of the module are not exported or the module is not in an unloaded state.
    fn load(&self, module: &ModuleHandle) -> Option<ModuleError>;

    /// Unloads a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid or the module is in an
    /// unloaded or ready state.
    fn unload(&self, module: &ModuleHandle) -> Option<ModuleError>;

    /// Initializes a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid, the runtime dependencies of
    /// the module are not exported or the module is not in a loaded state.
    fn initialize(&self, module: &ModuleHandle) -> Option<ModuleError>;

    /// Terminates a module.
    ///
    /// Terminating a module also removes the interfaces it exported.
    ///
    /// The modules that depend on the module are terminated.
    /// If they list the module as a load dependency, they are also unloaded.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid or the module is not in a ready state.
    fn terminate<'b>(&self, module: ModuleHandle<'b>) -> (Option<ModuleError>, ModuleHandle<'b>);

    /// Registers a new runtime dependency of the module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid.
    fn add_runtime_dependency(
        &self,
        module: &ModuleHandle,
        interface: &InterfaceDescriptor,
    ) -> Option<ModuleError>;

    /// Removes an existing runtime dependency from the module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid.
    fn remove_runtime_dependency(
        &self,
        module: &ModuleHandle,
        interface: &InterfaceDescriptor,
    ) -> Option<ModuleError>;

    /// Exports an interface of a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid, `interface` is already
    /// exported, `interface` is not contained in the module or the
    /// module is not yet initialized.
    fn export_interface(
        &self,
        module: &ModuleHandle,
        interface: &InterfaceDescriptor,
    ) -> Option<ModuleError>;

    /// Fetches the load status of a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid.
    fn fetch_status(&self, module: &ModuleHandleRef) -> Result<ModuleStatus, ModuleError>;

    /// Fetches the module info from a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid or the module is not yet loaded.
    fn get_module_info<'b>(
        &self,
        module: &'b ModuleHandleRef<'b>,
    ) -> Result<&'b ModuleInfo, ModuleError>;

    /// Fetches the path a module was loaded from.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid or the module is not yet loaded.
    fn get_module_path(&self, module: &ModuleHandleRef) -> Result<PathBuf, ModuleError>;

    /// Fetches the load dependencies of a module.
    ///
    /// The load dependencies of a module must all be present at the time of loading.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid.
    fn get_load_dependencies<'b>(
        &self,
        module: &'b ModuleHandleRef<'b>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError>;

    /// Fetches a list of the runtime dependencies from a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid or the module is not yet loaded.
    fn get_runtime_dependencies<'b>(
        &self,
        module: &'b ModuleHandleRef<'b>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError>;

    /// Fetches a list of the exportable interfaces from a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid or the module is not yet loaded.
    fn get_exportable_interfaces<'b>(
        &self,
        module: &'b ModuleHandleRef<'b>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError>;

    /// Fetches an interface from a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid, the module is not in a ready state or
    /// the interface is not contained in the module.
    fn get_interface<'b, T: Sized + FFIObject<ffi::module::ModuleInterface>>(
        &self,
        module: &'b ModuleHandleRef<'b>,
        interface: &InterfaceDescriptor,
    ) -> Result<ModuleInterface<'b, T>, ModuleError>;
}
