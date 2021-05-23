//! API of a module loader.
use crate::fat_ptr::FatPtr;
use crate::ffi::collections::NonNullConst;
use crate::ffi::library::{OSPathChar, OSPathString};
use crate::ffi::module::module_loader::{
    ModuleLoader as ModuleLoaderFFI, ModuleLoaderBinding, ModuleLoaderInterface,
    NativeModuleLoaderBinding, NativeModuleLoaderInterface,
};
use crate::module::native_module::{NativeModule, NativeModuleInstance};
use crate::module::{Interface, InterfaceDescriptor, InternalModule, ModuleInfo, ModuleStatus};
use crate::ownership::{
    AccessIdentifier, ImmutableAccessIdentifier, MutableAccessIdentifier, Owned,
};
use crate::Error;
use crate::ToOsPathBuff;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::path::Path;

/// Trait for identifying module loaders whose data layout is
/// compatible with the canonical module loader.
pub trait ModuleLoaderABICompat {
    /// Fetches a fat pointer that can be used with the interface.
    fn to_raw(&self) -> ModuleLoaderInterface;

    /// Construct a new instance from a fat pointer.
    ///
    /// # Safety
    ///
    /// This function should not be used directly.
    unsafe fn from_raw(handler: ModuleLoaderInterface) -> Self;
}

/// The API of a module loader.
pub trait ModuleLoaderAPI<'a> {
    /// Type of the extended loader.
    type ExtendedLoader: From<FatPtr<ModuleLoaderFFI, c_void>>;

    /// Construct a new instance from an untyped fat pointer.
    ///
    /// # Safety
    ///
    /// This function should not be used directly.
    unsafe fn from_fat_ptr(ptr: FatPtr<ModuleLoaderFFI, c_void>) -> Self;

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
    /// Direct usage of a [ModuleLoaderAPI] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn add_module(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<InternalModule<Owned>, Error<Owned>>;

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
    /// Direct usage of a [ModuleLoaderAPI] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn remove_module(&mut self, module: InternalModule<Owned>) -> Result<(), Error<Owned>>;

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
    /// Direct usage of a [ModuleLoaderAPI] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn load<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier;

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
    /// Direct usage of a [ModuleLoaderAPI] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn unload<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier;

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
    /// Direct usage of a [ModuleLoaderAPI] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn initialize<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier;

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
    /// Direct usage of a [ModuleLoaderAPI] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn terminate<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier;

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
    /// Direct usage of a [ModuleLoaderAPI] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn fetch_status<O>(
        &self,
        module: &InternalModule<O>,
    ) -> Result<ModuleStatus, Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

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
    /// Direct usage of a [ModuleLoaderAPI] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_interface<'module, O, T>(
        &self,
        module: &'module InternalModule<O>,
        interface: &InterfaceDescriptor,
        caster: impl FnOnce(crate::ffi::module::Interface) -> T,
    ) -> Result<Interface<'module, T>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

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
    /// Direct usage of a [ModuleLoaderAPI] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_module_info<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module ModuleInfo, Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

    /// Fetches the path a module was loaded from.
    ///
    /// The resulting slice is terminated with a `\0` character.
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
    /// Direct usage of a [ModuleLoaderAPI] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_module_path<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [OSPathChar], Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

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
    /// Direct usage of a [ModuleLoaderAPI] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_load_dependencies<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

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
    /// Direct usage of a [ModuleLoaderAPI] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_runtime_dependencies<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

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
    /// Direct usage of a [ModuleLoaderAPI] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_exportable_interfaces<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

    /// Fetches the extended loader.
    ///
    /// # Return
    ///
    /// Extended loader.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoaderAPI] may break some invariants
    /// of the module api, if not handled with care.
    unsafe fn get_extended_loader(&self) -> Self::ExtendedLoader;
}

/// A module loader.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ModuleLoader<T, O> {
    _loader: T,
    _ownership: PhantomData<fn() -> O>,
}

impl<'a, T, O> Deref for ModuleLoader<T, O>
where
    T: ModuleLoaderAPI<'a>,
    O: AccessIdentifier,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self._loader
    }
}

impl<'a, T, O> DerefMut for ModuleLoader<T, O>
where
    T: ModuleLoaderAPI<'a>,
    O: MutableAccessIdentifier,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._loader
    }
}

