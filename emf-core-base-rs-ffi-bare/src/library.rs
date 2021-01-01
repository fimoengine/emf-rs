//! A module for loading libraries.
//!
//! The `library` api is a collection of procedures that provide a platform agnostic interface
//! to loading shared libraries. The actual loading of a library is handled by a `library loader`.
//! Each `library loader` is associated to a `library type`.
//!
//! ## Loaders
//!
//! The job of a `library loader` is to manage the loading and unloading of libraries.
//! A `library loader` can be added by constructing a [LoaderInterface] and then calling the
//! [emf_cbase_library_register_loader()] function.
//!
//! ## Library types
//!
//! The `library` api allows the loading of custom library formats.
//! Each format is identified by a [LibraryType] and is associated to exactly one `library loader`.
//!
//! # Predefined Loaders
//!
//! Some `library loaders` are always present and can not be removed at runtime.
//!
//! ## Native
//!
//! The native `library loader` is able to load platform-specific libraries (e.g. dlopen()/LoadLibrary()).
//! It is associated to the [NATIVE_LIBRARY_TYPE_NAME] `library type`
//! and is reachable with the [LIBRARY_LOADER_DEFAULT_HANDLE] handle.
//!
//! On posix the `library loader` defaults to the `RTLD_LAZY` and `RTLD_LOCAL` flags. More control of how to load a library can
//! be achieved by fetching a pointer to the interface with
//! [emf_cbase_library_unsafe_get_loader_interface], casting it to a [NativeLoaderInterface] and
//! calling the [load_ext_fn](NativeLoaderInterface::load_ext_fn) function.
//! If the library has dependencies on other libraries, then it must specify them by specifying an `rpath` on posix or
//! embedding an `Activation Context manifest` into the library on Windows.
//!
//! # Example
//!
//! ```no_run
//! # use emf_core_base_rs_ffi_bare::sys::{emf_cbase_sys_lock, emf_cbase_sys_unlock, emf_cbase_sys_panic};
//! # use emf_core_base_rs_ffi_bare::library::{emf_cbase_library_load, LIBRARY_LOADER_DEFAULT_HANDLE,
//! #     emf_cbase_library_get_function_symbol, emf_cbase_library_unload};
//! # use emf_core_base_rs_ffi_bare::containers::NonNullConst;
//! # use std::ffi::CString;
//!
//! unsafe {
//!     emf_cbase_sys_lock();
//!
//!     const LIBRARY_PATH: &[u8; 17] = b"./example_lib.so\0";
//!     const SYMBOL_NAME: &[u8; 11] = b"example_fn\0";
//!
//!     /// Only works where `OsPathChar` is `u8`
//!     /// Other systems require a conversion.
//!     let library_handle = emf_cbase_library_load(
//!         LIBRARY_LOADER_DEFAULT_HANDLE,
//!         NonNullConst::new_unchecked(LIBRARY_PATH.as_ptr()).cast()
//!     );
//!
//!     let library_handle = match library_handle.to_native() {
//!         Ok(handle) => handle,
//!         Err(_) => {
//!             let error = CString::new("Could not construct version from string.").unwrap();
//!             emf_cbase_sys_panic(error.as_ptr());
//!         }
//!     };
//!
//!     let fn_symbol = emf_cbase_library_get_function_symbol(
//!         library_handle,
//!         NonNullConst::new_unchecked(SYMBOL_NAME.as_ptr()).cast()
//!     );
//!
//!     let fn_symbol: extern "C" fn(i32, i32) = match fn_symbol.to_native() {
//!         Ok(symbol) => std::mem::transmute(symbol.symbol),
//!         Err(_) => {
//!             let error = CString::new("Unable to load the `example_fn` function from the library.").unwrap();
//!             emf_cbase_sys_panic(error.as_ptr());
//!         }
//!     };
//!
//!     fn_symbol(5, 7);
//!
//!     let err = emf_cbase_library_unload(library_handle);
//!     match err.to_native() {
//!         Some(_) => {
//!             let error = CString::new("Unable to unload the `./example_lib.so` library.").unwrap();
//!             emf_cbase_sys_panic(error.as_ptr());
//!         },
//!         None => {}
//!     };
//!
//!     emf_cbase_sys_unlock();
//! }
//!
//! ```

use crate::containers::{MutSpan, NonNullConst, Optional, Result, StaticVec};
use crate::fn_ptr::BaseFn;
use crate::Bool;
#[allow(unused_imports)]
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::NonNull;

/// Max length of a library type name.
pub const LIBRARY_LOADER_TYPE_MAX_LENGTH: usize = 64;

/// Name of the native library type.
pub const NATIVE_LIBRARY_TYPE_NAME: &str = "emf::core_base::native";

/// Handle to the default library loader.
pub const LIBRARY_LOADER_DEFAULT_HANDLE: LoaderHandle = LoaderHandle {
    id: LibraryPredefinedHandles::Native as i32,
};

/// Character used by the os to represent a path.
#[cfg(target_os = "windows")]
pub type OsPathChar = u16;

