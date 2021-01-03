use super::{ReleaseType, Version, VersionError};
use crate::ffi;
use crate::ffi::containers::{MutSpan, NonNullConst, Span};
use std::cmp::Ordering;
use std::ptr::NonNull;

/// The implementation of a version using the global api.
pub trait VersionImplGlobal
where
    Self: Sized,
{
    /// Constructs a new version.
    ///
    /// Constructs a new version with `major`, `minor` and `patch`.
    fn new(major: i32, minor: i32, patch: i32) -> Self;

    /// Constructs a new version.
    ///
    /// Constructs a new version with `major`, `minor`, `patch`, `release_type` and `release_number`.
    fn new_long(
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
    ) -> Self;

    /// Constructs a new version.
    ///
    /// Constructs a new version with `major`, `minor`, `patch`, `release_type`, `release_number`
    /// and `build`.
    fn new_full(
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
        build: i64,
    ) -> Self;

    /// Constructs a version from a string.
    ///
    /// # Failure
    ///
    /// The function fails if `str` is not of the form
    /// `"Major.Minor.Patch(-((pre-)?alpha|beta|rc).Release)?\+Build"`
    fn from_string<T: AsRef<str>>(str: &T) -> Result<Self, VersionError>;

    /// Checks weather the version string is valid.
    ///
    /// The string is valid if it has the form
    /// `"Major.Minor.Patch(-((pre-)?alpha|beta|rc).Release)?\+Build"`.
    fn str_is_valid<T: AsRef<str>>(str: &T) -> bool;

    /// Represents the version as a string.
    ///
    /// The string has the form `"Major.Minor.Patch"`.
    ///
    /// # Failure
    ///
    /// The function can fail if `str` is too small.
    fn as_short_str<T: AsMut<str>>(&self, str: &mut T) -> Result<usize, VersionError>;

    /// Represents the version as a string.
    ///
    /// The string has the form `"Major.Minor.Patch(-((pre-)?alpha|beta|rc).Release)?"`.
    ///
    /// # Failure
    ///
    /// The function can fail if `str` is too small.
    fn as_long_str<T: AsMut<str>>(&self, str: &mut T) -> Result<usize, VersionError>;

    /// Represents the version as a string.
    ///
    /// The string has the form `"Major.Minor.Patch(-((pre-)?alpha|beta|rc).Release)?\+Build"`.
    ///
    /// # Failure
    ///
    /// The function can fail if `str` is too small.
    fn as_full_str<T: AsMut<str>>(&self, str: &mut T) -> Result<usize, VersionError>;

    /// Computes the length of the version string.
    ///
    /// Computes the minimum length a string of the form `"Major.Minor.Patch"` needs.
    fn short_str_len(&self) -> usize;

    /// Computes the length of the version string.
    ///
    /// Computes the minimum length a string of the form
    /// `"Major.Minor.Patch(-((pre-)?alpha|beta|rc).Release)?"` needs.
    fn long_str_len(&self) -> usize;

    /// Computes the length of the version string.
    ///
    /// Computes the minimum length a string of the form
    /// `"Major.Minor.Patch(-((pre-)?alpha|beta|rc).Release)?\+Build"` needs.
    fn full_str_len(&self) -> usize;

    /// Compares two versions.
    ///
    /// Compares two versions using their major-,minor- and patch version, release type
    /// and release number.
    ///
    /// # Return value
    ///
    /// Returns [Ordering::Less] if `self < other`.
    /// Returns [Ordering::Equal] if `self == other`.
    /// Returns [Ordering::Greater] if `self > other`.
    fn compare(&self, other: &Self) -> Ordering;

    /// Compares two versions.
    ///
    /// Compares two versions using their major-,minor- and patch version.
    ///
    /// # Return value
    ///
    /// Returns [Ordering::Less] if `self < other`.
    /// Returns [Ordering::Equal] if `self == other`.
    /// Returns [Ordering::Greater] if `self > other`.
    fn compare_weak(&self, other: &Self) -> Ordering;

    /// Compares two versions.
    ///
    /// Compares two versions using their major-,minor- and patch version, release type,
    /// release number and build number.
    ///
    /// # Return value
    ///
    /// Returns [Ordering::Less] if `self < other`.
    /// Returns [Ordering::Equal] if `self == other`.
    /// Returns [Ordering::Greater] if `self > other`.
    fn compare_strong(&self, other: &Self) -> Ordering;

    /// Compares weather two versions are compatible.
    ///
    /// Compatibility of versions is not commutative.
    ///
    /// # Return value
    ///
    /// Returns `false` if `self` and `other` are incompatible.
    /// Returns `true` if `self` is compatible with `other`.
    fn is_compatible(&self, other: &Self) -> bool;
}

