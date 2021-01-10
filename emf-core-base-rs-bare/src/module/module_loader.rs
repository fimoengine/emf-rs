use crate::ffi::module::{ModuleLoaderInterfaceBinding, NativeModuleLoaderInterfaceBinding};
use crate::library::{native_buff_ptr_to_os_str, os_str_to_native_buff};
use crate::module::native_module::NativeModuleInstance;
use crate::module::{
    InterfaceDescriptor, LoaderModuleHandle, LoaderModuleHandleRef, ModuleError, ModuleInfo,
    ModuleInterface, ModuleStatus,
};
use crate::{ffi, FFIObject};
use ffi::containers::NonNullConst;
use std::os::raw::c_void;
use std::path::{Path, PathBuf};

/// A trait providing the interface of a `ModuleLoader`.
pub trait ModuleLoaderWrapper<'a>:
    AsRef<ffi::module::ModuleLoaderInterface>
    + FFIObject<&'a ffi::module::ModuleLoaderInterface>
    + Sized
{
    /// Adds a new module using the `ModuleLoader`.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn add_module<'b, T: AsRef<Path>>(
        &self,
        path: &T,
    ) -> Result<LoaderModuleHandle<'b, 'a, Self>, ModuleError>;

    /// Removes a module using the `ModuleLoader`.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn remove_module<'b>(
        &self,
        module: LoaderModuleHandle<'b, 'a, Self>,
    ) -> Option<ModuleError>;

    /// Loads a module using the `ModuleLoader`.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn load<'b>(&self, module: &LoaderModuleHandle<'b, 'a, Self>) -> Option<ModuleError>;

    /// Unloads a module using the `ModuleLoader`.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn unload<'b>(&self, module: &LoaderModuleHandle<'b, 'a, Self>) -> Option<ModuleError>;

    /// Initializes a module using the `ModuleLoader`.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn initialize<'b>(
        &self,
        module: &LoaderModuleHandle<'b, 'a, Self>,
    ) -> Option<ModuleError>;

    /// Terminates a module using the `ModuleLoader`.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn terminate<'b>(
        &self,
        module: &LoaderModuleHandle<'b, 'a, Self>,
    ) -> Option<ModuleError>;

    /// Fetches the status of a module using the `ModuleLoader`.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn fetch_status<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<ModuleStatus, ModuleError>;

    /// Fetches the module info of a module using the `ModuleLoader`.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn get_module_info<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<&'b ModuleInfo, ModuleError>;

    /// Fetches the module path of a module using the `ModuleLoader`.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn get_module_path<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<PathBuf, ModuleError>;

    /// Fetches the load dependencies of a module using the `ModuleLoader`.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn get_load_dependencies<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError>;

    /// Fetches the runtime dependencies of a module using the `ModuleLoader`.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn get_runtime_dependencies<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError>;

    /// Fetches the exportable interfaces of a module using the `ModuleLoader`.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn get_exportable_interfaces<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError>;

    /// Fetches an interface of a module using the `ModuleLoader`.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn get_interface<'b, T: Sized + FFIObject<ffi::module::ModuleInterface>>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
        interface: &InterfaceDescriptor,
    ) -> Result<ModuleInterface<'b, T>, ModuleError>;

    /// Fetches the internal interface of the loader.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn get_internal_interface<T: 'a + Sized + FFIObject<NonNullConst<c_void>>>(&self) -> T;
}

/// A `ModuleLoader`
#[derive(Debug)]
pub struct ModuleLoader<'a> {
    interface: &'a ffi::module::ModuleLoaderInterface,
}

impl<'a> ModuleLoader<'a> {
    /// Extends the lifetime of the `ModuleLoader`.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the `ModuleLoader` lives long enough.
    #[inline]
    pub unsafe fn extend_lifetime<'b>(self) -> ModuleLoader<'b> {
        ModuleLoader {
            interface: std::mem::transmute(self.interface),
        }
    }
}

