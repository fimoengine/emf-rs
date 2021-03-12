//! Library api
//!
//! # Example
//!
//! ```no_run
//! # use emf_core_base_rs::{CBaseAccess, CBase};
//! # let base_interface: &mut CBase = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
//! use emf_core_base_rs::CBaseAPI;
//! use emf_core_base_rs::library::{LibraryAPI, DEFAULT_HANDLE, Symbol, Error};
//! use std::path::Path;
//! use std::ffi::CString;
//!
//! let result = CBaseAccess::lock(base_interface, |interface| -> Result<i32, Error> {
//!     let library_path = Path::new("path to my library");
//!     let symbol_name = CString::new("add_function").unwrap();
//!
//!     let library = LibraryAPI::load(interface, &DEFAULT_HANDLE, &library_path)?;
//!     let symbol: Symbol<extern "C" fn(i32, i32) -> i32> =
//!         LibraryAPI::get_function_symbol(
//!             interface,
//!             &library,
//!             &symbol_name,
//!             |f| unsafe { std::mem::transmute(f) }
//!         )?;
//!
//!     let result = symbol.as_ref()(5, 8);
//!     LibraryAPI::unload(interface, library)?;
//!     Ok(result)
//! });
//!
//! assert_eq!(result.is_ok(), true);
//! assert_eq!(result.unwrap(), 13);
//! ```
use crate::ffi::library::{InternalHandle, LibraryHandle, LoaderHandle};
use crate::ownership::{AccessIdentifier, BorrowImmutable, BorrowMutable, Owned};
use std::marker::PhantomData;

mod api;
pub mod library_loader;

pub use crate::ffi::library::LibraryType;
pub use crate::ffi::library::LOADER_TYPE_MAX_LENGTH;
pub use crate::ffi::library::NATIVE_LIBRARY_TYPE_NAME;

pub use api::LibraryAPI;

/// Handle of the default loader.
pub const DEFAULT_HANDLE: Loader<'static, BorrowMutable<'static>> =
    unsafe { Loader::new(crate::ffi::library::DEFAULT_HANDLE) };

/// Errors of the library api.
#[non_exhaustive]
#[derive(Debug, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Error {
    /// Error denoting an invalid library type.
    InvalidLibraryType(String),
    /// Raw ffi library error.
    FFIError(crate::ffi::library::Error),
}

/// A library handle.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Library<'a, O> {
    _handle: LibraryHandle,
    _lifetime: PhantomData<&'a LibraryHandle>,
    _ownership: PhantomData<*const O>,
}

impl<'a, O> Library<'a, O>
where
    O: AccessIdentifier,
{
    /// Construct a new instance from a handle.
    ///
    /// # Safety
    ///
    /// This function allows the creation of invalid handles
    /// by bypassing lifetimes.
    #[inline]
    pub const unsafe fn new(handle: LibraryHandle) -> Self {
        Self {
            _handle: handle,
            _lifetime: PhantomData,
            _ownership: PhantomData,
        }
    }

    /// Fetches the internal handle.
    #[inline]
    pub const fn as_handle(&self) -> LibraryHandle {
        self._handle
    }
}

impl<'a> Library<'a, Owned> {
    /// Borrows the library handle.
    #[inline]
    pub const fn as_borrowed(&self) -> Library<'a, BorrowImmutable<'_>> {
        unsafe { Library::<BorrowImmutable<'_>>::new(self._handle) }
    }

    /// Borrows the library handle mutably.
    #[inline]
    pub fn as_borrowed_mut(&mut self) -> Library<'a, BorrowMutable<'_>> {
        unsafe { Library::<BorrowMutable<'_>>::new(self._handle) }
    }
}

/// A loader handle.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Loader<'a, O> {
    _handle: LoaderHandle,
    _lifetime: PhantomData<&'a LoaderHandle>,
    _ownership: PhantomData<*const O>,
}

impl<'a, O> Loader<'a, O>
where
    O: AccessIdentifier,
{
    /// Construct a new instance from a handle.
    ///
    /// # Safety
    ///
    /// This function allows the creation of invalid handles
    /// by bypassing lifetimes.
    #[inline]
    pub const unsafe fn new(handle: LoaderHandle) -> Self {
        Self {
            _handle: handle,
            _lifetime: PhantomData,
            _ownership: PhantomData,
        }
    }

    /// Fetches the internal handle.
    #[inline]
    pub const fn as_handle(&self) -> LoaderHandle {
        self._handle
    }
}

impl<'a> Loader<'a, Owned> {
    /// Borrows the loader handle.
    #[inline]
    pub const fn as_borrowed(&self) -> Loader<'a, BorrowImmutable<'_>> {
        unsafe { Loader::<BorrowImmutable<'_>>::new(self._handle) }
    }

    /// Borrows the loader handle mutably.
    #[inline]
    pub fn as_borrowed_mut(&mut self) -> Loader<'a, BorrowMutable<'_>> {
        unsafe { Loader::<BorrowMutable<'_>>::new(self._handle) }
    }
}

/// A loader handle.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct InternalLibrary<O> {
    _handle: InternalHandle,
    _ownership: PhantomData<*const O>,
}

impl<O> InternalLibrary<O>
where
    O: AccessIdentifier,
{
    /// Construct a new instance from a handle.
    ///
    /// # Safety
    ///
    /// This function allows the creation of invalid handles
    /// by bypassing lifetimes.
    #[inline]
    pub const unsafe fn new(handle: InternalHandle) -> Self {
        Self {
            _handle: handle,
            _ownership: PhantomData,
        }
    }

    /// Fetches the internal handle.
    #[inline]
    pub const fn as_handle(&self) -> InternalHandle {
        self._handle
    }
}

impl InternalLibrary<Owned> {
    /// Borrows the loader handle.
    #[inline]
    pub const fn as_borrowed(&self) -> InternalLibrary<BorrowImmutable<'_>> {
        unsafe { InternalLibrary::<BorrowImmutable<'_>>::new(self._handle) }
    }

    /// Borrows the loader handle mutably.
    #[inline]
    pub fn as_borrowed_mut(&mut self) -> InternalLibrary<BorrowMutable<'_>> {
        unsafe { InternalLibrary::<BorrowMutable<'_>>::new(self._handle) }
    }
}

/// A library symbol.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Symbol<'a, T> {
    _symbol: T,
    _phantom: PhantomData<&'a ()>,
}

impl<T> Symbol<'_, T> {
    #[inline]
    fn new(symbol: T) -> Self {
        Self {
            _symbol: symbol,
            _phantom: PhantomData,
        }
    }
}

impl<T> AsRef<T> for Symbol<'_, T> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self._symbol
    }
}

impl<T> AsRef<T> for Symbol<'_, &T> {
    #[inline]
    fn as_ref(&self) -> &T {
        self._symbol
    }
}
