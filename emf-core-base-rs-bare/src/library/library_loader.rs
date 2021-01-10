use super::os_str_to_native_buff;
use crate::ffi::containers::NonNullConst;
use crate::ffi::library::{LibraryLoaderInterfaceBinding, NativeLibraryLoaderInterfaceBinding};
use crate::library::{LibraryError, LibrarySymbol, LoaderLibraryHandle, LoaderLibraryHandleRef};
use crate::{ffi, FFIObject};
use std::ffi::{c_void, CStr};
#[cfg(unix)]
use std::os::raw::c_int;
use std::path::Path;

/// A trait describing the functionality of a `LibraryLoader`.
pub trait LibraryLoaderWrapper<'a>:
    AsRef<ffi::library::LoaderInterface> + FFIObject<&'a ffi::library::LoaderInterface> + Sized
{
    /// Loads a library from a path.
    ///
    /// # Safety
    ///
    /// Direct usage of a `LibraryLoader` circumvents the safety of the `library` api.
    unsafe fn load<'b, T: AsRef<Path>>(
        &self,
        path: &T,
    ) -> Result<LoaderLibraryHandle<'b, 'a, Self>, LibraryError>;

    /// Unloads a library.
    ///
    /// # Safety
    ///
    /// Direct usage of a `LibraryLoader` circumvents the safety of the `library` api.
    unsafe fn unload<'b>(&self, library: LoaderLibraryHandle<'b, 'a, Self>)
        -> Option<LibraryError>;

    /// Fetches a data symbol from a library.
    ///
    /// # Safety
    ///
    /// Direct usage of a `LibraryLoader` circumvents the safety of the `library` api.
    unsafe fn get_data_symbol<'b, U: Sized + FFIObject<ffi::library::DataSymbol>, S: AsRef<CStr>>(
        &self,
        library: &'b LoaderLibraryHandleRef<'b, 'a, Self>,
        name: &S,
    ) -> Result<LibrarySymbol<'b, U>, LibraryError>;

    /// Fetches a function symbol from a library.
    ///
    /// # Safety
    ///
    /// Direct usage of a `LibraryLoader` circumvents the safety of the `library` api.
    unsafe fn get_function_symbol<
        'b,
        U: Sized + FFIObject<ffi::library::FnSymbol>,
        S: AsRef<CStr>,
    >(
        &self,
        library: &'b LoaderLibraryHandleRef<'b, 'a, Self>,
        name: &S,
    ) -> Result<LibrarySymbol<'b, U>, LibraryError>;

    /// Fetches the internal interface of the loader.
    ///
    /// # Safety
    ///
    /// Direct usage of a `LibraryLoader` circumvents the safety of the `library` api.
    unsafe fn get_internal_interface<U: 'a + Sized + FFIObject<NonNullConst<c_void>>>(&self) -> U;
}

/// A `LibraryLoader`.
#[derive(Debug)]
pub struct LibraryLoader<'a> {
    interface: &'a ffi::library::LoaderInterface,
}

impl<'a> LibraryLoader<'a> {
    /// Extends the lifetime of the `LibraryLoader`.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the `LibraryLoader` lives long enough.
    #[inline]
    pub unsafe fn extend_lifetime<'b>(self) -> LibraryLoader<'b> {
        std::mem::transmute(self)
    }
}

impl<'a> LibraryLoaderWrapper<'a> for LibraryLoader<'a> {
    #[inline]
    unsafe fn load<'b, T: AsRef<Path>>(
        &self,
        path: &T,
    ) -> Result<LoaderLibraryHandle<'b, 'a, Self>, LibraryError> {
        let path = path.as_ref().as_os_str();
        let native_path_buff = os_str_to_native_buff(path);
        self.interface
            .load(NonNullConst::new_unchecked(native_path_buff.as_ptr()))
            .to_native()
            .map(|h| LoaderLibraryHandle::from_native(h))
    }

    #[inline]
    unsafe fn unload<'b>(
        &self,
        library: LoaderLibraryHandle<'b, 'a, Self>,
    ) -> Option<LibraryError> {
        self.interface.unload(library.as_native()).to_native()
    }

    #[inline]
    unsafe fn get_data_symbol<
        'b,
        U: Sized + FFIObject<ffi::library::DataSymbol>,
        S: AsRef<CStr>,
    >(
        &self,
        library: &'b LoaderLibraryHandleRef<'b, 'a, Self>,
        name: &S,
    ) -> Result<LibrarySymbol<'b, U>, LibraryError> {
        self.interface
            .get_data_symbol(
                library.as_native(),
                NonNullConst::new_unchecked(name.as_ref().as_ptr()),
            )
            .to_native()
            .map(|sym| LibrarySymbol::from_native(sym))
    }

    #[inline]
    unsafe fn get_function_symbol<
        'b,
        U: Sized + FFIObject<ffi::library::FnSymbol>,
        S: AsRef<CStr>,
    >(
        &self,
        library: &'b LoaderLibraryHandleRef<'b, 'a, Self>,
        name: &S,
    ) -> Result<LibrarySymbol<'b, U>, LibraryError> {
        self.interface
            .get_function_symbol(
                library.as_native(),
                NonNullConst::new_unchecked(name.as_ref().as_ptr()),
            )
            .to_native()
            .map(|sym| LibrarySymbol::from_native(sym))
    }

    #[inline]
    #[must_use]
    unsafe fn get_internal_interface<U: 'a + Sized + FFIObject<NonNullConst<c_void>>>(&self) -> U {
        U::from_native(self.interface.get_internal_interface())
    }
}