impl<'a> ModuleLoaderWrapper<'a> for ModuleLoader<'a> {
    #[inline]
    unsafe fn add_module<'b, T: AsRef<Path>>(
        &self,
        path: &T,
    ) -> Result<LoaderModuleHandle<'b, 'a, Self>, ModuleError> {
        let path = path.as_ref().as_os_str();
        let native_path_buff = os_str_to_native_buff(path);
        self.interface
            .add_module(NonNullConst::new_unchecked(native_path_buff.as_ptr()))
            .to_native()
            .map(|h| LoaderModuleHandle::from_native(h))
    }

    #[inline]
    #[must_use]
    unsafe fn remove_module<'b>(
        &self,
        module: LoaderModuleHandle<'b, 'a, Self>,
    ) -> Option<ModuleError> {
        self.interface.remove_module(module.as_native()).to_native()
    }

    #[inline]
    #[must_use]
    unsafe fn load<'b>(&self, module: &LoaderModuleHandle<'b, 'a, Self>) -> Option<ModuleError> {
        self.interface.load(module.as_native()).to_native()
    }

    #[inline]
    #[must_use]
    unsafe fn unload<'b>(&self, module: &LoaderModuleHandle<'b, 'a, Self>) -> Option<ModuleError> {
        self.interface.unload(module.as_native()).to_native()
    }

    #[inline]
    #[must_use]
    unsafe fn initialize<'b>(
        &self,
        module: &LoaderModuleHandle<'b, 'a, Self>,
    ) -> Option<ModuleError> {
        self.interface.initialize(module.as_native()).to_native()
    }

    #[inline]
    #[must_use]
    unsafe fn terminate<'b>(
        &self,
        module: &LoaderModuleHandle<'b, 'a, Self>,
    ) -> Option<ModuleError> {
        self.interface.terminate(module.as_native()).to_native()
    }

    #[inline]
    unsafe fn fetch_status<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<ModuleStatus, ModuleError> {
        self.interface.fetch_status(module.as_native()).to_native()
    }

    #[inline]
    unsafe fn get_module_info<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<&'b ModuleInfo, ModuleError> {
        self.interface
            .get_module_info(module.as_native())
            .to_native()
            .map(|m| &*m.as_ptr())
    }

    #[inline]
    unsafe fn get_module_path<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<PathBuf, ModuleError> {
        self.interface
            .get_module_path(module.as_native())
            .to_native()
            .map(|p| native_buff_ptr_to_os_str(p).into())
    }

    #[inline]
    unsafe fn get_load_dependencies<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError> {
        self.interface
            .get_load_dependencies(module.as_native())
            .to_native()
            .map(|dep| std::slice::from_raw_parts(dep.as_ptr(), dep.len()))
    }

    #[inline]
    unsafe fn get_runtime_dependencies<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError> {
        self.interface
            .get_runtime_dependencies(module.as_native())
            .to_native()
            .map(|dep| std::slice::from_raw_parts(dep.as_ptr(), dep.len()))
    }

    #[inline]
    unsafe fn get_exportable_interfaces<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError> {
        self.interface
            .get_exportable_interfaces(module.as_native())
            .to_native()
            .map(|dep| std::slice::from_raw_parts(dep.as_ptr(), dep.len()))
    }

    #[inline]
    unsafe fn get_interface<'b, T: Sized + FFIObject<ffi::module::ModuleInterface>>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
        interface: &InterfaceDescriptor,
    ) -> Result<ModuleInterface<'b, T>, ModuleError> {
        self.interface
            .get_interface(module.as_native(), NonNullConst::from(interface))
            .to_native()
            .map(|i| ModuleInterface::from_native(i))
    }

    #[inline]
    #[must_use]
    unsafe fn get_internal_interface<T: 'a + Sized + FFIObject<NonNullConst<c_void>>>(&self) -> T {
        T::from_native(self.interface.get_internal_interface())
    }
}

impl<'a> AsRef<ffi::module::ModuleLoaderInterface> for ModuleLoader<'a> {
    fn as_ref(&self) -> &'a ffi::module::ModuleLoaderInterface {
        self.interface
    }
}