impl<'a, T, O> ModuleLoader<T, O>
where
    T: ModuleLoaderABICompat,
    O: AccessIdentifier,
{
    /// Fetches a fat pointer that can be used with the interface.
    #[inline]
    pub fn to_raw(&self) -> ModuleLoaderInterface {
        self._loader.to_raw()
    }

    /// Construct a new instance from a fat pointer.
    ///
    /// # Safety
    ///
    /// This function should not be used directly.
    #[inline]
    pub unsafe fn from_raw(loader: ModuleLoaderInterface) -> Self {
        Self {
            _loader: T::from_raw(loader),
            _ownership: PhantomData,
        }
    }
}

impl<'a, T, O> ModuleLoader<T, O>
where
    T: ModuleLoaderAPI<'a>,
    O: AccessIdentifier,
{
    /// Construct a new instance from an untyped fat pointer.
    ///
    /// # Safety
    ///
    /// This function should not be used directly.
    #[inline]
    pub unsafe fn from_void_ptr(ptr: FatPtr<ModuleLoaderFFI, c_void>) -> Self {
        Self {
            _loader: T::from_fat_ptr(ptr),
            _ownership: PhantomData,
        }
    }
}

impl<'a, T, O> ModuleLoader<T, O>
where
    T: ModuleLoaderAPI<'a>,
    O: MutableAccessIdentifier,
{
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
    /// Direct usage of a [ModuleLoader] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn add_module(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<InternalModule<Owned>, Error<Owned>> {
        self._loader.add_module(path)
    }

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
    /// Direct usage of a [ModuleLoader] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn remove_module(
        &mut self,
        module: InternalModule<Owned>,
    ) -> Result<(), Error<Owned>> {
        self._loader.remove_module(module)
    }

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
    /// Direct usage of a [ModuleLoader] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn load<MO>(&mut self, module: &mut InternalModule<MO>) -> Result<(), Error<Owned>>
    where
        MO: MutableAccessIdentifier,
    {
        self._loader.load(module)
    }

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
    /// Direct usage of a [ModuleLoader] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn unload<MO>(&mut self, module: &mut InternalModule<MO>) -> Result<(), Error<Owned>>
    where
        MO: MutableAccessIdentifier,
    {
        self._loader.unload(module)
    }

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
    /// Direct usage of a [ModuleLoader] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn initialize<MO>(
        &mut self,
        module: &mut InternalModule<MO>,
    ) -> Result<(), Error<Owned>>
    where
        MO: MutableAccessIdentifier,
    {
        self._loader.initialize(module)
    }

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
    /// Direct usage of a [ModuleLoader] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn terminate<MO>(
        &mut self,
        module: &mut InternalModule<MO>,
    ) -> Result<(), Error<Owned>>
    where
        MO: MutableAccessIdentifier,
    {
        self._loader.terminate(module)
    }
}

