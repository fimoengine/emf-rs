//! Version api.
//!
//! The version api is exposed by the [VersionAPI] trait.
use crate::ffi::collections::{ConstSpan, MutSpan, NonNullConst};
use crate::ffi::version::api::VersionBinding;
use crate::ffi::Bool;
use std::cmp::Ordering;
use std::ptr::NonNull;

pub use crate::ffi::version::{Error, ReleaseType, Version};

/// Trait for providing access to the version api.
pub trait VersionAPI {
    /// Constructs a new version.
    ///
    /// Constructs a new version with `major`, `minor` and `patch` and sets the rest to `0`.
    ///
    /// # Return
    ///
    /// Constructed version.
    fn new_short(&self, major: i32, minor: i32, patch: i32) -> Version;

    /// Constructs a new version.
    ///
    /// Constructs a new version with `major`, `minor`, `patch`, `release_type` and
    /// `release_number` and sets the rest to `0`.
    ///
    /// # Return
    ///
    /// Constructed version.
    fn new_long(
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
    fn new_full(
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
    /// Fails if `string_is_valid(buffer) == false`.
    ///
    /// # Return
    ///
    /// Constructed version.
    fn from_string(&self, buffer: impl AsRef<str>) -> Result<Version, Error>;

    /// Computes the length of the short version string.
    ///
    /// # Return
    ///
    /// Length of the string.
    fn string_length_short(&self, version: &Version) -> usize;

    /// Computes the length of the long version string.
    ///
    /// # Return
    ///
    /// Length of the string.
    fn string_length_long(&self, version: &Version) -> usize;

    /// Computes the length of the full version string.
    ///
    /// # Return
    ///
    /// Length of the string.
    fn string_length_full(&self, version: &Version) -> usize;

    /// Represents the version as a short string.
    ///
    /// # Failure
    ///
    /// This function fails if `buffer.len() < string_length_short(version)`.
    ///
    /// # Return
    ///
    /// Number of written characters on success, error otherwise.
    fn as_string_short(&self, version: &Version, buffer: impl AsMut<str>) -> Result<usize, Error>;

    /// Represents the version as a long string.
    ///
    /// # Failure
    ///
    /// This function fails if `buffer.len() < string_length_long(version)`.
    ///
    /// # Return
    ///
    /// Number of written characters on success, error otherwise.
    fn as_string_long(&self, version: &Version, buffer: impl AsMut<str>) -> Result<usize, Error>;

    /// Represents the version as a full string.
    ///
    /// # Failure
    ///
    /// This function fails if `buffer.len() < string_length_full(version)`.
    ///
    /// # Return
    ///
    /// Number of written characters on success, error otherwise.
    fn as_string_full(&self, version: &Version, buffer: impl AsMut<str>) -> Result<usize, Error>;

    /// Checks whether the version string is valid.
    ///
    /// # Return
    ///
    /// [true] if the string is valid, [false] otherwise.
    fn string_is_valid(&self, version_string: impl AsRef<str>) -> bool;

    /// Compares two versions.
    ///
    /// Compares two version, disregarding their build number.
    ///
    /// # Return
    ///
    /// Order of the versions.
    fn compare(&self, lhs: &Version, rhs: &Version) -> Ordering;

    /// Compares two versions.
    ///
    /// Compares two version, disregarding their build number and release type.
    ///
    /// # Return
    ///
    /// Order of the versions.
    fn compare_weak(&self, lhs: &Version, rhs: &Version) -> Ordering;

    /// Compares two versions.
    ///
    /// # Return
    ///
    /// Order of the versions.
    fn compare_strong(&self, lhs: &Version, rhs: &Version) -> Ordering;

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
    /// [true] if the versions are compatible, [false] otherwise.
    fn is_compatible(&self, lhs: &Version, rhs: &Version) -> bool;
}

impl<T> VersionAPI for T
where
    T: VersionBinding,
{
    #[inline]
    fn new_short(&self, major: i32, minor: i32, patch: i32) -> Version {
        unsafe { <T as VersionBinding>::new_short(self, major, minor, patch) }
    }

    #[inline]
    fn new_long(
        &self,
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
    ) -> Version {
        unsafe {
            <T as VersionBinding>::new_long(self, major, minor, patch, release_type, release_number)
        }
    }

    #[inline]
    fn new_full(
        &self,
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
        build: i64,
    ) -> Version {
        unsafe {
            <T as VersionBinding>::new_full(
                self,
                major,
                minor,
                patch,
                release_type,
                release_number,
                build,
            )
        }
    }

    #[inline]
    fn from_string(&self, buffer: impl AsRef<str>) -> Result<Version, Error> {
        unsafe {
            <T as VersionBinding>::from_string(
                self,
                NonNullConst::from(&ConstSpan::from(buffer.as_ref())),
            )
            .to_result()
        }
    }

    #[inline]
    fn string_length_short(&self, version: &Version) -> usize {
        unsafe { <T as VersionBinding>::string_length_short(self, NonNullConst::from(version)) }
    }

    #[inline]
    fn string_length_long(&self, version: &Version) -> usize {
        unsafe { <T as VersionBinding>::string_length_long(self, NonNullConst::from(version)) }
    }

    #[inline]
    fn string_length_full(&self, version: &Version) -> usize {
        unsafe { <T as VersionBinding>::string_length_full(self, NonNullConst::from(version)) }
    }

    #[inline]
    fn as_string_short(
        &self,
        version: &Version,
        mut buffer: impl AsMut<str>,
    ) -> Result<usize, Error> {
        unsafe {
            <T as VersionBinding>::as_string_short(
                self,
                NonNullConst::from(version),
                NonNull::from(&MutSpan::from(buffer.as_mut())),
            )
            .to_result()
        }
    }

    #[inline]
    fn as_string_long(
        &self,
        version: &Version,
        mut buffer: impl AsMut<str>,
    ) -> Result<usize, Error> {
        unsafe {
            <T as VersionBinding>::as_string_long(
                self,
                NonNullConst::from(version),
                NonNull::from(&MutSpan::from(buffer.as_mut())),
            )
            .to_result()
        }
    }

    #[inline]
    fn as_string_full(
        &self,
        version: &Version,
        mut buffer: impl AsMut<str>,
    ) -> Result<usize, Error> {
        unsafe {
            <T as VersionBinding>::as_string_full(
                self,
                NonNullConst::from(version),
                NonNull::from(&MutSpan::from(buffer.as_mut())),
            )
            .to_result()
        }
    }

    #[inline]
    fn string_is_valid(&self, version_string: impl AsRef<str>) -> bool {
        unsafe {
            <T as VersionBinding>::string_is_valid(
                self,
                NonNullConst::from(&ConstSpan::from(version_string.as_ref())),
            ) == Bool::True
        }
    }

    #[inline]
    fn compare(&self, lhs: &Version, rhs: &Version) -> Ordering {
        unsafe {
            match <T as VersionBinding>::compare(
                self,
                NonNullConst::from(lhs),
                NonNullConst::from(rhs),
            ) {
                -1 => Ordering::Greater,
                0 => Ordering::Equal,
                1 => Ordering::Less,
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn compare_weak(&self, lhs: &Version, rhs: &Version) -> Ordering {
        unsafe {
            match <T as VersionBinding>::compare_weak(
                self,
                NonNullConst::from(lhs),
                NonNullConst::from(rhs),
            ) {
                -1 => Ordering::Greater,
                0 => Ordering::Equal,
                1 => Ordering::Less,
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn compare_strong(&self, lhs: &Version, rhs: &Version) -> Ordering {
        unsafe {
            match <T as VersionBinding>::compare_strong(
                self,
                NonNullConst::from(lhs),
                NonNullConst::from(rhs),
            ) {
                -1 => Ordering::Greater,
                0 => Ordering::Equal,
                1 => Ordering::Less,
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn is_compatible(&self, lhs: &Version, rhs: &Version) -> bool {
        unsafe {
            <T as VersionBinding>::is_compatible(
                self,
                NonNullConst::from(lhs),
                NonNullConst::from(rhs),
            ) == Bool::True
        }
    }
}
