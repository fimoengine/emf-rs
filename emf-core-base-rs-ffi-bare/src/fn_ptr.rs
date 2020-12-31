//! Function types specified by the `emf-core-base` interface.
use crate::containers::{MutSpan, NonNullConst, Optional, Result, Span};
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

/// A type-erased function pointer.
pub type BaseFn = unsafe extern "C" fn();

/// Function pointer to the [emf_cbase_sys_lock](crate::sys::emf_cbase_sys_lock) function.
pub type SysLockFn = unsafe extern "C" fn(base_module: *mut BaseT);

/// Function pointer to the [emf_cbase_sys_try_lock](crate::sys::emf_cbase_sys_try_lock) function.
pub type SysTryLockFn = unsafe extern "C" fn(base_module: *mut BaseT) -> Bool;

/// Function pointer to the [emf_cbase_sys_unlock](crate::sys::emf_cbase_sys_unlock) function.
pub type SysUnlockFn = unsafe extern "C" fn(base_module: *mut BaseT);

/// Function pointer to the [emf_cbase_sys_shutdown](crate::sys::emf_cbase_sys_shutdown) function.
pub type SysShutdownFn = unsafe extern "C" fn(base_module: *mut BaseT) -> !;

/// Function pointer to the [emf_cbase_sys_panic](crate::sys::emf_cbase_sys_panic) function.
pub type SysPanicFn = unsafe extern "C" fn(base_module: *mut BaseT, error: *const c_char) -> !;

/// Function pointer to the
/// [emf_cbase_sys_has_function](crate::sys::emf_cbase_sys_has_function) function.
pub type SysHasFunctionFn = unsafe extern "C" fn(base_module: *mut BaseT, fn_id: FnId) -> Bool;

/// Function pointer to the
/// [emf_cbase_sys_get_function](crate::sys::emf_cbase_sys_get_function) function.
pub type SysGetFunctionFn =
    unsafe extern "C" fn(base_module: *mut BaseT, fn_id: FnId) -> Optional<BaseFn>;

/// Function pointer to the
/// [emf_cbase_sys_get_sync_handler](crate::sys::emf_cbase_sys_get_sync_handler) function.
pub type SysGetSyncHandlerFn =
    unsafe extern "C" fn(base_module: *mut BaseT) -> NonNullConst<SyncHandlerInterface>;

/// Function pointer to the
/// [emf_cbase_sys_set_sync_handler](crate::sys::emf_cbase_sys_set_sync_handler) function.
pub type SysSetSyncHandlerFn =
    unsafe extern "C" fn(base_module: *mut BaseT, sync_handler: *const SyncHandlerInterface);

/// Function pointer to the
/// [emf_cbase_version_construct_short](crate::version::emf_cbase_version_construct_short) function.
pub type VersionConstructShortFn =
    unsafe extern "C" fn(base_module: *mut BaseT, major: i32, minor: i32, patch: i32) -> Version;

/// Function pointer to the
/// [emf_cbase_version_construct_long](crate::version::emf_cbase_version_construct_long) function.
pub type VersionConstructLongFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    major: i32,
    minor: i32,
    patch: i32,
    release_type: ReleaseType,
    release_number: i8,
) -> Version;

/// Function pointer to the
/// [emf_cbase_version_construct_full](crate::version::emf_cbase_version_construct_full) function.
pub type VersionConstructFullFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    major: i32,
    minor: i32,
    patch: i32,
    release_type: ReleaseType,
    release_number: i8,
    build: i64,
) -> Version;

/// Function pointer to the
/// [emf_cbase_version_construct_from_string](crate::version::emf_cbase_version_construct_from_string)
/// function.
pub type VersionConstructFromStringFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    version_string: NonNullConst<Span<'_, c_char>>,
) -> Result<Version, VersionError>;

/// Function pointer to the
/// [emf_cbase_version_representation_is_valid](crate::version::emf_cbase_version_representation_is_valid)
/// function.
pub type VersionRepresentationIsValidFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    version_string: NonNullConst<Span<'_, c_char>>,
) -> Bool;

/// Function pointer to the
/// [emf_cbase_version_get_short_representation](crate::version::emf_cbase_version_get_short_representation)
/// function.
pub type VersionGetShortRepresentationFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    version: NonNullConst<Version>,
    buffer: NonNull<MutSpan<'_, c_char>>,
) -> Result<usize, VersionError>;

