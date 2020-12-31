//! Bootstrapping of the binding.
#![allow(dead_code)]

use crate::containers::{NonNullConst, Span};
use crate::fn_ptr::BaseFn;
use crate::module::{InterfaceDescriptor, InterfaceExtension, ModuleName};
use crate::version::Version;
use crate::{BaseInterface, FnId, BASE_INTERFACE_NAME};
use crate::{Bool, InterfaceBinding};
use emf_core_base_rs_ffi_bare::containers::{MutSpan, Optional, Result};
use emf_core_base_rs_ffi_bare::fn_ptr::{
    ModuleGetExportedInterfaceHandleFn, ModuleGetInterfaceFn, SysGetFunctionFn, SysPanicFn,
};
use emf_core_base_rs_ffi_bare::library::{
    DataSymbol, FnSymbol, LibraryError, LibraryHandle, LibraryType,
    LoaderHandle as LibraryLoaderHandle, LoaderInterface, LoaderLibraryHandle,
};
use emf_core_base_rs_ffi_bare::module::{
    LoaderHandle as ModuleLoaderHandle, LoaderModuleHandle, ModuleError, ModuleHandle, ModuleInfo,
    ModuleInterface, ModuleLoaderInterface, ModuleStatus, ModuleType,
};
use emf_core_base_rs_ffi_bare::sys::SyncHandlerInterface;
use emf_core_base_rs_ffi_bare::version::{ReleaseType, VersionError};
use emf_core_base_rs_ffi_bare::BaseT;
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::ptr::{null, NonNull};
use crate::library::OsPathChar;

#[cfg(test)]
mod tests;

#[repr(C)]
#[derive(Copy, Clone)]
struct ModuleNameInternal {
    pub data: [c_char; 32],
    pub length: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct InterfaceExtensionSpanInternal {
    pub data: *const InterfaceExtension,
    pub length: usize,
    pub phantom: PhantomData<&'static InterfaceExtension>,
}

const fn recursive_copy(buff: &mut [c_char; 32], i: usize) {
    if i < BASE_INTERFACE_NAME.len() {
        buff[i] = BASE_INTERFACE_NAME.as_bytes()[i] as c_char;
        recursive_copy(buff, i + 1);
    }
}

const fn generate_const_module_name() -> ModuleName {
    unsafe {
        let mut name = ModuleNameInternal {
            data: [0; 32],
            length: 0,
        };

        recursive_copy(&mut name.data, 0);
        name.length = BASE_INTERFACE_NAME.len();
        std::mem::transmute(name)
    }
}

const fn generate_const_empty_extensions() -> Span<'static, InterfaceExtension> {
    unsafe {
        let ext_span = InterfaceExtensionSpanInternal {
            data: null(),
            length: 0,
            phantom: PhantomData,
        };
        std::mem::transmute(ext_span)
    }
}

const BASE_VERSION: Version = Version {
    major: crate::VERSION_MAJOR,
    minor: crate::VERSION_MINOR,
    patch: crate::VERSION_PATCH,
    build_number: crate::VERSION_BUILD_NUMBER,
    release_number: crate::VERSION_RELEASE_NUMBER,
    release_type: crate::VERSION_RELEASE_TYPE,
};

const BASE_INTERFACE_DESC: InterfaceDescriptor<'static> = InterfaceDescriptor {
    name: generate_const_module_name(),
    version: BASE_VERSION,
    extensions: generate_const_empty_extensions(),
};

pub static mut BASE_INTERFACE: MaybeUninit<&'static BaseInterface> = MaybeUninit::uninit();

/// Initializes the binding to the `emf-core-base` interface.
///
/// Calling this is necessary if the user wishes to use a function defined
/// by the `emf-core-base` interface. Alternatively, a local object implementing the
/// [InterfaceBinding] trait, such as [BaseInterface], can be used.
pub fn initialize_base_binding(base_module: *mut BaseT, get_function_fn: SysGetFunctionFn) {
    unsafe {
        BASE_INTERFACE = MaybeUninit::new(BaseInterface::initialize(base_module, get_function_fn));
    }
}