impl<'a, T, O> ModuleLoader<T, O>
where
    T: ModuleLoaderAPI<'a>,
    O: ImmutableAccessIdentifier,
{
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
    /// Direct usage of a [ModuleLoader] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn fetch_status<MO>(
        &self,
        module: &InternalModule<MO>,
    ) -> Result<ModuleStatus, Error<Owned>>
    where
        MO: ImmutableAccessIdentifier,
    {
        self._loader.fetch_status(module)
    }

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
    /// Direct usage of a [ModuleLoader] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn get_interface<'module, MO, IT>(
        &self,
        module: &'module InternalModule<MO>,
        interface: &InterfaceDescriptor,
        caster: impl FnOnce(crate::ffi::module::Interface) -> IT,
    ) -> Result<Interface<'module, IT>, Error<Owned>>
    where
        MO: ImmutableAccessIdentifier,
    {
        self._loader.get_interface(module, interface, caster)
    }

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
    /// Direct usage of a [ModuleLoader] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn get_module_info<'module, MO>(
        &self,
        module: &'module InternalModule<MO>,
    ) -> Result<&'module ModuleInfo, Error<Owned>>
    where
        MO: ImmutableAccessIdentifier,
    {
        self._loader.get_module_info(module)
    }

    /// Fetches the path a module was loaded from.
    ///
    /// The resulting slice is terminated with a `\0` character.
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
    /// Direct usage of a [ModuleLoader] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn get_module_path<'module, MO>(
        &self,
        module: &'module InternalModule<MO>,
    ) -> Result<&'module [OSPathChar], Error<Owned>>
    where
        MO: ImmutableAccessIdentifier,
    {
        self._loader.get_module_path(module)
    }

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
    /// Direct usage of a [ModuleLoader] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn get_load_dependencies<'module, MO>(
        &self,
        module: &'module InternalModule<MO>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        MO: ImmutableAccessIdentifier,
    {
        self._loader.get_load_dependencies(module)
    }

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
    /// Direct usage of a [ModuleLoader] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn get_runtime_dependencies<'module, MO>(
        &self,
        module: &'module InternalModule<MO>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        MO: ImmutableAccessIdentifier,
    {
        self._loader.get_runtime_dependencies(module)
    }

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
    /// Direct usage of a [ModuleLoader] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn get_exportable_interfaces<'module, MO>(
        &self,
        module: &'module InternalModule<MO>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        MO: ImmutableAccessIdentifier,
    {
        self._loader.get_exportable_interfaces(module)
    }

    /// Fetches the extended loader.
    ///
    /// # Return
    ///
    /// Extended loader.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [ModuleLoader] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn get_extended_loader(&self) -> ModuleLoader<T::ExtendedLoader, O> {
        ModuleLoader {
            _loader: self._loader.get_extended_loader(),
            _ownership: PhantomData,
        }
    }
}

/// Invalid type erased module loader.
#[derive(Debug, Copy, Clone, Hash)]
pub struct InvalidLoader {
    _ptr: FatPtr<ModuleLoaderFFI, c_void>,
}

unsafe impl Send for InvalidLoader {}
unsafe impl Sync for InvalidLoader {}

impl InvalidLoader {
    /// Constructs a new instance.
    #[inline]
    pub fn new(ptr: FatPtr<ModuleLoaderFFI, c_void>) -> Self {
        Self { _ptr: ptr }
    }
}

impl Deref for InvalidLoader {
    type Target = FatPtr<ModuleLoaderFFI, c_void>;

    fn deref(&self) -> &Self::Target {
        &self._ptr
    }
}

impl DerefMut for InvalidLoader {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._ptr
    }
}

impl From<FatPtr<ModuleLoaderFFI, c_void>> for InvalidLoader {
    fn from(val: FatPtr<ModuleLoaderFFI, c_void>) -> Self {
        Self::new(val)
    }
}

/// Type erased module loader.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct UnknownLoader<'loader> {
    _interface: ModuleLoaderInterface,
    _phantom: PhantomData<&'loader ()>,
}

unsafe impl Send for UnknownLoader<'_> {}
unsafe impl Sync for UnknownLoader<'_> {}

impl Deref for UnknownLoader<'_> {
    type Target = ModuleLoaderInterface;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self._interface
    }
}

impl DerefMut for UnknownLoader<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._interface
    }
}

impl From<FatPtr<ModuleLoaderFFI, c_void>> for UnknownLoader<'_> {
    fn from(val: FatPtr<ModuleLoaderFFI, c_void>) -> Self {
        unsafe { Self::from_fat_ptr(val) }
    }
}

impl ModuleLoaderABICompat for UnknownLoader<'_> {
    #[inline]
    fn to_raw(&self) -> ModuleLoaderInterface {
        self._interface
    }

    #[inline]
    unsafe fn from_raw(interface: ModuleLoaderInterface) -> Self {
        Self {
            _interface: interface,
            _phantom: PhantomData,
        }
    }
}