impl<'a> AsRef<ffi::library::LoaderInterface> for LibraryLoader<'a> {
    fn as_ref(&self) -> &'a ffi::library::LoaderInterface {
        self.interface
    }
}

impl<'a> FFIObject<&'a ffi::library::LoaderInterface> for LibraryLoader<'a> {
    fn as_native(&self) -> &'a ffi::library::LoaderInterface {
        self.interface
    }

    unsafe fn from_native(val: &'a ffi::library::LoaderInterface) -> Self {
        Self { interface: val }
    }
}

/// Functionalities of the `NativeLibraryLoader`
pub trait NativeLibraryLoaderWrapper<'a>:
    AsRef<ffi::library::NativeLoaderInterface>
    + FFIObject<&'a ffi::library::NativeLoaderInterface>
    + LibraryLoaderWrapper<'a>
{
    /// Loads a library using the `NativeLibraryLoader`.
    ///
    /// # Safety
    ///
    /// This function is a low level api, elides lifetimes and is os specific.
    #[cfg(windows)]
    unsafe fn load_ext<'b, T: AsRef<Path>>(
        &self,
        path: &T,
        h_file: *mut c_void,
        flags: u32,
    ) -> Result<LoaderLibraryHandle<'b, 'a, Self>, LibraryError>;

    /// Loads a library using the `NativeLibraryLoader`.
    ///
    /// # Safety
    ///
    /// This function is a low level api, elides lifetimes and is os specific.
    #[cfg(unix)]
    unsafe fn load_ext<'b, T: AsRef<Path>>(
        &self,
        path: &T,
        flags: c_int,
    ) -> Result<LoaderLibraryHandle<'b, 'a, Self>, LibraryError>;
}

/// A `NativeLibraryLoader`.
#[derive(Debug)]
pub struct NativeLibraryLoader<'a> {
    interface: &'a ffi::library::NativeLoaderInterface,
}

impl<'a> LibraryLoaderWrapper<'a> for NativeLibraryLoader<'a> {
    #[inline]
    unsafe fn load<'b, T: AsRef<Path>>(
        &self,
        path: &T,
    ) -> Result<LoaderLibraryHandle<'b, 'a, Self>, LibraryError> {
        LibraryLoader::<'a>::from_native(self.interface.library_loader_interface.as_ref())
            .load(path)
            .map(|lib| lib.cast())
    }

    #[inline]
    unsafe fn unload<'b>(
        &self,
        library: LoaderLibraryHandle<'b, 'a, Self>,
    ) -> Option<LibraryError> {
        LibraryLoader::<'a>::from_native(self.interface.library_loader_interface.as_ref())
            .unload(library.cast())
    }

    #[inline]
    unsafe fn get_data_symbol<
        'b,
        U: Sized + FFIObject<ffi::library::DataSymbol>,
        S: AsRef<CStr>,
    >(
        &self,
        library: &'b LoaderLibraryHandleRef<'b, 'a, Self>,
        name: &S,
    ) -> Result<LibrarySymbol<'b, U>, LibraryError> {
        LibraryLoader::<'a>::from_native(self.interface.library_loader_interface.as_ref())
            .get_data_symbol(
                std::mem::transmute::<_, &'b _>(&library.cast_ref::<LibraryLoader>()),
                name,
            )
    }

    #[inline]
    unsafe fn get_function_symbol<
        'b,
        U: Sized + FFIObject<ffi::library::FnSymbol>,
        S: AsRef<CStr>,
    >(
        &self,
        library: &'b LoaderLibraryHandleRef<'b, 'a, Self>,
        name: &S,
    ) -> Result<LibrarySymbol<'b, U>, LibraryError> {
        LibraryLoader::<'a>::from_native(self.interface.library_loader_interface.as_ref())
            .get_function_symbol(
                std::mem::transmute::<_, &'b _>(&library.cast_ref::<LibraryLoader>()),
                name,
            )
    }

    #[inline]
    #[must_use]
    unsafe fn get_internal_interface<U: 'a + Sized + FFIObject<NonNullConst<c_void>>>(&self) -> U {
        LibraryLoader::<'a>::from_native(self.interface.library_loader_interface.as_ref())
            .get_internal_interface()
    }
}

