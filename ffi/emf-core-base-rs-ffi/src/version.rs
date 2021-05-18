//! Version api.
//!
//! # Example
//!
//! ```no_run
//! # use emf_core_base_rs_ffi::CBaseBinding;
//! # let base_interface: &mut dyn CBaseBinding = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
//! use emf_core_base_rs_ffi::sys::api::SysBinding;
//! use emf_core_base_rs_ffi::version::api::VersionBinding;
//! use emf_core_base_rs_ffi::collections::{NonNullConst, ConstSpan, Optional};
//!
//! unsafe {
//!     // `base_interface` has the type `&mut dyn CBaseBinding`.
//!     let v1 = VersionBinding::new_short(base_interface, 1, 2, 3);
//!
//!     let v2_string = ConstSpan::from("1.2.3-beta.5+54845652");
//!     let v2 = match VersionBinding::from_string(
//!                     base_interface,
//!                     NonNullConst::from(&v2_string)
//!                     ).into_rust() {
//!         Ok(v) => v,
//!         Err(e) => {
//!             SysBinding::lock(base_interface);
//!             SysBinding::panic(
//!                 base_interface,
//!                 Optional::Some(e)
//!             );
//!             SysBinding::unlock(base_interface);
//!         }
//!     };
//!
//!     if VersionBinding::compare_weak(
//!             base_interface,
//!             NonNullConst::from(&v1),
//!             NonNullConst::from(&v2)) != 0 {
//!         SysBinding::lock(base_interface);
//!         SysBinding::panic(
//!             base_interface,
//!             Optional::None
//!         );
//!         SysBinding::unlock(base_interface);
//!     }
//! }
//! ```

pub mod api;

/// Major version of the targeted version.
pub const VERSION_MAJOR: i32 = 0;

/// Minor version of the targeted version.
pub const VERSION_MINOR: i32 = 2;

/// Patch version of the targeted version.
pub const VERSION_PATCH: i32 = 0;

/// Release type of the targeted version.
pub const VERSION_RELEASE_TYPE: ReleaseType = ReleaseType::Unstable;

/// Release number of the targeted version.
pub const VERSION_RELEASE_NUMBER: i8 = 0;

/// Build number of the targeted version.
pub const VERSION_BUILD: i64 = 0;

/// Version string of the targeted version.
pub const VERSION_STRING: &str = "0.2.0-unstable.0";

/// Short version.
pub const VERSION: Version = Version {
    major: VERSION_MAJOR,
    minor: VERSION_MINOR,
    patch: VERSION_PATCH,
    build: VERSION_BUILD,
    release_number: VERSION_RELEASE_NUMBER,
    release_type: VERSION_RELEASE_TYPE,
};

/// Errors of the version api.
#[repr(i8)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum ReleaseType {
    Stable = 0,
    Unstable = 1,
    Beta = 2,
}

/// A version.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub build: i64,
    pub release_number: i8,
    pub release_type: ReleaseType,
}