impl<'a> ModuleLoaderAPI<'a> for UnknownLoader<'a> {
    type ExtendedLoader = InvalidLoader;

    #[inline]
    unsafe fn from_fat_ptr(ptr: FatPtr<ModuleLoaderFFI, c_void>) -> Self {
        // Assumes that the vtable pointer is of the type `*const ModuleLoaderInterfaceVTable`.
        Self::from_raw(ModuleLoaderInterface {
            loader: ptr.data,
            vtable: ptr.vtable.cast(),
        })
    }

    #[inline]
    unsafe fn add_module(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<InternalModule<Owned>, Error<Owned>> {
        let path_buff = path.as_ref().to_os_path_buff_null();
        self._interface
            .add_module(OSPathString::from(path_buff.as_slice()))
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |v| Ok(InternalModule::new(v)))
    }

    #[inline]
    unsafe fn remove_module(&mut self, module: InternalModule<Owned>) -> Result<(), Error<Owned>> {
        self._interface
            .remove_module(module.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
    }

    #[inline]
    unsafe fn load<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        self._interface
            .load(module.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
    }

    #[inline]
    unsafe fn unload<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        self._interface
            .unload(module.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
    }

    #[inline]
    unsafe fn initialize<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        self._interface
            .initialize(module.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
    }

    #[inline]
    unsafe fn terminate<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        self._interface
            .terminate(module.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
    }

    #[inline]
    unsafe fn fetch_status<O>(
        &self,
        module: &InternalModule<O>,
    ) -> Result<ModuleStatus, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface
            .fetch_status(module.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), Ok)
    }

    #[inline]
    unsafe fn get_interface<'module, O, T>(
        &self,
        module: &'module InternalModule<O>,
        interface: &InterfaceDescriptor,
        caster: impl FnOnce(crate::ffi::module::Interface) -> T,
    ) -> Result<Interface<'module, T>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface
            .get_interface(module.as_handle(), NonNullConst::from(interface))
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |v| Ok(Interface::new(caster(v))))
    }

    #[inline]
    unsafe fn get_module_info<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module ModuleInfo, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface
            .get_module_info(module.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |v| Ok(&*v.as_ptr()))
    }

    #[inline]
    unsafe fn get_module_path<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [OSPathChar], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface
            .get_module_path(module.as_handle())
            .into_rust()
            .map_or_else(
                |e| Err(Error::from(e)),
                |v| {
                    let slice = v.as_ref();
                    Ok(std::slice::from_raw_parts(slice.as_ptr(), slice.len()))
                },
            )
    }

    #[inline]
    unsafe fn get_load_dependencies<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface
            .get_load_dependencies(module.as_handle())
            .into_rust()
            .map_or_else(
                |e| Err(Error::from(e)),
                |v| {
                    let slice = v.as_ref();
                    Ok(std::slice::from_raw_parts(slice.as_ptr(), slice.len()))
                },
            )
    }

    #[inline]
    unsafe fn get_runtime_dependencies<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface
            .get_runtime_dependencies(module.as_handle())
            .into_rust()
            .map_or_else(
                |e| Err(Error::from(e)),
                |v| {
                    let slice = v.as_ref();
                    Ok(std::slice::from_raw_parts(slice.as_ptr(), slice.len()))
                },
            )
    }

    #[inline]
    unsafe fn get_exportable_interfaces<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface
            .get_exportable_interfaces(module.as_handle())
            .into_rust()
            .map_or_else(
                |e| Err(Error::from(e)),
                |v| {
                    let slice = v.as_ref();
                    Ok(std::slice::from_raw_parts(slice.as_ptr(), slice.len()))
                },
            )
    }

    #[inline]
    unsafe fn get_extended_loader(&self) -> Self::ExtendedLoader {
        Self::ExtendedLoader::from(FatPtr::from_raw(
            self.loader,
            self._interface.get_extended_vtable(),
        ))
    }
}

/// Native module loader.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NativeLoader<'loader> {
    _interface: UnknownLoader<'loader>,
}

impl Deref for NativeLoader<'_> {
    type Target = ModuleLoaderInterface;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self._interface.deref()
    }
}

impl DerefMut for NativeLoader<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self._interface.deref_mut()
    }
}

impl From<FatPtr<ModuleLoaderFFI, c_void>> for NativeLoader<'_> {
    fn from(val: FatPtr<ModuleLoaderFFI, c_void>) -> Self {
        unsafe { Self::from_fat_ptr(val) }
    }
}

impl ModuleLoaderABICompat for NativeLoader<'_> {
    #[inline]
    fn to_raw(&self) -> ModuleLoaderInterface {
        self._interface.to_raw()
    }

    #[inline]
    unsafe fn from_raw(interface: ModuleLoaderInterface) -> Self {
        Self {
            _interface: UnknownLoader::from_raw(interface),
        }
    }
}

