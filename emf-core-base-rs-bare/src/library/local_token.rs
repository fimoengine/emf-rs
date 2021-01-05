use crate::ffi::containers::{MutSpan, NonNullConst};
use crate::library::{
    os_str_to_native_buff, LibraryError, LibraryHandle, LibraryHandleRef, LibraryLoaderHandle,
    LibraryLoaderHandleRef, LibraryLoaderWrapper, LibrarySymbol, LibraryToken, LibraryType,
    LoaderLibraryHandle, LoaderLibraryHandleRef,
};
use crate::{ffi, FFIObject, LocalToken};
use std::ffi::CStr;
use std::path::Path;
use std::ptr::NonNull;

impl<'a, T: Sized + ffi::InterfaceBinding> LibraryToken<'a> for LocalToken<'a, T> {
    #[inline]
    fn register_loader<U: LibraryLoaderWrapper<'static>>(
        &self,
        loader: &U,
        lib_type: &LibraryType,
    ) -> Result<LibraryLoaderHandle<'static>, LibraryError> {
        unsafe {
            self.interface()
                .library_register_loader(
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
        unsafe {
            self.interface()
                .library_unregister_loader(loader.as_native())
                .to_native()
        }
    }

    #[inline]
    #[must_use]
    fn get_num_loaders(&self) -> usize {
        unsafe { self.interface().library_get_num_loaders() }
    }

    #[inline]
    fn get_library_types<U: AsMut<[LibraryType]>>(
        &self,
        buf: &mut U,
    ) -> Result<usize, LibraryError> {
        unsafe {
            self.interface()
                .library_get_library_types(NonNull::from(&MutSpan::from(buf.as_mut())))
                .to_native()
        }
    }

    #[inline]
    fn get_loader_handle(
        &self,
        lib_type: &LibraryType,
    ) -> Result<LibraryLoaderHandleRef<'a>, LibraryError> {
        unsafe {
            self.interface()
                .library_get_loader_handle(NonNullConst::from(lib_type))
                .to_native()
                .map(|h| LibraryLoaderHandleRef::from_native(h))
        }
    }

    #[inline]
    #[must_use]
    fn library_type_exists(&self, lib_type: &LibraryType) -> bool {
        unsafe {
            self.interface()
                .library_type_exists(NonNullConst::from(lib_type))
                .into()
        }
    }

    #[inline]
    #[must_use]
    fn library_exists(&self, library: &LibraryHandleRef) -> bool {
        unsafe {
            self.interface()
                .library_library_exists(library.as_native())
                .into()
        }
    }

    #[inline]
    #[must_use]
    unsafe fn create_library_handle<'b>(&self) -> LibraryHandle<'b> {
        LibraryHandle::from_native(self.interface().library_unsafe_create_library_handle())
    }

    #[inline]
    #[must_use]
    unsafe fn remove_library_handle(&self, library: LibraryHandle) -> Option<LibraryError> {
        self.interface()
            .library_unsafe_remove_library_handle(library.as_native())
            .to_native()
    }

    #[inline]
    #[must_use]
    unsafe fn link_library<'b, 'c: 'd, 'd: 'b, U: LibraryLoaderWrapper<'d>>(
        &self,
        library: &LibraryHandle,
        loader: &'c LibraryLoaderHandleRef<'c>,
        internal_handle: &'b LoaderLibraryHandle<'b, 'd, U>,
    ) -> Option<LibraryError> {
        self.interface()
            .library_unsafe_link_library(
                library.as_native(),
                loader.as_native(),
                internal_handle.as_native(),
            )
            .to_native()
    }

    #[inline]
    unsafe fn get_loader_library_handle<'b, U: LibraryLoaderWrapper<'a>>(
        &self,
        library: &'b LibraryHandleRef,
    ) -> Result<LoaderLibraryHandleRef<'b, 'a, U>, LibraryError> {
        self.interface()
            .library_unsafe_get_loader_library_handle(library.as_native())
            .to_native()
            .map(|h| LoaderLibraryHandleRef::from_native(h))
    }

    #[inline]
    unsafe fn get_loader_handle_from_lib(
        &self,
        library: &LibraryHandleRef,
    ) -> Result<LibraryLoaderHandleRef<'a>, LibraryError> {
        self.interface()
            .library_unsafe_get_loader_handle(library.as_native())
            .to_native()
            .map(|h| LibraryLoaderHandleRef::from_native(h))
    }

    #[inline]
    unsafe fn get_loader_interface<U: LibraryLoaderWrapper<'a>>(
        &self,
        loader: &LibraryLoaderHandleRef,
    ) -> Result<U, LibraryError> {
        self.interface()
            .library_unsafe_get_loader_interface(loader.as_native())
            .to_native()
            .map(|l| U::from_native(&*l.as_ptr()))
    }

    #[inline]
    fn load<'c, 'b: 'c, U: AsRef<Path>>(
        &self,
        loader: &'b LibraryLoaderHandleRef<'b>,
        path: &U,
    ) -> Result<LibraryHandle<'c>, LibraryError> {
        unsafe {
            let path = path.as_ref().as_os_str();
            let native_path_buff = os_str_to_native_buff(path);
            self.interface()
                .library_load(
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
        unsafe {
            self.interface()
                .library_unload(library.as_native())
                .to_native()
        }
    }

    #[inline]
    fn get_data_symbol<'b, U: 'b + Sized + FFIObject<ffi::library::DataSymbol>, S: AsRef<CStr>>(
        &self,
        library: &'b LibraryHandleRef<'b>,
        name: &S,
    ) -> Result<LibrarySymbol<'b, U>, LibraryError> {
        unsafe {
            self.interface()
                .library_get_data_symbol(
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
        U: 'b + Sized + FFIObject<ffi::library::FnSymbol>,
        S: AsRef<CStr>,
    >(
        &self,
        library: &'b LibraryHandleRef<'b>,
        name: &S,
    ) -> Result<LibrarySymbol<'b, U>, LibraryError> {
        unsafe {
            self.interface()
                .library_get_function_symbol(
                    library.as_native(),
                    NonNullConst::new_unchecked(name.as_ref().as_ptr()),
                )
                .to_native()
                .map(|s| LibrarySymbol::from_native(s))
        }
    }
}
