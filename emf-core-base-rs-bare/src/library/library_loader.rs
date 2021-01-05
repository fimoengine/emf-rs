use super::os_str_to_native_buff;
use crate::ffi::containers::NonNullConst;
use crate::ffi::library::{LibraryLoaderInterfaceBinding, NativeLibraryLoaderInterfaceBinding};
use crate::ffi::library::{LoaderInterface, NativeLoaderInterface};
use crate::library::{LibraryError, LibrarySymbol, LoaderLibraryHandle, LoaderLibraryHandleRef};
use crate::{ffi, FFIObject, FromFFI};
#[cfg(windows)]
use std::ffi::c_void;
use std::ffi::CStr;
#[cfg(unix)]
use std::os::raw::c_int;
use std::path::Path;

/// A trait describing the functionality of a `LibraryLoader`.
pub trait LibraryLoaderWrapper<'a>:
    AsRef<&'a ffi::library::LoaderInterface> + FromFFI<&'a ffi::library::LoaderInterface> + Sized
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
}

impl<'a> AsRef<&'a ffi::library::LoaderInterface> for LibraryLoader<'a> {
    fn as_ref(&self) -> &&'a LoaderInterface {
        &self.interface
    }
}

impl<'a> FromFFI<&'a ffi::library::LoaderInterface> for LibraryLoader<'a> {
    unsafe fn from_ffi(v: &'a ffi::library::LoaderInterface) -> Self {
        Self { interface: v }
    }
}

/// Functionalities of the `NativeLibraryLoader`
pub trait NativeLibraryLoaderWrapper<'a>:
    AsRef<&'a ffi::library::NativeLoaderInterface>
    + FromFFI<&'a ffi::library::NativeLoaderInterface>
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
        LibraryLoader::<'a>::from_native(&self.interface.library_loader_interface)
            .load(path)
            .map(|lib| lib.cast())
    }

    #[inline]
    unsafe fn unload<'b>(
        &self,
        library: LoaderLibraryHandle<'b, 'a, Self>,
    ) -> Option<LibraryError> {
        LibraryLoader::<'a>::from_native(&self.interface.library_loader_interface)
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
        LibraryLoader::<'a>::from_native(&self.interface.library_loader_interface).get_data_symbol(
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
        LibraryLoader::<'a>::from_native(&self.interface.library_loader_interface)
            .get_function_symbol(
                std::mem::transmute::<_, &'b _>(&library.cast_ref::<LibraryLoader>()),
                name,
            )
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
        unsafe { LibraryLoader::from_native(loader.as_native()) }
    }
}

impl<'a> AsRef<&'a ffi::library::LoaderInterface> for NativeLibraryLoader<'a> {
    fn as_ref(&self) -> &&'a LoaderInterface {
        unsafe {
            &*(&self.interface as *const &ffi::library::NativeLoaderInterface
                as *const &ffi::library::LoaderInterface)
        }
    }
}

impl<'a> FromFFI<&'a ffi::library::LoaderInterface> for NativeLibraryLoader<'a> {
    unsafe fn from_ffi(v: &'a ffi::library::LoaderInterface) -> Self {
        Self {
            interface: &*(v as *const ffi::library::LoaderInterface
                as *const ffi::library::NativeLoaderInterface),
        }
    }
}

impl<'a> AsRef<&'a ffi::library::NativeLoaderInterface> for NativeLibraryLoader<'a> {
    fn as_ref(&self) -> &&'a NativeLoaderInterface {
        &self.interface
    }
}

impl<'a> FromFFI<&'a ffi::library::NativeLoaderInterface> for NativeLibraryLoader<'a> {
    unsafe fn from_ffi(v: &'a NativeLoaderInterface) -> Self {
        Self { interface: v }
    }
}
