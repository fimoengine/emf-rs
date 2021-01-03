//! Utilities for managing versions.
//!
//! # Example
//!
//! ```no_run
//! # use emf_core_base_rs_bare::version::{Version, VersionImplGlobal};
//! # use emf_core_base_rs_bare::sys::{GlobalSysToken, SysToken};
//! # use std::ffi::CString;
//! # use std::cmp::Ordering;
//! let v1 = Version::new(1,2,3);
//! let v2 = match Version::from_string(&"1.2.3-rc.5+54845652") {
//!     Ok(v) => v,
//!     Err(_) => {
//!         let error = CString::new("Could not construct version from string.").unwrap();
//!         GlobalSysToken::new().panic(Some(&error));
//!     }
//! };
//!
//! assert_eq!(v1.compare_weak(&v2), Ordering::Equal)
//! ```

use crate::ffi;

#[cfg(feature = "global_api")]
mod version_impl_global;
mod version_impl_local;

#[cfg(feature = "global_api")]
pub use version_impl_global::VersionImplGlobal;
pub use version_impl_local::VersionImplLocal;

/// An enum describing the release type of a version.
pub type ReleaseType = ffi::version::ReleaseType;

/// An enum describing the possible error values of the `version` api.
pub type VersionError = ffi::version::VersionError;

/// A version.
pub type Version = ffi::version::Version;
