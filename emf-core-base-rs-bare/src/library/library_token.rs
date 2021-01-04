use crate::library::loader_library_handle::LoaderLibraryHandle;
use crate::library::{
    LibraryError, LibraryHandle, LibraryLoaderHandle, LibraryLoaderWrapper, LibrarySymbol,
    LibraryType,
};
use crate::{ffi, FFIObject};
use std::ffi::CStr;
use std::path::Path;

/// Access point to the `library` api.
pub trait LibraryToken<'a> {
    /// Registers a new `LibraryLoader` with the `library` api.
    ///
    /// # Failure
    ///
    /// The function fails if the library type already exists.
    fn register_loader<T: LibraryLoaderWrapper<'static>>(
        &self,
        loader: &T,
        lib_type: &LibraryType,
    ) -> Result<LibraryLoaderHandle<'static>, LibraryError>;

    /// Removes a `LibraryLoader` from the api.
    ///
    /// # Failure
    ///
    /// The function fails if `loader` is invalid.
    fn unregister_loader(&self, loader: LibraryLoaderHandle) -> Option<LibraryError>;

    /// Fetches the number of registered loaders.
    fn get_num_loaders(&self) -> usize;

    /// Copies the strings of the registered library types into a buffer.
    ///
    /// Returns the number of copied elements.
    ///
    /// # Failure
    ///
    /// The function fails if `buf` is smaller than [LibraryToken::get_num_loaders()].
    fn get_library_types<T: AsMut<[LibraryType]>>(
        &self,
        buf: &mut T,
    ) -> Result<usize, LibraryError>;

    /// Fetches the loader handle associated with the library type.
    ///
    /// # Failure
    ///
    /// The function fails if `lib_type` is not registered.
    fn get_loader_handle(
        &self,
        lib_type: &LibraryType,
    ) -> Result<LibraryLoaderHandle<'a>, LibraryError>;

    /// Checks if a library type exists.
    fn library_type_exists(&self, lib_type: &LibraryType) -> bool;

    /// Checks if a the library handle is valid.
    fn library_exists(&self, library: &LibraryHandle) -> bool;

    /// Creates a new unlinked library handle.
    ///
    /// # Safety
    ///
    /// The handle must be linked before use (See [LibraryToken::link_library()]).
    unsafe fn create_library_handle<'b>(&self) -> LibraryHandle<'b>;

    /// Removes an existing library handle.
    ///
    /// # Failure
    ///
    /// The function fails if `lib_handle` is invalid.
    ///
    /// # Safety
    ///
    /// Removing the handle does not unload the library.
    unsafe fn remove_library_handle(&self, library: LibraryHandle) -> Option<LibraryError>;

    /// Links a library handle to an internal library handle.
    ///
    /// Overrides the internal link of the library handle by setting it to the
    /// new library loader and internal handle.
    ///
    /// # Failure
    ///
    /// The function fails if `lib_handle` or `loader_handle` are invalid.
    ///
    /// # Safety
    ///
    /// Incorrect usage can lead to dangling handles or use-after-free errors.
    unsafe fn link_library<'b, 'c: 'd, 'd: 'b, T: LibraryLoaderWrapper<'d>>(
        &self,
        library: &LibraryHandle,
        loader: &'c LibraryLoaderHandle<'c>,
        internal_handle: &'b LoaderLibraryHandle<'b, 'd, T>,
    ) -> Option<LibraryError>;

    /// Fetches the internal handle linked with the library handle.
    ///
    /// # Failure
    ///
    /// The function fails if `lib_handle` is invalid.
    ///
    /// # Safety
    ///
    /// Care must be taken when dealing with internal handles.
    unsafe fn get_loader_library_handle<'b, T: LibraryLoaderWrapper<'a>>(
        &self,
        library: &'b LibraryHandle,
    ) -> Result<LoaderLibraryHandle<'b, 'a, T>, LibraryError>;

    /// Fetches the loader handle linked with the library handle.
    ///
    /// # Failure
    ///
    /// The function fails if `lib_handle` is invalid.
    ///
    /// # Safety
    ///
    /// Care must be taken when dealing with internal handles.
    unsafe fn get_loader_handle_from_lib(
        &self,
        library: &LibraryHandle,
    ) -> Result<LibraryLoaderHandle<'a>, LibraryError>;

    /// Fetches the interface of a library loader.
    ///
    /// # Failure
    ///
    /// The function fails if `loader` is invalid.
    ///
    /// # Safety
    ///
    /// Direct usage of a `LibraryLoader` is discouraged, as it bypasses
    /// the safety of the `library` api.
    unsafe fn get_loader_interface<T: LibraryLoaderWrapper<'a>>(
        &self,
        loader: &LibraryLoaderHandle,
    ) -> Result<T, LibraryError>;

    /// Loads a library from a path.
    ///
    /// The resulting handle is unique.
    ///
    /// # Failure
    ///
    /// The function fails if `loader` or `path` is invalid or the type
    /// of the library can not be loaded by the loader.
    fn load<'c, 'b: 'c, T: AsRef<Path>>(
        &self,
        loader: &'b LibraryLoaderHandle<'b>,
        path: &T,
    ) -> Result<LibraryHandle<'c>, LibraryError>;

    /// Unloads a library.
    ///
    /// # Failure
    ///
    /// The function fails if `library` is invalid.
    fn unload(&self, library: LibraryHandle) -> Option<LibraryError>;

    /// Fetches a data symbol from a library.
    ///
    /// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
    /// See [LibraryToken::get_function_symbol()] when fetching a function.
    ///
    /// # Failure
    ///
    /// The function fails if `library` is invalid or library does not contain `name`.
    fn get_data_symbol<'b, T: 'b + Sized + FFIObject<ffi::library::DataSymbol>, S: AsRef<CStr>>(
        &self,
        library: &'b LibraryHandle<'b>,
        name: &S,
    ) -> Result<LibrarySymbol<'b, T>, LibraryError>;

    /// Fetches a function symbol from a library.
    ///
    /// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
    /// See [LibraryToken::get_data_symbol()] when fetching some data.
    ///
    /// # Failure
    ///
    /// The function fails if `library` is invalid or library does not contain `name`.
    fn get_function_symbol<'b, T: 'b + Sized + FFIObject<ffi::library::FnSymbol>, S: AsRef<CStr>>(
        &self,
        library: &'b LibraryHandle<'b>,
        name: &S,
    ) -> Result<LibrarySymbol<'b, T>, LibraryError>;
}
