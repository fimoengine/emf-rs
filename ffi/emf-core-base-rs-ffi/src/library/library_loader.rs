//! Interface of a library loader.
//!
//! Any object that can be wrapped into a [LibraryLoaderInterface] can be used as a library loader.
use crate::collections::{NonNullConst, Result};
use crate::errors::Error;
use crate::library::{InternalHandle, OSPathString, Symbol, SymbolName};
use crate::{CBaseFn, TypeWrapper};
use std::ffi::c_void;
#[cfg(windows)]
use std::os::windows::raw::HANDLE;
use std::ptr::NonNull;

/// Opaque structure representing a loader.
#[repr(C)]
pub struct LibraryLoader {
    _dummy: [u8; 0],
}

pub type LoadFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<LibraryLoader>>,
        path: OSPathString,
    ) -> Result<InternalHandle, Error>,
>;

pub type UnloadFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<LibraryLoader>>,
        handle: InternalHandle,
    ) -> Result<i8, Error>,
>;

pub type GetDataSymbolFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<LibraryLoader>>,
        handle: InternalHandle,
        name: SymbolName,
    ) -> Result<Symbol<NonNullConst<c_void>>, Error>,
>;

pub type GetFnSymbolFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<LibraryLoader>>,
        handle: InternalHandle,
        name: SymbolName,
    ) -> Result<Symbol<CBaseFn>, Error>,
>;

pub type GetExtendedVTableFn = TypeWrapper<
    unsafe extern "C-unwind" fn(loader: Option<NonNull<LibraryLoader>>) -> NonNullConst<c_void>,
>;

/// VTable of a library loader.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LibraryLoaderVTable {
    pub load_fn: LoadFn,
    pub unload_fn: UnloadFn,
    pub get_data_symbol_fn: GetDataSymbolFn,
    pub get_function_symbol_fn: GetFnSymbolFn,
    pub get_extended_vtable_fn: GetExtendedVTableFn,
}

/// Interface of a library loader.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LibraryLoaderInterface {
    pub loader: Option<NonNull<LibraryLoader>>,
    pub vtable: NonNullConst<LibraryLoaderVTable>,
}

unsafe impl Send for LibraryLoaderInterface {}
unsafe impl Sync for LibraryLoaderInterface {}

/// Helper trait for using a library loader.
pub trait LibraryLoaderBinding {
    /// Loads a library. The resulting handle is unique.
    ///
    /// # Failure
    ///
    /// The function fails if `loader` or `path` is invalid or
    /// the type of the library can not be loaded with the loader.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoaderBinding] may break some invariants
    /// of the library api, if not handled with care.
    unsafe fn load(&mut self, path: OSPathString) -> Result<InternalHandle, Error>;

    /// Unloads a library.
    ///
    /// # Failure
    ///
    /// The function fails if `handle` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoaderBinding] may break some invariants
    /// of the library api, if not handled with care.
    unsafe fn unload(&mut self, handle: InternalHandle) -> Result<i8, Error>;

    /// Fetches a data symbol from a library.
    ///
    /// # Failure
    ///
    /// The function fails if `handle` is invalid or library does not contain `name`.
    ///
    /// # Note
    ///
    /// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
    /// See [LibraryLoaderBinding::get_function_symbol()] for fetching a function.
    ///
    /// # Return
    ///
    /// Symbol on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoaderBinding] may break some invariants
    /// of the library api, if not handled with care.
    unsafe fn get_data_symbol(
        &self,
        handle: InternalHandle,
        name: SymbolName,
    ) -> Result<Symbol<NonNullConst<c_void>>, Error>;

    /// Fetches a function symbol from a library.
    ///
    /// # Failure
    ///
    /// The function fails if `handle` is invalid or library does not contain `name`.
    ///
    /// # Note
    ///
    /// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
    /// See [LibraryLoaderBinding::get_data_symbol()] for fetching some data.
    ///
    /// # Return
    ///
    /// Symbol on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoaderBinding] may break some invariants
    /// of the library api, if not handled with care.
    unsafe fn get_function_symbol(
        &self,
        handle: InternalHandle,
        name: SymbolName,
    ) -> Result<Symbol<CBaseFn>, Error>;