impl InterfaceBinding for BaseInterface {
    fn initialize(base_module: *mut BaseT, get_function_fn: SysGetFunctionFn) -> &'static Self {
        unsafe {
            let panic_fn: SysPanicFn =
                match get_function_fn(base_module, FnId::SysPanic).to_native() {
                    Some(func) => std::mem::transmute(func),
                    None => panic!(),
                };

            let module_get_exported_interface_handle_fn: ModuleGetExportedInterfaceHandleFn =
                match get_function_fn(base_module, FnId::ModuleGetExportedInterfaceHandle)
                    .to_native()
                {
                    Some(func) => std::mem::transmute(func),
                    None => {
                        let error = CString::new(
                            "Could not fetch the function pointer to `emf_cbase_module_get_exported_interface_handle`"
                        ).unwrap();
                        panic_fn(base_module, error.as_ptr());
                    }
                };

            let module_get_interface_fn: ModuleGetInterfaceFn =
                match get_function_fn(base_module, FnId::ModuleGetInterface).to_native() {
                    Some(func) => std::mem::transmute(func),
                    None => {
                        let error = CString::new(
                        "Could not fetch the function pointer to `emf_cbase_module_get_interface`",
                    )
                    .unwrap();
                        panic_fn(base_module, error.as_ptr());
                    }
                };

            let interface_handle = match module_get_exported_interface_handle_fn(
                base_module,
                NonNullConst::from(&BASE_INTERFACE_DESC),
            )
            .to_native()
            {
                Ok(handle) => handle,
                Err(_) => {
                    let error = CString::new(format!(
                        "Could not fetch the handle to the '{}' interface module",
                        BASE_INTERFACE_NAME
                    ))
                    .unwrap();
                    panic_fn(base_module, error.as_ptr());
                }
            };

            match module_get_interface_fn(
                base_module,
                interface_handle,
                NonNullConst::from(&BASE_INTERFACE_DESC),
            )
            .to_native()
            {
                Ok(interface) => {
                    let base_interface: &'static BaseInterface =
                        std::mem::transmute(interface.interface.cast::<BaseInterface>().as_ref());
                    base_interface
                }
                Err(_) => {
                    let error = CString::new(format!(
                        "Could not initialize the bindings to the '{}' interface",
                        BASE_INTERFACE_NAME
                    ))
                    .unwrap();
                    panic_fn(base_module, error.as_ptr());
                }
            }
        }
    }

    #[inline]
    #[must_use]
    fn get_version(&self) -> Version {
        self.interface_version
    }

    #[inline]
    #[must_use]
    fn get_module_ptr(&self) -> *mut BaseT {
        self.cbase_module
    }

    #[inline]
    unsafe fn sys_lock(&self) {
        (self.sys_lock_fn)(self.cbase_module)
    }

    #[inline]
    #[must_use]
    unsafe fn sys_try_lock(&self) -> Bool {
        (self.sys_try_lock_fn)(self.cbase_module)
    }

    #[inline]
    unsafe fn sys_unlock(&self) {
        (self.sys_unlock_fn)(self.cbase_module)
    }

    #[inline]
    unsafe fn sys_shutdown(&self) -> ! {
        (self.sys_shutdown_fn)(self.cbase_module)
    }

    #[inline]
    unsafe fn sys_panic(&self, error: *const c_char) -> ! {
        (self.sys_panic_fn)(self.cbase_module, error)
    }

    #[inline]
    #[must_use]
    unsafe fn sys_has_function(&self, fn_id: FnId) -> Bool {
        (self.sys_has_function_fn)(self.cbase_module, fn_id)
    }

    #[inline]
    #[must_use]
    unsafe fn sys_get_function(&self, fn_id: FnId) -> Optional<BaseFn> {
        (self.sys_get_function_fn)(self.cbase_module, fn_id)
    }

    #[inline]
    #[must_use]
    unsafe fn sys_get_sync_handler(&self) -> NonNullConst<SyncHandlerInterface> {
        (self.sys_get_sync_handler_fn)(self.cbase_module)
    }

    #[inline]
    #[must_use]
    unsafe fn sys_set_sync_handler(&self, sync_handler: *const SyncHandlerInterface) {
        (self.sys_set_sync_handler_fn)(self.cbase_module, sync_handler)
    }

    #[inline]
    #[must_use]
    unsafe fn version_construct_short(&self, major: i32, minor: i32, patch: i32) -> Version {
        (self.version_construct_short_fn)(self.cbase_module, major, minor, patch)
    }

    #[inline]
    #[must_use]
    unsafe fn version_construct_long(
        &self,
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
    ) -> Version {
        (self.version_construct_long_fn)(
            self.cbase_module,
            major,
            minor,
            patch,
            release_type,
            release_number,
        )
    }

    #[inline]
    #[must_use]
    unsafe fn version_construct_full(
        &self,
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
        build: i64,
    ) -> Version {
        (self.version_construct_full_fn)(
            self.cbase_module,
            major,
            minor,
            patch,
            release_type,
            release_number,
            build,
        )
    }

    #[inline]
    #[must_use]
    unsafe fn version_construct_from_string(
        &self,
        version_string: NonNullConst<Span<'_, i8>>,
    ) -> Result<Version, VersionError> {
        (self.version_construct_from_string_fn)(self.cbase_module, version_string)
    }

    #[inline]
    #[must_use]
    unsafe fn version_representation_is_valid(
        &self,
        version_string: NonNullConst<Span<'_, i8>>,
    ) -> Bool {
        (self.version_representation_is_valid_fn)(self.cbase_module, version_string)
    }

    #[inline]
    #[must_use]
    unsafe fn version_get_short_representation(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<'_, i8>>,
    ) -> Result<usize, VersionError> {
        (self.version_get_short_representation_fn)(self.cbase_module, version, buffer)
    }

    #[inline]
    #[must_use]
    unsafe fn version_get_short_representation_length(
        &self,
        version: NonNullConst<Version>,
    ) -> usize {
        (self.version_get_short_representation_length_fn)(self.cbase_module, version)
    }

    #[inline]
    #[must_use]
    unsafe fn version_get_long_representation(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<'_, i8>>,
    ) -> Result<usize, VersionError> {
        (self.version_get_long_representation_fn)(self.cbase_module, version, buffer)
    }

    #[inline]
    #[must_use]
    unsafe fn version_get_long_representation_length(
        &self,
        version: NonNullConst<Version>,
    ) -> usize {
        (self.version_get_long_representation_length_fn)(self.cbase_module, version)
    }

    #[inline]
    #[must_use]
    unsafe fn version_get_full_representation(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<'_, i8>>,
    ) -> Result<usize, VersionError> {
        (self.version_get_full_representation_fn)(self.cbase_module, version, buffer)
    }

    #[inline]
    #[must_use]
    unsafe fn version_get_full_representation_length(
        &self,
        version: NonNullConst<Version>,
    ) -> usize {
        (self.version_get_full_representation_length_fn)(self.cbase_module, version)
    }

    #[inline]
    #[must_use]
    unsafe fn version_compare(
        &self,
        lhs: NonNullConst<Version>,
        rhs: NonNullConst<Version>,
    ) -> i32 {
        (self.version_compare_fn)(self.cbase_module, lhs, rhs)
    }

    #[inline]
    #[must_use]
    unsafe fn version_compare_weak(
        &self,
        lhs: NonNullConst<Version>,
        rhs: NonNullConst<Version>,
    ) -> i32 {
        (self.version_compare_weak_fn)(self.cbase_module, lhs, rhs)
    }

    #[inline]
    #[must_use]
    unsafe fn version_compare_strong(
        &self,
        lhs: NonNullConst<Version>,
        rhs: NonNullConst<Version>,
    ) -> i32 {
        (self.version_compare_strong_fn)(self.cbase_module, lhs, rhs)
    }

    #[inline]
    #[must_use]
    unsafe fn version_is_compatible(
        &self,
        lhs: NonNullConst<Version>,
        rhs: NonNullConst<Version>,
    ) -> Bool {
        (self.version_is_compatible_fn)(self.cbase_module, lhs, rhs)
    }

    #[inline]
    #[must_use]
    unsafe fn library_register_loader(
        &self,
        loader_interface: NonNullConst<LoaderInterface>,
        library_type: NonNullConst<LibraryType>,
    ) -> Result<LibraryLoaderHandle, LibraryError> {
        (self.library_register_loader_fn)(self.cbase_module, loader_interface, library_type)
    }

    #[inline]
    #[must_use]
    unsafe fn library_unregister_loader(
        &self,
        loader_handle: LibraryLoaderHandle,
    ) -> Optional<LibraryError> {
        (self.library_unregister_loader_fn)(self.cbase_module, loader_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_get_num_loaders(&self) -> usize {
        (self.library_get_num_loaders_fn)(self.cbase_module)
    }

    #[inline]
    #[must_use]
    unsafe fn library_get_library_types(
        &self,
        buffer: NonNull<MutSpan<'_, LibraryType>>,
    ) -> Result<usize, LibraryError> {
        (self.library_get_library_types_fn)(self.cbase_module, buffer)
    }

    #[inline]
    #[must_use]
    unsafe fn library_get_loader_handle(
        &self,
        library_type: NonNullConst<LibraryType>,
    ) -> Result<LibraryLoaderHandle, LibraryError> {
        (self.library_get_loader_handle_fn)(self.cbase_module, library_type)
    }

    #[inline]
    #[must_use]
    unsafe fn library_type_exists(&self, library_type: NonNullConst<LibraryType>) -> Bool {
        (self.library_type_exists_fn)(self.cbase_module, library_type)
    }

    #[inline]
    #[must_use]
    unsafe fn library_library_exists(&self, library_handle: LibraryHandle) -> Bool {
        (self.library_library_exists_fn)(self.cbase_module, library_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_unsafe_create_library_handle(&self) -> LibraryHandle {
        (self.library_unsafe_create_library_handle_fn)(self.cbase_module)
    }

    #[inline]
    #[must_use]
    unsafe fn library_unsafe_remove_library_handle(
        &self,
        library_handle: LibraryHandle,
    ) -> Optional<LibraryError> {
        (self.library_unsafe_remove_library_handle_fn)(self.cbase_module, library_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_unsafe_link_library(
        &self,
        library_handle: LibraryHandle,
        loader_handle: LibraryLoaderHandle,
        internal_handle: LoaderLibraryHandle,
    ) -> Optional<LibraryError> {
        (self.library_unsafe_link_library_fn)(
            self.cbase_module,
            library_handle,
            loader_handle,
            internal_handle,
        )
    }

    #[inline]
    #[must_use]
    unsafe fn library_unsafe_get_loader_library_handle(
        &self,
        library_handle: LibraryHandle,
    ) -> Result<LoaderLibraryHandle, LibraryError> {
        (self.library_unsafe_get_loader_library_handle_fn)(self.cbase_module, library_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_unsafe_get_loader_handle(
        &self,
        library_handle: LibraryHandle,
    ) -> Result<LibraryLoaderHandle, LibraryError> {
        (self.library_unsafe_get_loader_handle_fn)(self.cbase_module, library_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_unsafe_get_loader_interface(
        &self,
        loader_handle: LibraryLoaderHandle,
    ) -> Result<NonNullConst<LoaderInterface>, LibraryError> {
        (self.library_unsafe_get_loader_interface_fn)(self.cbase_module, loader_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_load(
        &self,
        loader_handle: LibraryLoaderHandle,
        library_path: NonNullConst<OsPathChar>,
    ) -> Result<LibraryHandle, LibraryError> {
        (self.library_load_fn)(self.cbase_module, loader_handle, library_path)
    }

    #[inline]
    #[must_use]
    unsafe fn library_unload(&self, library_handle: LibraryHandle) -> Optional<LibraryError> {
        (self.library_unload_fn)(self.cbase_module, library_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_get_data_symbol(
        &self,
        library_handle: LibraryHandle,
        symbol_name: NonNullConst<i8>,
    ) -> Result<DataSymbol, LibraryError> {
        (self.library_get_data_symbol_fn)(self.cbase_module, library_handle, symbol_name)
    }

    #[inline]
    #[must_use]
    unsafe fn library_get_function_symbol(
        &self,
        library_handle: LibraryHandle,
        symbol_name: NonNullConst<i8>,
    ) -> Result<FnSymbol, LibraryError> {
        (self.library_get_function_symbol_fn)(self.cbase_module, library_handle, symbol_name)
    }

    #[inline]
    #[must_use]
    unsafe fn module_register_loader(
        &self,
        loader_interface: NonNullConst<ModuleLoaderInterface>,
        module_type: NonNullConst<ModuleType>,
    ) -> Result<ModuleLoaderHandle, ModuleError> {
        (self.module_register_loader_fn)(self.cbase_module, loader_interface, module_type)
    }

    #[inline]
    #[must_use]
    unsafe fn module_unregister_loader(
        &self,
        loader_handle: ModuleLoaderHandle,
    ) -> Optional<ModuleError> {
        (self.module_unregister_loader_fn)(self.cbase_module, loader_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_num_loaders(&self) -> usize {
        (self.module_get_num_loaders_fn)(self.cbase_module)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_module_types(
        &self,
        buffer: NonNull<MutSpan<'_, ModuleType>>,
    ) -> Result<usize, ModuleError> {
        (self.module_get_module_types_fn)(self.cbase_module, buffer)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_num_modules(&self) -> usize {
        (self.module_get_num_modules_fn)(self.cbase_module)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_modules(
        &self,
        buffer: NonNull<MutSpan<'_, ModuleInfo>>,
    ) -> Result<usize, ModuleError> {
        (self.module_get_modules_fn)(self.cbase_module, buffer)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_num_exported_interfaces(&self) -> usize {
        (self.module_get_num_exported_interfaces_fn)(self.cbase_module)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_exported_interfaces(
        &self,
        buffer: NonNull<MutSpan<'_, InterfaceDescriptor>>,
    ) -> Result<usize, ModuleError> {
        (self.module_get_exported_interfaces_fn)(self.cbase_module, buffer)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_loader_handle(
        &self,
        module_type: NonNullConst<ModuleType>,
    ) -> Result<ModuleLoaderHandle, ModuleError> {
        (self.module_get_loader_handle_fn)(self.cbase_module, module_type)
    }

    #[inline]
    #[must_use]
    unsafe fn module_type_exists(&self, module_type: NonNullConst<ModuleType>) -> Bool {
        (self.module_type_exists_fn)(self.cbase_module, module_type)
    }

    #[inline]
    #[must_use]
    unsafe fn module_module_exists(&self, module_handle: ModuleHandle) -> Bool {
        (self.module_module_exists_fn)(self.cbase_module, module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_exported_interface_handle(
        &self,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<ModuleHandle, ModuleError> {
        (self.module_get_exported_interface_handle_fn)(self.cbase_module, interface)
    }

    #[inline]
    #[must_use]
    unsafe fn module_exported_interface_exists(
        &self,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Bool {
        (self.module_exported_interface_exists_fn)(self.cbase_module, interface)
    }

    #[inline]
    #[must_use]
    unsafe fn module_unsafe_create_module_handle(&self) -> ModuleHandle {
        (self.module_unsafe_create_module_handle_fn)(self.cbase_module)
    }

    #[inline]
    #[must_use]
    unsafe fn module_unsafe_remove_module_handle(
        &self,
        module_handle: ModuleHandle,
    ) -> Optional<ModuleError> {
        (self.module_unsafe_remove_module_handle_fn)(self.cbase_module, module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_unsafe_link_module(
        &self,
        module_handle: ModuleHandle,
        loader_handle: ModuleLoaderHandle,
        internal_handle: LoaderModuleHandle,
    ) -> Optional<ModuleError> {
        (self.module_unsafe_link_module_fn)(
            self.cbase_module,
            module_handle,
            loader_handle,
            internal_handle,
        )
    }

    #[inline]
    #[must_use]
    unsafe fn module_unsafe_get_loader_module_handle(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<LoaderModuleHandle, ModuleError> {
        (self.module_unsafe_get_loader_module_handle_fn)(self.cbase_module, module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_unsafe_get_loader_handle(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<ModuleLoaderHandle, ModuleError> {
        (self.module_unsafe_get_loader_handle_fn)(self.cbase_module, module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_unsafe_get_loader(
        &self,
        loader_handle: ModuleLoaderHandle,
    ) -> Result<NonNullConst<ModuleLoaderInterface>, ModuleError> {
        (self.module_unsafe_get_loader_fn)(self.cbase_module, loader_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_add_module(
        &self,
        loader_handle: ModuleLoaderHandle,
        module_path: NonNullConst<OsPathChar>,
    ) -> Result<ModuleHandle, ModuleError> {
        (self.module_add_module_fn)(self.cbase_module, loader_handle, module_path)
    }

    #[inline]
    #[must_use]
    unsafe fn module_remove_module(&self, module_handle: ModuleHandle) -> Optional<ModuleError> {
        (self.module_remove_module_fn)(self.cbase_module, module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_load_dependencies(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError> {
        (self.module_get_load_dependencies_fn)(self.cbase_module, module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_fetch_status(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<ModuleStatus, ModuleError> {
        (self.module_fetch_status_fn)(self.cbase_module, module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_add_dependency(
        &self,
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Optional<ModuleError> {
        (self.module_add_dependency_fn)(self.cbase_module, module_handle, interface_descriptor)
    }

    #[inline]
    #[must_use]
    unsafe fn module_remove_dependency(
        &self,
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Optional<ModuleError> {
        (self.module_remove_dependency_fn)(self.cbase_module, module_handle, interface_descriptor)
    }

    #[inline]
    #[must_use]
    unsafe fn module_export_interface(
        &self,
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Optional<ModuleError> {
        (self.module_export_interface_fn)(self.cbase_module, module_handle, interface_descriptor)
    }

    #[inline]
    #[must_use]
    unsafe fn module_load(&self, module_handle: ModuleHandle) -> Optional<ModuleError> {
        (self.module_load_fn)(self.cbase_module, module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_unload(&self, module_handle: ModuleHandle) -> Optional<ModuleError> {
        (self.module_unload_fn)(self.cbase_module, module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_initialize(&self, module_handle: ModuleHandle) -> Optional<ModuleError> {
        (self.module_initialize_fn)(self.cbase_module, module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_terminate(&self, module_handle: ModuleHandle) -> Optional<ModuleError> {
        (self.module_terminate_fn)(self.cbase_module, module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_module_info(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<NonNullConst<ModuleInfo>, ModuleError> {
        (self.module_get_module_info_fn)(self.cbase_module, module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_exportable_interfaces(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError> {
        (self.module_get_exportable_interfaces_fn)(self.cbase_module, module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_runtime_dependencies(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError> {
        (self.module_get_runtime_dependencies_fn)(self.cbase_module, module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_interface(
        &self,
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Result<ModuleInterface, ModuleError> {
        (self.module_get_interface_fn)(self.cbase_module, module_handle, interface_descriptor)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_module_path(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<NonNullConst<OsPathChar>, ModuleError> {
        (self.module_get_module_path_fn)(self.cbase_module, module_handle)
    }
}
