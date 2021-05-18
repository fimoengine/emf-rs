//! API of a library loader.
use crate::ffi::collections::NonNullConst;
use crate::ffi::library::library_loader::{
    LibraryLoaderBinding, LibraryLoaderInterface, NativeLibraryHandle, NativeLibraryLoaderInterface,
};
use crate::ffi::CBaseFn;
use crate::library::{InternalLibrary, Symbol};
use crate::ownership::{
    AccessIdentifier, ImmutableAccessIdentifier, MutableAccessIdentifier, Owned,
};
use crate::Error;
use crate::ToOsPathBuff;
use std::borrow::Borrow;
use std::ffi::{c_void, CStr};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
#[cfg(windows)]
use std::os::windows::raw::HANDLE;
use std::path::Path;
#[cfg(windows)]
use std::ptr::NonNull;

/// Trait for identifying library loaders whose data structure is
/// compatible with the canonical library loader.
pub trait LibraryLoaderABICompat {}

/// The API of a library loader.
pub trait LibraryLoaderAPI<'a> {
    /// Type of the internal loader.
    type InternalLoader;

    /// Fetches a pointer that can be used with the interface.
    fn to_interface(&self) -> NonNullConst<LibraryLoaderInterface>;

    /// Construct a new instance from a pointer.
    ///
    /// # Safety
    ///
    /// This function should not be used directly.
    unsafe fn from_interface(handler: NonNullConst<LibraryLoaderInterface>) -> Self;

    /// Construct a new instance from a void pointer.
    ///
    /// # Safety
    ///
    /// This function should not be used directly.
    unsafe fn from_void_ptr(handler: NonNullConst<c_void>) -> Self;

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
    /// Direct usage of a [LibraryLoaderAPI] may break some invariants
    /// of the library api, if not handled with care.
    unsafe fn load(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<InternalLibrary<Owned>, Error<Owned>>;

    /// Unloads a library.
    ///
    /// # Failure
    ///
    /// The function fails if `internal` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoaderAPI] may break some invariants
    /// of the library api, if not handled with care.
    unsafe fn unload(&mut self, internal: InternalLibrary<Owned>) -> Result<(), Error<Owned>>;

    /// Fetches a data symbol from a library.
    ///
    /// # Failure
    ///
    /// The function fails if `internal` is invalid or library does not contain `symbol`.
    ///
    /// # Note
    ///
    /// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
    /// See [LibraryLoaderAPI::get_function_symbol()] for fetching a function.
    ///
    /// # Return
    ///
    /// Symbol on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoaderAPI] may break some invariants
    /// of the library api, if not handled with care.
    unsafe fn get_data_symbol<O, U>(
        &self,
        internal: &InternalLibrary<O>,
        symbol: impl AsRef<CStr>,
        caster: impl FnOnce(NonNullConst<c_void>) -> &'a U,
    ) -> Result<Symbol<'a, &'a U>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

    /// Fetches a function symbol from a library.
    ///
    /// # Failure
    ///
    /// The function fails if `internal` is invalid or library does not contain `symbol`.
    ///
    /// # Note
    ///
    /// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
    /// See [LibraryLoaderAPI::get_data_symbol()] for fetching some data.
    ///
    /// # Return
    ///
    /// Symbol on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoaderAPI] may break some invariants
    /// of the library api, if not handled with care.
    unsafe fn get_function_symbol<O, U>(
        &self,
        internal: &InternalLibrary<O>,
        symbol: impl AsRef<CStr>,
        caster: impl FnOnce(CBaseFn) -> U,
    ) -> Result<Symbol<'a, U>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

    /// Fetches a pointer to the internal interface.
    ///
    /// # Return
    ///
    /// Pointer to the interface.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoaderAPI] may break some invariants
    /// of the library api, if not handled with care.
    unsafe fn get_internal_interface(&self) -> Self::InternalLoader;
}

/// A library loader.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct LibraryLoader<T, O> {
    _loader: T,
    _ownership: PhantomData<*const O>,
}

