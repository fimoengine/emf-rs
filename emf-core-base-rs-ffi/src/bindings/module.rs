use crate::bindings::BASE_INTERFACE;
use crate::containers::{MutSpan, NonNullConst, Optional, Result, Span};
use crate::library::OsPathChar;
use crate::module::{
    InterfaceDescriptor, LoaderHandle, LoaderModuleHandle, ModuleError, ModuleHandle, ModuleInfo,
    ModuleInterface, ModuleLoaderInterface, ModuleStatus, ModuleType,
};
use crate::{Bool, InterfaceBinding};
use std::ptr::NonNull;

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_register_loader(
    loader_interface: NonNullConst<ModuleLoaderInterface>,
    module_type: NonNullConst<ModuleType>,
) -> Result<LoaderHandle, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_register_loader(loader_interface, module_type)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_unregister_loader(
    loader_handle: LoaderHandle,
) -> Optional<ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_unregister_loader(loader_handle)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_module_get_num_loaders() -> usize {
    (*BASE_INTERFACE.as_ptr()).module_get_num_loaders()
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_get_module_types(
    buffer: NonNull<MutSpan<ModuleType>>,
) -> Result<usize, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_get_module_types(buffer)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_module_get_num_modules() -> usize {
    (*BASE_INTERFACE.as_ptr()).module_get_num_modules()
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_get_modules(
    buffer: NonNull<MutSpan<ModuleInfo>>,
) -> Result<usize, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_get_modules(buffer)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_module_get_num_exported_interfaces() -> usize {
    (*BASE_INTERFACE.as_ptr()).module_get_num_exported_interfaces()
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_get_exported_interfaces(
    buffer: NonNull<MutSpan<InterfaceDescriptor>>,
) -> Result<usize, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_get_exported_interfaces(buffer)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_get_loader_handle(
    module_type: NonNullConst<ModuleType>,
) -> Result<LoaderHandle, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_get_loader_handle(module_type)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_module_type_exists(module_type: NonNullConst<ModuleType>) -> Bool {
    (*BASE_INTERFACE.as_ptr()).module_type_exists(module_type)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_module_module_exists(module_handle: ModuleHandle) -> Bool {
    (*BASE_INTERFACE.as_ptr()).module_module_exists(module_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_get_exported_interface_handle(
    interface: NonNullConst<InterfaceDescriptor>,
) -> Result<ModuleHandle, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_get_exported_interface_handle(interface)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_module_exported_interface_exists(
    interface: NonNullConst<InterfaceDescriptor>,
) -> Bool {
    (*BASE_INTERFACE.as_ptr()).module_exported_interface_exists(interface)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_module_unsafe_create_module_handle() -> ModuleHandle {
    (*BASE_INTERFACE.as_ptr()).module_unsafe_create_module_handle()
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_unsafe_remove_module_handle(
    module_handle: ModuleHandle,
) -> Optional<ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_unsafe_remove_module_handle(module_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_unsafe_link_module(
    module_handle: ModuleHandle,
    loader_handle: LoaderHandle,
    internal_handle: LoaderModuleHandle,
) -> Optional<ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_unsafe_link_module(
        module_handle,
        loader_handle,
        internal_handle,
    )
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_unsafe_get_loader_module_handle(
    module_handle: ModuleHandle,
) -> Result<LoaderModuleHandle, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_unsafe_get_loader_module_handle(module_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_unsafe_get_loader_handle(
    module_handle: ModuleHandle,
) -> Result<LoaderHandle, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_unsafe_get_loader_handle(module_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_unsafe_get_loader(
    loader_handle: LoaderHandle,
) -> Result<NonNullConst<ModuleLoaderInterface>, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_unsafe_get_loader(loader_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_add_module(
    loader_handle: LoaderHandle,
    module_path: NonNullConst<OsPathChar>,
) -> Result<ModuleHandle, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_add_module(loader_handle, module_path)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_remove_module(
    module_handle: ModuleHandle,
) -> Optional<ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_remove_module(module_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_get_load_dependencies(
    module_handle: ModuleHandle,
) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_get_load_dependencies(module_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_fetch_status(
    module_handle: ModuleHandle,
) -> Result<ModuleStatus, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_fetch_status(module_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_add_dependency(
    module_handle: ModuleHandle,
    interface_descriptor: NonNullConst<InterfaceDescriptor>,
) -> Optional<ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_add_dependency(module_handle, interface_descriptor)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_remove_dependency(
    module_handle: ModuleHandle,
    interface_descriptor: NonNullConst<InterfaceDescriptor>,
) -> Optional<ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_remove_dependency(module_handle, interface_descriptor)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_export_interface(
    module_handle: ModuleHandle,
    interface_descriptor: NonNullConst<InterfaceDescriptor>,
) -> Optional<ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_export_interface(module_handle, interface_descriptor)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_load(module_handle: ModuleHandle) -> Optional<ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_load(module_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_unload(module_handle: ModuleHandle) -> Optional<ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_unload(module_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_initialize(
    module_handle: ModuleHandle,
) -> Optional<ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_initialize(module_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_terminate(
    module_handle: ModuleHandle,
) -> Optional<ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_terminate(module_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_get_module_info(
    module_handle: ModuleHandle,
) -> Result<NonNullConst<ModuleInfo>, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_get_module_info(module_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_get_exportable_interfaces(
    module_handle: ModuleHandle,
) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_get_exportable_interfaces(module_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_get_runtime_dependencies(
    module_handle: ModuleHandle,
) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_get_runtime_dependencies(module_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_get_interface(
    module_handle: ModuleHandle,
    interface_descriptor: NonNullConst<InterfaceDescriptor>,
) -> Result<ModuleInterface, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_get_interface(module_handle, interface_descriptor)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_module_get_module_path(
    module_handle: ModuleHandle,
) -> Result<NonNullConst<OsPathChar>, ModuleError> {
    (*BASE_INTERFACE.as_ptr()).module_get_module_path(module_handle)
}