/// Function pointer to the
/// [emf_cbase_version_get_short_representation_length](crate::version::emf_cbase_version_get_short_representation_length)
/// function.
pub type VersionGetShortRepresentationLengthFn =
    unsafe extern "C" fn(base_module: *mut BaseT, version: NonNullConst<Version>) -> usize;

/// Function pointer to the
/// [emf_cbase_version_get_long_representation](crate::version::emf_cbase_version_get_long_representation)
/// function.
pub type VersionGetLongRepresentationFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    version: NonNullConst<Version>,
    buffer: NonNull<MutSpan<'_, c_char>>,
) -> Result<usize, VersionError>;

/// Function pointer to the
/// [emf_cbase_version_get_long_representation_length](crate::version::emf_cbase_version_get_long_representation_length)
/// function.
pub type VersionGetLongRepresentationLengthFn =
    unsafe extern "C" fn(base_module: *mut BaseT, version: NonNullConst<Version>) -> usize;

/// Function pointer to the
/// [emf_cbase_version_get_full_representation](crate::version::emf_cbase_version_get_full_representation)
/// function.
pub type VersionGetFullRepresentationFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    version: NonNullConst<Version>,
    buffer: NonNull<MutSpan<'_, c_char>>,
) -> Result<usize, VersionError>;

/// Function pointer to the
/// [emf_cbase_version_get_full_representation_length](crate::version::emf_cbase_version_get_full_representation_length)
/// function.
pub type VersionGetFullRepresentationLengthFn =
    unsafe extern "C" fn(base_module: *mut BaseT, version: NonNullConst<Version>) -> usize;

/// Function pointer to the
/// [emf_cbase_version_compare](crate::version::emf_cbase_version_compare) function.
pub type VersionCompareFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    lhs: NonNullConst<Version>,
    rhs: NonNullConst<Version>,
) -> i32;

/// Function pointer to the
/// [emf_cbase_version_compare_weak](crate::version::emf_cbase_version_compare_weak) function.
pub type VersionCompareWeakFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    lhs: NonNullConst<Version>,
    rhs: NonNullConst<Version>,
) -> i32;

/// Function pointer to the
/// [emf_cbase_version_compare_strong](crate::version::emf_cbase_version_compare_strong) function.
pub type VersionCompareStrongFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    lhs: NonNullConst<Version>,
    rhs: NonNullConst<Version>,
) -> i32;

/// Function pointer to the
/// [emf_cbase_version_is_compatible](crate::version::emf_cbase_version_is_compatible) function.
pub type VersionIsCompatibleFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    lhs: NonNullConst<Version>,
    rhs: NonNullConst<Version>,
) -> Bool;

/// Function pointer to the
/// [emf_cbase_library_register_loader](crate::library::emf_cbase_library_register_loader) function.
pub type LibraryRegisterLoaderFn =
    unsafe extern "C" fn(
        base_module: *mut BaseT,
        loader_interface: NonNullConst<LoaderInterface>,
        library_type: NonNullConst<LibraryType>,
    ) -> Result<LibraryLoaderHandle, LibraryError>;

/// Function pointer to the
/// [emf_cbase_library_unregister_loader](crate::library::emf_cbase_library_unregister_loader)
/// function.
pub type LibraryUnregisterLoaderFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    loader_handle: LibraryLoaderHandle,
) -> Optional<LibraryError>;

/// Function pointer to the
/// [emf_cbase_library_get_num_loaders](crate::library::emf_cbase_library_get_num_loaders) function.
pub type LibraryGetNumLoadersFn = unsafe extern "C" fn(base_module: *mut BaseT) -> usize;

/// Function pointer to the
/// [emf_cbase_library_get_library_types](crate::library::emf_cbase_library_get_library_types)
/// function.
pub type LibraryGetLibraryTypesFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    buffer: NonNull<MutSpan<LibraryType>>,
) -> Result<usize, LibraryError>;

/// Function pointer to the
/// [emf_cbase_library_get_loader_handle](crate::library::emf_cbase_library_get_loader_handle)
/// function.
pub type LibraryGetLoaderHandleFn =
    unsafe extern "C" fn(
        base_module: *mut BaseT,
        library_type: NonNullConst<LibraryType>,
    ) -> Result<LibraryLoaderHandle, LibraryError>;