impl<'a, T, O> Deref for LibraryLoader<T, O>
where
    T: LibraryLoaderAPI<'a>,
    O: AccessIdentifier,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self._loader
    }
}

impl<'a, T, O> DerefMut for LibraryLoader<T, O>
where
    T: LibraryLoaderAPI<'a>,
    O: MutableAccessIdentifier,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._loader
    }
}

impl<'a, T, O> LibraryLoader<T, O>
where
    T: LibraryLoaderAPI<'a>,
    O: AccessIdentifier,
{
    /// Fetches a pointer that can be used with the interface.
    #[inline]
    pub fn to_interface(&self) -> NonNullConst<LibraryLoaderInterface> {
        self._loader.to_interface()
    }

    /// Construct a new instance from a pointer.
    ///
    /// # Safety
    ///
    /// This function should not be used directly.
    #[inline]
    pub unsafe fn from_interface(handler: NonNullConst<LibraryLoaderInterface>) -> Self {
        Self {
            _loader: T::from_interface(handler),
            _ownership: PhantomData,
        }
    }

    /// Construct a new instance from a void pointer.
    ///
    /// # Safety
    ///
    /// This function should not be used directly.
    #[inline]
    pub unsafe fn from_void_ptr(handler: NonNullConst<c_void>) -> Self {
        Self {
            _loader: T::from_void_ptr(handler),
            _ownership: PhantomData,
        }
    }
}

impl<'a, T, O> LibraryLoader<T, O>
where
    T: LibraryLoaderAPI<'a>,
    O: MutableAccessIdentifier,
{
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
    /// Direct usage of a [LibraryLoader] may break some invariants
    /// of the library api, if not handled with care.
    #[inline]
    pub unsafe fn load(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<InternalLibrary<Owned>, Error<Owned>> {
        self._loader.load(path)
    }

    /// Unloads a library.
    ///
    /// # Failure
    ///
    /// The function fails if `internal` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoader] may break some invariants
    /// of the library api, if not handled with care.
    #[inline]
    pub unsafe fn unload(&mut self, internal: InternalLibrary<Owned>) -> Result<(), Error<Owned>> {
        self._loader.unload(internal)
    }
}

impl<'a, T, O> LibraryLoader<T, O>
where
    T: LibraryLoaderAPI<'a>,
    O: ImmutableAccessIdentifier,
{
    /// Fetches a data symbol from a library.
    ///
    /// # Failure
    ///
    /// The function fails if `internal` is invalid or library does not contain `symbol`.
    ///
    /// # Note
    ///
    /// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
    /// See [LibraryLoader::get_function_symbol()] for fetching a function.
    ///
    /// # Return
    ///
    /// Symbol on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoader] may break some invariants
    /// of the library api, if not handled with care.
    #[inline]
    pub unsafe fn get_data_symbol<LO, U>(
        &self,
        internal: &InternalLibrary<LO>,
        symbol: impl AsRef<CStr>,
        caster: impl FnOnce(NonNullConst<c_void>) -> &'a U,
    ) -> Result<Symbol<'a, &'a U>, Error<Owned>>
    where
        LO: ImmutableAccessIdentifier,
    {
        self._loader.get_data_symbol(internal, symbol, caster)
    }

    /// Fetches a function symbol from a library.
    ///
    /// # Failure
    ///
    /// The function fails if `internal` is invalid or library does not contain `symbol`.
    ///
    /// # Note
    ///
    /// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
    /// See [LibraryLoader::get_data_symbol()] for fetching some data.
    ///
    /// # Return
    ///
    /// Symbol on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoader] may break some invariants
    /// of the library api, if not handled with care.
    #[inline]
    pub unsafe fn get_function_symbol<LO, U>(
        &self,
        internal: &InternalLibrary<LO>,
        symbol: impl AsRef<CStr>,
        caster: impl FnOnce(CBaseFn) -> U,
    ) -> Result<Symbol<'a, U>, Error<Owned>>
    where
        LO: ImmutableAccessIdentifier,
    {
        self._loader.get_function_symbol(internal, symbol, caster)
    }

    /// Fetches a pointer to the internal interface.
    ///
    /// # Return
    ///
    /// Pointer to the interface.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [LibraryLoader] may break some invariants
    /// of the library api, if not handled with care.
    #[inline]
    pub unsafe fn get_internal_interface(&self) -> LibraryLoader<T::InternalLoader, O> {
        LibraryLoader {
            _loader: self._loader.get_internal_interface(),
            _ownership: PhantomData,
        }
    }
}

/// Invalid type erased library loader.
#[derive(Debug, Copy, Clone)]
pub struct InvalidLoader {
    _interface: NonNullConst<c_void>,
}

unsafe impl Send for InvalidLoader {}
unsafe impl Sync for InvalidLoader {}

impl InvalidLoader {
    /// Constructs a new instance.
    #[inline]
    pub fn new(interface: NonNullConst<c_void>) -> Self {
        Self {
            _interface: interface,
        }
    }
}

impl Deref for InvalidLoader {
    type Target = NonNullConst<c_void>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self._interface
    }
}

