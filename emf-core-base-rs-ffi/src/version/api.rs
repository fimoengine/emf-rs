//! Version api.
//!
//! The version api is exposed by the [VersionBinding] trait.
use crate::collections::{ConstSpan, MutSpan, NonNullConst, Result};
use crate::version::{Error, ReleaseType, Version};
use crate::{Bool, CBase, CBaseInterface};
use std::ptr::NonNull;

pub type NewShortFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    major: i32,
    minor: i32,
    patch: i32,
) -> Version;

pub type NewLongFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    major: i32,
    minor: i32,
    patch: i32,
    release_type: ReleaseType,
    release_number: i8,
) -> Version;

pub type NewFullFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    major: i32,
    minor: i32,
    patch: i32,
    release_type: ReleaseType,
    release_number: i8,
    build: i64,
) -> Version;

pub type FromStringFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    buffer: NonNullConst<ConstSpan<u8>>,
) -> Result<Version, Error>;

pub type StringLengthShortFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    version: NonNullConst<Version>,
) -> usize;

pub type StringLengthLongFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    version: NonNullConst<Version>,
) -> usize;

pub type StringLengthFullFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    version: NonNullConst<Version>,
) -> usize;

pub type AsStringShortFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    version: NonNullConst<Version>,
    buffer: NonNull<MutSpan<u8>>,
) -> Result<usize, Error>;

pub type AsStringLongFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    version: NonNullConst<Version>,
    buffer: NonNull<MutSpan<u8>>,
) -> Result<usize, Error>;

pub type AsStringFullFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    version: NonNullConst<Version>,
    buffer: NonNull<MutSpan<u8>>,
) -> Result<usize, Error>;

pub type StringIsValidFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    version_string: NonNullConst<ConstSpan<u8>>,
) -> Bool;

pub type CompareFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    lhs: NonNullConst<Version>,
    rhs: NonNullConst<Version>,
) -> i32;

pub type CompareWeakFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    lhs: NonNullConst<Version>,
    rhs: NonNullConst<Version>,
) -> i32;

pub type CompareStrongFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    lhs: NonNullConst<Version>,
    rhs: NonNullConst<Version>,
) -> i32;

pub type IsCompatibleFn = unsafe extern "C" fn(
    base_module: Option<NonNull<CBase>>,
    lhs: NonNullConst<Version>,
    rhs: NonNullConst<Version>,
) -> Bool;

/// Helper trait for using the version api.
pub trait VersionBinding {
    /// Constructs a new version.
    ///
    /// Constructs a new version with `major`, `minor` and `patch` and sets the rest to `0`.
    ///
    /// # Return
    ///
    /// Constructed version.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn new_short(&self, major: i32, minor: i32, patch: i32) -> Version;

    /// Constructs a new version.
    ///
    /// Constructs a new version with `major`, `minor`, `patch`, `release_type` and
    /// `release_number` and sets the rest to `0`.
    ///
    /// # Return
    ///
    /// Constructed version.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn new_long(
        &self,
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
    ) -> Version;

    /// Constructs a new version.
    ///
    /// Constructs a new version with `major`, `minor`, `patch`, `release_type`,
    /// `release_number` and `build`.
    ///
    /// # Return
    ///
    /// Constructed version.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn new_full(
        &self,
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
    /// Fails if `string_is_valid(buffer) == Bool::False`.
    ///
    /// # Return
    ///
    /// Constructed version.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn from_string(&self, buffer: NonNullConst<ConstSpan<u8>>) -> Result<Version, Error>;

    /// Computes the length of the short version string.
    ///
    /// # Return
    ///
    /// Length of the string.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn string_length_short(&self, version: NonNullConst<Version>) -> usize;

    /// Computes the length of the long version string.
    ///
    /// # Return
    ///
    /// Length of the string.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn string_length_long(&self, version: NonNullConst<Version>) -> usize;

    /// Computes the length of the full version string.
    ///
    /// # Return
    ///
    /// Length of the string.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn string_length_full(&self, version: NonNullConst<Version>) -> usize;

    /// Represents the version as a short string.
    ///
    /// # Failure
    ///
    /// This function fails if `buffer.as_ref.len() < string_length_short(version)`.
    ///
    /// # Return
    ///
    /// Number of written characters on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn as_string_short(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<u8>>,
    ) -> Result<usize, Error>;

    /// Represents the version as a long string.
    ///
    /// # Failure
    ///
    /// This function fails if `buffer.as_ref.len() < string_length_long(version)`.
    ///
    /// # Return
    ///
    /// Number of written characters on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn as_string_long(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<u8>>,
    ) -> Result<usize, Error>;

    /// Represents the version as a full string.
    ///
    /// # Failure
    ///
    /// This function fails if `buffer.as_ref.len() < string_length_full(version)`.
    ///
    /// # Return
    ///
    /// Number of written characters on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn as_string_full(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<u8>>,
    ) -> Result<usize, Error>;

    /// Checks whether the version string is valid.
    ///
    /// # Return
    ///
    /// [Bool::True] if the string is valid, [Bool::False] otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn string_is_valid(&self, version_string: NonNullConst<ConstSpan<u8>>) -> Bool;

    /// Compares two versions.
    ///
    /// Compares two version, disregarding their build number.
    ///
    /// # Return
    ///
    /// - `-1` if `lhs` > `rhs`.
    /// - `0` if `lhs` == `rhs`.
    /// - `1` if `lhs` < `rhs`.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn compare(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> i32;

    /// Compares two versions.
    ///
    /// Compares two version, disregarding their build number and release type.
    ///
    /// # Return
    ///
    /// - `-1` if `lhs` > `rhs`.
    /// - `0` if `lhs` == `rhs`.
    /// - `1` if `lhs` < `rhs`.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn compare_weak(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> i32;

    /// Compares two versions.
    ///
    /// # Return
    ///
    /// - `-1` if `lhs` > `rhs`.
    /// - `0` if `lhs` == `rhs`.
    /// - `1` if `lhs` < `rhs`.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn compare_strong(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> i32;

    /// Checks for compatibility of two versions.
    ///
    /// Two compatible versions can be used interchangeably.
    ///
    /// # Note
    ///
    /// This function is not commutative.
    ///
    /// # Return
    ///
    /// [Bool::True] if the versions are compatible, [Bool::False] otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    unsafe fn is_compatible(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> Bool;
}

