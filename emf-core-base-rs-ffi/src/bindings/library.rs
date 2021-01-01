use crate::bindings::BASE_INTERFACE;
use crate::containers::{MutSpan, NonNullConst, Optional, Result};
use crate::library::{
    DataSymbol, FnSymbol, LibraryError, LibraryHandle, LibraryType, LoaderHandle, LoaderInterface,
    LoaderLibraryHandle, OsPathChar,
};
use crate::{Bool, InterfaceBinding};
use std::os::raw::c_char;
use std::ptr::NonNull;

#[cfg(test)]
mod tests;

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_library_register_loader(
    loader_interface: NonNullConst<LoaderInterface>,
    library_type: NonNullConst<LibraryType>,
) -> Result<LoaderHandle, LibraryError> {
    (*BASE_INTERFACE.as_ptr()).library_register_loader(loader_interface, library_type)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_library_unregister_loader(
    loader_handle: LoaderHandle,
) -> Optional<LibraryError> {
    (*BASE_INTERFACE.as_ptr()).library_unregister_loader(loader_handle)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_library_get_num_loaders() -> usize {
    (*BASE_INTERFACE.as_ptr()).library_get_num_loaders()
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_library_get_library_types(
    buffer: NonNull<MutSpan<LibraryType>>,
) -> Result<usize, LibraryError> {
    (*BASE_INTERFACE.as_ptr()).library_get_library_types(buffer)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_library_get_loader_handle(
    library_type: NonNullConst<LibraryType>,
) -> Result<LoaderHandle, LibraryError> {
    (*BASE_INTERFACE.as_ptr()).library_get_loader_handle(library_type)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_library_type_exists(
    library_type: NonNullConst<LibraryType>,
) -> Bool {
    (*BASE_INTERFACE.as_ptr()).library_type_exists(library_type)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_library_library_exists(library_handle: LibraryHandle) -> Bool {
    (*BASE_INTERFACE.as_ptr()).library_library_exists(library_handle)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_library_unsafe_create_library_handle() -> LibraryHandle {
    (*BASE_INTERFACE.as_ptr()).library_unsafe_create_library_handle()
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_library_unsafe_remove_library_handle(
    library_handle: LibraryHandle,
) -> Optional<LibraryError> {
    (*BASE_INTERFACE.as_ptr()).library_unsafe_remove_library_handle(library_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_library_unsafe_link_library(
    library_handle: LibraryHandle,
    loader_handle: LoaderHandle,
    internal_handle: LoaderLibraryHandle,
) -> Optional<LibraryError> {
    (*BASE_INTERFACE.as_ptr()).library_unsafe_link_library(
        library_handle,
        loader_handle,
        internal_handle,
    )
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_library_unsafe_get_loader_library_handle(
    library_handle: LibraryHandle,
) -> Result<LoaderLibraryHandle, LibraryError> {
    (*BASE_INTERFACE.as_ptr()).library_unsafe_get_loader_library_handle(library_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_library_unsafe_get_loader_handle(
    library_handle: LibraryHandle,
) -> Result<LoaderHandle, LibraryError> {
    (*BASE_INTERFACE.as_ptr()).library_unsafe_get_loader_handle(library_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_library_unsafe_get_loader_interface(
    loader_handle: LoaderHandle,
) -> Result<NonNullConst<LoaderInterface>, LibraryError> {
    (*BASE_INTERFACE.as_ptr()).library_unsafe_get_loader_interface(loader_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_library_load(
    loader_handle: LoaderHandle,
    library_path: NonNullConst<OsPathChar>,
) -> Result<LibraryHandle, LibraryError> {
    (*BASE_INTERFACE.as_ptr()).library_load(loader_handle, library_path)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_library_unload(
    library_handle: LibraryHandle,
) -> Optional<LibraryError> {
    (*BASE_INTERFACE.as_ptr()).library_unload(library_handle)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_library_get_data_symbol(
    library_handle: LibraryHandle,
    symbol_name: NonNullConst<c_char>,
) -> Result<DataSymbol, LibraryError> {
    (*BASE_INTERFACE.as_ptr()).library_get_data_symbol(library_handle, symbol_name)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_library_get_function_symbol(
    library_handle: LibraryHandle,
    symbol_name: NonNullConst<c_char>,
) -> Result<FnSymbol, LibraryError> {
    (*BASE_INTERFACE.as_ptr()).library_get_function_symbol(library_handle, symbol_name)
}