impl DerefMut for InvalidLoader {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._interface
    }
}

/// Type erased library loader.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct UnknownLoader<'loader> {
    _interface: NonNullConst<LibraryLoaderInterface>,
    _phantom: PhantomData<&'loader ()>,
}

unsafe impl Send for UnknownLoader<'_> {}
unsafe impl Sync for UnknownLoader<'_> {}

impl Deref for UnknownLoader<'_> {
    type Target = NonNullConst<LibraryLoaderInterface>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self._interface
    }
}

impl DerefMut for UnknownLoader<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._interface
    }
}

impl LibraryLoaderABICompat for UnknownLoader<'_> {}

impl<'a> LibraryLoaderAPI<'a> for UnknownLoader<'a> {
    type InternalLoader = InvalidLoader;

    #[inline]
    fn to_interface(&self) -> NonNullConst<LibraryLoaderInterface> {
        self._interface
    }

    #[inline]
    unsafe fn from_interface(interface: NonNullConst<LibraryLoaderInterface>) -> Self {
        Self {
            _interface: interface,
            _phantom: PhantomData,
        }
    }

    #[inline]
    unsafe fn from_void_ptr(interface: NonNullConst<c_void>) -> Self {
        Self::from_interface(interface.cast())
    }

    #[inline]
    unsafe fn load(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<InternalLibrary<Owned>, Error<Owned>> {
        let path_buff = path.as_ref().to_os_path_buff_null();
        self._interface
            .into_mut()
            .as_mut()
            .load(NonNullConst::from(path_buff.as_slice()))
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |v| Ok(InternalLibrary::new(v)))
    }

    #[inline]
    unsafe fn unload(&mut self, internal: InternalLibrary<Owned>) -> Result<(), Error<Owned>> {
        self._interface
            .into_mut()
            .as_mut()
            .unload(internal.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
    }

    #[inline]
    unsafe fn get_data_symbol<O, U>(
        &self,
        internal: &InternalLibrary<O>,
        symbol: impl AsRef<CStr>,
        caster: impl FnOnce(NonNullConst<c_void>) -> &'a U,
    ) -> Result<Symbol<'a, &'a U>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface
            .as_ref()
            .get_data_symbol(
                internal.borrow().as_handle(),
                NonNullConst::from(symbol.as_ref().to_bytes_with_nul()),
            )
            .into_rust()
            .map_or_else(
                |e| Err(Error::from(e)),
                |v| Ok(Symbol::new(caster(v.symbol))),
            )
    }

    #[inline]
    unsafe fn get_function_symbol<O, U>(
        &self,
        internal: &InternalLibrary<O>,
        symbol: impl AsRef<CStr>,
        caster: impl FnOnce(CBaseFn) -> U,
    ) -> Result<Symbol<'a, U>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface
            .as_ref()
            .get_function_symbol(
                internal.borrow().as_handle(),
                NonNullConst::from(symbol.as_ref().to_bytes_with_nul()),
            )
            .into_rust()
            .map_or_else(
                |e| Err(Error::from(e)),
                |v| Ok(Symbol::new(caster(v.symbol))),
            )
    }

    #[inline]
    unsafe fn get_internal_interface(&self) -> Self::InternalLoader {
        Self::InternalLoader::new(self._interface.as_ref().get_internal_interface())
    }
}