/// Function pointer to the
/// [emf_cbase_library_type_exists](crate::library::emf_cbase_library_type_exists) function.
pub type LibraryTypeExistsFn =
    unsafe extern "C" fn(base_module: *mut BaseT, library_type: NonNullConst<LibraryType>) -> Bool;

/// Function pointer to the
/// [emf_cbase_library_library_exists](crate::library::emf_cbase_library_library_exists) function.
pub type LibraryLibraryExistsFn =
    unsafe extern "C" fn(base_module: *mut BaseT, library_handle: LibraryHandle) -> Bool;

/// Function pointer to the
/// [emf_cbase_library_unsafe_create_library_handle](crate::library::emf_cbase_library_unsafe_create_library_handle)
/// function.
pub type LibraryUnsafeCreateLibraryHandleFn =
    unsafe extern "C" fn(base_module: *mut BaseT) -> LibraryHandle;

/// Function pointer to the
/// [emf_cbase_library_unsafe_remove_library_handle](crate::library::emf_cbase_library_unsafe_remove_library_handle)
/// function.
pub type LibraryUnsafeRemoveLibraryHandleFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    library_handle: LibraryHandle,
) -> Optional<LibraryError>;

/// Function pointer to the
/// [emf_cbase_library_unsafe_link_library](crate::library::emf_cbase_library_unsafe_link_library)
/// function.
pub type LibraryUnsafeLinkLibraryFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    library_handle: LibraryHandle,
    loader_handle: LibraryLoaderHandle,
    internal_handle: LoaderLibraryHandle,
) -> Optional<LibraryError>;

/// Function pointer to the
/// [emf_cbase_library_unsafe_get_loader_library_handle](crate::library::emf_cbase_library_unsafe_get_loader_library_handle)
/// function.
pub type LibraryUnsafeGetLoaderLibraryHandleFn =
    unsafe extern "C" fn(
        base_module: *mut BaseT,
        library_handle: LibraryHandle,
    ) -> Result<LoaderLibraryHandle, LibraryError>;

/// Function pointer to the
/// [emf_cbase_library_unsafe_get_loader_handle](crate::library::emf_cbase_library_unsafe_get_loader_handle)
/// function.
pub type LibraryUnsafeGetLoaderHandleFn =
    unsafe extern "C" fn(
        base_module: *mut BaseT,
        library_handle: LibraryHandle,
    ) -> Result<LibraryLoaderHandle, LibraryError>;

/// Function pointer to the
/// [emf_cbase_library_unsafe_get_loader_interface](crate::library::emf_cbase_library_unsafe_get_loader_interface)
/// function.
pub type LibraryUnsafeGetLoaderInterfaceFn =
    unsafe extern "C" fn(
        base_module: *mut BaseT,
        loader_handle: LibraryLoaderHandle,
    ) -> Result<NonNullConst<LoaderInterface>, LibraryError>;

/// Function pointer to the
/// [emf_cbase_library_load](crate::library::emf_cbase_library_load)
/// function.
pub type LibraryLoadFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    loader_handle: LibraryLoaderHandle,
    library_path: NonNullConst<OsPathChar>,
) -> Result<LibraryHandle, LibraryError>;

/// Function pointer to the
/// [emf_cbase_library_unload](crate::library::emf_cbase_library_unload) function.
pub type LibraryUnloadFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    library_handle: LibraryHandle,
) -> Optional<LibraryError>;

/// Function pointer to the
/// [emf_cbase_library_get_data_symbol](crate::library::emf_cbase_library_get_data_symbol) function.
pub type LibraryGetDataSymbolFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    library_handle: LibraryHandle,
    symbol_name: NonNullConst<c_char>,
) -> Result<DataSymbol, LibraryError>;

/// Function pointer to the
/// [emf_cbase_library_get_function_symbol](crate::library::emf_cbase_library_get_function_symbol)
/// function.
pub type LibraryGetFunctionSymbolFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    library_handle: LibraryHandle,
    symbol_name: NonNullConst<c_char>,
) -> Result<FnSymbol, LibraryError>;

