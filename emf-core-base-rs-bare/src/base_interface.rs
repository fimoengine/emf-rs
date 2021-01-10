use crate::ffi::library::OsPathChar;
use crate::{ffi, FFIObject};
use ffi::containers::{MutSpan, NonNullConst, Optional, Result, Span};
use ffi::fn_ptr::BaseFn;
use ffi::library::{
    DataSymbol, FnSymbol, LibraryError, LibraryHandle, LibraryType,
    LoaderHandle as LibraryLoaderHandle, LoaderInterface, LoaderLibraryHandle,
};
use ffi::module::{
    InterfaceDescriptor, LoaderHandle as ModuleLoaderHandle, LoaderModuleHandle, ModuleError,
    ModuleHandle, ModuleInfo, ModuleInterface, ModuleLoaderInterface, ModuleStatus, ModuleType,
};
use ffi::sys::SyncHandlerInterface;
use ffi::version::{ReleaseType, Version, VersionError};
use ffi::{BaseT, Bool, FnId};
use std::os::raw::c_char;
use std::ptr::NonNull;

/// The interface to `emf-core-base`.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct BaseInterface<'a> {
    interface: &'a ffi::BaseInterface,
}

impl<'a> ffi::InterfaceBinding for BaseInterface<'a> {
    #[inline]
    #[must_use]
    fn get_version(&self) -> Version {
        self.interface.get_version()
    }

    #[inline]
    #[must_use]
    fn get_module_ptr(&self) -> *mut BaseT {
        self.interface.get_module_ptr()
    }

    #[inline]
    unsafe fn sys_lock(&self) {
        self.interface.sys_lock()
    }

    #[inline]
    #[must_use]
    unsafe fn sys_try_lock(&self) -> Bool {
        self.interface.sys_try_lock()
    }

    #[inline]
    unsafe fn sys_unlock(&self) {
        self.interface.sys_unlock()
    }

    #[inline]
    unsafe fn sys_shutdown(&self) -> ! {
        self.interface.sys_shutdown()
    }

    #[inline]
    unsafe fn sys_panic(&self, error: *const c_char) -> ! {
        self.interface.sys_panic(error)
    }

    #[inline]
    #[must_use]
    unsafe fn sys_has_function(&self, fn_id: FnId) -> Bool {
        self.interface.sys_has_function(fn_id)
    }

    #[inline]
    #[must_use]
    unsafe fn sys_get_function(&self, fn_id: FnId) -> Optional<BaseFn> {
        self.interface.sys_get_function(fn_id)
    }

    #[inline]
    #[must_use]
    unsafe fn sys_get_sync_handler(&self) -> NonNullConst<SyncHandlerInterface> {
        self.interface.sys_get_sync_handler()
    }

    #[inline]
    unsafe fn sys_set_sync_handler(&self, sync_handler: *const SyncHandlerInterface) {
        self.interface.sys_set_sync_handler(sync_handler)
    }