/// Native library loader.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NativeLoader<'loader> {
    _interface: UnknownLoader<'loader>,
}

impl Deref for NativeLoader<'_> {
    type Target = NonNullConst<LibraryLoaderInterface>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self._interface.deref()
    }
}

impl DerefMut for NativeLoader<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self._interface.deref_mut()
    }
}

impl LibraryLoaderABICompat for NativeLoader<'_> {}

impl<'a> LibraryLoaderAPI<'a> for NativeLoader<'a> {
    type InternalLoader = NativeLoaderInternal<'a>;

    #[inline]
    fn to_interface(&self) -> NonNullConst<LibraryLoaderInterface> {
        self._interface.to_interface()
    }

    #[inline]
    unsafe fn from_interface(interface: NonNullConst<LibraryLoaderInterface>) -> Self {
        Self {
            _interface: UnknownLoader::from_interface(interface),
        }
    }

    #[inline]
    unsafe fn from_void_ptr(interface: NonNullConst<c_void>) -> Self {
        Self::from_interface(interface.cast())
    }

    #[inline]
    unsafe fn load(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<InternalLibrary<Owned>, Error<Owned>> {
        self._interface.load(path)
    }

    #[inline]
    unsafe fn unload(&mut self, internal: InternalLibrary<Owned>) -> Result<(), Error<Owned>> {
        self._interface.unload(internal)
    }

    #[inline]
    unsafe fn get_data_symbol<O, U>(
        &self,
        internal: &InternalLibrary<O>,
        symbol: impl AsRef<CStr>,
        caster: impl FnOnce(NonNullConst<c_void>) -> &'a U,
    ) -> Result<Symbol<'a, &'a U>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface.get_data_symbol(internal, symbol, caster)
    }

    #[inline]
    unsafe fn get_function_symbol<O, U>(
        &self,
        internal: &InternalLibrary<O>,
        symbol: impl AsRef<CStr>,
        caster: impl FnOnce(CBaseFn) -> U,
    ) -> Result<Symbol<'a, U>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        self._interface
            .get_function_symbol(internal, symbol, caster)
    }

    #[inline]
    unsafe fn get_internal_interface(&self) -> Self::InternalLoader {
        Self::InternalLoader::from_void_ptr(*self._interface.get_internal_interface())
    }
}

/// Native library loader internal interface.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NativeLoaderInternal<'loader> {
    _interface: NonNullConst<NativeLibraryLoaderInterface>,
    _phantom: PhantomData<&'loader ()>,
}

unsafe impl Send for NativeLoaderInternal<'_> {}
unsafe impl Sync for NativeLoaderInternal<'_> {}

impl Deref for NativeLoaderInternal<'_> {
    type Target = NonNullConst<NativeLibraryLoaderInterface>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self._interface
    }
}

impl DerefMut for NativeLoaderInternal<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._interface
    }
}

