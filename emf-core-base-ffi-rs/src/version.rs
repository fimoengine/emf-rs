//! The `version` api implements the versioning scheme as specified in the
//! [conventions](https://fimoengine.github.io/emf/conventions.html#versions).
//!
//! # Example
//!
//! ```no_run
//! use emf_core_base_ffi_rs::version::{
//!     emf_cbase_version_construct_short, emf_cbase_version_construct_from_string,
//!     emf_cbase_version_compare_weak
//! };
//! use emf_core_base_ffi_rs::containers::Span;
//! use std::os::raw::c_char;
//! use emf_core_base_ffi_rs::sys::{
//!     emf_cbase_sys_lock, emf_cbase_sys_panic, emf_cbase_sys_unlock
//! };
//! use std::ffi::CString;
//!
//! unsafe {
//!     let v1 = emf_cbase_version_construct_short(1, 2, 3);
//!
//!     let v2_str = "1.2.3-rc.5+54845652";
//!     let v2_buff = Span::from(v2_str).as_c_char_span();
//!     let v2_res = emf_cbase_version_construct_from_string(&v2_buff);
//!
//!     let v2 = match v2_res.to_native() {
//!         Ok(v2) => v2,
//!         Err(_) => {
//!             let error = CString::new("Could not construct version from string.").unwrap();
//!             emf_cbase_sys_lock();
//!             emf_cbase_sys_panic(error.as_ptr());
//!             emf_cbase_sys_unlock();
//!         }
//!     };
//!
//!     assert_eq!(emf_cbase_version_compare_weak(&v1, &v2), 0);
//! }
//! ```

use crate::containers::{MutSpan, Result, Span};
use crate::Bool;
use std::os::raw::c_char;
use std::ptr::NonNull;

/// An enum describing the release type of a version.
#[repr(C)]
#[derive(Copy, Clone)]
pub enum ReleaseType {
    Gold = 0,
    PreAlpha = 1,
    Alpha = 2,
    Beta = 3,
    RC = 4,
}

/// A version.
///
/// # Invariants
///
/// If `release_type == ReleaseType::Gold` then `release_number == 0`.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Version {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub build_number: i64,
    pub release_number: i8,
    pub release_type: ReleaseType,
}

/// An enum describing the possible error values of the `version` api.
///
/// The values `0-99` are reserved for future use.
#[repr(C)]
#[derive(Copy, Clone)]
pub enum VersionError {
    InvalidString = 0,
    BufferOverflow = 1,
}

