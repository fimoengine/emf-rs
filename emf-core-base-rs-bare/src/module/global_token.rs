use crate::library::{native_buff_ptr_to_os_str, os_str_to_native_buff};
use crate::module::{
    LoaderModuleHandle, LoaderModuleHandleRef, ModuleHandle, ModuleHandleRef, ModuleInterface,
    ModuleLoaderHandle, ModuleLoaderHandleRef, ModuleLoaderWrapper, ModuleToken,
};
use crate::{ffi, FFIObject, GlobalToken};
use ffi::containers::{MutSpan, NonNullConst};
use ffi::module::{InterfaceDescriptor, ModuleError, ModuleInfo, ModuleStatus, ModuleType};
use std::path::{Path, PathBuf};
use std::ptr::NonNull;

impl<'a> ModuleToken<'a> for GlobalToken<'a> {
    #[inline]
    fn register_loader<T: ModuleLoaderWrapper<'static>>(
        &self,
        loader: &T,
        mod_type: &ModuleType,
    ) -> Result<ModuleLoaderHandle<'static>, ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_register_loader(
                NonNullConst::from(loader.as_native()),
                NonNullConst::from(mod_type),
            )
            .to_native()
            .map(|l| ModuleLoaderHandle::from_native(l))
        }
    }

    #[inline]
    #[must_use]
    fn unregister_loader(&self, loader: ModuleLoaderHandle) -> Option<ModuleError> {
        unsafe { ffi::module::emf_cbase_module_unregister_loader(loader.as_native()).to_native() }
    }

    #[inline]
    #[must_use]
    fn get_num_loaders(&self) -> usize {
        unsafe { ffi::module::emf_cbase_module_get_num_loaders() }
    }

    #[inline]
    fn get_module_types(&self, buffer: &mut [ModuleType]) -> Result<usize, ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_get_module_types(NonNull::from(&MutSpan::from(buffer)))
                .to_native()
        }
    }

    #[inline]
    #[must_use]
    fn get_num_modules(&self) -> usize {
        unsafe { ffi::module::emf_cbase_module_get_num_modules() }
    }

    #[inline]
    fn get_modules(&self, buffer: &mut [ModuleInfo]) -> Result<usize, ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_get_modules(NonNull::from(&MutSpan::from(buffer)))
                .to_native()
        }
    }

    #[inline]
    #[must_use]
    fn get_num_exported_interfaces(&self) -> usize {
        unsafe { ffi::module::emf_cbase_module_get_num_exported_interfaces() }
    }

    #[inline]
    fn get_exported_interfaces(
        &self,
        buffer: &mut [InterfaceDescriptor],
    ) -> Result<usize, ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_get_exported_interfaces(NonNull::from(&MutSpan::from(
                buffer,
            )))
            .to_native()
        }
    }

    #[inline]
    fn get_loader_handle(
        &self,
        mod_type: &ModuleType,
    ) -> Result<ModuleLoaderHandleRef<'a>, ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_get_loader_handle(NonNullConst::from(mod_type))
                .to_native()
                .map(|h| ModuleLoaderHandleRef::from_native(h))
        }
    }

    #[inline]
    #[must_use]
    fn module_type_exist(&self, mod_type: &ModuleType) -> bool {
        unsafe { ffi::module::emf_cbase_module_type_exists(NonNullConst::from(mod_type)).into() }
    }

    #[inline]
    #[must_use]
    fn module_exists(&self, module: &ModuleHandleRef) -> bool {
        unsafe { ffi::module::emf_cbase_module_module_exists(module.as_native()).into() }
    }

    #[inline]
    fn get_exported_interface_handle(
        &self,
        interface: &InterfaceDescriptor,
    ) -> Result<ModuleHandleRef<'a>, ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_get_exported_interface_handle(NonNullConst::from(
                interface,
            ))
            .to_native()
            .map(|h| ModuleHandleRef::from_native(h))
        }
    }

    #[inline]
    #[must_use]
    fn exported_interface_exists(&self, interface: &InterfaceDescriptor) -> bool {
        unsafe {
            ffi::module::emf_cbase_module_exported_interface_exists(NonNullConst::from(interface))
                .into()
        }
    }

    #[inline]
    #[must_use]
    unsafe fn create_module_handle(&self) -> ModuleHandle<'static> {
        ModuleHandle::from_native(ffi::module::emf_cbase_module_unsafe_create_module_handle())
    }

    #[inline]
    #[must_use]
    unsafe fn remove_module_handle(&self, module: ModuleHandle) -> Option<ModuleError> {
        ffi::module::emf_cbase_module_unsafe_remove_module_handle(module.as_native()).to_native()
    }

    #[inline]
    #[must_use]
    unsafe fn link_module<'b, 'c: 'd, 'd: 'b, T: ModuleLoaderWrapper<'d>>(
        &self,
        module: &ModuleHandle,
        loader: &'c ModuleLoaderHandleRef<'c>,
        internal_handle: &'b LoaderModuleHandle<'b, 'd, T>,
    ) -> Option<ModuleError> {
        ffi::module::emf_cbase_module_unsafe_link_module(
            module.as_native(),
            loader.as_native(),
            internal_handle.as_native(),
        )
        .to_native()
    }

    #[inline]
    unsafe fn get_loader_module_handle<'b, T: ModuleLoaderWrapper<'a>>(
        &self,
        module: &'b ModuleHandleRef<'b>,
    ) -> Result<LoaderModuleHandleRef<'b, 'a, T>, ModuleError> {
        ffi::module::emf_cbase_module_unsafe_get_loader_module_handle(module.as_native())
            .to_native()
            .map(|h| LoaderModuleHandleRef::from_native(h))
    }

    #[inline]
    unsafe fn get_loader_handle_from_mod(
        &self,
        module: &ModuleHandleRef,
    ) -> Result<ModuleLoaderHandleRef<'a>, ModuleError> {
        ffi::module::emf_cbase_module_unsafe_get_loader_handle(module.as_native())
            .to_native()
            .map(|h| ModuleLoaderHandleRef::from_native(h))
    }

    #[inline]
    unsafe fn get_loader_interface<T: ModuleLoaderWrapper<'a>>(
        &self,
        loader: &ModuleLoaderHandleRef,
    ) -> Result<T, ModuleError> {
        ffi::module::emf_cbase_module_unsafe_get_loader(loader.as_native())
            .to_native()
            .map(|l| T::from_native(&*l.as_ptr()))
    }

    #[inline]
    fn add_module<'b, 'c: 'b, T: AsRef<Path>>(
        &self,
        loader: &'c ModuleLoaderHandleRef<'c>,
        path: &T,
    ) -> Result<ModuleHandle<'b>, ModuleError> {
        unsafe {
            let path = path.as_ref().as_os_str();
            let native_path_buff = os_str_to_native_buff(path);
            ffi::module::emf_cbase_module_add_module(
                loader.as_native(),
                NonNullConst::new_unchecked(native_path_buff.as_ptr()),
            )
            .to_native()
            .map(|h| ModuleHandle::from_native(h))
        }
    }

    #[inline]
    #[must_use]
    fn remove_module(&self, module: ModuleHandle) -> Option<ModuleError> {
        unsafe { ffi::module::emf_cbase_module_remove_module(module.as_native()).to_native() }
    }

    #[inline]
    #[must_use]
    fn load(&self, module: &ModuleHandle) -> Option<ModuleError> {
        unsafe { ffi::module::emf_cbase_module_load(module.as_native()).to_native() }
    }

    #[inline]
    #[must_use]
    fn unload(&self, module: &ModuleHandle) -> Option<ModuleError> {
        unsafe { ffi::module::emf_cbase_module_unload(module.as_native()).to_native() }
    }

    #[inline]
    #[must_use]
    fn initialize(&self, module: &ModuleHandle) -> Option<ModuleError> {
        unsafe { ffi::module::emf_cbase_module_initialize(module.as_native()).to_native() }
    }

    #[inline]
    #[must_use]
    fn terminate<'b>(&self, module: ModuleHandle<'b>) -> (Option<ModuleError>, ModuleHandle<'b>) {
        unsafe {
            (
                ffi::module::emf_cbase_module_terminate(module.as_native()).to_native(),
                module,
            )
        }
    }

    #[inline]
    #[must_use]
    fn add_runtime_dependency(
        &self,
        module: &ModuleHandle,
        interface: &InterfaceDescriptor,
    ) -> Option<ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_add_dependency(
                module.as_native(),
                NonNullConst::from(interface),
            )
            .to_native()
        }
    }

    #[inline]
    #[must_use]
    fn remove_runtime_dependency(
        &self,
        module: &ModuleHandle,
        interface: &InterfaceDescriptor,
    ) -> Option<ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_remove_dependency(
                module.as_native(),
                NonNullConst::from(interface),
            )
            .to_native()
        }
    }

    #[inline]
    #[must_use]
    fn export_interface(
        &self,
        module: &ModuleHandle,
        interface: &InterfaceDescriptor,
    ) -> Option<ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_export_interface(
                module.as_native(),
                NonNullConst::from(interface),
            )
            .to_native()
        }
    }

    #[inline]
    fn fetch_status(&self, module: &ModuleHandleRef) -> Result<ModuleStatus, ModuleError> {
        unsafe { ffi::module::emf_cbase_module_fetch_status(module.as_native()).to_native() }
    }

    #[inline]
    fn get_module_info<'b>(
        &self,
        module: &'b ModuleHandleRef<'b>,
    ) -> Result<&'b ModuleInfo, ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_get_module_info(module.as_native())
                .to_native()
                .map(|m| &*m.as_ptr())
        }
    }

    #[inline]
    fn get_module_path(&self, module: &ModuleHandleRef) -> Result<PathBuf, ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_get_module_path(module.as_native())
                .to_native()
                .map(|path| native_buff_ptr_to_os_str(path).into())
        }
    }

    #[inline]
    fn get_load_dependencies<'b>(
        &self,
        module: &'b ModuleHandleRef<'b>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_get_load_dependencies(module.as_native())
                .to_native()
                .map(|i| std::slice::from_raw_parts(i.as_ptr(), i.len()))
        }
    }

    #[inline]
    fn get_runtime_dependencies<'b>(
        &self,
        module: &'b ModuleHandleRef<'b>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_get_runtime_dependencies(module.as_native())
                .to_native()
                .map(|i| std::slice::from_raw_parts(i.as_ptr(), i.len()))
        }
    }

    #[inline]
    fn get_exportable_interfaces<'b>(
        &self,
        module: &'b ModuleHandleRef<'b>,
    ) -> Result<&'b [InterfaceDescriptor<'b>], ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_get_exportable_interfaces(module.as_native())
                .to_native()
                .map(|i| std::slice::from_raw_parts(i.as_ptr(), i.len()))
        }
    }

    #[inline]
    fn get_interface<'b, T: Sized + FFIObject<ffi::module::ModuleInterface>>(
        &self,
        module: &'b ModuleHandleRef<'b>,
        interface: &InterfaceDescriptor,
    ) -> Result<ModuleInterface<'b, T>, ModuleError> {
        unsafe {
            ffi::module::emf_cbase_module_get_interface(
                module.as_native(),
                NonNullConst::from(interface),
            )
            .to_native()
            .map(|i| ModuleInterface::from_native(i))
        }
    }
}