/// Character used by the os to represent a path.
#[cfg(not(target_os = "windows"))]
pub type OsPathChar = c_char;

/// The handle to a library.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LibraryHandle {
    pub id: i32,
}

/// A data symbol contained in a library.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct DataSymbol {
    pub symbol: NonNull<c_void>,
}

/// A function symbol contained in a library.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct FnSymbol {
    pub symbol: BaseFn,
}

/// The handle to a library loader.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LoaderHandle {
    pub id: i32,
}

/// The internal handle to a library.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LoaderLibraryHandle {
    pub id: isize,
}

/// The type of a library.
///
/// A library type is modelled as an `UTF-8` encoded string, without a `\0` terminator.
pub type LibraryType = StaticVec<c_char, 64>;

/// An enum describing all predefined library loader handles.
///
/// The values `0-99` are reserved for future use.
#[repr(i32)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LibraryPredefinedHandles {
    Native = 0,
}

/// An enum describing all defined error values.
///
/// The values `0-99` are reserved for future use.
#[repr(i32)]
#[non_exhaustive]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum LibraryError {
    PathNotFound = 0,
    LibraryHandleInvalid = 1,
    LoaderHandleInvalid = 2,
    LoaderLibraryHandleInvalid = 3,
    LibraryTypeInvalid = 4,
    LibraryTypeNotFound = 5,
    DuplicatedLibraryType = 6,
    SymbolNotFound = 7,
    BufferOverflow = 8,
}

/// An opaque structure representing a library loader.
#[repr(C)]
pub struct LibraryLoader {
    _private: [u8; 0],
}

/// A function pointer to a `load` function.
///
/// The function loads the library, which is located at `library_path`, and returns its handle.
/// The function must be thread-safe.
pub type LoaderInterfaceLoadFn = extern "C" fn(
    library_loader: *mut LibraryLoader,
    library_path: NonNullConst<OsPathChar>,
) -> Result<LoaderLibraryHandle, LibraryError>;

/// A function pointer to a `unload` function.
///
/// The function unloads a library, that was loaded previously.
/// The function must be thread-safe.
pub type LoaderInterfaceUnloadFn = extern "C" fn(
    library_loader: *mut LibraryLoader,
    library_handle: LoaderLibraryHandle,
) -> Optional<LibraryError>;

/// A function pointer to a `get_data_symbol` function.
///
/// The function fetches a pointer to the symbol `symbol_name` from the library.
/// The function must be thread-safe.
pub type LoaderInterfaceGetDataSymbolFn = extern "C" fn(
    library_loader: *mut LibraryLoader,
    library_handle: LoaderLibraryHandle,
    symbol_name: NonNullConst<c_char>,
) -> Result<DataSymbol, LibraryError>;

/// A function pointer to a `get_function_symbol` function.
///
/// The function fetches a pointer to the function symbol `symbol_name` from the library.
/// The function must be thread-safe.
pub type LoaderInterfaceGetFunctionSymbolFn = extern "C" fn(
    library_loader: *mut LibraryLoader,
    library_handle: LoaderLibraryHandle,
    symbol_name: NonNullConst<c_char>,
) -> Result<FnSymbol, LibraryError>;

/// Interface of a library loader.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct LoaderInterface {
    pub library_loader: *mut LibraryLoader,
    pub load_fn: LoaderInterfaceLoadFn,
    pub unload_fn: LoaderInterfaceUnloadFn,
    pub get_data_symbol_fn: LoaderInterfaceGetDataSymbolFn,
    pub get_function_symbol_fn: LoaderInterfaceGetFunctionSymbolFn,
}

/// A function pointer to a `load_ext` function.
///
/// The function loads the library, which is located at `library_path`, and returns its handle.
/// Directly matches the win32 `LoadLibraryExW` function.
#[cfg(target_os = "windows")]
pub type NativeLoaderInterfaceLoadExtFn =
    extern "C" fn(
        library_loader: *mut LibraryLoader,
        library_path: NonNullConst<OsPathChar>,
        h_file: *mut c_void,
        flags: u32,
    ) -> Result<LoaderLibraryHandle, LibraryError>;

/// A function pointer to a `load_ext` function.
///
/// The function loads the library, which is located at `library_path`, and returns its handle.
/// Directly matches the posix `dlopen` function.
#[cfg(not(target_os = "windows"))]
pub type NativeLoaderInterfaceLoadExtFn =
    extern "C" fn(
        library_loader: *mut LibraryLoader,
        library_path: NonNullConst<OsPathChar>,
        flags: c_int,
    ) -> Result<LoaderLibraryHandle, LibraryError>;

/// Interface of a native library loader.
#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct NativeLoaderInterface {
    pub library_loader_interface: LoaderInterface,
    pub load_ext_fn: NativeLoaderInterfaceLoadExtFn,
}

