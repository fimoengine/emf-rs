use crate::bindings::BASE_INTERFACE;
use crate::containers::{NonNullConst, Result};
use crate::library::{LibraryError, LibraryType, LoaderHandle, LoaderInterface};

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
pub unsafe extern "C" fn emf_cbase_library_register_loader(
    loader_interface: NonNullConst<LoaderInterface>,
    library_type: NonNullConst<LibraryType>,
) -> Result<LoaderHandle, LibraryError> {
    let interface_ptr = BASE_INTERFACE.as_ptr();
    ((*interface_ptr).library_register_loader_fn)(
        (*interface_ptr).cbase_module,
        loader_interface,
        library_type,
    )
}
