//! Utilities for loading libraries.
//!
//! The `library` api is a collection of procedures that provide a platform agnostic interface
//! to loading shared libraries. The actual loading of a library is handled by a [LibraryLoader].
//! Each [LibraryLoader] is associated to a [LibraryType].
//!
//! ## Loaders
//!
//! The job of a [LibraryLoader] is to manage the loading and unloading of libraries.
//!
//! ## Library types
//!
//! The `library` api allows the loading of custom library formats.
//! Each format is identified by a [LibraryType] and is associated to exactly one [LibraryLoader].
//!
//! # Predefined Loaders
//!
//! Some loaders are always present and can not be removed at runtime.
//!
//! ## Native
//!
//! The [DEFAULT_LIBRARY_LOADER] is able to load platform-specific libraries (e.g. dlopen()/LoadLibrary()).
//! It is associated to the [DEFAULT_LIBRARY_TYPE_NAME] [LibraryType].
//!
//! On posix the [DEFAULT_LIBRARY_LOADER] defaults to the `RTLD_LAZY` and `RTLD_LOCAL` flags.
//! More control of how to load a library can be achieved by fetching a pointer to the interface with
//! [LibraryToken::get_loader_interface()], casting it to a [NativeLibraryLoader] and
//! calling the [NativeLibraryLoaderWrapper::load_ext()] function.
//! If the library has dependencies on other libraries, then it must specify them by specifying an `rpath` on posix or
//! embedding an `Activation Context manifest` into the library on Windows.
//!
//! # Example
//!
//! ```no_run
//! # use std::ffi::{CString, CStr};
//! # use emf_core_base_rs_bare::sys::SysToken;
//! # use emf_core_base_rs_bare::{ffi, GlobalToken, FFIObject};
//! # use emf_core_base_rs_bare::library::{LibrarySymbol, LibraryToken, DEFAULT_LIBRARY_LOADER};
//! # use emf_core_base_rs_ffi_bare::library::FnSymbol;
//! #
//! # struct FnWrapper {
//! #    func: unsafe extern "C" fn(i32, i32) -> i32
//! # }
//! #
//! # impl FnWrapper {
//! #    pub fn call(&self, _:i32,_:i32) -> i32 { 0 }
//! # }
//! #
//! # impl FFIObject<ffi::library::FnSymbol> for FnWrapper {
//! #   fn as_native(&self) -> FnSymbol {
//! #       unimplemented!()
//! #   }
//! #
//! #   unsafe fn from_native(val: FnSymbol) -> Self {
//! #       unimplemented!()
//! #   }
//! #
//! # }
//! let token = GlobalToken::new();
//! let lib_handle = match LibraryToken::load(&token, &DEFAULT_LIBRARY_LOADER, &"./example_lib.so") {
//!     Ok(handle) => handle,
//!     Err(_) => {
//!         let error = CString::new("Could not load the library.").unwrap();
//!         SysToken::panic(&token, Some(&error));
//!     }
//! };
//!
//! let symbol_name = CStr::from_bytes_with_nul(b"example_fn\0".as_ref()).unwrap();
//! let symbol: LibrarySymbol<'_, FnWrapper> = match LibraryToken::get_function_symbol(
//!     &token,
//!     &lib_handle,
//!     &symbol_name
//! ) {
//!     Ok(sym) => sym,
//!     Err(_) => {
//!         let error = CString::new("Could not load the `example_fn` symbol.").unwrap();
//!         SysToken::panic(&token, Some(&error));
//!     }
//! };
//!
//! // Use the symbol by calling `as_ref` or `as_mut`
//! let res = symbol.as_ref().call(5, 7);
//!
//! match LibraryToken::unload(&token, lib_handle) {
//!     None => {},
//!     Some(_) => {
//!         let error = CString::new("Could not unload the library.").unwrap();
//!         SysToken::panic(&token, Some(&error));
//!     }
//! };
//! ```

use crate::ffi;
use std::ffi::OsStr;
#[cfg(unix)]
use std::os::raw::c_char;

#[cfg(feature = "global_api")]
mod global_token;
mod library_handle;
mod library_loader;
mod library_symbol;
mod library_token;
mod loader_handle;
mod loader_library_handle;
mod local_token;

pub use library_handle::LibraryHandle;
pub use library_loader::{
    LibraryLoader, LibraryLoaderWrapper, NativeLibraryLoader, NativeLibraryLoaderWrapper,
};
pub use library_symbol::LibrarySymbol;
pub use library_token::LibraryToken;
pub use loader_handle::{LibraryLoaderHandle, DEFAULT_LIBRARY_LOADER};
pub use loader_library_handle::LoaderLibraryHandle;

pub use ffi::library::LibraryError;
pub use ffi::library::LibraryType;
pub use ffi::library::NATIVE_LIBRARY_TYPE_NAME as DEFAULT_LIBRARY_TYPE_NAME;

#[cfg(windows)]
pub fn os_str_to_native_buff(path: &OsStr) -> Vec<u16> {
    use std::os::windows::prelude::*;
    let mut vec: Vec<u16> = path.encode_wide().collect();
    vec.push(0u16);
    vec
}

#[cfg(unix)]
pub fn os_str_to_native_buff(path: &OsStr) -> Vec<c_char> {
    use std::os::unix::prelude::*;
    let bytes = path.as_bytes();
    let mut vec: Vec<c_char> = Vec::from(unsafe {
        std::slice::from_raw_parts(bytes.as_ptr() as *const c_char, bytes.len())
    });
    vec.push(0);
    vec
}