extern "C" {

    /// Constructs a new version.
    ///
    /// Constructs a new version with `major`, `minor` and `patch` and sets the rest to `0`.
    #[must_use]
    pub fn emf_cbase_version_construct_short(major: i32, minor: i32, patch: i32) -> Version;

    /// Constructs a new version.
    ///
    /// Constructs a new version with `major`, `minor`, `patch`, `release_type` and `release_number`
    /// and sets the rest to `0`.
    #[must_use]
    pub fn emf_cbase_version_construct_long(
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
    ) -> Version;

    /// Constructs a new version.
    ///
    /// Constructs a new version with `major`, `minor`, `patch`, `release_type`, `release_number`
    /// and `build`.
    #[must_use]
    pub fn emf_cbase_version_construct_full(
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
        build: i64,
    ) -> Version;

    /// Constructs a version from a string.
    ///
    /// # Failure
    ///
    /// The function fails if `version_string` is not of the form
    /// `"Major.Minor.Patch(-((pre-)?alpha|beta|rc).Release)?\+Build"`.
    ///
    /// # Undefined Behaviour
    ///
    /// The caller must ensure that the following preconditions hold:
    ///
    /// ```no_run
    /// # use emf_core_base_ffi_rs::containers::Span;
    /// # use std::os::raw::c_char;
    /// # use std::ptr::null;
    /// # let version_string: *const Span<'static, c_char> = null();
    /// assert_ne!(
    ///     version_string,
    ///     null()
    /// );
    /// assert_ne!(
    ///     unsafe { (*version_string).is_empty() },
    ///     true
    /// );
    /// ```
    #[must_use]
    pub fn emf_cbase_version_construct_from_string(
        version_string: *const Span<'_, c_char>,
    ) -> Result<Version, VersionError>;

    /// Checks weather the version string is valid.
    ///
    /// The string is valid if it has the form
    /// `"Major.Minor.Patch(-((pre-)?alpha|beta|rc).Release)?\+Build"`.
    ///
    /// # Undefined Behaviour
    ///
    /// The caller must ensure that the following preconditions hold:
    ///
    /// ```no_run
    /// # use emf_core_base_ffi_rs::containers::Span;
    /// # use std::os::raw::c_char;
    /// # use std::ptr::null;
    /// # let version_string: *const Span<'static, c_char> = null();
    /// assert_ne!(
    ///     version_string,
    ///     null()
    /// );
    /// assert_ne!(
    ///     unsafe { (*version_string).is_empty() },
    ///     true
    /// );
    /// ```
    #[must_use]
    pub fn emf_cbase_version_representation_is_valid(
        version_string: *const Span<'_, c_char>,
    ) -> Bool;

    /// Represents the version as a string.
    ///
    /// The string has the form `"Major.Minor.Patch"`.
    ///
    /// # Failure
    ///
    /// The function can fail if `buffer` is too small.
    ///
    /// # Undefined Behaviour
    ///
    /// The caller must ensure that the following preconditions hold:
    ///
    /// ```no_run
    /// # use emf_core_base_ffi_rs::version::Version;
    /// # use std::ptr::null;
    /// # let version: *const Version = null();
    /// assert_ne!(
    ///     version,
    ///     null()
    /// );
    /// ```
    #[must_use]
    pub fn emf_cbase_version_get_short_representation(
        version: *const Version,
        buffer: NonNull<MutSpan<'_, c_char>>,
    ) -> Result<usize, VersionError>;

    /// Computes the length of the version string.
    ///
    /// Computes the minimum length a string of the form `"Major.Minor.Patch"` needs.
    ///
    /// # Undefined Behaviour
    ///
    /// The caller must ensure that the following preconditions hold:
    ///
    /// ```no_run
    /// # use emf_core_base_ffi_rs::version::Version;
    /// # use std::ptr::null;
    /// # let version: *const Version = null();
    /// assert_ne!(
    ///     version,
    ///     null()
    /// );
    /// ```
    #[must_use]
    pub fn emf_cbase_version_get_short_representation_length(version: *const Version) -> usize;

    /// Represents the version as a string.
    ///
    /// The string has the form `"Major.Minor.Patch(-((pre-)?alpha|beta|rc).Release)?"`.
    ///
    /// # Failure
    ///
    /// The function can fail if `buffer` is too small.
    ///
    /// # Undefined Behaviour
    ///
    /// The caller must ensure that the following preconditions hold:
    ///
    /// ```no_run
    /// # use emf_core_base_ffi_rs::version::Version;
    /// # use std::ptr::null;
    /// # let version: *const Version = null();
    /// assert_ne!(
    ///     version,
    ///     null()
    /// );
    /// ```
    #[must_use]
    pub fn emf_cbase_version_get_long_representation(
        version: *const Version,
        buffer: NonNull<MutSpan<'_, c_char>>,
    ) -> Result<usize, VersionError>;

    /// Computes the length of the version string.
    ///
    /// Computes the minimum length a string of the form
    /// `"Major.Minor.Patch(-((pre-)?alpha|beta|rc).Release)?"` needs.
    ///
    /// # Undefined Behaviour
    ///
    /// The caller must ensure that the following preconditions hold:
    ///
    /// ```no_run
    /// # use emf_core_base_ffi_rs::version::Version;
    /// # use std::ptr::null;
    /// # let version: *const Version = null();
    /// assert_ne!(
    ///     version,
    ///     null()
    /// );
    /// ```
    #[must_use]
    pub fn emf_cbase_version_get_long_representation_length(version: *const Version) -> usize;

    /// Represents the version as a string.
    ///
    /// The string has the form `"Major.Minor.Patch(-((pre-)?alpha|beta|rc).Release)?\+Build"`.
    ///
    /// # Failure
    ///
    /// The function can fail if `buffer` is too small.
    ///
    /// # Undefined Behaviour
    ///
    /// The caller must ensure that the following preconditions hold:
    ///
    /// ```no_run
    /// # use emf_core_base_ffi_rs::version::Version;
    /// # use std::ptr::null;
    /// # let version: *const Version = null();
    /// assert_ne!(
    ///     version,
    ///     null()
    /// );
    /// ```
    #[must_use]
    pub fn emf_cbase_version_get_full_representation(
        version: *const Version,
        buffer: NonNull<MutSpan<'_, c_char>>,
    ) -> Result<usize, VersionError>;

    /// Computes the length of the version string.
    ///
    /// Computes the minimum length a string of the form
    /// `"Major.Minor.Patch(-((pre-)?alpha|beta|rc).Release)?\+Build"` needs.
    ///
    /// # Undefined Behaviour
    ///
    /// The caller must ensure that the following preconditions hold:
    ///
    /// ```no_run
    /// # use emf_core_base_ffi_rs::version::Version;
    /// # use std::ptr::null;
    /// # let version: *const Version = null();
    /// assert_ne!(
    ///     version,
    ///     null()
    /// );
    /// ```
    #[must_use]
    pub fn emf_cbase_version_get_full_representation_length(version: *const Version) -> usize;

    /// Compares two versions.
    ///
    /// Compares two versions using their major-,minor- and patch version, release type
    /// and release number.
    ///
    /// # Return value
    ///
    /// Returns `-1` if `lhs > rhs`.
    /// Returns `0` if `lhs == rhs`.
    /// Returns `1` if `lhs < rhs`.
    ///
    /// # Undefined Behaviour
    ///
    /// The caller must ensure that the following preconditions hold:
    ///
    /// ```no_run
    /// # use emf_core_base_ffi_rs::version::Version;
    /// # use std::ptr::null;
    /// # let lhs: *const Version = null();
    /// # let rhs: *const Version = null();
    /// assert_ne!(
    ///     lhs,
    ///     null()
    /// );
    /// assert_ne!(
    ///     rhs,
    ///     null()
    /// );
    /// ```
    #[must_use]
    pub fn emf_cbase_version_compare(lhs: *const Version, rhs: *const Version) -> i32;

    /// Compares two versions.
    ///
    /// Compares two versions using their major-,minor- and patch version.
    ///
    /// # Return value
    ///
    /// Returns `-1` if `lhs > rhs`.
    /// Returns `0` if `lhs == rhs`.
    /// Returns `1` if `lhs < rhs`.
    ///
    /// # Undefined Behaviour
    ///
    /// The caller must ensure that the following preconditions hold:
    ///
    /// ```no_run
    /// # use emf_core_base_ffi_rs::version::Version;
    /// # use std::ptr::null;
    /// # let lhs: *const Version = null();
    /// # let rhs: *const Version = null();
    /// assert_ne!(
    ///     lhs,
    ///     null()
    /// );
    /// assert_ne!(
    ///     rhs,
    ///     null()
    /// );
    /// ```
    #[must_use]
    pub fn emf_cbase_version_compare_weak(lhs: *const Version, rhs: *const Version) -> i32;

    /// Compares two versions.
    ///
    /// Compares two versions using their major-,minor- and patch version, release type,
    /// release number and build number.
    ///
    /// # Return value
    ///
    /// Returns `-1` if `lhs > rhs`.
    /// Returns `0` if `lhs == rhs`.
    /// Returns `1` if `lhs < rhs`.
    ///
    /// # Undefined Behaviour
    ///
    /// The caller must ensure that the following preconditions hold:
    ///
    /// ```no_run
    /// # use emf_core_base_ffi_rs::version::Version;
    /// # use std::ptr::null;
    /// # let lhs: *const Version = null();
    /// # let rhs: *const Version = null();
    /// assert_ne!(
    ///     lhs,
    ///     null()
    /// );
    /// assert_ne!(
    ///     rhs,
    ///     null()
    /// );
    /// ```
    #[must_use]
    pub fn emf_cbase_version_compare_strong(lhs: *const Version, rhs: *const Version) -> i32;

    /// Compares weather two versions are compatible.
    ///
    /// Compatibility of versions is not commutative.
    ///
    /// # Return value
    ///
    /// Returns [Bool::False] if `lhs` and `rhs` are incompatible.
    /// Returns [Bool::True] if `lhs` is compatible with `rhs`.
    ///
    /// # Undefined Behaviour
    ///
    /// The caller must ensure that the following preconditions hold:
    ///
    /// ```no_run
    /// # use emf_core_base_ffi_rs::version::Version;
    /// # use std::ptr::null;
    /// # let lhs: *const Version = null();
    /// # let rhs: *const Version = null();
    /// assert_ne!(
    ///     lhs,
    ///     null()
    /// );
    /// assert_ne!(
    ///     rhs,
    ///     null()
    /// );
    /// ```
    #[must_use]
    pub fn emf_cbase_version_is_compatible(lhs: *const Version, rhs: *const Version) -> Bool;
}