impl<'a> NativeLibraryLoaderWrapper<'a> for NativeLibraryLoader<'a> {
    #[inline]
    #[cfg(windows)]
    unsafe fn load_ext<'b, T: AsRef<Path>>(
        &self,
        path: &T,
        h_file: *mut c_void,
        flags: u32,
    ) -> Result<LoaderLibraryHandle<'b, 'a, Self>, LibraryError> {
        let path = path.as_ref().as_os_str();
        let native_path_buff = os_str_to_native_buff(path);
        self.interface
            .load_ext(
                NonNullConst::new_unchecked(native_path_buff.as_ptr()),
                h_file,
                flags,
            )
            .to_native()
            .map(|h| LoaderLibraryHandle::from_native(h))
    }

    #[inline]
    #[cfg(unix)]
    unsafe fn load_ext<'b, T: AsRef<Path>>(
        &self,
        path: &T,
        flags: c_int,
    ) -> Result<LoaderLibraryHandle<'b, 'a, Self>, LibraryError> {
        let path = path.as_ref().as_os_str();
        let native_path_buff = os_str_to_native_buff(path);
        self.interface
            .load_ext(
                NonNullConst::new_unchecked(native_path_buff.as_ptr()),
                flags,
            )
            .to_native()
            .map(|h| LoaderLibraryHandle::from_native(h))
    }
}

impl<'a> From<LibraryLoader<'a>> for NativeLibraryLoader<'a> {
    fn from(loader: LibraryLoader<'a>) -> Self {
        unsafe { NativeLibraryLoader::from_native(loader.interface) }
    }
}

impl<'a> From<NativeLibraryLoader<'a>> for LibraryLoader<'a> {
    fn from(loader: NativeLibraryLoader<'a>) -> Self {
        unsafe {
            let interface: &ffi::library::LoaderInterface = loader.as_native();
            LibraryLoader::from_native(interface)
        }
    }
}

impl<'a> AsRef<ffi::library::LoaderInterface> for NativeLibraryLoader<'a> {
    fn as_ref(&self) -> &'a ffi::library::LoaderInterface {
        unsafe { &*self.interface.library_loader_interface.as_ptr() }
    }
}

impl<'a> AsRef<ffi::library::NativeLoaderInterface> for NativeLibraryLoader<'a> {
    fn as_ref(&self) -> &'a ffi::library::NativeLoaderInterface {
        self.interface
    }
}

impl<'a> FFIObject<&'a ffi::library::LoaderInterface> for NativeLibraryLoader<'a> {
    fn as_native(&self) -> &'a ffi::library::LoaderInterface {
        unsafe { &*self.interface.library_loader_interface.as_ptr() }
    }

    unsafe fn from_native(val: &'a ffi::library::LoaderInterface) -> Self {
        NativeLibraryLoader::from_native(val.get_internal_interface())
    }
}

impl<'a> FFIObject<&'a ffi::library::NativeLoaderInterface> for NativeLibraryLoader<'a> {
    fn as_native(&self) -> &'a ffi::library::NativeLoaderInterface {
        self.interface
    }

    unsafe fn from_native(val: &'a ffi::library::NativeLoaderInterface) -> Self {
        Self { interface: val }
    }
}

impl<'a> FFIObject<NonNullConst<c_void>> for NativeLibraryLoader<'a> {
    fn as_native(&self) -> NonNullConst<c_void> {
        NonNullConst::from(self.interface).cast()
    }

    unsafe fn from_native(val: NonNullConst<c_void>) -> Self {
        Self {
            interface: &*(val.as_ptr() as *const ffi::library::NativeLoaderInterface),
        }
    }
}
