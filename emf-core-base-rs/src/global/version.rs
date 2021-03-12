//! Global version api.
use crate::ffi::version::Version;
use crate::global::get_interface;
use crate::version::{Error, ReleaseType, VersionAPI};
use std::cmp::Ordering;

/// Constructs a new version.
///
/// Constructs a new version with `major`, `minor` and `patch` and sets the rest to `0`.
///
/// # Return
///
/// Constructed version.
#[inline]
pub fn new_short(major: i32, minor: i32, patch: i32) -> Version {
    VersionAPI::new_short(get_interface(), major, minor, patch)
}

/// Constructs a new version.
///
/// Constructs a new version with `major`, `minor`, `patch`, `release_type` and
/// `release_number` and sets the rest to `0`.
///
/// # Return
///
/// Constructed version.
#[inline]
pub fn new_long(
    major: i32,
    minor: i32,
    patch: i32,
    release_type: ReleaseType,
    release_number: i8,
) -> Version {
    VersionAPI::new_long(
        get_interface(),
        major,
        minor,
        patch,
        release_type,
        release_number,
    )
}

/// Constructs a new version.
///
/// Constructs a new version with `major`, `minor`, `patch`, `release_type`,
/// `release_number` and `build`.
///
/// # Return
///
/// Constructed version.
#[inline]
pub fn new_full(
    major: i32,
    minor: i32,
    patch: i32,
    release_type: ReleaseType,
    release_number: i8,
    build: i64,
) -> Version {
    VersionAPI::new_full(
        get_interface(),
        major,
        minor,
        patch,
        release_type,
        release_number,
        build,
    )
}

/// Constructs a version from a string.
///
/// # Failure
///
/// Fails if `string_is_valid(buffer) == false`.
///
/// # Return
///
/// Constructed version.
#[inline]
pub fn from_string(buffer: impl AsRef<str>) -> Result<Version, Error> {
    VersionAPI::from_string(get_interface(), buffer)
}

/// Computes the length of the short version string.
///
/// # Return
///
/// Length of the string.
#[inline]
pub fn string_length_short(version: &Version) -> usize {
    VersionAPI::string_length_short(get_interface(), version)
}

/// Computes the length of the long version string.
///
/// # Return
///
/// Length of the string.
#[inline]
pub fn string_length_long(version: &Version) -> usize {
    VersionAPI::string_length_long(get_interface(), version)
}

/// Computes the length of the full version string.
///
/// # Return
///
/// Length of the string.
#[inline]
pub fn string_length_full(version: &Version) -> usize {
    VersionAPI::string_length_full(get_interface(), version)
}

/// Represents the version as a short string.
///
/// # Failure
///
/// This function fails if `buffer.len() < string_length_short(version)`.
///
/// # Return
///
/// Number of written characters on success, error otherwise.
#[inline]
pub fn as_string_short(version: &Version, buffer: impl AsMut<str>) -> Result<usize, Error> {
    VersionAPI::as_string_short(get_interface(), version, buffer)
}

/// Represents the version as a long string.
///
/// # Failure
///
/// This function fails if `buffer.len() < string_length_long(version)`.
///
/// # Return
///
/// Number of written characters on success, error otherwise.
#[inline]
pub fn as_string_long(version: &Version, buffer: impl AsMut<str>) -> Result<usize, Error> {
    VersionAPI::as_string_long(get_interface(), version, buffer)
}

/// Represents the version as a full string.
///
/// # Failure
///
/// This function fails if `buffer.len() < string_length_full(version)`.
///
/// # Return
///
/// Number of written characters on success, error otherwise.
#[inline]
pub fn as_string_full(version: &Version, buffer: impl AsMut<str>) -> Result<usize, Error> {
    VersionAPI::as_string_full(get_interface(), version, buffer)
}

/// Checks whether the version string is valid.
///
/// # Return
///
/// [true] if the string is valid, [false] otherwise.
#[inline]
pub fn string_is_valid(version_string: impl AsRef<str>) -> bool {
    VersionAPI::string_is_valid(get_interface(), version_string)
}

/// Compares two versions.
///
/// Compares two version, disregarding their build number.
///
/// # Return
///
/// Order of the versions.
#[inline]
pub fn compare(lhs: &Version, rhs: &Version) -> Ordering {
    VersionAPI::compare(get_interface(), lhs, rhs)
}

/// Compares two versions.
///
/// Compares two version, disregarding their build number and release type.
///
/// # Return
///
/// Order of the versions.
#[inline]
pub fn compare_weak(lhs: &Version, rhs: &Version) -> Ordering {
    VersionAPI::compare_weak(get_interface(), lhs, rhs)
}

/// Compares two versions.
///
/// # Return
///
/// Order of the versions.
#[inline]
pub fn compare_strong(lhs: &Version, rhs: &Version) -> Ordering {
    VersionAPI::compare_strong(get_interface(), lhs, rhs)
}

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
#[inline]
pub fn is_compatible(lhs: &Version, rhs: &Version) -> bool {
    VersionAPI::is_compatible(get_interface(), lhs, rhs)
}