impl<'a> ModuleLoaderAPI<'a> for NativeLoader<'a> {
    type ExtendedLoader = NativeLoaderInternal<'a>;

    #[inline]
    unsafe fn from_fat_ptr(ptr: FatPtr<ModuleLoaderFFI, c_void>) -> Self {
        Self {
            _interface: UnknownLoader::from_fat_ptr(ptr),
        }
    }

    #[inline]
    unsafe fn add_module(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<InternalModule<Owned>, Error<Owned>> {
        self._interface.add_module(path)
    }

    #[inline]
    unsafe fn remove_module(&mut self, module: InternalModule<Owned>) -> Result<(), Error<Owned>> {
        self._interface.remove_module(module)
    }

    #[inline]
    unsafe fn load<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        self._interface.load(module)
    }

    #[inline]
    unsafe fn unload<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        self._interface.unload(module)
    }

    #[inline]
    unsafe fn initialize<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        self._interface.initialize(module)
    }

    #[inline]
    unsafe fn terminate<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        self._interface.terminate(module)
    }

    #[inline]
    unsafe fn fetch_status<O>(
        &self,
        module: &InternalModule<O>,
    ) -> Result<ModuleStatus, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface.fetch_status(module)
    }

    #[inline]
    unsafe fn get_interface<'module, O, T>(
        &self,
        module: &'module InternalModule<O>,
        interface: &InterfaceDescriptor,
        caster: impl FnOnce(crate::ffi::module::Interface) -> T,
    ) -> Result<Interface<'module, T>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface.get_interface(module, interface, caster)
    }

    #[inline]
    unsafe fn get_module_info<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module ModuleInfo, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface.get_module_info(module)
    }

    #[inline]
    unsafe fn get_module_path<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [OSPathChar], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface.get_module_path(module)
    }

    #[inline]
    unsafe fn get_load_dependencies<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface.get_load_dependencies(module)
    }

    #[inline]
    unsafe fn get_runtime_dependencies<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface.get_runtime_dependencies(module)
    }

    #[inline]
    unsafe fn get_exportable_interfaces<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface.get_exportable_interfaces(module)
    }

    #[inline]
    unsafe fn get_extended_loader(&self) -> Self::ExtendedLoader {
        Self::ExtendedLoader::from(self._interface.get_extended_loader()._ptr)
    }
}

/// Native library loader internal interface.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NativeLoaderInternal<'loader> {
    _interface: NativeModuleLoaderInterface,
    _phantom: PhantomData<&'loader ()>,
}

unsafe impl Send for NativeLoaderInternal<'_> {}
unsafe impl Sync for NativeLoaderInternal<'_> {}

impl Deref for NativeLoaderInternal<'_> {
    type Target = NativeModuleLoaderInterface;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self._interface
    }
}

impl DerefMut for NativeLoaderInternal<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._interface
    }
}

impl From<FatPtr<ModuleLoaderFFI, c_void>> for NativeLoaderInternal<'_> {
    fn from(val: FatPtr<ModuleLoaderFFI, c_void>) -> Self {
        unsafe { Self::from_fat_ptr(val) }
    }
}