    #[inline]
    #[must_use]
    unsafe fn version_construct_short(&self, major: i32, minor: i32, patch: i32) -> Version {
        self.interface.version_construct_short(major, minor, patch)
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
        self.interface
            .version_construct_long(major, minor, patch, release_type, release_number)
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
        self.interface.version_construct_full(
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
        version_string: NonNullConst<Span<c_char>>,
    ) -> Result<Version, VersionError> {
        self.interface.version_construct_from_string(version_string)
    }

    #[inline]
    #[must_use]
    unsafe fn version_representation_is_valid(
        &self,
        version_string: NonNullConst<Span<c_char>>,
    ) -> Bool {
        self.interface
            .version_representation_is_valid(version_string)
    }

    #[inline]
    #[must_use]
    unsafe fn version_get_short_representation(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<c_char>>,
    ) -> Result<usize, VersionError> {
        self.interface
            .version_get_short_representation(version, buffer)
    }

    #[inline]
    #[must_use]
    unsafe fn version_get_short_representation_length(
        &self,
        version: NonNullConst<Version>,
    ) -> usize {
        self.interface
            .version_get_short_representation_length(version)
    }

    #[inline]
    #[must_use]
    unsafe fn version_get_long_representation(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<c_char>>,
    ) -> Result<usize, VersionError> {
        self.interface
            .version_get_long_representation(version, buffer)
    }

    #[inline]
    #[must_use]
    unsafe fn version_get_long_representation_length(
        &self,
        version: NonNullConst<Version>,
    ) -> usize {
        self.interface
            .version_get_long_representation_length(version)
    }

    #[inline]
    #[must_use]
    unsafe fn version_get_full_representation(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<c_char>>,
    ) -> Result<usize, VersionError> {
        self.interface
            .version_get_full_representation(version, buffer)
    }

    #[inline]
    #[must_use]
    unsafe fn version_get_full_representation_length(
        &self,
        version: NonNullConst<Version>,
    ) -> usize {
        self.interface
            .version_get_full_representation_length(version)
    }

    #[inline]
    #[must_use]
    unsafe fn version_compare(
        &self,
        lhs: NonNullConst<Version>,
        rhs: NonNullConst<Version>,
    ) -> i32 {
        self.interface.version_compare(lhs, rhs)
    }

    #[inline]
    #[must_use]
    unsafe fn version_compare_weak(
        &self,
        lhs: NonNullConst<Version>,
        rhs: NonNullConst<Version>,
    ) -> i32 {
        self.interface.version_compare_weak(lhs, rhs)
    }

    #[inline]
    #[must_use]
    unsafe fn version_compare_strong(
        &self,
        lhs: NonNullConst<Version>,
        rhs: NonNullConst<Version>,
    ) -> i32 {
        self.interface.version_compare_strong(lhs, rhs)
    }

    #[inline]
    #[must_use]
    unsafe fn version_is_compatible(
        &self,
        lhs: NonNullConst<Version>,
        rhs: NonNullConst<Version>,
    ) -> Bool {
        self.interface.version_is_compatible(lhs, rhs)
    }

    #[inline]
    #[must_use]
    unsafe fn library_register_loader(
        &self,
        loader_interface: NonNullConst<LoaderInterface>,
        library_type: NonNullConst<LibraryType>,
    ) -> Result<LibraryLoaderHandle, LibraryError> {
        self.interface
            .library_register_loader(loader_interface, library_type)
    }

    #[inline]
    #[must_use]
    unsafe fn library_unregister_loader(
        &self,
        loader_handle: LibraryLoaderHandle,
    ) -> Optional<LibraryError> {
        self.interface.library_unregister_loader(loader_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_get_num_loaders(&self) -> usize {
        self.interface.library_get_num_loaders()
    }

    #[inline]
    #[must_use]
    unsafe fn library_get_library_types(
        &self,
        buffer: NonNull<MutSpan<LibraryType>>,
    ) -> Result<usize, LibraryError> {
        self.interface.library_get_library_types(buffer)
    }

    #[inline]
    #[must_use]
    unsafe fn library_get_loader_handle(
        &self,
        library_type: NonNullConst<LibraryType>,
    ) -> Result<LibraryLoaderHandle, LibraryError> {
        self.interface.library_get_loader_handle(library_type)
    }

    #[inline]
    #[must_use]
    unsafe fn library_type_exists(&self, library_type: NonNullConst<LibraryType>) -> Bool {
        self.interface.library_type_exists(library_type)
    }

    #[inline]
    #[must_use]
    unsafe fn library_library_exists(&self, library_handle: LibraryHandle) -> Bool {
        self.interface.library_library_exists(library_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_unsafe_create_library_handle(&self) -> LibraryHandle {
        self.interface.library_unsafe_create_library_handle()
    }

    #[inline]
    #[must_use]
    unsafe fn library_unsafe_remove_library_handle(
        &self,
        library_handle: LibraryHandle,
    ) -> Optional<LibraryError> {
        self.interface
            .library_unsafe_remove_library_handle(library_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_unsafe_link_library(
        &self,
        library_handle: LibraryHandle,
        loader_handle: LibraryLoaderHandle,
        internal_handle: LoaderLibraryHandle,
    ) -> Optional<LibraryError> {
        self.interface
            .library_unsafe_link_library(library_handle, loader_handle, internal_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_unsafe_get_loader_library_handle(
        &self,
        library_handle: LibraryHandle,
    ) -> Result<LoaderLibraryHandle, LibraryError> {
        self.interface
            .library_unsafe_get_loader_library_handle(library_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_unsafe_get_loader_handle(
        &self,
        library_handle: LibraryHandle,
    ) -> Result<LibraryLoaderHandle, LibraryError> {
        self.interface
            .library_unsafe_get_loader_handle(library_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_unsafe_get_loader_interface(
        &self,
        loader_handle: LibraryLoaderHandle,
    ) -> Result<NonNullConst<LoaderInterface>, LibraryError> {
        self.interface
            .library_unsafe_get_loader_interface(loader_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_load(
        &self,
        loader_handle: LibraryLoaderHandle,
        library_path: NonNullConst<OsPathChar>,
    ) -> Result<LibraryHandle, LibraryError> {
        self.interface.library_load(loader_handle, library_path)
    }

    #[inline]
    #[must_use]
    unsafe fn library_unload(&self, library_handle: LibraryHandle) -> Optional<LibraryError> {
        self.interface.library_unload(library_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn library_get_data_symbol(
        &self,
        library_handle: LibraryHandle,
        symbol_name: NonNullConst<c_char>,
    ) -> Result<DataSymbol, LibraryError> {
        self.interface
            .library_get_data_symbol(library_handle, symbol_name)
    }

    #[inline]
    #[must_use]
    unsafe fn library_get_function_symbol(
        &self,
        library_handle: LibraryHandle,
        symbol_name: NonNullConst<c_char>,
    ) -> Result<FnSymbol, LibraryError> {
        self.interface
            .library_get_function_symbol(library_handle, symbol_name)
    }

    #[inline]
    #[must_use]
    unsafe fn module_register_loader(
        &self,
        loader_interface: NonNullConst<ModuleLoaderInterface>,
        module_type: NonNullConst<ModuleType>,
    ) -> Result<ModuleLoaderHandle, ModuleError> {
        self.interface
            .module_register_loader(loader_interface, module_type)
    }

    #[inline]
    #[must_use]
    unsafe fn module_unregister_loader(
        &self,
        loader_handle: ModuleLoaderHandle,
    ) -> Optional<ModuleError> {
        self.interface.module_unregister_loader(loader_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_num_loaders(&self) -> usize {
        self.interface.module_get_num_loaders()
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_module_types(
        &self,
        buffer: NonNull<MutSpan<ModuleType>>,
    ) -> Result<usize, ModuleError> {
        self.interface.module_get_module_types(buffer)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_num_modules(&self) -> usize {
        self.interface.module_get_num_modules()
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_modules(
        &self,
        buffer: NonNull<MutSpan<ModuleInfo>>,
    ) -> Result<usize, ModuleError> {
        self.interface.module_get_modules(buffer)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_num_exported_interfaces(&self) -> usize {
        self.interface.module_get_num_exported_interfaces()
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_exported_interfaces(
        &self,
        buffer: NonNull<MutSpan<InterfaceDescriptor>>,
    ) -> Result<usize, ModuleError> {
        self.interface.module_get_exported_interfaces(buffer)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_loader_handle(
        &self,
        module_type: NonNullConst<ModuleType>,
    ) -> Result<ModuleLoaderHandle, ModuleError> {
        self.interface.module_get_loader_handle(module_type)
    }

    #[inline]
    #[must_use]
    unsafe fn module_type_exists(&self, module_type: NonNullConst<ModuleType>) -> Bool {
        self.interface.module_type_exists(module_type)
    }

    #[inline]
    #[must_use]
    unsafe fn module_module_exists(&self, module_handle: ModuleHandle) -> Bool {
        self.interface.module_module_exists(module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_exported_interface_handle(
        &self,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<ModuleHandle, ModuleError> {
        self.interface
            .module_get_exported_interface_handle(interface)
    }

    #[inline]
    #[must_use]
    unsafe fn module_exported_interface_exists(
        &self,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Bool {
        self.interface.module_exported_interface_exists(interface)
    }

    #[inline]
    #[must_use]
    unsafe fn module_unsafe_create_module_handle(&self) -> ModuleHandle {
        self.interface.module_unsafe_create_module_handle()
    }

    #[inline]
    #[must_use]
    unsafe fn module_unsafe_remove_module_handle(
        &self,
        module_handle: ModuleHandle,
    ) -> Optional<ModuleError> {
        self.interface
            .module_unsafe_remove_module_handle(module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_unsafe_link_module(
        &self,
        module_handle: ModuleHandle,
        loader_handle: ModuleLoaderHandle,
        internal_handle: LoaderModuleHandle,
    ) -> Optional<ModuleError> {
        self.interface
            .module_unsafe_link_module(module_handle, loader_handle, internal_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_unsafe_get_loader_module_handle(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<LoaderModuleHandle, ModuleError> {
        self.interface
            .module_unsafe_get_loader_module_handle(module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_unsafe_get_loader_handle(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<ModuleLoaderHandle, ModuleError> {
        self.interface
            .module_unsafe_get_loader_handle(module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_unsafe_get_loader(
        &self,
        loader_handle: ModuleLoaderHandle,
    ) -> Result<NonNullConst<ModuleLoaderInterface>, ModuleError> {
        self.interface.module_unsafe_get_loader(loader_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_add_module(
        &self,
        loader_handle: ModuleLoaderHandle,
        module_path: NonNullConst<OsPathChar>,
    ) -> Result<ModuleHandle, ModuleError> {
        self.interface.module_add_module(loader_handle, module_path)
    }

    #[inline]
    #[must_use]
    unsafe fn module_remove_module(&self, module_handle: ModuleHandle) -> Optional<ModuleError> {
        self.interface.module_remove_module(module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_load_dependencies(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError> {
        self.interface.module_get_load_dependencies(module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_fetch_status(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<ModuleStatus, ModuleError> {
        self.interface.module_fetch_status(module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_add_dependency(
        &self,
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Optional<ModuleError> {
        self.interface
            .module_add_dependency(module_handle, interface_descriptor)
    }

    #[inline]
    #[must_use]
    unsafe fn module_remove_dependency(
        &self,
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Optional<ModuleError> {
        self.interface
            .module_remove_dependency(module_handle, interface_descriptor)
    }

    #[inline]
    #[must_use]
    unsafe fn module_export_interface(
        &self,
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Optional<ModuleError> {
        self.interface
            .module_export_interface(module_handle, interface_descriptor)
    }

    #[inline]
    #[must_use]
    unsafe fn module_load(&self, module_handle: ModuleHandle) -> Optional<ModuleError> {
        self.interface.module_load(module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_unload(&self, module_handle: ModuleHandle) -> Optional<ModuleError> {
        self.interface.module_unload(module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_initialize(&self, module_handle: ModuleHandle) -> Optional<ModuleError> {
        self.interface.module_initialize(module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_terminate(&self, module_handle: ModuleHandle) -> Optional<ModuleError> {
        self.interface.module_terminate(module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_module_info(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<NonNullConst<ModuleInfo>, ModuleError> {
        self.interface.module_get_module_info(module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_exportable_interfaces(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError> {
        self.interface
            .module_get_exportable_interfaces(module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_runtime_dependencies(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError> {
        self.interface
            .module_get_runtime_dependencies(module_handle)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_interface(
        &self,
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Result<ModuleInterface, ModuleError> {
        self.interface
            .module_get_interface(module_handle, interface_descriptor)
    }

    #[inline]
    #[must_use]
    unsafe fn module_get_module_path(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<NonNullConst<OsPathChar>, ModuleError> {
        self.interface.module_get_module_path(module_handle)
    }
}

impl<'a> AsRef<ffi::BaseInterface> for BaseInterface<'a> {
    fn as_ref(&self) -> &ffi::BaseInterface {
        self.interface
    }
}

impl<'a> FFIObject<&'a ffi::BaseInterface> for BaseInterface<'a> {
    fn as_native(&self) -> &'a ffi::BaseInterface {
        self.interface
    }

    unsafe fn from_native(val: &'a ffi::BaseInterface) -> Self {
        Self {
            interface: val,
        }
    }
}

impl<'a> FFIObject<ModuleInterface> for BaseInterface<'a> {
    fn as_native(&self) -> ModuleInterface {
        ModuleInterface {
            interface: NonNull::from(self.interface).cast(),
        }
    }

    unsafe fn from_native(val: ModuleInterface) -> Self {
        Self {
            interface: &*val.interface.cast::<ffi::BaseInterface>().as_ptr(),
        }
    }
}