    /// Fetches a pointer to the extended loader vtable.
    ///
    /// # Return
    ///
    /// Pointer to the vtable.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoaderBinding] may break some invariants
    /// of the library api, if not handled with care.
    unsafe fn get_extended_vtable(&self) -> NonNullConst<c_void>;
}

impl LibraryLoaderBinding for LibraryLoaderInterface {
    #[inline]
    unsafe fn load(&mut self, path: OSPathString) -> Result<InternalHandle, Error> {
        (self.vtable.as_ref().load_fn)(self.loader, path)
    }

    #[inline]
    unsafe fn unload(&mut self, handle: InternalHandle) -> Result<i8, Error> {
        (self.vtable.as_ref().unload_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_data_symbol(
        &self,
        handle: InternalHandle,
        name: SymbolName,
    ) -> Result<Symbol<NonNullConst<c_void>>, Error> {
        (self.vtable.as_ref().get_data_symbol_fn)(self.loader, handle, name)
    }

    #[inline]
    unsafe fn get_function_symbol(
        &self,
        handle: InternalHandle,
        name: SymbolName,
    ) -> Result<Symbol<CBaseFn>, Error> {
        (self.vtable.as_ref().get_function_symbol_fn)(self.loader, handle, name)
    }

    #[inline]
    unsafe fn get_extended_vtable(&self) -> NonNullConst<c_void> {
        (self.vtable.as_ref().get_extended_vtable_fn)(self.loader)
    }
}

/// Type used by unix to identify a library.
#[cfg(unix)]
pub type NativeLibraryHandleUnix = *mut c_void;

/// Type used by windows to identify a library.
#[cfg(windows)]
pub type NativeLibraryHandleWindows = HANDLE;

/// Type used by the os to identify a library.
#[cfg(unix)]
pub type NativeLibraryHandle = NativeLibraryHandleUnix;

/// Type used by the os to identify a library.
#[cfg(windows)]
pub type NativeLibraryHandle = NativeLibraryHandleWindows;

#[cfg(unix)]
pub type LoadExtFnUnix = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<LibraryLoader>>,
        path: OSPathString,
        flags: i32,
    ) -> Result<InternalHandle, Error>,
>;

#[cfg(windows)]
pub type LoadExtFnWindows = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<LibraryLoader>>,
        path: OSPathString,
        h_file: Option<NonNull<HANDLE>>,
        flags: u32,
    ) -> Result<InternalHandle, Error>,
>;

pub type GetNativeHandleFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        loader: Option<NonNull<LibraryLoader>>,
        handle: InternalHandle,
    ) -> Result<NativeLibraryHandle, Error>,
>;

#[cfg(unix)]
pub type LoadExtFn = LoadExtFnUnix;

#[cfg(windows)]
pub type LoadExtFn = LoadExtFnWindows;

/// VTable of a native library loader.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NativeLibraryLoaderVTable {
    pub loader_vtable: NonNullConst<LibraryLoaderVTable>,
    pub load_ext_fn: LoadExtFn,
    pub get_native_handle_fn: GetNativeHandleFn,
}

/// Interface of a native library loader.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct NativeLibraryLoaderInterface {
    pub loader: Option<NonNull<LibraryLoader>>,
    pub vtable: NonNullConst<NativeLibraryLoaderVTable>,
}

unsafe impl Send for NativeLibraryLoaderInterface {}
unsafe impl Sync for NativeLibraryLoaderInterface {}

#[cfg(unix)]
pub trait NativeLibraryLoaderBindingUnix: LibraryLoaderBinding {
    /// Loads a library. The resulting handle is unique.
    ///
    /// The argument `flags` is passed to `dlopen`.
    ///
    /// # Failure
    ///
    /// The function fails if `loader` or `path` is invalid or
    /// the call to `dlopen` fails.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoaderBinding] may break some invariants
    /// of the library api, if not handled with care.
    unsafe fn load_ext(&mut self, path: OSPathString, flags: i32) -> Result<InternalHandle, Error>;

