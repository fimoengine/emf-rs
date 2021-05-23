use crate::collections::NonNullConst;
use crate::library::api as lib_api;
use crate::library::api::LibraryBinding;
use crate::module::api as mod_api;
use crate::module::api::ModuleBinding;
use crate::sys::api as sys_api;
use crate::sys::api::SysBinding;
use crate::version::api::VersionBinding;
use crate::version::{api as ver_api, Version};
use std::ptr::NonNull;

/// Name of the `emf-core-base` interface.
pub const CBASE_INTERFACE_NAME: &str = "emf::core_base";

/// Opaque structure representing the `emf-core-base` interface.
#[repr(C)]
pub struct CBase {
    _dummy: [u8; 0],
}

/// Type erased function.
pub type CBaseFn = fn() -> ();

/// `emf-core-base` interface.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CBaseInterfaceVTable {
    pub version: Version,

    pub sys_shutdown_fn: sys_api::ShutdownFn,
    pub sys_panic_fn: sys_api::PanicFn,
    pub sys_has_function_fn: sys_api::HasFunctionFn,
    pub sys_get_function_fn: sys_api::GetFunctionFn,
    pub sys_lock_fn: sys_api::LockFn,
    pub sys_try_lock_fn: sys_api::TryLockFn,
    pub sys_unlock_fn: sys_api::UnlockFn,
    pub sys_get_sync_handler_fn: sys_api::GetSyncHandlerFn,
    pub sys_set_sync_handler_fn: sys_api::SetSyncHandlerFn,

    pub version_new_short_fn: ver_api::NewShortFn,
    pub version_new_long_fn: ver_api::NewLongFn,
    pub version_new_full_fn: ver_api::NewFullFn,
    pub version_from_string_fn: ver_api::FromStringFn,
    pub version_string_length_short_fn: ver_api::StringLengthShortFn,
    pub version_string_length_long_fn: ver_api::StringLengthLongFn,
    pub version_string_length_full_fn: ver_api::StringLengthFullFn,
    pub version_as_string_short_fn: ver_api::AsStringShortFn,
    pub version_as_string_long_fn: ver_api::AsStringLongFn,
    pub version_as_string_full_fn: ver_api::AsStringFullFn,
    pub version_string_is_valid_fn: ver_api::StringIsValidFn,
    pub version_compare_fn: ver_api::CompareFn,
    pub version_compare_weak_fn: ver_api::CompareWeakFn,
    pub version_compare_strong_fn: ver_api::CompareStrongFn,
    pub version_is_compatible_fn: ver_api::IsCompatibleFn,

    pub library_register_loader_fn: lib_api::RegisterLoaderFn,
    pub library_unregister_loader_fn: lib_api::UnregisterLoaderFn,
    pub library_get_loader_interface_fn: lib_api::GetLoaderInterfaceFn,
    pub library_get_loader_handle_from_type_fn: lib_api::GetLoaderHandleFromTypeFn,
    pub library_get_loader_handle_from_library_fn: lib_api::GetLoaderHandleFromLibraryFn,
    pub library_get_num_loaders_fn: lib_api::GetNumLoadersFn,
    pub library_library_exists_fn: lib_api::LibraryExistsFn,
    pub library_type_exists_fn: lib_api::TypeExistsFn,
    pub library_get_library_types_fn: lib_api::GetLibraryTypesFn,
    pub library_create_library_handle_fn: lib_api::CreateLibraryHandleFn,
    pub library_remove_library_handle_fn: lib_api::RemoveLibraryHandleFn,
    pub library_link_library_fn: lib_api::LinkLibraryFn,
    pub library_get_internal_library_handle_fn: lib_api::GetInternalLibraryHandleFn,
    pub library_load_fn: lib_api::LoadFn,
    pub library_unload_fn: lib_api::UnloadFn,
    pub library_get_data_symbol_fn: lib_api::GetDataSymbolFn,
    pub library_get_function_symbol_fn: lib_api::GetFunctionSymbolFn,

    pub module_register_loader_fn: mod_api::RegisterLoaderFn,
    pub module_unregister_loader_fn: mod_api::UnregisterLoaderFn,
    pub module_get_loader_interface_fn: mod_api::GetLoaderInterfaceFn,
    pub module_get_loader_handle_from_type_fn: mod_api::GetLoaderHandleFromTypeFn,
    pub module_get_loader_handle_from_module_fn: mod_api::GetLoaderHandleFromModuleFn,
    pub module_get_num_modules_fn: mod_api::GetNumModulesFn,
    pub module_get_num_loaders_fn: mod_api::GetNumLoadersFn,
    pub module_get_num_exported_interfaces_fn: mod_api::GetNumExportedInterfacesFn,
    pub module_module_exists_fn: mod_api::ModuleExistsFn,
    pub module_type_exists_fn: mod_api::TypeExistsFn,
    pub module_exported_interface_exists_fn: mod_api::ExportedInterfaceExistsFn,
    pub module_get_modules_fn: mod_api::GetModulesFn,
    pub module_get_module_types_fn: mod_api::GetModuleTypesFn,
    pub module_get_exported_interfaces_fn: mod_api::GetExportedInterfacesFn,
    pub module_get_exported_interface_handle_fn: mod_api::GetExportedInterfaceHandleFn,
    pub module_create_module_handle_fn: mod_api::CreateModuleHandleFn,
    pub module_remove_module_handle_fn: mod_api::RemoveModuleHandleFn,
    pub module_link_module_fn: mod_api::LinkModuleFn,
    pub module_get_internal_module_handle_fn: mod_api::GetInternalModuleHandleFn,
    pub module_add_module_fn: mod_api::AddModuleFn,
    pub module_remove_module_fn: mod_api::RemoveModuleFn,
    pub module_load_fn: mod_api::LoadFn,
    pub module_unload_fn: mod_api::UnloadFn,
    pub module_initialize_fn: mod_api::InitializeFn,
    pub module_terminate_fn: mod_api::TerminateFn,
    pub module_add_dependency_fn: mod_api::AddDependencyFn,
    pub module_remove_dependency_fn: mod_api::RemoveDependencyFn,
    pub module_export_interface_fn: mod_api::ExportInterfaceFn,
    pub module_get_load_dependencies_fn: mod_api::GetLoadDependenciesFn,
    pub module_get_runtime_dependencies_fn: mod_api::GetRuntimeDependenciesFn,
    pub module_get_exportable_interfaces_fn: mod_api::GetExportableInterfacesFn,
    pub module_fetch_status_fn: mod_api::FetchStatusFn,
    pub module_get_module_path_fn: mod_api::GetModulePathFn,
    pub module_get_module_info_fn: mod_api::GetModuleInfoFn,
    pub module_get_interface_fn: mod_api::GetInterfaceFn,
}

