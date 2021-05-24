use crate::ffi::collections::{MutSpan, NonNullConst};
use crate::ffi::errors::SimpleError;
use crate::ffi::library::{OSPathChar, OSPathString};
use crate::ffi::module::api::ModuleBinding;
use crate::ffi::Bool;
use crate::module::module_loader::{ModuleLoader, ModuleLoaderABICompat, ModuleLoaderAPI};
use crate::module::{
    Interface, InterfaceDescriptor, InternalModule, Loader, Module, ModuleInfo, ModuleStatus,
    ModuleType, MODULE_LOADER_TYPE_MAX_LENGTH,
};
use crate::ownership::{
    BorrowImmutable, BorrowMutable, ImmutableAccessIdentifier, MutableAccessIdentifier, Owned,
};
use crate::Error;
use crate::ToOsPathBuff;
use std::path::Path;
use std::pin::Pin;

const MODULE_TYPE_LENGTH_ERROR: &str = "Module type too long";

/// Idiomatic module api.
pub trait ModuleAPI<'interface> {
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
    fn register_loader<LT, L>(
        &mut self,
        loader: Pin<&'interface LT>,
        mod_type: impl AsRef<str>,
    ) -> Result<Loader<'interface, Owned>, Error<Owned>>
    where
        L: ModuleLoaderAPI<'interface> + ModuleLoaderABICompat,
        ModuleLoader<L, Owned>: From<&'interface LT>;

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
    fn unregister_loader(&mut self, loader: Loader<'_, Owned>) -> Result<(), Error<Owned>>;

    /// Fetches the interface of a module loader.
    ///
    /// # Failure
    ///
    /// The function fails if `loader` is invalid.
    ///
    /// # Return
    ///
    /// Interface on success, error otherwise.
    fn get_loader_interface<'loader, O, L>(
        &self,
        loader: &Loader<'loader, O>,
    ) -> Result<ModuleLoader<L, O>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
        L: ModuleLoaderAPI<'loader> + ModuleLoaderABICompat;

    /// Fetches the handle of the loader associated with a module type.
    ///
    /// # Failure
    ///
    /// The function fails if `mod_type` does not exist.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    fn get_loader_handle_from_type(
        &self,
        mod_type: impl AsRef<str>,
    ) -> Result<Loader<'interface, BorrowMutable<'_>>, Error<Owned>>;

    /// Fetches the handle of the loader linked with the module handle.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    fn get_loader_handle_from_module<'module, O>(
        &self,
        module: &Module<'module, O>,
    ) -> Result<Loader<'module, BorrowMutable<'_>>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

    /// Fetches the number of loaded modules.
    ///
    /// # Return
    ///
    /// Number of modules.
    fn get_num_modules(&self) -> usize;

    /// Fetches the number of loaders.
    ///
    /// # Return
    ///
    /// Number of module loaders.
    fn get_num_loaders(&self) -> usize;

    /// Fetches the number of exported interfaces.
    ///
    /// # Return
    ///
    /// Number of exported interfaces.
    fn get_num_exported_interfaces(&self) -> usize;

    /// Checks if a module exists.
    ///
    /// # Return
    ///
    /// [true] if it exists, [false] otherwise.
    fn module_exists<'module, O>(&self, module: &Module<'module, O>) -> bool
    where
        O: ImmutableAccessIdentifier;

    /// Checks if a module type exists.
    ///
    /// # Return
    ///
    /// [true] if it exists, [false] otherwise.
    fn type_exists(&self, mod_type: impl AsRef<str>) -> Result<bool, Error<Owned>>;

    /// Checks whether an exported interface exists.
    ///
    /// # Return
    ///
    /// [true] if it exists, [false] otherwise.
    fn exported_interface_exists(&self, interface: &InterfaceDescriptor) -> bool;

    /// Copies the available module info into a buffer.
    ///
    /// # Failure
    ///
    /// Fails if `buffer.as_ref().len() < get_num_modules()`.
    ///
    /// # Return
    ///
    /// Number if written module info on success, error otherwise.
    fn get_modules(&self, buffer: impl AsMut<[ModuleInfo]>) -> Result<usize, Error<Owned>>;

    /// Copies the available module types into a buffer.
    ///
    /// # Failure
    ///
    /// Fails if `buffer.as_ref().len() < get_num_loaders()`.
    ///
    /// # Return
    ///
    /// Number if written module types on success, error otherwise.
    fn get_module_types(&self, buffer: impl AsMut<[ModuleType]>) -> Result<usize, Error<Owned>>;

    /// Copies the descriptors of the exported interfaces into a buffer.
    ///
    /// # Failure
    ///
    /// Fails if `buffer.as_ref().len() < get_num_exported_interfaces()`.
    ///
    /// # Return
    ///
    /// Number if written descriptors on success, error otherwise.
    fn get_exported_interfaces(
        &self,
        buffer: impl AsMut<[InterfaceDescriptor]>,
    ) -> Result<usize, Error<Owned>>;

    /// Fetches the module handle of the exported interface.
    ///
    /// # Failure
    ///
    /// Fails if `interface` does not exist.
    ///
    /// # Return
    ///
    /// Module handle on success, error otherwise.
    fn get_exported_interface_handle(
        &self,
        interface: &InterfaceDescriptor,
    ) -> Result<Module<'interface, BorrowImmutable<'_>>, Error<Owned>>;

    /// Creates a new unlinked module handle.
    ///
    /// # Return
    ///
    /// Module handle.
    ///
    /// # Safety
    ///
    /// The handle remains invalid until it's linked with [ModuleAPI::link_module].
    unsafe fn create_module_handle(&mut self) -> Module<'interface, Owned>;

    /// Links a module handle to an internal module handle.
    ///
    /// # Failure
    ///
    /// Fails if `module` or`loader` are invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// Removing the handle does not unload the module.
    unsafe fn remove_module_handle(
        &mut self,
        module: Module<'_, Owned>,
    ) -> Result<(), Error<Owned>>;

    /// Links a module handle to an internal module handle.
    ///
    /// # Failure
    ///
    /// Fails if `module` or`loader` are invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// Incorrect usage can lead to dangling handles or use-after-free errors.
    unsafe fn link_module<'module, 'loader, O, LO, IO>(
        &mut self,
        module: &Module<'module, O>,
        loader: &Loader<'loader, LO>,
        internal: &InternalModule<IO>,
    ) -> Result<(), Error<Owned>>
    where
        'loader: 'module,
        O: MutableAccessIdentifier,
        LO: ImmutableAccessIdentifier,
        IO: ImmutableAccessIdentifier;

    /// Fetches the internal handle linked with the module handle.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid.
    ///
    /// # Return
    ///
    /// Internal handle on success, error otherwise.
    fn get_internal_module_handle<'module, O>(
        &self,
        module: &Module<'module, O>,
    ) -> Result<InternalModule<O>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

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
    fn add_module<O>(
        &mut self,
        loader: &Loader<'interface, O>,
        path: impl AsRef<Path>,
    ) -> Result<Module<'interface, Owned>, Error<Owned>>
    where
        O: MutableAccessIdentifier;

    /// Removes a module.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid or the module is not in an unloaded state.
    ///
    /// # Return
    ///
    /// Error on failure.
    fn remove_module(&mut self, module: Module<'_, Owned>) -> Result<(), Error<Owned>>;

    /// Loads a module.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid, the load dependencies of the module are
    /// not exported or the module is not in an unloaded state.
    ///
    /// # Return
    ///
    /// Error on failure.
    fn load<O>(&mut self, module: &mut Module<'_, O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier;

    /// Unloads a module.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid or the module is in an unloaded or ready state.
    ///
    /// # Return
    ///
    /// Error on failure.
    fn unload<O>(&mut self, module: &mut Module<'_, O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier;

    /// Initializes a module.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid, the runtime dependencies of the
    /// module are not exported or the module is not in a loaded state.
    ///
    /// # Return
    ///
    /// Error on failure.
    fn initialize<O>(&mut self, module: &mut Module<'_, O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier;

    /// Terminates a module.
    ///
    /// Terminating a module also removes the interfaces it exported.
    /// The modules that depend on the module are terminated.
    /// If they list the module as a load dependency, they are also unloaded.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid or the module is not in a ready state.
    ///
    /// # Return
    ///
    /// Error on failure.
    fn terminate<O>(&mut self, module: &mut Module<'_, O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier;

    /// Registers a new runtime dependency of the module.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    fn add_dependency<O>(
        &mut self,
        module: &mut Module<'_, O>,
        interface: &InterfaceDescriptor,
    ) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier;

    /// Removes an existing runtime dependency from the module.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    fn remove_dependency<O>(
        &mut self,
        module: &mut Module<'_, O>,
        interface: &InterfaceDescriptor,
    ) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier;

    /// Exports an interface of a module.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid, `interface` is already exported,
    /// `interface` is not contained in the module or the module is not yet initialized.
    ///
    /// # Return
    ///
    /// Error on failure.
    fn export_interface<O>(
        &mut self,
        module: &Module<'_, O>,
        interface: &InterfaceDescriptor,
    ) -> Result<(), Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

    /// Fetches the load dependencies of a module.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid.
    ///
    /// # Return
    ///
    /// Load dependencies on success, error otherwise.
    fn get_load_dependencies<'module, O>(
        &self,
        module: &Module<'module, O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

    /// Fetches the runtime dependencies of a module.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid or the module is not yet loaded.
    ///
    /// # Return
    ///
    /// Runtime dependencies on success, error otherwise.
    fn get_runtime_dependencies<'module, O>(
        &self,
        module: &Module<'module, O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

    /// Fetches the exportable interfaces of a module.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid or the module is not yet loaded.
    ///
    /// # Return
    ///
    /// Exportable interfaces on success, error otherwise.
    fn get_exportable_interfaces<'module, O>(
        &self,
        module: &Module<'module, O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

    /// Fetches the load status of a module.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid.
    ///
    /// # Return
    ///
    /// Module status on success, error otherwise.
    fn fetch_status<O>(&self, module: &Module<'_, O>) -> Result<ModuleStatus, Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

    /// Fetches the path a module was loaded from.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid or the module is not yet loaded.
    ///
    /// # Return
    ///
    /// Module path on success, error otherwise.
    fn get_module_path<'module, O>(
        &self,
        module: &Module<'module, O>,
    ) -> Result<&'module [OSPathChar], Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

    /// Fetches the module info from a module.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid or the module is not yet loaded.
    ///
    /// # Return
    ///
    /// Module info on success, error otherwise.
    fn get_module_info<'module, O>(
        &self,
        module: &Module<'module, O>,
    ) -> Result<&'module ModuleInfo, Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

    /// Fetches an interface from a module.
    ///
    /// # Failure
    ///
    /// Fails if `module` is invalid, the module is not in a ready
    /// state or the interface is not contained in the module.
    ///
    /// # Return
    ///
    /// Interface on success, error otherwise.
    fn get_interface<'module, O, T>(
        &self,
        module: &'module Module<'_, O>,
        interface: &InterfaceDescriptor,
        caster: impl FnOnce(crate::ffi::module::Interface) -> T,
    ) -> Result<Interface<'module, T>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier;
}