extern "C" {

    /// Registers a new loader.
    ///
    /// The loader can load libraries of the type `library_type`.
    ///
    /// # Failure
    ///
    /// The function fails if the library type already exists.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_register_loader(
        loader_interface: NonNullConst<LoaderInterface>,
        library_type: NonNullConst<LibraryType>,
    ) -> Result<LoaderHandle, LibraryError>;

    /// Unregisters an existing loader.
    ///
    /// # Failure
    ///
    /// The function fails if `loader_handle` is invalid.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_unregister_loader(
        loader_handle: LoaderHandle,
    ) -> Optional<LibraryError>;

    /// Fetches the number of registered loaders.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_get_num_loaders() -> usize;

    /// Copies the strings of the registered library types into a buffer.
    ///
    /// Returns the number of copied elements.
    ///
    /// # Failure
    ///
    /// The function fails if `buffer` is smaller than [emf_cbase_library_get_num_loaders()].
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_get_library_types(
        buffer: NonNull<MutSpan<LibraryType>>,
    ) -> Result<usize, LibraryError>;

    /// Fetches the loader handle associated with the library type.
    ///
    /// # Failure
    ///
    /// The function fails if `library_type` is not registered.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_get_loader_handle(
        library_type: NonNullConst<LibraryType>,
    ) -> Result<LoaderHandle, LibraryError>;

    /// Checks if a library type exists.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_type_exists(library_type: NonNullConst<LibraryType>) -> Bool;

    /// Checks if a the library handle is valid.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_library_exists(library_handle: LibraryHandle) -> Bool;

    /// Creates a new unlinked library handle.
    ///
    /// # Warning
    ///
    /// The handle must be linked before use (See [emf_cbase_library_unsafe_link_library()]).
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_unsafe_create_library_handle() -> LibraryHandle;

    /// Removes an existing library handle.
    ///
    /// # Failure
    ///
    /// The function fails if `library_handle` is invalid.
    ///
    /// # Warning
    ///
    /// Removing the handle does not unload the library.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_unsafe_remove_library_handle(
        library_handle: LibraryHandle,
    ) -> Optional<LibraryError>;

    /// Links a library handle to an internal library handle.
    ///
    /// Overrides the internal link of the library handle by setting it to the
    /// new library loader and internal handle.
    ///
    /// # Failure
    ///
    /// The function fails if `library_handle` or `loader_handle` are invalid.
    ///
    /// # Warning
    ///
    /// Incorrect usage can lead to dangling handles or use-after-free errors.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_unsafe_link_library(
        library_handle: LibraryHandle,
        loader_handle: LoaderHandle,
        internal_handle: LoaderLibraryHandle,
    ) -> Optional<LibraryError>;

    /// Fetches the internal handle linked with the library handle.
    ///
    /// # Failure
    ///
    /// The function fails if `library_handle` is invalid.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_unsafe_get_loader_library_handle(
        library_handle: LibraryHandle,
    ) -> Result<LoaderLibraryHandle, LibraryError>;

    /// Fetches the loader handle linked with the library handle.
    ///
    /// # Failure
    ///
    /// The function fails if `library_handle` is invalid.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_unsafe_get_loader_handle(
        library_handle: LibraryHandle,
    ) -> Result<LoaderHandle, LibraryError>;

    /// Fetches the interface of a library loader.
    ///
    /// # Failure
    ///
    /// The function fails if `loader_handle` is invalid.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_unsafe_get_loader_interface(
        loader_handle: LoaderHandle,
    ) -> Result<NonNullConst<LoaderInterface>, LibraryError>;

    /// Loads a library.
    ///
    /// The resulting handle is unique.
    ///
    /// # Failure
    ///
    /// The function fails if `loader_handle` or `library_path` is invalid or the type
    /// of the library can not be loaded by the loader.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_load(
        loader_handle: LoaderHandle,
        library_path: NonNullConst<OsPathChar>,
    ) -> Result<LibraryHandle, LibraryError>;

    /// Unloads a library.
    ///
    /// # Failure
    ///
    /// The function fails if `library_handle` is invalid.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_unload(library_handle: LibraryHandle) -> Optional<LibraryError>;

    /// Fetches a data symbol from a library.
    ///
    /// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
    /// See [emf_cbase_library_get_function_symbol()] when fetching a function.
    ///
    /// # Failure
    ///
    /// The function fails if `library_handle` is invalid or library does not contain `symbol_name`.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_get_data_symbol(
        library_handle: LibraryHandle,
        symbol_name: NonNullConst<c_char>,
    ) -> Result<DataSymbol, LibraryError>;

    /// Fetches a data symbol from a library.
    ///
    /// Some platforms may differentiate between a `function-pointer` and a `data-pointer`.
    /// See [emf_cbase_library_get_data_symbol()] when fetching some data.
    ///
    /// # Failure
    ///
    /// The function fails if `library_handle` is invalid or library does not contain `symbol_name`.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_library_get_function_symbol(
        library_handle: LibraryHandle,
        symbol_name: NonNullConst<c_char>,
    ) -> Result<FnSymbol, LibraryError>;
}
