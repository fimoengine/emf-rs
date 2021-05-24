use crate::ffi::collections::{MutSpan, NonNullConst};
use crate::ffi::errors::SimpleError;
use crate::ffi::library::api::LibraryBinding;
use crate::ffi::library::{OSPathString, SymbolName};
use crate::ffi::{Bool, CBaseFn};
use crate::library::library_loader::{LibraryLoader, LibraryLoaderABICompat, LibraryLoaderAPI};
use crate::library::{
    InternalLibrary, Library, LibraryType, Loader, Symbol, LOADER_TYPE_MAX_LENGTH,
};
use crate::ownership::{BorrowMutable, ImmutableAccessIdentifier, MutableAccessIdentifier, Owned};
use crate::Error;
use crate::ToOsPathBuff;
use std::ffi::{c_void, CStr};
use std::path::Path;
use std::pin::Pin;

/// Idiomatic library api.
pub trait LibraryAPI<'interface> {
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
    fn register_loader<LT, L>(
        &mut self,
        loader: Pin<&'interface LT>,
        lib_type: impl AsRef<str>,
    ) -> Result<Loader<'interface, Owned>, Error<Owned>>
    where
        L: LibraryLoaderAPI<'interface> + LibraryLoaderABICompat,
        LibraryLoader<L, Owned>: From<&'interface LT>;

    /// Unregisters an existing loader.
    ///
    /// # Failure
    ///
    /// The function fails if `loader` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    fn unregister_loader(&mut self, loader: Loader<'_, Owned>) -> Result<(), Error<Owned>>;

    /// Fetches the interface of a library loader.
    ///
    /// # Failure
    ///
    /// The function fails if `loader` is invalid.
    ///
    /// # Return
    ///
    /// Interface on success, error otherwise.
    fn get_loader_interface<'loader, O, L>(
        &self,
        loader: &Loader<'loader, O>,
    ) -> Result<LibraryLoader<L, O>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
        L: LibraryLoaderAPI<'loader> + LibraryLoaderABICompat;

    /// Fetches the loader handle associated with the library type.
    ///
    /// # Failure
    ///
    /// The function fails if `lib_type` is not registered.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    fn get_loader_handle_from_type(
        &self,
        lib_type: impl AsRef<str>,
    ) -> Result<Loader<'interface, BorrowMutable<'_>>, Error<Owned>>;

    /// Fetches the loader handle linked with the library handle.
    ///
    /// # Failure
    ///
    /// The function fails if `library` is invalid.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    fn get_loader_handle_from_library<'library, O>(
        &self,
        library: &Library<'library, O>,
    ) -> Result<Loader<'library, BorrowMutable<'_>>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

    /// Fetches the number of registered loaders.
    ///
    /// # Return
    ///
    /// Number of registered loaders.
    fn get_num_loaders(&self) -> usize;

    /// Checks if a the library handle is valid.
    ///
    /// # Return
    ///
    /// [true] if the handle is valid, [false] otherwise.
    fn library_exists<'library, O>(&self, library: &Library<'library, O>) -> bool
    where
        O: ImmutableAccessIdentifier;

    /// Checks if a library type exists.
    ///
    /// # Return
    ///
    /// [true] if the type exists, [false] otherwise.
    fn type_exists(&self, lib_type: impl AsRef<str>) -> Result<bool, Error<Owned>>;

    /// Copies the strings of the registered library types into a buffer.
    ///
    /// # Failure
    ///
    /// The function fails if `buffer.as_ref().len() < get_num_loaders()`.
    ///
    /// # Return
    ///
    /// Number of written types on success, error otherwise.
    fn get_library_types(&self, buffer: impl AsMut<[LibraryType]>) -> Result<usize, Error<Owned>>;

    /// Creates a new unlinked library handle.
    ///
    /// # Return
    ///
    /// Library handle.
    ///
    /// # Safety
    ///
    /// The handle must be linked before use.
    unsafe fn create_library_handle(&mut self) -> Library<'interface, Owned>;

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
    unsafe fn remove_library_handle(
        &mut self,
        library: Library<'_, Owned>,
    ) -> Result<(), Error<Owned>>;

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
    unsafe fn link_library<'library, 'loader, O, LO, IO>(
        &mut self,
        library: &Library<'library, O>,
        loader: &Loader<'loader, LO>,
        internal: &InternalLibrary<IO>,
    ) -> Result<(), Error<Owned>>
    where
        'loader: 'library,
        O: MutableAccessIdentifier,
        LO: ImmutableAccessIdentifier,
        IO: ImmutableAccessIdentifier;

    /// Fetches the internal handle linked with the library handle.
    ///
    /// # Failure
    ///
    /// The function fails if `handle` is invalid.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    fn get_internal_library_handle<'library, O>(
        &self,
        library: &Library<'library, O>,
    ) -> Result<InternalLibrary<O>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

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
    fn load<O>(
        &mut self,
        loader: &Loader<'interface, O>,
        path: impl AsRef<Path>,
    ) -> Result<Library<'interface, Owned>, Error<Owned>>
    where
        O: MutableAccessIdentifier;

    /// Unloads a library.
    ///
    /// # Failure
    ///
    /// The function fails if `library` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    fn unload(&mut self, library: Library<'_, Owned>) -> Result<(), Error<Owned>>;

    /// Fetches a data symbol from a library.
    ///
    /// # Failure
    ///
    /// The function fails if `library` is invalid or library does not contain `symbol`.
    ///
    /// # Note
    ///
    /// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
    /// See [LibraryAPI::get_function_symbol()] for fetching a function.
    ///
    /// # Return
    ///
    /// Symbol on success, error otherwise.
    fn get_data_symbol<'library, 'handle, O, U>(
        &self,
        library: &'handle Library<'library, O>,
        symbol: impl AsRef<CStr>,
        caster: impl FnOnce(NonNullConst<c_void>) -> &'library U,
    ) -> Result<Symbol<'handle, &'library U>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier;

    /// Fetches a function symbol from a library.
    ///
    /// # Failure
    ///
    /// The function fails if `library` is invalid or library does not contain `symbol`.
    ///
    /// # Note
    ///
    /// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
    /// See [LibraryAPI::get_data_symbol()] for fetching some data.
    ///
    /// # Return
    ///
    /// Symbol on success, error otherwise.
    fn get_function_symbol<'library, 'handle, O, U>(
        &self,
        library: &'handle Library<'library, O>,
        symbol: impl AsRef<CStr>,
        caster: impl FnOnce(CBaseFn) -> U,
    ) -> Result<Symbol<'handle, U>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier;
}