impl<'a> ModuleLoaderAPI<'a> for NativeLoaderInternal<'a> {
    type ExtendedLoader = Self;

    #[inline]
    unsafe fn from_fat_ptr(ptr: FatPtr<ModuleLoaderFFI, c_void>) -> Self {
        // Assumes that the vtable has the type `*const NativeModuleLoaderVTable`.
        Self {
            _interface: NativeModuleLoaderInterface {
                loader: ptr.data,
                vtable: ptr.vtable.cast(),
            },
            _phantom: PhantomData,
        }
    }

    #[inline]
    unsafe fn add_module(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<InternalModule<Owned>, Error<Owned>> {
        NativeLoader::from_fat_ptr(FatPtr::from_raw(
            self.loader,
            self.vtable.as_ref().loader_vtable.cast(),
        ))
        .add_module(path)
    }

    #[inline]
    unsafe fn remove_module(&mut self, module: InternalModule<Owned>) -> Result<(), Error<Owned>> {
        NativeLoader::from_fat_ptr(FatPtr::from_raw(
            self.loader,
            self.vtable.as_ref().loader_vtable.cast(),
        ))
        .remove_module(module)
    }

    #[inline]
    unsafe fn load<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        NativeLoader::from_fat_ptr(FatPtr::from_raw(
            self.loader,
            self.vtable.as_ref().loader_vtable.cast(),
        ))
        .load(module)
    }

    #[inline]
    unsafe fn unload<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        NativeLoader::from_fat_ptr(FatPtr::from_raw(
            self.loader,
            self.vtable.as_ref().loader_vtable.cast(),
        ))
        .unload(module)
    }

    #[inline]
    unsafe fn initialize<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        NativeLoader::from_fat_ptr(FatPtr::from_raw(
            self.loader,
            self.vtable.as_ref().loader_vtable.cast(),
        ))
        .initialize(module)
    }

    #[inline]
    unsafe fn terminate<O>(&mut self, module: &mut InternalModule<O>) -> Result<(), Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        NativeLoader::from_fat_ptr(FatPtr::from_raw(
            self.loader,
            self.vtable.as_ref().loader_vtable.cast(),
        ))
        .terminate(module)
    }

    #[inline]
    unsafe fn fetch_status<O>(
        &self,
        module: &InternalModule<O>,
    ) -> Result<ModuleStatus, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        NativeLoader::from_fat_ptr(FatPtr::from_raw(
            self.loader,
            self.vtable.as_ref().loader_vtable.cast(),
        ))
        .fetch_status(module)
    }

    #[inline]
    unsafe fn get_interface<'module, O, T>(
        &self,
        module: &'module InternalModule<O>,
        interface: &InterfaceDescriptor,
        caster: impl FnOnce(crate::ffi::module::Interface) -> T,
    ) -> Result<Interface<'module, T>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        NativeLoader::from_fat_ptr(FatPtr::from_raw(
            self.loader,
            self.vtable.as_ref().loader_vtable.cast(),
        ))
        .get_interface(module, interface, caster)
    }

    #[inline]
    unsafe fn get_module_info<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module ModuleInfo, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        NativeLoader::from_fat_ptr(FatPtr::from_raw(
            self.loader,
            self.vtable.as_ref().loader_vtable.cast(),
        ))
        .get_module_info(module)
    }

    #[inline]
    unsafe fn get_module_path<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [OSPathChar], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        NativeLoader::from_fat_ptr(FatPtr::from_raw(
            self.loader,
            self.vtable.as_ref().loader_vtable.cast(),
        ))
        .get_module_path(module)
    }

    #[inline]
    unsafe fn get_load_dependencies<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        NativeLoader::from_fat_ptr(FatPtr::from_raw(
            self.loader,
            self.vtable.as_ref().loader_vtable.cast(),
        ))
        .get_load_dependencies(module)
    }

    #[inline]
    unsafe fn get_runtime_dependencies<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        NativeLoader::from_fat_ptr(FatPtr::from_raw(
            self.loader,
            self.vtable.as_ref().loader_vtable.cast(),
        ))
        .get_runtime_dependencies(module)
    }

    #[inline]
    unsafe fn get_exportable_interfaces<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<&'module [InterfaceDescriptor], Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        NativeLoader::from_fat_ptr(FatPtr::from_raw(
            self.loader,
            self.vtable.as_ref().loader_vtable.cast(),
        ))
        .get_exportable_interfaces(module)
    }

    #[inline]
    unsafe fn get_extended_loader(&self) -> Self::ExtendedLoader {
        *self
    }
}

impl<'a> NativeLoaderInternal<'a> {
    /// Fetches the native module handle.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid.
    ///
    /// # Return
    ///
    /// Native module handle.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeLoaderInternal] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn get_native_module<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<NativeModuleInstance<'module, O>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface
            .get_native_module(module.as_handle())
            .into_rust()
            .map_or_else(
                |e| Err(Error::from(e)),
                |v| Ok(NativeModuleInstance::new(v)),
            )
    }

    /// Fetches the native module interface.
    ///
    /// # Failure
    ///
    /// The function fails if `module` is invalid.
    ///
    /// # Return
    ///
    /// Native module interface.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeLoaderInternal] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn get_native_module_interface<'module, O>(
        &self,
        module: &'module InternalModule<O>,
    ) -> Result<NativeModule<'module, O>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface
            .get_native_module_interface(module.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |v| Ok(NativeModule::new(v)))
    }
}