impl VersionImplGlobal for Version {
    #[inline]
    fn new(major: i32, minor: i32, patch: i32) -> Self {
        unsafe { ffi::version::emf_cbase_version_construct_short(major, minor, patch) }
    }

    #[inline]
    fn new_long(
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
    ) -> Self {
        unsafe {
            ffi::version::emf_cbase_version_construct_long(
                major,
                minor,
                patch,
                release_type,
                release_number,
            )
        }
    }

    #[inline]
    fn new_full(
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
        build: i64,
    ) -> Self {
        unsafe {
            ffi::version::emf_cbase_version_construct_full(
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
    fn from_string<T: AsRef<str>>(str: &T) -> Result<Self, VersionError> {
        let str_span = Span::from(str.as_ref()).as_c_char_span();
        unsafe {
            ffi::version::emf_cbase_version_construct_from_string(NonNullConst::from(&str_span))
                .to_native()
        }
    }

    #[inline]
    fn str_is_valid<T: AsRef<str>>(str: &T) -> bool {
        let str_span = Span::from(str.as_ref()).as_c_char_span();
        unsafe {
            ffi::version::emf_cbase_version_representation_is_valid(NonNullConst::from(&str_span))
                .into()
        }
    }

    #[inline]
    fn as_short_str<T: AsMut<str>>(&self, str: &mut T) -> Result<usize, VersionError> {
        let str_span = MutSpan::from(str.as_mut()).as_c_char_span();
        unsafe {
            ffi::version::emf_cbase_version_get_short_representation(
                NonNullConst::from(self),
                NonNull::from(&str_span),
            )
            .to_native()
        }
    }

    #[inline]
    fn as_long_str<T: AsMut<str>>(&self, str: &mut T) -> Result<usize, VersionError> {
        let str_span = MutSpan::from(str.as_mut()).as_c_char_span();
        unsafe {
            ffi::version::emf_cbase_version_get_long_representation(
                NonNullConst::from(self),
                NonNull::from(&str_span),
            )
            .to_native()
        }
    }

    #[inline]
    fn as_full_str<T: AsMut<str>>(&self, str: &mut T) -> Result<usize, VersionError> {
        let str_span = MutSpan::from(str.as_mut()).as_c_char_span();
        unsafe {
            ffi::version::emf_cbase_version_get_full_representation(
                NonNullConst::from(self),
                NonNull::from(&str_span),
            )
            .to_native()
        }
    }

    #[inline]
    fn short_str_len(&self) -> usize {
        unsafe {
            ffi::version::emf_cbase_version_get_short_representation_length(NonNullConst::from(
                self,
            ))
        }
    }

    #[inline]
    fn long_str_len(&self) -> usize {
        unsafe {
            ffi::version::emf_cbase_version_get_long_representation_length(NonNullConst::from(self))
        }
    }

    #[inline]
    fn full_str_len(&self) -> usize {
        unsafe {
            ffi::version::emf_cbase_version_get_full_representation_length(NonNullConst::from(self))
        }
    }

    #[inline]
    fn compare(&self, other: &Self) -> Ordering {
        unsafe {
            match ffi::version::emf_cbase_version_compare(
                NonNullConst::from(self),
                NonNullConst::from(other),
            ) {
                -1 => Ordering::Greater,
                0 => Ordering::Equal,
                1 => Ordering::Less,
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn compare_weak(&self, other: &Self) -> Ordering {
        unsafe {
            match ffi::version::emf_cbase_version_compare_weak(
                NonNullConst::from(self),
                NonNullConst::from(other),
            ) {
                -1 => Ordering::Greater,
                0 => Ordering::Equal,
                1 => Ordering::Less,
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn compare_strong(&self, other: &Self) -> Ordering {
        unsafe {
            match ffi::version::emf_cbase_version_compare_strong(
                NonNullConst::from(self),
                NonNullConst::from(other),
            ) {
                -1 => Ordering::Greater,
                0 => Ordering::Equal,
                1 => Ordering::Less,
                _ => unreachable!(),
            }
        }
    }

    #[inline]
    fn is_compatible(&self, other: &Self) -> bool {
        unsafe {
            ffi::version::emf_cbase_version_is_compatible(
                NonNullConst::from(self),
                NonNullConst::from(other),
            )
            .into()
        }
    }
}