impl<'a> LibraryLoaderAPI<'a> for NativeLoaderInternal<'a> {
    type InternalLoader = Self;

    #[inline]
    fn to_interface(&self) -> NonNullConst<LibraryLoaderInterface> {
        unsafe { self._interface.as_ref().loader }
    }

    #[inline]
    unsafe fn from_interface(interface: NonNullConst<LibraryLoaderInterface>) -> Self {
        NativeLoader::from_interface(interface).get_internal_interface()
    }

    #[inline]
    unsafe fn from_void_ptr(interface: NonNullConst<c_void>) -> Self {
        Self {
            _interface: interface.cast(),
            _phantom: PhantomData,
        }
    }

    #[inline]
    unsafe fn load(
        &mut self,
        path: impl AsRef<Path>,
    ) -> Result<InternalLibrary<Owned>, Error<Owned>> {
        NativeLoader::from_interface(self.to_interface()).load(path)
    }

    #[inline]
    unsafe fn unload(&mut self, internal: InternalLibrary<Owned>) -> Result<(), Error<Owned>> {
        NativeLoader::from_interface(self.to_interface()).unload(internal)
    }

    #[inline]
    unsafe fn get_data_symbol<O, U>(
        &self,
        internal: &InternalLibrary<O>,
        symbol: impl AsRef<CStr>,
        caster: impl FnOnce(NonNullConst<c_void>) -> &'a U,
    ) -> Result<Symbol<'a, &'a U>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        NativeLoader::from_interface(self.to_interface()).get_data_symbol(internal, symbol, caster)
    }

    #[inline]
    unsafe fn get_function_symbol<O, U>(
        &self,
        internal: &InternalLibrary<O>,
        symbol: impl AsRef<CStr>,
        caster: impl FnOnce(CBaseFn) -> U,
    ) -> Result<Symbol<'a, U>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        NativeLoader::from_interface(self.to_interface())
            .get_function_symbol(internal, symbol, caster)
    }

    #[inline]
    unsafe fn get_internal_interface(&self) -> Self::InternalLoader {
        *self
    }
}

impl NativeLoaderInternal<'_> {
    /// Loads a library. The resulting handle is unique.
    ///
    /// The argument `flags` is passed to `dlopen`.
    ///
    /// # Failure
    ///
    /// The function fails if `path` is invalid or
    /// the call to `dlopen` fails.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeLoaderInternal] may break some invariants
    /// of the library api, if not handled with care.
    #[inline]
    #[cfg(unix)]
    pub unsafe fn load_ext(
        &mut self,
        path: impl AsRef<Path>,
        flags: i32,
    ) -> Result<InternalLibrary<Owned>, Error<Owned>> {
        use crate::ffi::library::library_loader::NativeLibraryLoaderBindingUnix;

        let path_buff = path.as_ref().to_os_path_buff_null();
        self._interface
            .into_mut()
            .as_mut()
            .load_ext(NonNullConst::from(path_buff.as_slice()), flags)
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |v| Ok(InternalLibrary::new(v)))
    }

    /// Loads a library. The resulting handle is unique.
    ///
    /// The arguments `h_file` and `flags` are passed to `LoadLibraryExW`.
    ///
    /// # Failure
    ///
    /// The function fails if `path` is invalid or
    /// the call to `LoadLibraryExW` fails.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeLoaderInternal] may break some invariants
    /// of the library api, if not handled with care.
    #[inline]
    #[cfg(windows)]
    pub unsafe fn load_ext(
        &mut self,
        path: impl AsRef<Path>,
        h_file: Option<NonNull<HANDLE>>,
        flags: u32,
    ) -> Result<InternalLibrary<Owned>, Error<Owned>> {
        use crate::ffi::library::library_loader::NativeLibraryLoaderBindingWindows;

        let path_buff = path.as_ref().to_os_path_buff_null();
        self._interface
            .into_mut()
            .as_mut()
            .load_ext(NonNullConst::from(path_buff.as_slice()), h_file, flags)
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |v| Ok(InternalLibrary::new(v)))
    }

    /// Returns the underlying handle of a library.
    ///
    /// # Failure
    ///
    /// The function fails if `internal` is invalid.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeLoaderInternal] may break some invariants
    /// of the library api, if not handled with care.
    #[inline]
    pub unsafe fn get_native_handle<O>(
        &self,
        internal: &InternalLibrary<O>,
    ) -> Result<NativeLibraryHandle, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        #[cfg(unix)]
        use crate::ffi::library::library_loader::NativeLibraryLoaderBindingUnix;
        #[cfg(windows)]
        use crate::ffi::library::library_loader::NativeLibraryLoaderBindingWindows;

        self._interface
            .as_ref()
            .get_native_handle(internal.borrow().as_handle())
            .into_rust()
            .map_err(Error::from)
    }
}
