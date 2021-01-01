//! Definition of the `InterfaceBinding` trait.
use crate::containers::{MutSpan, NonNullConst, Optional, Result, Span};
use crate::fn_ptr::{BaseFn, SysGetFunctionFn};
use crate::library::{
    DataSymbol, FnSymbol, LibraryError, LibraryHandle, LibraryType,
    LoaderHandle as LibraryLoaderHandle, LoaderInterface, LoaderLibraryHandle, OsPathChar,
};
use crate::module::{
    InterfaceDescriptor, LoaderHandle as ModuleLoaderHandle, LoaderModuleHandle, ModuleError,
    ModuleHandle, ModuleInfo, ModuleInterface, ModuleLoaderInterface, ModuleStatus, ModuleType,
};
use crate::sys::SyncHandlerInterface;
use crate::version::{ReleaseType, Version, VersionError};
use crate::{BaseT, Bool, FnId};
use std::os::raw::c_char;
use std::ptr::NonNull;

/// A trait to denote a binding object to the `emf-core-base` interface.
pub trait InterfaceBinding {
    /// Initializes the binding object.
    ///
    /// # Safety
    ///
    /// The parameter `get_function_fn` must be able to accept `base_module`.
    ///
    /// # Panics
    ///
    /// This function panics if it can not initialize the binding
    unsafe fn initialize(
        base_module: *mut BaseT,
        get_function_fn: SysGetFunctionFn,
    ) -> &'static Self;

    /// Fetches the version of the bound `emf-core-base` interface.
    fn get_version(&self) -> Version;

    /// Fetches a pointer to the bound `emf-core-base` interface.
    fn get_module_ptr(&self) -> *mut BaseT;

    /// Binding to [emf_cbase_sys_lock](crate::sys::emf_cbase_sys_lock).
    ///
    /// # Safety
    ///
    /// Wrong usage of this function will lead to a deadlock.
    /// The locked interface must be manually unlocked.
    unsafe fn sys_lock(&self);

    /// Binding to [emf_cbase_sys_try_lock](crate::sys::emf_cbase_sys_try_lock).
    ///
    /// # Safety
    ///
    /// The locked interface must be manually unlocked.
    unsafe fn sys_try_lock(&self) -> Bool;

    /// Binding to [emf_cbase_sys_unlock](crate::sys::emf_cbase_sys_unlock).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn sys_unlock(&self);

    /// Binding to [emf_cbase_sys_shutdown](crate::sys::emf_cbase_sys_shutdown).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn sys_shutdown(&self) -> !;

    /// Binding to [emf_cbase_sys_panic](crate::sys::emf_cbase_sys_panic).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn sys_panic(&self, error: *const c_char) -> !;

    /// Binding to [emf_cbase_sys_has_function](crate::sys::emf_cbase_sys_has_function).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn sys_has_function(&self, fn_id: FnId) -> Bool;

    /// Binding to [emf_cbase_sys_get_function](crate::sys::emf_cbase_sys_get_function).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn sys_get_function(&self, fn_id: FnId) -> Optional<BaseFn>;

    /// Binding to [emf_cbase_sys_get_sync_handler](crate::sys::emf_cbase_sys_get_sync_handler).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn sys_get_sync_handler(&self) -> NonNullConst<SyncHandlerInterface>;

    /// Binding to [emf_cbase_sys_set_sync_handler](crate::sys::emf_cbase_sys_set_sync_handler).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn sys_set_sync_handler(&self, sync_handler: *const SyncHandlerInterface);

    /// Binding to
    /// [emf_cbase_version_construct_short](crate::version::emf_cbase_version_construct_short).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_construct_short(&self, major: i32, minor: i32, patch: i32) -> Version;