impl VersionBinding for CBaseInterface {
    #[inline]
    unsafe fn new_short(&self, major: i32, minor: i32, patch: i32) -> Version {
        (self.version_new_short_fn)(self.base_module, major, minor, patch)
    }

    #[inline]
    unsafe fn new_long(
        &self,
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
    ) -> Version {
        (self.version_new_long_fn)(
            self.base_module,
            major,
            minor,
            patch,
            release_type,
            release_number,
        )
    }

    #[inline]
    unsafe fn new_full(
        &self,
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
        build: i64,
    ) -> Version {
        (self.version_new_full_fn)(
            self.base_module,
            major,
            minor,
            patch,
            release_type,
            release_number,
            build,
        )
    }

    #[inline]
    unsafe fn from_string(&self, buffer: NonNullConst<ConstSpan<u8>>) -> Result<Version, Error> {
        (self.version_from_string_fn)(self.base_module, buffer)
    }

    #[inline]
    unsafe fn string_length_short(&self, version: NonNullConst<Version>) -> usize {
        (self.version_string_length_short_fn)(self.base_module, version)
    }

    #[inline]
    unsafe fn string_length_long(&self, version: NonNullConst<Version>) -> usize {
        (self.version_string_length_long_fn)(self.base_module, version)
    }

    #[inline]
    unsafe fn string_length_full(&self, version: NonNullConst<Version>) -> usize {
        (self.version_string_length_full_fn)(self.base_module, version)
    }

    #[inline]
    unsafe fn as_string_short(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<u8>>,
    ) -> Result<usize, Error> {
        (self.version_as_string_short_fn)(self.base_module, version, buffer)
    }

    #[inline]
    unsafe fn as_string_long(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<u8>>,
    ) -> Result<usize, Error> {
        (self.version_as_string_long_fn)(self.base_module, version, buffer)
    }

    #[inline]
    unsafe fn as_string_full(
        &self,
        version: NonNullConst<Version>,
        buffer: NonNull<MutSpan<u8>>,
    ) -> Result<usize, Error> {
        (self.version_as_string_full_fn)(self.base_module, version, buffer)
    }

    #[inline]
    unsafe fn string_is_valid(&self, version_string: NonNullConst<ConstSpan<u8>>) -> Bool {
        (self.version_string_is_valid_fn)(self.base_module, version_string)
    }

    #[inline]
    unsafe fn compare(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> i32 {
        (self.version_compare_fn)(self.base_module, lhs, rhs)
    }

    #[inline]
    unsafe fn compare_weak(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> i32 {
        (self.version_compare_weak_fn)(self.base_module, lhs, rhs)
    }

    #[inline]
    unsafe fn compare_strong(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> i32 {
        (self.version_compare_strong_fn)(self.base_module, lhs, rhs)
    }

    #[inline]
    unsafe fn is_compatible(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> Bool {
        (self.version_is_compatible_fn)(self.base_module, lhs, rhs)
    }
}