impl<'a> FFIObject<&'a ffi::module::ModuleLoaderInterface> for ModuleLoader<'a> {
    fn as_native(&self) -> &'a ffi::module::ModuleLoaderInterface {
        self.interface
    }

    unsafe fn from_native(val: &'a ffi::module::ModuleLoaderInterface) -> Self {
        Self { interface: val }
    }
}

/// Functionalities of the `NativeLibraryLoader`
pub trait NativeModuleLoaderWrapper<'a>:
    AsRef<ffi::module::NativeModuleLoaderInterface>
    + FFIObject<&'a ffi::module::NativeModuleLoaderInterface>
    + ModuleLoaderWrapper<'a>
{
    /// Fetches a reference which can be used with the functions of a `NativeModule`.
    ///
    /// # Safety
    ///
    /// Direct usage of a `ModuleLoader` circumvents the safety of the `module` api.
    unsafe fn get_native_module<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<NativeModuleInstance<'b>, ModuleError>;
}

/// A native `ModuleLoader`
#[derive(Debug)]
pub struct NativeModuleLoader<'a> {
    interface: &'a ffi::module::NativeModuleLoaderInterface,
}

impl<'a> ModuleLoaderWrapper<'a> for NativeModuleLoader<'a> {
    #[inline]
    unsafe fn add_module<'b, T: AsRef<Path>>(
        &self,
        path: &T,
    ) -> Result<LoaderModuleHandle<'b, 'a, Self>, ModuleError> {
        ModuleLoader::<'a>::from_native(self.interface.module_loader_interface.as_ref())
            .add_module(path)
            .map(|h| h.cast())
    }

    #[inline]
    #[must_use]
    unsafe fn remove_module<'b>(
        &self,
        module: LoaderModuleHandle<'b, 'a, Self>,
    ) -> Option<ModuleError> {
        ModuleLoader::<'a>::from_native(self.interface.module_loader_interface.as_ref())
            .remove_module(module.cast())
    }

    #[inline]
    #[must_use]
    unsafe fn load<'b>(&self, module: &LoaderModuleHandle<'b, 'a, Self>) -> Option<ModuleError> {
        ModuleLoader::<'a>::from_native(self.interface.module_loader_interface.as_ref())
            .load(&module.cast_ref())
    }

    #[inline]
    #[must_use]
    unsafe fn unload<'b>(&self, module: &LoaderModuleHandle<'b, 'a, Self>) -> Option<ModuleError> {
        ModuleLoader::<'a>::from_native(self.interface.module_loader_interface.as_ref())
            .unload(&module.cast_ref())
    }

    #[inline]
    #[must_use]
    unsafe fn initialize<'b>(
        &self,
        module: &LoaderModuleHandle<'b, 'a, Self>,
    ) -> Option<ModuleError> {
        ModuleLoader::<'a>::from_native(self.interface.module_loader_interface.as_ref())
            .initialize(&module.cast_ref())
    }

    #[inline]
    #[must_use]
    unsafe fn terminate<'b>(
        &self,
        module: &LoaderModuleHandle<'b, 'a, Self>,
    ) -> Option<ModuleError> {
        ModuleLoader::<'a>::from_native(self.interface.module_loader_interface.as_ref())
            .terminate(&module.cast_ref())
    }

    #[inline]
    unsafe fn fetch_status<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<ModuleStatus, ModuleError> {
        ModuleLoader::<'a>::from_native(self.interface.module_loader_interface.as_ref())
            .fetch_status(&module.cast_ref())
    }

    #[inline]
    unsafe fn get_module_info<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<&'b ModuleInfo, ModuleError> {
        ModuleLoader::<'a>::from_native(self.interface.module_loader_interface.as_ref())
            .get_module_info(&module.cast_ref())
    }

    #[inline]
    unsafe fn get_module_path<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<PathBuf, ModuleError> {
        ModuleLoader::<'a>::from_native(self.interface.module_loader_interface.as_ref())
            .get_module_path(&module.cast_ref())
    }

    #[inline]
    unsafe fn get_load_dependencies<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError> {
        ModuleLoader::<'a>::from_native(self.interface.module_loader_interface.as_ref())
            .get_load_dependencies(&module.cast_ref())
    }

    #[inline]
    unsafe fn get_runtime_dependencies<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError> {
        ModuleLoader::<'a>::from_native(self.interface.module_loader_interface.as_ref())
            .get_runtime_dependencies(&module.cast_ref())
    }

    #[inline]
    unsafe fn get_exportable_interfaces<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError> {
        ModuleLoader::<'a>::from_native(self.interface.module_loader_interface.as_ref())
            .get_exportable_interfaces(&module.cast_ref())
    }

    #[inline]
    unsafe fn get_interface<'b, T: Sized + FFIObject<ffi::module::ModuleInterface>>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
        interface: &InterfaceDescriptor,
    ) -> Result<ModuleInterface<'b, T>, ModuleError> {
        ModuleLoader::<'a>::from_native(self.interface.module_loader_interface.as_ref())
            .get_interface(&module.cast_ref(), interface)
    }

    #[inline]
    #[must_use]
    unsafe fn get_internal_interface<T: 'a + Sized + FFIObject<NonNullConst<c_void>>>(&self) -> T {
        ModuleLoader::<'a>::from_native(self.interface.module_loader_interface.as_ref())
            .get_internal_interface()
    }
}

