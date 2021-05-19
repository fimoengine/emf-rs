//! Library api.
//!
//! # Example
//!
//! ```no_run
//! # use emf_core_base_rs_ffi::CBaseBinding;
//! # let base_interface: &mut dyn CBaseBinding = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
//! use emf_core_base_rs_ffi::sys::api::SysBinding;
//! use emf_core_base_rs_ffi::library::api::LibraryBinding;
//! use emf_core_base_rs_ffi::collections::{NonNullConst, Optional};
//! use emf_core_base_rs_ffi::library::{OSPathChar, DEFAULT_HANDLE, LibraryHandle};
//!
//! unsafe {
//!     // `base_interface` has the type `&mut dyn CBaseBinding`.
//!     SysBinding::lock(base_interface);
//!
//!     // Path of the library. Platform dependent initialisation.
//!     let lib_path: &OSPathChar = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
//!
//!     let handle = match LibraryBinding::load(
//!             base_interface,
//!             DEFAULT_HANDLE,
//!             NonNullConst::from(lib_path)
//!             ).into_rust() {
//!         Ok(handle) => handle,
//!         Err(e) => {
//!             SysBinding::panic(base_interface, Optional::Some(e));
//!         }
//!     };
//!
//!     let symbol: unsafe extern "C" fn(i32, i32) -> i32 =
//!         match LibraryBinding::get_function_symbol(
//!             base_interface,
//!             handle,
//!             NonNullConst::from(b"add_fn\0")
//!             ).into_rust() {
//!         Ok(sym) => {
//!             std::mem::transmute(sym.symbol)
//!         }
//!         Err(e) => {
//!             SysBinding::panic(base_interface, Optional::Some(e));
//!         }
//!     };
//!
//!     assert_eq!(symbol(3, 5), 8);
//!
//!     match LibraryBinding::unload(base_interface, handle).into_rust() {
//!         Ok(_) => {}
//!         Err(e) => {
//!             SysBinding::panic(base_interface, Optional::Some(e));
//!         }
//!     }
//!
//!     SysBinding::unlock(base_interface);
//! }
//! ```
use crate::collections::StaticVec;
use std::fmt::{Display, Formatter};

pub mod api;
pub mod library_loader;

/// Max length of a library type.
pub const LOADER_TYPE_MAX_LENGTH: usize = 64;

/// Name of the native library type.
pub const NATIVE_LIBRARY_TYPE_NAME: &str = "emf::core_base::native";

/// Handle of the native library loader.
pub const DEFAULT_HANDLE: LoaderHandle = LoaderHandle {
    id: PredefinedHandles::Native as i32,
};

/// Predefined loader handles.
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PredefinedHandles {
    Native = 0,
}

impl Display for PredefinedHandles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PredefinedHandles::Native => write!(f, "Native"),
        }
    }
}

/// Character type of a windows path.
pub type OSPathCharWindows = u16;

/// Character type of a unix path.
pub type OSPathCharUnix = u8;

/// Character type of a path.
#[cfg(unix)]
pub type OSPathChar = OSPathCharUnix;

/// Character type of a path.
#[cfg(windows)]
pub type OSPathChar = OSPathCharWindows;

/// Handle to a library.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LibraryHandle {
    pub id: i32,
}

impl Display for LibraryHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

/// Handle to a loader.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LoaderHandle {
    pub id: i32,
}

impl Display for LoaderHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

/// Internal handle to a library.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InternalHandle {
    pub id: isize,
}

impl Display for InternalHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

/// A symbol from a library.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Symbol<T> {
    pub symbol: T,
}

impl<T> Display for Symbol<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.symbol)
    }
}

/// Library type.
pub type LibraryType = StaticVec<u8, LOADER_TYPE_MAX_LENGTH>;