/// Function pointer to the
/// [emf_cbase_module_register_loader](crate::module::emf_cbase_module_register_loader) function.
pub type ModuleRegisterLoaderFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    loader_interface: NonNullConst<ModuleLoaderInterface>,
    module_type: NonNullConst<ModuleType>,
) -> Result<ModuleLoaderHandle, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_unregister_loader](crate::module::emf_cbase_module_unregister_loader) function.
pub type ModuleUnregisterLoaderFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    loader_handle: ModuleLoaderHandle,
) -> Optional<ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_get_num_loaders](crate::module::emf_cbase_module_get_num_loaders) function.
pub type ModuleGetNumLoadersFn = unsafe extern "C" fn(base_module: *mut BaseT) -> usize;

/// Function pointer to the
/// [emf_cbase_module_get_module_types](crate::module::emf_cbase_module_get_module_types) function.
pub type ModuleGetModuleTypesFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    buffer: NonNull<MutSpan<ModuleType>>,
) -> Result<usize, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_get_num_modules](crate::module::emf_cbase_module_get_num_modules) function.
pub type ModuleGetNumModulesFn = unsafe extern "C" fn(base_module: *mut BaseT) -> usize;

/// Function pointer to the
/// [emf_cbase_module_get_modules](crate::module::emf_cbase_module_get_modules) function.
pub type ModuleGetModulesFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    buffer: NonNull<MutSpan<ModuleInfo>>,
) -> Result<usize, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_get_num_exported_interfaces](crate::module::emf_cbase_module_get_num_exported_interfaces)
/// function.
pub type ModuleGetNumExportedInterfacesFn = unsafe extern "C" fn(base_module: *mut BaseT) -> usize;

/// Function pointer to the
/// [emf_cbase_module_get_exported_interfaces](crate::module::emf_cbase_module_get_exported_interfaces)
/// function.
pub type ModuleGetExportedInterfacesFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    buffer: NonNull<MutSpan<InterfaceDescriptor>>,
) -> Result<usize, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_get_loader_handle](crate::module::emf_cbase_module_get_loader_handle) function.
pub type ModuleGetLoaderHandleFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    module_type: NonNullConst<ModuleType>,
) -> Result<ModuleLoaderHandle, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_type_exists](crate::module::emf_cbase_module_type_exists) function.
pub type ModuleTypeExistsFn =
    unsafe extern "C" fn(base_module: *mut BaseT, module_type: NonNullConst<ModuleType>) -> Bool;

/// Function pointer to the
/// [emf_cbase_module_module_exists](crate::module::emf_cbase_module_module_exists) function.
pub type ModuleModuleExistsFn =
    unsafe extern "C" fn(base_module: *mut BaseT, module_handle: ModuleHandle) -> Bool;

/// Function pointer to the
/// [emf_cbase_module_get_exported_interface_handle](crate::module::emf_cbase_module_get_exported_interface_handle)
/// function.
pub type ModuleGetExportedInterfaceHandleFn =
    unsafe extern "C" fn(
        base_module: *mut BaseT,
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<ModuleHandle, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_exported_interface_exists](crate::module::emf_cbase_module_exported_interface_exists)
/// function.
pub type ModuleExportedInterfaceExistsFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    interface: NonNullConst<InterfaceDescriptor>,
) -> Bool;

/// Function pointer to the
/// [emf_cbase_module_unsafe_create_module_handle](crate::module::emf_cbase_module_unsafe_create_module_handle)
/// function.
pub type ModuleUnsafeCreateModuleHandleFn =
    unsafe extern "C" fn(base_module: *mut BaseT) -> ModuleHandle;

/// Function pointer to the
/// [emf_cbase_module_unsafe_remove_module_handle](crate::module::emf_cbase_module_unsafe_remove_module_handle)
/// function.
pub type ModuleUnsafeRemoveModuleHandleFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    module_handle: ModuleHandle,
) -> Optional<ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_unsafe_link_module](crate::module::emf_cbase_module_unsafe_link_module)
/// function.
pub type ModuleUnsafeLinkModuleFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    module_handle: ModuleHandle,
    loader_handle: ModuleLoaderHandle,
    internal_handle: LoaderModuleHandle,
) -> Optional<ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_unsafe_get_loader_module_handle](crate::module::emf_cbase_module_unsafe_get_loader_module_handle)
/// function.
pub type ModuleUnsafeGetLoaderModuleHandleFn =
    unsafe extern "C" fn(
        base_module: *mut BaseT,
        module_handle: ModuleHandle,
    ) -> Result<LoaderModuleHandle, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_unsafe_get_loader_handle](crate::module::emf_cbase_module_unsafe_get_loader_handle)
