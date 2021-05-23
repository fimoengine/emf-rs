//! Global library api.
//!
//! # Example
//!
//! ```no_run
//! use emf_core_base_rs::global::{LockToken, Unlock, library};
//! use emf_core_base_rs::library::{DEFAULT_HANDLE, Symbol};
//! use std::path::Path;
//! use std::ffi::CString;
//!
//! # use emf_core_base_rs::Error;
//! # use emf_core_base_rs::ownership::Owned;
//! # fn main() -> Result<(), Error<Owned>> {
//! let mut lock = LockToken::<Unlock>::lock();
//!
//! let library_path = Path::new("path to my library");
//! let symbol_name = CString::new("add_function").unwrap();
//!
//! let mut  library = library::load(&mut lock, &DEFAULT_HANDLE, &library_path)?;
//! let symbol: Symbol<extern "C" fn(i32, i32) -> i32> =
//!     library::get_function_symbol(
//!         &lock,
//!         &library,
//!         &symbol_name,
//!         |f| unsafe { std::mem::transmute(f) }
//!     )?;
//!
//! assert_eq!(symbol.as_ref()(5, 8), 13);
//! # Ok(())
//! # }
//! ```
use crate::ffi::collections::NonNullConst;
use crate::ffi::CBaseFn;
use crate::global::{get_interface, get_mut_interface, LockToken};
use crate::library::library_loader::{LibraryLoader, LibraryLoaderABICompat, LibraryLoaderAPI};
use crate::library::{InternalLibrary, Library, LibraryAPI, LibraryType, Loader, Symbol};
use crate::ownership::{BorrowMutable, ImmutableAccessIdentifier, MutableAccessIdentifier, Owned};
use crate::Error;
use std::ffi::{c_void, CStr};
use std::path::Path;

/// Registers a new loader.
///
/// The loader can load libraries of the type `lib_type`.
/// The loader must outlive the binding to the interface.
///
/// # Failure
///
/// The function fails if the library type already exists.
///
/// # Return
///
/// Handle on success, error otherwise.
#[inline]
pub fn register_loader<'loader, L, LT, T>(
    _token: &mut LockToken<L>,
    loader: &'loader LT,
    lib_type: impl AsRef<str>,
) -> Result<Loader<'static, Owned>, Error<Owned>>
where
    T: LibraryLoaderAPI<'static> + LibraryLoaderABICompat,
    LibraryLoader<T, Owned>: From<&'loader LT>,
{
    LibraryAPI::register_loader(get_mut_interface(), loader, lib_type)
}

/// Unregisters an existing loader.
///
/// # Failure
///
/// The function fails if `loader` is invalid.
///
/// # Return
///
/// Error on failure.
#[inline]
pub fn unregister_loader<L>(
    _token: &mut LockToken<L>,
    loader: Loader<'_, Owned>,
) -> Result<(), Error<Owned>> {
    LibraryAPI::unregister_loader(get_mut_interface(), loader)
}

/// Fetches the interface of a library loader.
///
/// # Failure
///
/// The function fails if `loader` is invalid.
///
/// # Return
///
/// Interface on success, error otherwise.
#[inline]
pub fn get_loader_interface<'loader, L, O, T>(
    _token: &LockToken<L>,
    loader: &Loader<'loader, O>,
) -> Result<LibraryLoader<T, O>, Error<Owned>>
where
    O: ImmutableAccessIdentifier,
    T: LibraryLoaderAPI<'loader> + LibraryLoaderABICompat,
{
    LibraryAPI::get_loader_interface(get_interface(), loader)
}

/// Fetches the loader handle associated with the library type.
///
/// # Failure
///
/// The function fails if `lib_type` is not registered.
///
/// # Return
///
/// Handle on success, error otherwise.
#[inline]
pub fn get_loader_handle_from_type<'tok, L>(
    _token: &'tok LockToken<L>,
    lib_type: impl AsRef<str>,
) -> Result<Loader<'static, BorrowMutable<'tok>>, Error<Owned>> {
    LibraryAPI::get_loader_handle_from_type(get_interface(), lib_type)
}

/// Fetches the loader handle linked with the library handle.
///
/// # Failure
///
/// The function fails if `library` is invalid.
///
/// # Return
///
/// Handle on success, error otherwise.
#[inline]
pub fn get_loader_handle_from_library<'l, 'library, L, O>(
    _token: &'l LockToken<L>,
    library: &Library<'library, O>,
) -> Result<Loader<'library, BorrowMutable<'l>>, Error<Owned>>
where
    O: ImmutableAccessIdentifier,
{
    LibraryAPI::get_loader_handle_from_library(get_interface(), library)
}

/// Fetches the number of registered loaders.
///
/// # Return
///
/// Number of registered loaders.
#[inline]
pub fn get_num_loaders<L>(_token: &LockToken<L>) -> usize {
    LibraryAPI::get_num_loaders(get_interface())
}

/// Checks if a the library handle is valid.
///
/// # Return
///
/// [true] if the handle is valid, [false] otherwise.
#[inline]
pub fn library_exists<'library, L, O>(_token: &LockToken<L>, library: &Library<'library, O>) -> bool
where
    O: ImmutableAccessIdentifier,
{
    LibraryAPI::library_exists(get_interface(), library)
}

