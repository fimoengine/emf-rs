//! Library api.
//!
//! The library api is exposed be the [LibraryBinding] trait.
use crate::collections::{MutSpan, NonNullConst, Result};
use crate::errors::Error;
use crate::library::library_loader::LibraryLoaderInterface;
use crate::library::{
    InternalHandle, LibraryHandle, LibraryType, LoaderHandle, OSPathChar, Symbol,
};
use crate::{Bool, CBase, CBaseFn, CBaseInterface, TypeWrapper};
use std::ffi::c_void;
use std::ptr::NonNull;

pub type RegisterLoaderFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        loader: NonNullConst<LibraryLoaderInterface>,
        lib_type: NonNullConst<LibraryType>,
    ) -> Result<LoaderHandle, Error>,
>;

pub type UnregisterLoaderFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: LoaderHandle,
    ) -> Result<i8, Error>,
>;

pub type GetLoaderInterfaceFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: LoaderHandle,
    ) -> Result<NonNullConst<LibraryLoaderInterface>, Error>,
>;

pub type GetLoaderHandleFromTypeFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        lib_type: NonNullConst<LibraryType>,
    ) -> Result<LoaderHandle, Error>,
>;

pub type GetLoaderHandleFromLibraryFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: LibraryHandle,
    ) -> Result<LoaderHandle, Error>,
>;

pub type GetNumLoadersFn =
    TypeWrapper<unsafe extern "C-unwind" fn(base_module: Option<NonNull<CBase>>) -> usize>;

pub type LibraryExistsFn = TypeWrapper<
    unsafe extern "C-unwind" fn(base_module: Option<NonNull<CBase>>, handle: LibraryHandle) -> Bool,
>;

pub type TypeExistsFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        lib_type: NonNullConst<LibraryType>,
    ) -> Bool,
>;

pub type GetLibraryTypesFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        buffer: NonNull<MutSpan<LibraryType>>,
    ) -> Result<usize, Error>,
>;

pub type CreateLibraryHandleFn =
    TypeWrapper<unsafe extern "C-unwind" fn(base_module: Option<NonNull<CBase>>) -> LibraryHandle>;

pub type RemoveLibraryHandleFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: LibraryHandle,
    ) -> Result<i8, Error>,
>;

pub type LinkLibraryFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: LibraryHandle,
        loader: LoaderHandle,
        internal: InternalHandle,
    ) -> Result<i8, Error>,
>;

pub type GetInternalLibraryHandleFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: LibraryHandle,
    ) -> Result<InternalHandle, Error>,
>;

pub type LoadFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        loader: LoaderHandle,
        path: NonNullConst<OSPathChar>,
    ) -> Result<LibraryHandle, Error>,
>;

pub type UnloadFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: LibraryHandle,
    ) -> Result<i8, Error>,
>;

pub type GetDataSymbolFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: LibraryHandle,
        symbol: NonNullConst<u8>,
    ) -> Result<Symbol<NonNullConst<c_void>>, Error>,
>;

pub type GetFunctionSymbolFn = TypeWrapper<
    unsafe extern "C-unwind" fn(
        base_module: Option<NonNull<CBase>>,
        handle: LibraryHandle,
        symbol: NonNullConst<u8>,
    ) -> Result<Symbol<CBaseFn>, Error>,
>;