/// function.
pub type ModuleUnsafeGetLoaderHandleFn =
    unsafe extern "C" fn(
        base_module: *mut BaseT,
        module_handle: ModuleHandle,
    ) -> Result<ModuleLoaderHandle, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_unsafe_get_loader](crate::module::emf_cbase_module_unsafe_get_loader)
/// function.
pub type ModuleUnsafeGetLoaderFn =
    unsafe extern "C" fn(
        base_module: *mut BaseT,
        loader_handle: ModuleLoaderHandle,
    ) -> Result<NonNullConst<ModuleLoaderInterface>, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_add_module](crate::module::emf_cbase_module_add_module) function.
pub type ModuleAddModuleFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    loader_handle: ModuleLoaderHandle,
    module_path: NonNullConst<OsPathChar>,
) -> Result<ModuleHandle, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_remove_module](crate::module::emf_cbase_module_remove_module) function.
pub type ModuleRemoveModuleFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    module_handle: ModuleHandle,
) -> Optional<ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_get_load_dependencies](crate::module::emf_cbase_module_get_load_dependencies)
/// function.
pub type ModuleGetLoadDependenciesFn =
    unsafe extern "C" fn(
        base_module: *mut BaseT,
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_fetch_status](crate::module::emf_cbase_module_fetch_status) function.
pub type ModuleFetchStatusFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    module_handle: ModuleHandle,
) -> Result<ModuleStatus, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_add_dependency](crate::module::emf_cbase_module_add_dependency) function.
pub type ModuleAddDependencyFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    module_handle: ModuleHandle,
    interface_descriptor: NonNullConst<InterfaceDescriptor>,
) -> Optional<ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_remove_dependency](crate::module::emf_cbase_module_remove_dependency) function.
pub type ModuleRemoveDependencyFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    module_handle: ModuleHandle,
    interface_descriptor: NonNullConst<InterfaceDescriptor>,
) -> Optional<ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_export_interface](crate::module::emf_cbase_module_export_interface) function.
pub type ModuleExportInterfaceFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    module_handle: ModuleHandle,
    interface_descriptor: NonNullConst<InterfaceDescriptor>,
) -> Optional<ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_load](crate::module::emf_cbase_module_load) function.
pub type ModuleLoadFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    module_handle: ModuleHandle,
) -> Optional<ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_unload](crate::module::emf_cbase_module_unload) function.
pub type ModuleUnloadFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    module_handle: ModuleHandle,
) -> Optional<ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_initialize](crate::module::emf_cbase_module_initialize) function.
pub type ModuleInitializeFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    module_handle: ModuleHandle,
) -> Optional<ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_terminate](crate::module::emf_cbase_module_terminate) function.
pub type ModuleTerminateFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    module_handle: ModuleHandle,
) -> Optional<ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_get_module_info](crate::module::emf_cbase_module_get_module_info) function.
pub type ModuleGetModuleInfoFn =
    unsafe extern "C" fn(
        base_module: *mut BaseT,
        module_handle: ModuleHandle,
    ) -> Result<NonNullConst<ModuleInfo>, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_get_exportable_interfaces](crate::module::emf_cbase_module_get_exportable_interfaces)
/// function.
pub type ModuleGetExportableInterfacesFn =
    unsafe extern "C" fn(
        base_module: *mut BaseT,
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_get_runtime_dependencies](crate::module::emf_cbase_module_get_runtime_dependencies)
/// function.
pub type ModuleGetRuntimeDependenciesFn =
    unsafe extern "C" fn(
        base_module: *mut BaseT,
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_get_interface](crate::module::emf_cbase_module_get_interface) function.
pub type ModuleGetInterfaceFn = unsafe extern "C" fn(
    base_module: *mut BaseT,
    module_handle: ModuleHandle,
    interface_descriptor: NonNullConst<InterfaceDescriptor>,
) -> Result<ModuleInterface, ModuleError>;

/// Function pointer to the
/// [emf_cbase_module_get_module_path](crate::module::emf_cbase_module_get_module_path) function.
pub type ModuleGetModulePathFn =
    unsafe extern "C" fn(
        base_module: *mut BaseT,
        module_handle: ModuleHandle,
    ) -> Result<NonNullConst<OsPathChar>, ModuleError>;