impl<'a> NativeModuleLoaderWrapper<'a> for NativeModuleLoader<'a> {
    #[inline]
    unsafe fn get_native_module<'b>(
        &self,
        module: &LoaderModuleHandleRef<'b, 'a, Self>,
    ) -> Result<NativeModuleInstance<'b>, ModuleError> {
        self.interface
            .get_native_module(module.as_native())
            .to_native()
            .map(|m| NativeModuleInstance::from_native(m))
    }
}

impl<'a> From<ModuleLoader<'a>> for NativeModuleLoader<'a> {
    fn from(loader: ModuleLoader<'a>) -> Self {
        unsafe { NativeModuleLoader::from_native(loader.interface) }
    }
}

impl<'a> From<NativeModuleLoader<'a>> for ModuleLoader<'a> {
    fn from(loader: NativeModuleLoader<'a>) -> Self {
        unsafe {
            let interface: &ffi::module::ModuleLoaderInterface = loader.as_native();
            ModuleLoader::from_native(interface)
        }
    }
}

impl<'a> AsRef<ffi::module::ModuleLoaderInterface> for NativeModuleLoader<'a> {
    fn as_ref(&self) -> &ffi::module::ModuleLoaderInterface {
        unsafe { &*self.interface.module_loader_interface.as_ptr() }
    }
}

impl<'a> AsRef<ffi::module::NativeModuleLoaderInterface> for NativeModuleLoader<'a> {
    fn as_ref(&self) -> &ffi::module::NativeModuleLoaderInterface {
        self.interface
    }
}

impl<'a> FFIObject<&'a ffi::module::ModuleLoaderInterface> for NativeModuleLoader<'a> {
    fn as_native(&self) -> &'a ffi::module::ModuleLoaderInterface {
        unsafe { &*self.interface.module_loader_interface.as_ptr() }
    }

    unsafe fn from_native(val: &'a ffi::module::ModuleLoaderInterface) -> Self {
        NativeModuleLoader::from_native(val.get_internal_interface())
    }
}

impl<'a> FFIObject<&'a ffi::module::NativeModuleLoaderInterface> for NativeModuleLoader<'a> {
    fn as_native(&self) -> &'a ffi::module::NativeModuleLoaderInterface {
        self.interface
    }

    unsafe fn from_native(val: &'a ffi::module::NativeModuleLoaderInterface) -> Self {
        Self { interface: val }
    }
}

impl<'a> FFIObject<NonNullConst<c_void>> for NativeModuleLoader<'a> {
    fn as_native(&self) -> NonNullConst<c_void> {
        NonNullConst::from(self.interface).cast()
    }

    unsafe fn from_native(val: NonNullConst<c_void>) -> Self {
        Self {
            interface: &*(val.as_ptr() as *const ffi::module::NativeModuleLoaderInterface),
        }
    }
}