impl<'interface, T> LibraryAPI<'interface> for T
where
    T: LibraryBinding,
{
    #[inline]
    fn register_loader<LT, L>(
        &mut self,
        loader: Pin<&'interface LT>,
        lib_type: impl AsRef<str>,
    ) -> Result<Loader<'interface, Owned>, Error<Owned>>
    where
        L: LibraryLoaderAPI<'interface> + LibraryLoaderABICompat,
        LibraryLoader<L, Owned>: From<&'interface LT>,
    {
        let lib_str = lib_type.as_ref();
        if lib_str.as_bytes().len() > LOADER_TYPE_MAX_LENGTH {
            return Err(Error::from(SimpleError::new(format!(
                "Loader type too long: {}",
                lib_str
            ))));
        }

        let lib_type = LibraryType::from(lib_str);

        unsafe {
            self.register_loader(
                LibraryLoader::<L, Owned>::from(loader.get_ref()).to_raw(),
                NonNullConst::from(&lib_type),
            )
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |v| Ok(Loader::new(v)))
        }
    }

    #[inline]
    fn unregister_loader(&mut self, loader: Loader<'_, Owned>) -> Result<(), Error<Owned>> {
        unsafe {
            self.unregister_loader(loader.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
        }
    }

    #[inline]
    fn get_loader_interface<'loader, O, L>(
        &self,
        loader: &Loader<'loader, O>,
    ) -> Result<LibraryLoader<L, O>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
        L: LibraryLoaderAPI<'loader> + LibraryLoaderABICompat,
    {
        unsafe {
            self.get_loader_interface(loader.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |v| Ok(LibraryLoader::from_raw(v)))
        }
    }

    #[inline]
    fn get_loader_handle_from_type(
        &self,
        lib_type: impl AsRef<str>,
    ) -> Result<Loader<'interface, BorrowMutable<'_>>, Error<Owned>> {
        let lib_str = lib_type.as_ref();
        if lib_str.as_bytes().len() > LOADER_TYPE_MAX_LENGTH {
            return Err(Error::from(SimpleError::new(format!(
                "Loader type too long: {}",
                lib_str
            ))));
        }

        let lib_type = LibraryType::from(lib_str);

        unsafe {
            self.get_loader_handle_from_type(NonNullConst::from(&lib_type))
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |v| Ok(Loader::new(v)))
        }
    }

    #[inline]
    fn get_loader_handle_from_library<'library, O>(
        &self,
        library: &Library<'library, O>,
    ) -> Result<Loader<'library, BorrowMutable<'_>>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe {
            self.get_loader_handle_from_library(library.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |v| Ok(Loader::new(v)))
        }
    }

    #[inline]
    fn get_num_loaders(&self) -> usize {
        unsafe { self.get_num_loaders() }
    }

    #[inline]
    fn library_exists<'library, O>(&self, library: &Library<'library, O>) -> bool
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe { self.library_exists(library.as_handle()) == Bool::True }
    }

    #[inline]
    fn type_exists(&self, lib_type: impl AsRef<str>) -> Result<bool, Error<Owned>> {
        let lib_str = lib_type.as_ref();
        if lib_str.as_bytes().len() > LOADER_TYPE_MAX_LENGTH {
            return Err(Error::from(SimpleError::new(format!(
                "Loader type too long: {}",
                lib_str
            ))));
        }

        let lib_type = LibraryType::from(lib_str);

        unsafe { Ok(self.type_exists(NonNullConst::from(&lib_type)) == Bool::True) }
    }

    #[inline]
    fn get_library_types(
        &self,
        mut buffer: impl AsMut<[LibraryType]>,
    ) -> Result<usize, Error<Owned>> {
        unsafe {
            self.get_library_types(MutSpan::from(buffer.as_mut()))
                .into_rust()
                .map_err(Error::from)
        }
    }

    #[inline]
    unsafe fn create_library_handle(&mut self) -> Library<'interface, Owned> {
        Library::new(self.create_library_handle())
    }

    #[inline]
    unsafe fn remove_library_handle(
        &mut self,
        library: Library<'_, Owned>,
    ) -> Result<(), Error<Owned>> {
        self.remove_library_handle(library.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
    }

    #[inline]
    unsafe fn link_library<'library, 'loader, O, LO, IO>(
        &mut self,
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
        self.link_library(
            library.as_handle(),
            loader.as_handle(),
            internal.as_handle(),
        )
        .into_rust()
        .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
    }

    #[inline]
    fn get_internal_library_handle<'library, O>(
        &self,
        library: &Library<'library, O>,
    ) -> Result<InternalLibrary<O>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe {
            self.get_internal_library_handle(library.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |v| Ok(InternalLibrary::new(v)))
        }
    }

    #[inline]
    fn load<O>(
        &mut self,
        loader: &Loader<'interface, O>,
        path: impl AsRef<Path>,
    ) -> Result<Library<'interface, Owned>, Error<Owned>>
    where
        O: MutableAccessIdentifier,
    {
        let path_buff = path.as_ref().to_os_path_buff_null();
        unsafe {
            self.load(loader.as_handle(), OSPathString::from(path_buff.as_slice()))
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |v| Ok(Library::new(v)))
        }
    }

    #[inline]
    fn unload(&mut self, library: Library<'_, Owned>) -> Result<(), Error<Owned>> {
        unsafe {
            self.unload(library.as_handle())
                .into_rust()
                .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
        }
    }

    #[inline]
    fn get_data_symbol<'library, 'handle, O, U>(
        &self,
        library: &'handle Library<'library, O>,
        symbol: impl AsRef<CStr>,
        caster: impl FnOnce(NonNullConst<c_void>) -> &'library U,
    ) -> Result<Symbol<'handle, &'library U>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe {
            self.get_data_symbol(
                library.as_handle(),
                SymbolName::from(symbol.as_ref().to_bytes_with_nul()),
            )
            .into_rust()
            .map_or_else(
                |e| Err(Error::from(e)),
                |v| Ok(Symbol::new(caster(v.symbol))),
            )
        }
    }

    #[inline]
    fn get_function_symbol<'library, 'handle, O, U>(
        &self,
        library: &'handle Library<'library, O>,
        symbol: impl AsRef<CStr>,
        caster: impl FnOnce(CBaseFn) -> U,
    ) -> Result<Symbol<'handle, U>, Error<Owned>>
    where
        O: ImmutableAccessIdentifier,
    {
        unsafe {
            self.get_function_symbol(
                library.as_handle(),
                SymbolName::from(symbol.as_ref().to_bytes_with_nul()),
            )
            .into_rust()
            .map_or_else(
                |e| Err(Error::from(e)),
                |v| Ok(Symbol::new(caster(v.symbol))),
            )
        }
    }
}