    /// Returns the underlying handle of a library.
    ///
    /// # Failure
    ///
    /// The function fails if `handle` is invalid.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoaderBinding] may break some invariants
    /// of the library api, if not handled with care.
    unsafe fn get_native_handle(
        &self,
        handle: InternalHandle,
    ) -> Result<NativeLibraryHandle, Error>;
}

#[cfg(windows)]
pub trait NativeLibraryLoaderBindingWindows: LibraryLoaderBinding {
    /// Loads a library. The resulting handle is unique.
    ///
    /// The arguments `h_file` and `flags` are passed to `LoadLibraryExW`.
    ///
    /// # Failure
    ///
    /// The function fails if `loader` or `path` is invalid or
    /// the call to `LoadLibraryExW` fails.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoaderBinding] may break some invariants
    /// of the library api, if not handled with care.
    unsafe fn load_ext(
        &mut self,
        path: OSPathString,
        h_file: Option<NonNull<HANDLE>>,
        flags: u32,
    ) -> Result<InternalHandle, Error>;

    /// Returns the underlying handle of a library.
    ///
    /// # Failure
    ///
    /// The function fails if `handle` is invalid.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoaderBinding] may break some invariants
    /// of the library api, if not handled with care.
    unsafe fn get_native_handle(
        &self,
        handle: InternalHandle,
    ) -> Result<NativeLibraryHandle, Error>;
}

impl LibraryLoaderBinding for NativeLibraryLoaderInterface {
    #[inline]
    unsafe fn load(&mut self, path: OSPathString) -> Result<InternalHandle, Error> {
        (self.vtable.as_ref().loader_vtable.as_ref().load_fn)(self.loader, path)
    }

    #[inline]
    unsafe fn unload(&mut self, handle: InternalHandle) -> Result<i8, Error> {
        (self.vtable.as_ref().loader_vtable.as_ref().unload_fn)(self.loader, handle)
    }

    #[inline]
    unsafe fn get_data_symbol(
        &self,
        handle: InternalHandle,
        name: SymbolName,
    ) -> Result<Symbol<NonNullConst<c_void>>, Error> {
        (self
            .vtable
            .as_ref()
            .loader_vtable
            .as_ref()
            .get_data_symbol_fn)(self.loader, handle, name)
    }

    #[inline]
    unsafe fn get_function_symbol(
        &self,
        handle: InternalHandle,
        name: SymbolName,
    ) -> Result<Symbol<CBaseFn>, Error> {
        (self
            .vtable
            .as_ref()
            .loader_vtable
            .as_ref()
            .get_function_symbol_fn)(self.loader, handle, name)
    }

    #[inline]
    unsafe fn get_extended_vtable(&self) -> NonNullConst<c_void> {
        (self
            .vtable
            .as_ref()
            .loader_vtable
            .as_ref()
            .get_extended_vtable_fn)(self.loader)
    }
}

/// Helper trait for using a native library loader.
#[cfg(unix)]
impl NativeLibraryLoaderBindingUnix for NativeLibraryLoaderInterface {
    #[inline]
    unsafe fn load_ext(&mut self, path: OSPathString, flags: i32) -> Result<InternalHandle, Error> {
        (self.vtable.as_ref().load_ext_fn)(self.loader, path, flags)
    }

    #[inline]
    unsafe fn get_native_handle(
        &self,
        handle: InternalHandle,
    ) -> Result<NativeLibraryHandle, Error> {
        (self.vtable.as_ref().get_native_handle_fn)(self.loader, handle)
    }
}

/// Helper trait for using a native library loader.
#[cfg(windows)]
impl NativeLibraryLoaderBindingWindows for NativeLibraryLoaderInterface {
    #[inline]
    unsafe fn load_ext(
        &mut self,
        path: OSPathString,
        h_file: Option<NonNull<HANDLE>>,
        flags: u32,
    ) -> Result<InternalHandle, Error> {
        (self.vtable.as_ref().load_ext_fn)(self.loader, path, h_file, flags)
    }

    #[inline]
    unsafe fn get_native_handle(
        &self,
        handle: InternalHandle,
    ) -> Result<NativeLibraryHandle, Error> {
        (self.vtable.as_ref().get_native_handle_fn)(self.loader, handle)
    }
}