/// Helper trait for using the library api.
pub trait LibraryBinding {
    /// Registers a new loader.
    ///
    /// The loader can load libraries of the type `lib_type`.
    ///
    /// # Failure
    ///
    /// The function fails if the library type already exists.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn register_loader(
        &mut self,
        loader: NonNullConst<LibraryLoaderInterface>,
        lib_type: NonNullConst<LibraryType>,
    ) -> Result<LoaderHandle, Error>;

    /// Unregisters an existing loader.
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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn unregister_loader(&mut self, handle: LoaderHandle) -> Result<i8, Error>;

    /// Fetches the interface of a library loader.
    ///
    /// # Failure
    ///
    /// The function fails if `handle` is invalid.
    ///
    /// # Return
    ///
    /// Interface on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_loader_interface(
        &mut self,
        handle: LoaderHandle,
    ) -> Result<NonNullConst<LibraryLoaderInterface>, Error>;

    /// Fetches the loader handle associated with the library type.
    ///
    /// # Failure
    ///
    /// The function fails if `lib_type` is not registered.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_loader_handle_from_type(
        &self,
        lib_type: NonNullConst<LibraryType>,
    ) -> Result<LoaderHandle, Error>;

    /// Fetches the loader handle linked with the library handle.
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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_loader_handle_from_library(
        &self,
        handle: LibraryHandle,
    ) -> Result<LoaderHandle, Error>;

    /// Fetches the number of registered loaders.
    ///
    /// # Return
    ///
    /// Number of registered loaders.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_num_loaders(&self) -> usize;

    /// Checks if a the library handle is valid.
    ///
    /// # Return
    ///
    /// [Bool::True] if the handle is valid, [Bool::False] otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn library_exists(&self, handle: LibraryHandle) -> Bool;

    /// Checks if a library type exists.
    ///
    /// # Return
    ///
    /// [Bool::True] if the type exists, [Bool::False] otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn type_exists(&self, lib_type: NonNullConst<LibraryType>) -> Bool;

    /// Copies the strings of the registered library types into a buffer.
    ///
    /// # Failure
    ///
    /// The function fails if `buffer.as_ref().len() < get_num_loaders()`.
    ///
    /// # Return
    ///
    /// Number of written types on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_library_types(
        &self,
        buffer: NonNull<MutSpan<LibraryType>>,
    ) -> Result<usize, Error>;

    /// Creates a new unlinked library handle.
    ///
    /// # Note
    ///
    /// The handle must be linked before use.
    ///
    /// # Return
    ///
    /// Library handle.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn create_library_handle(&mut self) -> LibraryHandle;

    /// Removes an existing library handle.
    ///
    /// # Failure
    ///
    /// The function fails if `handle` is invalid.
    ///
    /// # Note
    ///
    /// Removing the handle does not unload the library.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn remove_library_handle(&mut self, handle: LibraryHandle) -> Result<i8, Error>;

    /// Links a library handle to an internal library handle.
    ///
    /// Overrides the internal link of the library handle by setting
    /// it to the new library loader and internal handle.
    ///
    /// # Failure
    ///
    /// The function fails if `handle` or `loader` are invalid.
    ///
    /// # Note
    ///
    /// Incorrect usage can lead to dangling handles or use-after-free errors.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn link_library(
        &mut self,
        handle: LibraryHandle,
        loader: LoaderHandle,
        internal: InternalHandle,
    ) -> Result<i8, Error>;

    /// Fetches the internal handle linked with the library handle.
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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_internal_library_handle(
        &self,
        handle: LibraryHandle,
    ) -> Result<InternalHandle, Error>;

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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn load(
        &mut self,
        loader: LoaderHandle,
        path: NonNullConst<OSPathChar>,
    ) -> Result<LibraryHandle, Error>;

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
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn unload(&mut self, handle: LibraryHandle) -> Result<i8, Error>;

    /// Fetches a data symbol from a library.
    ///
    /// # Failure
    ///
    /// The function fails if `handle` is invalid or library does not contain `symbol`.
    ///
    /// # Note
    ///
    /// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
    /// See [LibraryBinding::get_function_symbol()] for fetching a function.
    ///
    /// # Return
    ///
    /// Symbol on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_data_symbol(
        &self,
        handle: LibraryHandle,
        symbol: NonNullConst<u8>,
    ) -> Result<Symbol<NonNullConst<c_void>>, Error>;

    /// Fetches a function symbol from a library.
    ///
    /// # Failure
    ///
    /// The function fails if `handle` is invalid or library does not contain `symbol`.
    ///
    /// # Note
    ///
    /// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
    /// See [LibraryBinding::get_data_symbol()] for fetching some data.
    ///
    /// # Return
    ///
    /// Symbol on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    unsafe fn get_function_symbol(
        &self,
        handle: LibraryHandle,
        symbol: NonNullConst<u8>,
    ) -> Result<Symbol<CBaseFn>, Error>;
}

