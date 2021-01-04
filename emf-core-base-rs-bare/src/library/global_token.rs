use crate::ffi::containers::{MutSpan, NonNullConst};
use crate::library::loader_library_handle::LoaderLibraryHandle;
use crate::library::{
    os_str_to_native_buff, LibraryError, LibraryHandle, LibraryLoaderHandle, LibraryLoaderWrapper,
    LibrarySymbol, LibraryToken, LibraryType,
};
use crate::GlobalToken;
use crate::{ffi, FFIObject};
use std::ffi::CStr;
use std::path::Path;
use std::ptr::NonNull;

impl<'a> LibraryToken<'a> for GlobalToken<'a> {
    #[inline]
    fn register_loader<T: LibraryLoaderWrapper<'static>>(
        &self,
        loader: &T,
        lib_type: &LibraryType,
    ) -> Result<LibraryLoaderHandle<'static>, LibraryError> {
        unsafe {
            ffi::library::emf_cbase_library_register_loader(
                NonNullConst::from(loader.as_native()),
                NonNullConst::from(lib_type),
            )
            .to_native()
            .map(|l| LibraryLoaderHandle::from_native(l))
        }
    }

    #[inline]
    #[must_use]
    fn unregister_loader(&self, loader: LibraryLoaderHandle) -> Option<LibraryError> {
        unsafe { ffi::library::emf_cbase_library_unregister_loader(loader.as_native()).to_native() }
    }

    #[inline]
    #[must_use]
    fn get_num_loaders(&self) -> usize {
        unsafe { ffi::library::emf_cbase_library_get_num_loaders() }
    }

    #[inline]
    fn get_library_types<T: AsMut<[LibraryType]>>(
        &self,
        buf: &mut T,
    ) -> Result<usize, LibraryError> {
        unsafe {
            ffi::library::emf_cbase_library_get_library_types(NonNull::from(&MutSpan::from(
                buf.as_mut(),
            )))
            .to_native()
        }
    }

    #[inline]
    fn get_loader_handle(
        &self,
        lib_type: &LibraryType,
    ) -> Result<LibraryLoaderHandle<'a>, LibraryError> {
        unsafe {
            ffi::library::emf_cbase_library_get_loader_handle(NonNullConst::from(lib_type))
                .to_native()
                .map(|h| LibraryLoaderHandle::from_native(h))
        }
    }

    #[inline]
    #[must_use]
    fn library_type_exists(&self, lib_type: &LibraryType) -> bool {
        unsafe { ffi::library::emf_cbase_library_type_exists(NonNullConst::from(lib_type)).into() }
    }

    #[inline]
    #[must_use]
    fn library_exists(&self, library: &LibraryHandle) -> bool {
        unsafe { ffi::library::emf_cbase_library_library_exists(library.as_native()).into() }
    }

    #[inline]
    #[must_use]
    unsafe fn create_library_handle<'b>(&self) -> LibraryHandle<'b> {
        LibraryHandle::from_native(ffi::library::emf_cbase_library_unsafe_create_library_handle())
    }

    #[inline]
    #[must_use]
    unsafe fn remove_library_handle(&self, library: LibraryHandle) -> Option<LibraryError> {
        ffi::library::emf_cbase_library_unsafe_remove_library_handle(library.as_native())
            .to_native()
    }

    #[inline]
    #[must_use]
    unsafe fn link_library<'b, 'c: 'd, 'd: 'b, T: LibraryLoaderWrapper<'d>>(
        &self,
        library: &LibraryHandle,
        loader: &'c LibraryLoaderHandle<'c>,
        internal_handle: &'b LoaderLibraryHandle<'b, 'd, T>,
    ) -> Option<LibraryError> {
        ffi::library::emf_cbase_library_unsafe_link_library(
            library.as_native(),
            loader.as_native(),
            internal_handle.as_native(),
        )
        .to_native()
    }

    #[inline]
    unsafe fn get_loader_library_handle<'b, T: LibraryLoaderWrapper<'a>>(
        &self,
        library: &'b LibraryHandle,
    ) -> Result<LoaderLibraryHandle<'b, 'a, T>, LibraryError> {
        ffi::library::emf_cbase_library_unsafe_get_loader_library_handle(library.as_native())
            .to_native()
            .map(|h| LoaderLibraryHandle::from_native(h))
    }

    #[inline]
    unsafe fn get_loader_handle_from_lib(
        &self,
        library: &LibraryHandle,
    ) -> Result<LibraryLoaderHandle<'a>, LibraryError> {
        ffi::library::emf_cbase_library_unsafe_get_loader_handle(library.as_native())
            .to_native()
            .map(|h| LibraryLoaderHandle::from_native(h))
    }

    #[inline]
    unsafe fn get_loader_interface<T: LibraryLoaderWrapper<'a>>(
        &self,
        loader: &LibraryLoaderHandle,
    ) -> Result<T, LibraryError> {
        ffi::library::emf_cbase_library_unsafe_get_loader_interface(loader.as_native())
            .to_native()
            .map(|l| T::from_native(&*l.as_ptr()))
    }

    #[inline]
    fn load<'c, 'b: 'c, T: AsRef<Path>>(
        &self,
        loader: &'b LibraryLoaderHandle<'b>,
        path: &T,
    ) -> Result<LibraryHandle<'c>, LibraryError> {
        unsafe {
            let path = path.as_ref().as_os_str();
            let native_path_buff = os_str_to_native_buff(path);
            ffi::library::emf_cbase_library_load(
                loader.as_native(),
                NonNullConst::new_unchecked(native_path_buff.as_ptr()),
            )
            .to_native()
            .map(|h| LibraryHandle::from_native(h))
        }
    }

    #[inline]
    #[must_use]
    fn unload(&self, library: LibraryHandle) -> Option<LibraryError> {
        unsafe { ffi::library::emf_cbase_library_unload(library.as_native()).to_native() }
    }

    #[inline]
    fn get_data_symbol<'b, T: 'b + Sized + FFIObject<ffi::library::DataSymbol>, S: AsRef<CStr>>(
        &self,
        library: &'b LibraryHandle<'b>,
        name: &S,
    ) -> Result<LibrarySymbol<'b, T>, LibraryError> {
        unsafe {
            ffi::library::emf_cbase_library_get_data_symbol(
                library.as_native(),
                NonNullConst::new_unchecked(name.as_ref().as_ptr()),
            )
            .to_native()
            .map(|s| LibrarySymbol::from_native(s))
        }
    }

    #[inline]
    fn get_function_symbol<
        'b,
        T: 'b + Sized + FFIObject<ffi::library::FnSymbol>,
        S: AsRef<CStr>,
    >(
        &self,
        library: &'b LibraryHandle<'b>,
        name: &S,
    ) -> Result<LibrarySymbol<'b, T>, LibraryError> {
        unsafe {
            ffi::library::emf_cbase_library_get_function_symbol(
                library.as_native(),
                NonNullConst::new_unchecked(name.as_ref().as_ptr()),
            )
            .to_native()
            .map(|s| LibrarySymbol::from_native(s))
        }
    }
}