    /// Binding to
    /// [emf_cbase_version_construct_long](crate::version::emf_cbase_version_construct_long).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_construct_long(
        &self,
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
    ) -> Version;

    /// Binding to
    /// [emf_cbase_version_construct_full](crate::version::emf_cbase_version_construct_full).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_construct_full(
        &self,
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
        build: i64,
    ) -> Version;

    /// Binding to
    /// [emf_cbase_version_construct_from_string](crate::version::emf_cbase_version_construct_from_string).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_construct_from_string(
        &self,
        version_string: NonNullConst<Span<'_, c_char>>,
    ) -> Result<Version, VersionError>;

    /// Binding to
    /// [emf_cbase_version_representation_is_valid](crate::version::emf_cbase_version_representation_is_valid).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_representation_is_valid(
        &self,
        version_string: NonNullConst<Span<'_, c_char>>,
    ) -> Bool;

    /// Binding to
    /// [emf_cbase_version_get_short_representation](crate::version::emf_cbase_version_get_short_representation).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_get_short_representation(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<'_, c_char>>,
    ) -> Result<usize, VersionError>;

    /// Binding to
    /// [emf_cbase_version_get_short_representation_length](crate::version::emf_cbase_version_get_short_representation_length).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_get_short_representation_length(
        &self,
        version: NonNullConst<Version>,
    ) -> usize;

    /// Binding to
    /// [emf_cbase_version_get_long_representation](crate::version::emf_cbase_version_get_long_representation).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_get_long_representation(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<'_, c_char>>,
    ) -> Result<usize, VersionError>;

    /// Binding to
    /// [emf_cbase_version_get_long_representation_length](crate::version::emf_cbase_version_get_long_representation_length).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_get_long_representation_length(
        &self,
        version: NonNullConst<Version>,
    ) -> usize;

    /// Binding to
    /// [emf_cbase_version_get_full_representation](crate::version::emf_cbase_version_get_full_representation).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_get_full_representation(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<'_, c_char>>,
    ) -> Result<usize, VersionError>;

    /// Binding to
    /// [emf_cbase_version_get_full_representation_length](crate::version::emf_cbase_version_get_full_representation_length).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_get_full_representation_length(
        &self,
        version: NonNullConst<Version>,
    ) -> usize;

    /// Binding to [emf_cbase_version_compare](crate::version::emf_cbase_version_compare).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_compare(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>)
        -> i32;

    /// Binding to [emf_cbase_version_compare_weak](crate::version::emf_cbase_version_compare_weak).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_compare_weak(
        &self,
        lhs: NonNullConst<Version>,
        rhs: NonNullConst<Version>,
    ) -> i32;

    /// Binding to
    /// [emf_cbase_version_compare_strong](crate::version::emf_cbase_version_compare_strong).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_compare_strong(
        &self,
        lhs: NonNullConst<Version>,
        rhs: NonNullConst<Version>,
    ) -> i32;

    /// Binding to
    /// [emf_cbase_version_is_compatible](crate::version::emf_cbase_version_is_compatible).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn version_is_compatible(
        &self,
        lhs: NonNullConst<Version>,
        rhs: NonNullConst<Version>,
    ) -> Bool;

    /// Binding to
    /// [emf_cbase_library_register_loader](crate::library::emf_cbase_library_register_loader).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_register_loader(
        &self,
        loader_interface: NonNullConst<LoaderInterface>,
        library_type: NonNullConst<LibraryType>,
    ) -> Result<LibraryLoaderHandle, LibraryError>;

    /// Binding to
    /// [emf_cbase_library_unregister_loader](crate::library::emf_cbase_library_unregister_loader).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_unregister_loader(
        &self,
        loader_handle: LibraryLoaderHandle,
    ) -> Optional<LibraryError>;

    /// Binding to
    /// [emf_cbase_library_get_num_loaders](crate::library::emf_cbase_library_get_num_loaders).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_get_num_loaders(&self) -> usize;

    /// Binding to
    /// [emf_cbase_library_get_library_types](crate::library::emf_cbase_library_get_library_types).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_get_library_types(
        &self,
        buffer: NonNull<MutSpan<LibraryType>>,
    ) -> Result<usize, LibraryError>;

    /// Binding to
    /// [emf_cbase_library_get_loader_handle](crate::library::emf_cbase_library_get_loader_handle).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_get_loader_handle(
        &self,
        library_type: NonNullConst<LibraryType>,
    ) -> Result<LibraryLoaderHandle, LibraryError>;

    /// Binding to [emf_cbase_library_type_exists](crate::library::emf_cbase_library_type_exists).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_type_exists(&self, library_type: NonNullConst<LibraryType>) -> Bool;

    /// Binding to [emf_cbase_library_library_exists](crate::library::emf_cbase_library_library_exists).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_library_exists(&self, library_handle: LibraryHandle) -> Bool;

    /// Binding to
    /// [emf_cbase_library_unsafe_create_library_handle](crate::library::emf_cbase_library_unsafe_create_library_handle).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_unsafe_create_library_handle(&self) -> LibraryHandle;

    /// Binding to
    /// [emf_cbase_library_unsafe_remove_library_handle](crate::library::emf_cbase_library_unsafe_remove_library_handle).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_unsafe_remove_library_handle(
        &self,
        library_handle: LibraryHandle,
    ) -> Optional<LibraryError>;

    /// Binding to
    /// [emf_cbase_library_unsafe_link_library](crate::library::emf_cbase_library_unsafe_link_library).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_unsafe_link_library(
        &self,
        library_handle: LibraryHandle,
        loader_handle: LibraryLoaderHandle,
        internal_handle: LoaderLibraryHandle,
    ) -> Optional<LibraryError>;

    /// Binding to
    /// [emf_cbase_library_unsafe_get_loader_library_handle](crate::library::emf_cbase_library_unsafe_get_loader_library_handle).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_unsafe_get_loader_library_handle(
        &self,
        library_handle: LibraryHandle,
    ) -> Result<LoaderLibraryHandle, LibraryError>;

    /// Binding to
    /// [emf_cbase_library_unsafe_get_loader_handle](crate::library::emf_cbase_library_unsafe_get_loader_handle).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_unsafe_get_loader_handle(
        &self,
        library_handle: LibraryHandle,
    ) -> Result<LibraryLoaderHandle, LibraryError>;

    /// Binding to
    /// [emf_cbase_library_unsafe_get_loader_interface](crate::library::emf_cbase_library_unsafe_get_loader_interface).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_unsafe_get_loader_interface(
        &self,
        loader_handle: LibraryLoaderHandle,
    ) -> Result<NonNullConst<LoaderInterface>, LibraryError>;

    /// Binding to [emf_cbase_library_load](crate::library::emf_cbase_library_load).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_load(
        &self,
        loader_handle: LibraryLoaderHandle,
        library_path: NonNullConst<OsPathChar>,
    ) -> Result<LibraryHandle, LibraryError>;

    /// Binding to [emf_cbase_library_unload](crate::library::emf_cbase_library_unload).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_unload(&self, library_handle: LibraryHandle) -> Optional<LibraryError>;

    /// Binding to
    /// [emf_cbase_library_get_data_symbol](crate::library::emf_cbase_library_get_data_symbol).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_get_data_symbol(
        &self,
        library_handle: LibraryHandle,
        symbol_name: NonNullConst<c_char>,
    ) -> Result<DataSymbol, LibraryError>;

    /// Binding to
    /// [emf_cbase_library_get_function_symbol](crate::library::emf_cbase_library_get_function_symbol).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn library_get_function_symbol(
        &self,
        library_handle: LibraryHandle,
        symbol_name: NonNullConst<c_char>,
    ) -> Result<FnSymbol, LibraryError>;

    /// Binding to
    /// [emf_cbase_module_register_loader](crate::module::emf_cbase_module_register_loader).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_register_loader(
        &self,
        loader_interface: NonNullConst<ModuleLoaderInterface>,
        module_type: NonNullConst<ModuleType>,
    ) -> Result<ModuleLoaderHandle, ModuleError>;

    /// Binding to
    /// [emf_cbase_module_unregister_loader](crate::module::emf_cbase_module_unregister_loader).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_unregister_loader(
        &self,
        loader_handle: ModuleLoaderHandle,
    ) -> Optional<ModuleError>;

    /// Binding to
    /// [emf_cbase_module_get_num_loaders](crate::module::emf_cbase_module_get_num_loaders).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_get_num_loaders(&self) -> usize;

    /// Binding to
    /// [emf_cbase_module_get_module_types](crate::module::emf_cbase_module_get_module_types).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_get_module_types(
        &self,
        buffer: NonNull<MutSpan<ModuleType>>,
    ) -> Result<usize, ModuleError>;

    /// Binding to
    /// [emf_cbase_module_get_num_modules](crate::module::emf_cbase_module_get_num_modules).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_get_num_modules(&self) -> usize;

    /// Binding to [emf_cbase_module_get_modules](crate::module::emf_cbase_module_get_modules).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_get_modules(
        &self,
        buffer: NonNull<MutSpan<ModuleInfo>>,
    ) -> Result<usize, ModuleError>;

    /// Binding to
    /// [emf_cbase_module_get_num_exported_interfaces](crate::module::emf_cbase_module_get_num_exported_interfaces).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_get_num_exported_interfaces(&self) -> usize;

    /// Binding to
    /// [emf_cbase_module_get_exported_interfaces](crate::module::emf_cbase_module_get_exported_interfaces).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_get_exported_interfaces(
        &self,
        buffer: NonNull<MutSpan<InterfaceDescriptor>>,
    ) -> Result<usize, ModuleError>;

    /// Binding to
    /// [emf_cbase_module_get_loader_handle](crate::module::emf_cbase_module_get_loader_handle).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_get_loader_handle(
        &self,
        module_type: NonNullConst<ModuleType>,
    ) -> Result<ModuleLoaderHandle, ModuleError>;

    /// Binding to [emf_cbase_module_type_exists](crate::module::emf_cbase_module_type_exists).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_type_exists(&self, module_type: NonNullConst<ModuleType>) -> Bool;

    /// Binding to [emf_cbase_module_module_exists](crate::module::emf_cbase_module_module_exists).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_module_exists(&self, module_handle: ModuleHandle) -> Bool;

    /// Binding to
    /// [emf_cbase_module_get_exported_interface_handle](crate::module::emf_cbase_module_get_exported_interface_handle).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_get_exported_interface_handle(
        &self,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<ModuleHandle, ModuleError>;

    /// Binding to
    /// [emf_cbase_module_exported_interface_exists](crate::module::emf_cbase_module_exported_interface_exists).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_exported_interface_exists(
        &self,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Bool;

    /// Binding to
    /// [emf_cbase_module_unsafe_create_module_handle](crate::module::emf_cbase_module_unsafe_create_module_handle).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_unsafe_create_module_handle(&self) -> ModuleHandle;

    /// Binding to
    /// [emf_cbase_module_unsafe_remove_module_handle](crate::module::emf_cbase_module_unsafe_remove_module_handle).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_unsafe_remove_module_handle(
        &self,
        module_handle: ModuleHandle,
    ) -> Optional<ModuleError>;

    /// Binding to
    /// [emf_cbase_module_unsafe_link_module](crate::module::emf_cbase_module_unsafe_link_module).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_unsafe_link_module(
        &self,
        module_handle: ModuleHandle,
        loader_handle: ModuleLoaderHandle,
        internal_handle: LoaderModuleHandle,
    ) -> Optional<ModuleError>;

    /// Binding to
    /// [emf_cbase_module_unsafe_get_loader_module_handle](crate::module::emf_cbase_module_unsafe_get_loader_module_handle).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_unsafe_get_loader_module_handle(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<LoaderModuleHandle, ModuleError>;

    /// Binding to
    /// [emf_cbase_module_unsafe_get_loader_handle](crate::module::emf_cbase_module_unsafe_get_loader_handle).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_unsafe_get_loader_handle(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<ModuleLoaderHandle, ModuleError>;

    /// Binding to
    /// [emf_cbase_module_unsafe_get_loader](crate::module::emf_cbase_module_unsafe_get_loader).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_unsafe_get_loader(
        &self,
        loader_handle: ModuleLoaderHandle,
    ) -> Result<NonNullConst<ModuleLoaderInterface>, ModuleError>;

    /// Binding to [emf_cbase_module_add_module](crate::module::emf_cbase_module_add_module).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_add_module(
        &self,
        loader_handle: ModuleLoaderHandle,
        module_path: NonNullConst<OsPathChar>,
    ) -> Result<ModuleHandle, ModuleError>;

    /// Binding to [emf_cbase_module_remove_module](crate::module::emf_cbase_module_remove_module).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_remove_module(&self, module_handle: ModuleHandle) -> Optional<ModuleError>;

    /// Binding to
    /// [emf_cbase_module_get_load_dependencies](crate::module::emf_cbase_module_get_load_dependencies).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_get_load_dependencies(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError>;

    /// Binding to [emf_cbase_module_fetch_status](crate::module::emf_cbase_module_fetch_status).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_fetch_status(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<ModuleStatus, ModuleError>;

    /// Binding to [emf_cbase_module_add_dependency](crate::module::emf_cbase_module_add_dependency).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_add_dependency(
        &self,
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Optional<ModuleError>;

    /// Binding to
    /// [emf_cbase_module_remove_dependency](crate::module::emf_cbase_module_remove_dependency).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_remove_dependency(
        &self,
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Optional<ModuleError>;

    /// Binding to
    /// [emf_cbase_module_export_interface](crate::module::emf_cbase_module_export_interface).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_export_interface(
        &self,
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Optional<ModuleError>;

    /// Binding to [emf_cbase_module_load](crate::module::emf_cbase_module_load).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_load(&self, module_handle: ModuleHandle) -> Optional<ModuleError>;

    /// Binding to [emf_cbase_module_unload](crate::module::emf_cbase_module_unload).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_unload(&self, module_handle: ModuleHandle) -> Optional<ModuleError>;

    /// Binding to [emf_cbase_module_initialize](crate::module::emf_cbase_module_initialize).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_initialize(&self, module_handle: ModuleHandle) -> Optional<ModuleError>;

    /// Binding to [emf_cbase_module_terminate](crate::module::emf_cbase_module_terminate).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_terminate(&self, module_handle: ModuleHandle) -> Optional<ModuleError>;

    /// Binding to [emf_cbase_module_get_module_info](crate::module::emf_cbase_module_get_module_info).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_get_module_info(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<NonNullConst<ModuleInfo>, ModuleError>;

    /// Binding to
    /// [emf_cbase_module_get_exportable_interfaces](crate::module::emf_cbase_module_get_exportable_interfaces).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_get_exportable_interfaces(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError>;

    /// Binding to
    /// [emf_cbase_module_get_runtime_dependencies](crate::module::emf_cbase_module_get_runtime_dependencies).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_get_runtime_dependencies(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError>;

    /// Binding to [emf_cbase_module_get_interface](crate::module::emf_cbase_module_get_interface).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_get_interface(
        &self,
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Result<ModuleInterface, ModuleError>;

    /// Binding to [emf_cbase_module_get_module_path](crate::module::emf_cbase_module_get_module_path).
    ///
    /// # Safety
    ///
    /// Calling this function before calling [sys_lock](InterfaceBinding::sys_lock)
    /// leads to undefined behaviour.
    unsafe fn module_get_module_path(
        &self,
        module_handle: ModuleHandle,
    ) -> Result<NonNullConst<OsPathChar>, ModuleError>;
}