impl LibraryBinding for CBaseInterface {
    #[inline]
    unsafe fn register_loader(
        &mut self,
        loader: NonNullConst<LibraryLoaderInterface>,
        lib_type: NonNullConst<LibraryType>,
    ) -> Result<LoaderHandle, Error> {
        (self.library_register_loader_fn)(self.base_module, loader, lib_type)
    }

    #[inline]
    unsafe fn unregister_loader(&mut self, handle: LoaderHandle) -> Result<i8, Error> {
        (self.library_unregister_loader_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn get_loader_interface(
        &mut self,
        handle: LoaderHandle,
    ) -> Result<NonNullConst<LibraryLoaderInterface>, Error> {
        (self.library_get_loader_interface_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn get_loader_handle_from_type(
        &self,
        lib_type: NonNullConst<LibraryType>,
    ) -> Result<LoaderHandle, Error> {
        (self.library_get_loader_handle_from_type_fn)(self.base_module, lib_type)
    }

    #[inline]
    unsafe fn get_loader_handle_from_library(
        &self,
        handle: LibraryHandle,
    ) -> Result<LoaderHandle, Error> {
        (self.library_get_loader_handle_from_library_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn get_num_loaders(&self) -> usize {
        (self.library_get_num_loaders_fn)(self.base_module)
    }

    #[inline]
    unsafe fn library_exists(&self, handle: LibraryHandle) -> Bool {
        (self.library_library_exists_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn type_exists(&self, lib_type: NonNullConst<LibraryType>) -> Bool {
        (self.library_type_exists_fn)(self.base_module, lib_type)
    }

    #[inline]
    unsafe fn get_library_types(
        &self,
        buffer: NonNull<MutSpan<LibraryType>>,
    ) -> Result<usize, Error> {
        (self.library_get_library_types_fn)(self.base_module, buffer)
    }

    #[inline]
    unsafe fn create_library_handle(&mut self) -> LibraryHandle {
        (self.library_create_library_handle_fn)(self.base_module)
    }

    #[inline]
    unsafe fn remove_library_handle(&mut self, handle: LibraryHandle) -> Result<i8, Error> {
        (self.library_remove_library_handle_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn link_library(
        &mut self,
        handle: LibraryHandle,
        loader: LoaderHandle,
        internal: InternalHandle,
    ) -> Result<i8, Error> {
        (self.library_link_library_fn)(self.base_module, handle, loader, internal)
    }

    #[inline]
    unsafe fn get_internal_library_handle(
        &self,
        handle: LibraryHandle,
    ) -> Result<InternalHandle, Error> {
        (self.library_get_internal_library_handle_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn load(
        &mut self,
        loader: LoaderHandle,
        path: NonNullConst<OSPathChar>,
    ) -> Result<LibraryHandle, Error> {
        (self.library_load_fn)(self.base_module, loader, path)
    }

    #[inline]
    unsafe fn unload(&mut self, handle: LibraryHandle) -> Result<i8, Error> {
        (self.library_unload_fn)(self.base_module, handle)
    }

    #[inline]
    unsafe fn get_data_symbol(
        &self,
        handle: LibraryHandle,
        symbol: NonNullConst<u8>,
    ) -> Result<Symbol<NonNullConst<c_void>>, Error> {
        (self.library_get_data_symbol_fn)(self.base_module, handle, symbol)
    }

    #[inline]
    unsafe fn get_function_symbol(
        &self,
        handle: LibraryHandle,
        symbol: NonNullConst<u8>,
    ) -> Result<Symbol<CBaseFn>, Error> {
        (self.library_get_function_symbol_fn)(self.base_module, handle, symbol)
    }
}