/// Checks if a library type exists.
///
/// # Return
///
/// [true] if the type exists, [false] otherwise.
#[inline]
pub fn type_exists<L>(
    _token: &LockToken<L>,
    lib_type: impl AsRef<str>,
) -> Result<bool, Error<Owned>> {
    LibraryAPI::type_exists(get_interface(), lib_type)
}

/// Copies the strings of the registered library types into a buffer.
///
/// # Failure
///
/// The function fails if `buffer.as_ref().len() < get_num_loaders()`.
///
/// # Return
///
/// Number of written types on success, error otherwise.
#[inline]
pub fn get_library_types<L>(
    _token: &LockToken<L>,
    buffer: impl AsMut<[LibraryType]>,
) -> Result<usize, Error<Owned>> {
    LibraryAPI::get_library_types(get_interface(), buffer)
}

/// Creates a new unlinked library handle.
///
/// # Return
///
/// Library handle.
///
/// # Safety
///
/// The handle must be linked before use.
#[inline]
pub unsafe fn create_library_handle<L>(_token: &mut LockToken<L>) -> Library<'static, Owned> {
    LibraryAPI::create_library_handle(get_mut_interface())
}

/// Removes an existing library handle.
///
/// # Failure
///
/// The function fails if `library` is invalid.
///
/// # Return
///
/// Error on failure.
///
/// # Safety
///
/// Removing the handle does not unload the library.
#[inline]
pub unsafe fn remove_library_handle<L>(
    _token: &mut LockToken<L>,
    library: Library<'_, Owned>,
) -> Result<(), Error<Owned>> {
    LibraryAPI::remove_library_handle(get_mut_interface(), library)
}

/// Links a library handle to an internal library handle.
///
/// Overrides the internal link of the library handle by setting
/// it to the new library loader and internal handle.
///
/// # Failure
///
/// The function fails if `library` or `loader` are invalid.
///
/// # Return
///
/// Error on failure.
///
/// # Safety
///
/// Incorrect usage can lead to dangling handles or use-after-free errors.
#[inline]
pub unsafe fn link_library<'library, 'loader, L, O, LO, IO>(
    _token: &mut LockToken<L>,
    library: &Library<'library, O>,
    loader: &Loader<'loader, LO>,
    internal: &InternalLibrary<IO>,
) -> Result<(), Error<Owned>>
where
    'loader: 'library,
    O: MutableAccessIdentifier,
    LO: ImmutableAccessIdentifier,
    IO: ImmutableAccessIdentifier,
{
    LibraryAPI::link_library(get_mut_interface(), library, loader, internal)
}

/// Fetches the internal handle linked with the library handle.
///
/// # Failure
///
/// The function fails if `handle` is invalid.
///
/// # Return
///
/// Handle on success, error otherwise.
#[inline]
pub fn get_internal_library_handle<'library, L, O>(
    _token: &LockToken<L>,
    library: &Library<'library, O>,
) -> Result<InternalLibrary<O>, Error<Owned>>
where
    O: ImmutableAccessIdentifier,
{
    LibraryAPI::get_internal_library_handle(get_interface(), library)
}

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
#[inline]
pub fn load<L, O>(
    _token: &mut LockToken<L>,
    loader: &Loader<'static, O>,
    path: impl AsRef<Path>,
) -> Result<Library<'static, Owned>, Error<Owned>>
where
    O: MutableAccessIdentifier,
{
    LibraryAPI::load(get_mut_interface(), loader, path)
}

/// Unloads a library.
///
/// # Failure
///
/// The function fails if `library` is invalid.
///
/// # Return
///
/// Error on failure.
#[inline]
pub fn unload<L>(
    _token: &mut LockToken<L>,
    library: Library<'_, Owned>,
) -> Result<(), Error<Owned>> {
    LibraryAPI::unload(get_mut_interface(), library)
}

/// Fetches a data symbol from a library.
///
/// # Failure
///
/// The function fails if `library` is invalid or library does not contain `symbol`.
///
/// # Note
///
/// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
/// See [get_function_symbol()] for fetching a function.
///
/// # Return
///
/// Symbol on success, error otherwise.
#[inline]
pub fn get_data_symbol<'library, 'handle, L, O, U>(
    _token: &LockToken<L>,
    library: &'handle Library<'library, O>,
    symbol: impl AsRef<CStr>,
    caster: impl FnOnce(NonNullConst<c_void>) -> &'library U,
) -> Result<Symbol<'handle, &'library U>, Error<Owned>>
where
    O: ImmutableAccessIdentifier,
{
    LibraryAPI::get_data_symbol(get_interface(), library, symbol, caster)
}

/// Fetches a function symbol from a library.
///
/// # Failure
///
/// The function fails if `library` is invalid or library does not contain `symbol`.
///
/// # Note
///
/// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
/// See [get_data_symbol()] for fetching some data.
///
/// # Return
///
/// Symbol on success, error otherwise.
#[inline]
pub fn get_function_symbol<'library, 'handle, L, O, U>(
    _token: &LockToken<L>,
    library: &'handle Library<'library, O>,
    symbol: impl AsRef<CStr>,
    caster: impl FnOnce(CBaseFn) -> U,
) -> Result<Symbol<'handle, U>, Error<Owned>>
where
    O: ImmutableAccessIdentifier,
{
    LibraryAPI::get_function_symbol(get_interface(), library, symbol, caster)
}