impl<'interface, T> ModuleAPI<'interface> for T
where
    T: ModuleBinding,
{
    #[inline]
    fn register_loader<LT, L>(
        &mut self,
        loader: Pin<&'interface LT>,
        mod_type: impl AsRef<str>,
    ) -> Result<Loader<'interface, Owned>, Error<Owned>>
    where
        L: ModuleLoaderAPI<'interface> + ModuleLoaderABICompat,
        ModuleLoader<L, Owned>: From<&'interface LT>,
    {
        let mod_str = mod_type.as_ref();
        if mod_str.as_bytes().len() > MODULE_LOADER_TYPE_MAX_LENGTH {
            return Err(Error::from(SimpleError::new(format!(
                "{}: {}",
                MODULE_TYPE_LENGTH_ERROR, mod_str
            ))));
        }

        let mod_type = ModuleType::from(mod_str);

        unsafe {
            self.register_loader(
                ModuleLoader::<L, Owned>::from(loader.get_ref()).to_raw(),
                NonNullConst::from(&mod_type),
            )
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |v| Ok(Loader::new(v)))
        }
    }

    #[inline]
    fn unregister_loader(&mut self, loader: Loader<'_, Owned>) -> Result<(), Error<Owned>> {
        unsafe {
            self.unregister_loader(loader.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
        }
    }

    #[inline]
    fn get_loader_interface<'loader, O, L>(
        &self,
        loader: &Loader<'loader, O>,
    ) -> Result<ModuleLoader<L, O>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
        L: ModuleLoaderAPI<'loader> + ModuleLoaderABICompat,
    {
        unsafe {
            self.get_loader_interface(loader.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |v| Ok(ModuleLoader::from_raw(v)))
        }
    }

    #[inline]
    fn get_loader_handle_from_type(
        &self,
        mod_type: impl AsRef<str>,
    ) -> Result<Loader<'interface, BorrowMutable<'_>>, Error<Owned>> {
        let mod_str = mod_type.as_ref();
        if mod_str.as_bytes().len() > MODULE_LOADER_TYPE_MAX_LENGTH {
            return Err(Error::from(SimpleError::new(format!(
                "{}: {}",
                MODULE_TYPE_LENGTH_ERROR, mod_str
            ))));
        }

        let mod_type = ModuleType::from(mod_str);

        unsafe {
            self.get_loader_handle_from_type(NonNullConst::from(&mod_type))
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |v| Ok(Loader::new(v)))
        }
    }

    #[inline]
    fn get_loader_handle_from_module<'module, O>(
        &self,
        module: &Module<'module, O>,
    ) -> Result<Loader<'module, BorrowMutable<'_>>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe {
            self.get_loader_handle_from_module(module.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |v| Ok(Loader::new(v)))
        }
    }

    #[inline]
    fn get_num_modules(&self) -> usize {
        unsafe { self.get_num_modules() }
    }

    #[inline]
    fn get_num_loaders(&self) -> usize {
        unsafe { self.get_num_loaders() }
    }

    #[inline]
    fn get_num_exported_interfaces(&self) -> usize {
        unsafe { self.get_num_exported_interfaces() }
    }

    #[inline]
    fn module_exists<'module, O>(&self, module: &Module<'module, O>) -> bool
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe { self.module_exists(module.as_handle()) == Bool::True }
    }

    #[inline]
    fn type_exists(&self, mod_type: impl AsRef<str>) -> Result<bool, Error<Owned>> {
        let mod_str = mod_type.as_ref();
        if mod_str.as_bytes().len() > MODULE_LOADER_TYPE_MAX_LENGTH {
            return Err(Error::from(SimpleError::new(format!(
                "{}: {}",
                MODULE_TYPE_LENGTH_ERROR, mod_str
            ))));
        }

        let mod_type = ModuleType::from(mod_str);

        unsafe { Ok(self.type_exists(NonNullConst::from(&mod_type)) == Bool::True) }
    }

    #[inline]
    fn exported_interface_exists(&self, interface: &InterfaceDescriptor) -> bool {
        unsafe { self.exported_interface_exists(NonNullConst::from(interface)) == Bool::True }
    }

    #[inline]
    fn get_modules(&self, mut buffer: impl AsMut<[ModuleInfo]>) -> Result<usize, Error<Owned>> {
        unsafe {
            self.get_modules(MutSpan::from(buffer.as_mut()))
                .into_rust()
                .map_err(Error::from)
        }
    }

    #[inline]
    fn get_module_types(
        &self,
        mut buffer: impl AsMut<[ModuleType]>,
    ) -> Result<usize, Error<Owned>> {
        unsafe {
            self.get_module_types(MutSpan::from(buffer.as_mut()))
                .into_rust()
                .map_err(Error::from)
        }
    }

    #[inline]
    fn get_exported_interfaces(
        &self,
        mut buffer: impl AsMut<[InterfaceDescriptor]>,
    ) -> Result<usize, Error<Owned>> {
        unsafe {
            self.get_exported_interfaces(MutSpan::from(buffer.as_mut()))
                .into_rust()
                .map_err(Error::from)
        }
    }

    #[inline]
    fn get_exported_interface_handle(
        &self,
        interface: &InterfaceDescriptor,
    ) -> Result<Module<'interface, BorrowImmutable<'_>>, Error<Owned>> {
        unsafe {
            self.get_exported_interface_handle(NonNullConst::from(interface))
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |v| Ok(Module::new(v)))
        }
    }

    #[inline]
    unsafe fn create_module_handle(&mut self) -> Module<'interface, Owned> {
        Module::new(self.create_module_handle())
    }

    #[inline]
    unsafe fn remove_module_handle(
        &mut self,
        module: Module<'_, Owned>,
    ) -> Result<(), Error<Owned>> {
        self.remove_module_handle(module.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
    }

    #[inline]
    unsafe fn link_module<'module, 'loader, O, LO, IO>(
        &mut self,
        module: &Module<'module, O>,
        loader: &Loader<'loader, LO>,
        internal: &InternalModule<IO>,
    ) -> Result<(), Error<Owned>>
    where
        'loader: 'module,
        O: MutableAccessIdentifier,
        LO: ImmutableAccessIdentifier,
        IO: ImmutableAccessIdentifier,
    {
        self.link_module(module.as_handle(), loader.as_handle(), internal.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
    }

    #[inline]
    fn get_internal_module_handle<'module, O>(
        &self,
        module: &Module<'module, O>,
    ) -> Result<InternalModule<O>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe {
            self.get_internal_module_handle(module.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |v| Ok(InternalModule::new(v)))
        }
    }

    #[inline]
    fn add_module<O>(
        &mut self,
        loader: &Loader<'interface, O>,
        path: impl AsRef<Path>,
    ) -> Result<Module<'interface, Owned>, Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        let path_buff = path.as_ref().to_os_path_buff_null();
        unsafe {
            self.add_module(loader.as_handle(), OSPathString::from(path_buff.as_slice()))
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |v| Ok(Module::new(v)))
        }
    }

    #[inline]
    fn remove_module(&mut self, module: Module<'_, Owned>) -> Result<(), Error<Owned>> {
        unsafe {
            self.remove_module(module.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
        }
    }

    #[inline]
    fn load<O>(&mut self, module: &mut Module<'_, O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        unsafe {
            self.load(module.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
        }
    }

    #[inline]
    fn unload<O>(&mut self, module: &mut Module<'_, O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        unsafe {
            self.unload(module.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
        }
    }

    #[inline]
    fn initialize<O>(&mut self, module: &mut Module<'_, O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        unsafe {
            self.initialize(module.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
        }
    }

    #[inline]
    fn terminate<O>(&mut self, module: &mut Module<'_, O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        unsafe {
            self.terminate(module.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
        }
    }

    #[inline]
    fn add_dependency<O>(
        &mut self,
        module: &mut Module<'_, O>,
        interface: &InterfaceDescriptor,
    ) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        unsafe {
            self.add_dependency(module.as_handle(), NonNullConst::from(interface))
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
        }
    }

    #[inline]
    fn remove_dependency<O>(
        &mut self,
        module: &mut Module<'_, O>,
        interface: &InterfaceDescriptor,
    ) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        unsafe {
            self.remove_dependency(module.as_handle(), NonNullConst::from(interface))
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
        }
    }

    #[inline]
    fn export_interface<O>(
        &mut self,
        module: &Module<'_, O>,
        interface: &InterfaceDescriptor,
    ) -> Result<(), Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe {
            self.export_interface(module.as_handle(), NonNullConst::from(interface))
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
        }
    }

    #[inline]
    fn get_load_dependencies<'module, O>(
        &self,
        module: &Module<'module, O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe {
            self.get_load_dependencies(module.as_handle())
                .into_rust()
                .map_or_else(
                    |e| Err(Error::from(e)),
                    |v| {
                        if v.is_empty() {
                            Ok(<&[_]>::default())
                        } else {
                            Ok(std::slice::from_raw_parts(v.as_ptr(), v.len()))
                        }
                    },
                )
        }
    }

    #[inline]
    fn get_runtime_dependencies<'module, O>(
        &self,
        module: &Module<'module, O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe {
            self.get_runtime_dependencies(module.as_handle())
                .into_rust()
                .map_or_else(
                    |e| Err(Error::from(e)),
                    |v| {
                        if v.is_empty() {
                            Ok(<&[_]>::default())
                        } else {
                            Ok(std::slice::from_raw_parts(v.as_ptr(), v.len()))
                        }
                    },
                )
        }
    }

    #[inline]
    fn get_exportable_interfaces<'module, O>(
        &self,
        module: &Module<'module, O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe {
            self.get_exportable_interfaces(module.as_handle())
                .into_rust()
                .map_or_else(
                    |e| Err(Error::from(e)),
                    |v| {
                        if v.is_empty() {
                            Ok(<&[_]>::default())
                        } else {
                            Ok(std::slice::from_raw_parts(v.as_ptr(), v.len()))
                        }
                    },
                )
        }
    }

    #[inline]
    fn fetch_status<O>(&self, module: &Module<'_, O>) -> Result<ModuleStatus, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe {
            self.fetch_status(module.as_handle())
                .into_rust()
                .map_err(Error::from)
        }
    }

    #[inline]
    fn get_module_path<'module, O>(
        &self,
        module: &Module<'module, O>,
    ) -> Result<&'module [OSPathChar], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe {
            self.get_module_path(module.as_handle())
                .into_rust()
                .map_or_else(
                    |e| Err(Error::from(e)),
                    |v| {
                        let slice = v.as_ref();
                        Ok(std::slice::from_raw_parts(slice.as_ptr(), slice.len()))
                    },
                )
        }
    }

    #[inline]
    fn get_module_info<'module, O>(
        &self,
        module: &Module<'module, O>,
    ) -> Result<&'module ModuleInfo, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe {
            self.get_module_info(module.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |v| Ok(&*v.as_ptr()))
        }
    }

    #[inline]
    fn get_interface<'module, O, IT>(
        &self,
        module: &'module Module<'_, O>,
        interface: &InterfaceDescriptor,
        caster: impl FnOnce(crate::ffi::module::Interface) -> IT,
    ) -> Result<Interface<'module, IT>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe {
            self.get_interface(module.as_handle(), NonNullConst::from(interface))
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |v| Ok(Interface::new(caster(v))))
        }
    }
}