/// `emf-core-base` interface.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct CBaseInterface {
    pub base_module: Option<NonNull<CBase>>,
    pub vtable: NonNullConst<CBaseInterfaceVTable>,
}

unsafe impl Send for CBaseInterface {}
unsafe impl Sync for CBaseInterface {}

/// Helper trait for using the `emf-core-base` interface.
pub trait CBaseBinding: SysBinding + VersionBinding + LibraryBinding + ModuleBinding {
    /// Returns the version of the interface.
    fn interface_version(&self) -> Version;

    /// Returns a handle to the interface.
    fn base_module(&self) -> Option<NonNull<CBase>>;

    /// Returns a fn pointer to the `has_function` fn.
    fn fetch_has_function_fn(&self) -> sys_api::HasFunctionFn;

    /// Returns a fn pointer to the `get_function` fn.
    fn fetch_get_function_fn(&self) -> sys_api::GetFunctionFn;
}

impl CBaseBinding for CBaseInterface {
    #[inline]
    fn interface_version(&self) -> Version {
        unsafe { self.vtable.as_ref().version }
    }

    #[inline]
    fn base_module(&self) -> Option<NonNull<CBase>> {
        self.base_module
    }

    #[inline]
    fn fetch_has_function_fn(&self) -> sys_api::HasFunctionFn {
        unsafe { self.vtable.as_ref().sys_has_function_fn }
    }

    #[inline]
    fn fetch_get_function_fn(&self) -> sys_api::GetFunctionFn {
        unsafe { self.vtable.as_ref().sys_get_function_fn }
    }
}
