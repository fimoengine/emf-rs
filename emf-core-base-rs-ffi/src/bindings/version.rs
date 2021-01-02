use crate::bindings::BASE_INTERFACE;
use crate::containers::{MutSpan, NonNullConst, Result, Span};
use crate::version::{ReleaseType, Version, VersionError};
use crate::{Bool, InterfaceBinding};
use std::os::raw::c_char;
use std::ptr::NonNull;

#[cfg(test)]
mod tests;

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_version_construct_short(
    major: i32,
    minor: i32,
    patch: i32,
) -> Version {
    (*BASE_INTERFACE.as_ptr()).version_construct_short(major, minor, patch)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_version_construct_long(
    major: i32,
    minor: i32,
    patch: i32,
    release_type: ReleaseType,
    release_number: i8,
) -> Version {
    (*BASE_INTERFACE.as_ptr()).version_construct_long(
        major,
        minor,
        patch,
        release_type,
        release_number,
    )
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_version_construct_full(
    major: i32,
    minor: i32,
    patch: i32,
    release_type: ReleaseType,
    release_number: i8,
    build: i64,
) -> Version {
    (*BASE_INTERFACE.as_ptr()).version_construct_full(
        major,
        minor,
        patch,
        release_type,
        release_number,
        build,
    )
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_version_construct_from_string(
    version_string: NonNullConst<Span<c_char>>,
) -> Result<Version, VersionError> {
    (*BASE_INTERFACE.as_ptr()).version_construct_from_string(version_string)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_version_representation_is_valid(
    version_string: NonNullConst<Span<c_char>>,
) -> Bool {
    (*BASE_INTERFACE.as_ptr()).version_representation_is_valid(version_string)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_version_get_short_representation(
    version: NonNullConst<Version>,
    buffer: NonNull<MutSpan<c_char>>,
) -> Result<usize, VersionError> {
    (*BASE_INTERFACE.as_ptr()).version_get_short_representation(version, buffer)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_version_get_short_representation_length(
    version: NonNullConst<Version>,
) -> usize {
    (*BASE_INTERFACE.as_ptr()).version_get_short_representation_length(version)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_version_get_long_representation(
    version: NonNullConst<Version>,
    buffer: NonNull<MutSpan<c_char>>,
) -> Result<usize, VersionError> {
    (*BASE_INTERFACE.as_ptr()).version_get_long_representation(version, buffer)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_version_get_long_representation_length(
    version: NonNullConst<Version>,
) -> usize {
    (*BASE_INTERFACE.as_ptr()).version_get_long_representation_length(version)
}

#[must_use]
#[no_mangle]
#[allow(improper_ctypes_definitions)]
unsafe extern "C" fn emf_cbase_version_get_full_representation(
    version: NonNullConst<Version>,
    buffer: NonNull<MutSpan<c_char>>,
) -> Result<usize, VersionError> {
    (*BASE_INTERFACE.as_ptr()).version_get_full_representation(version, buffer)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_version_get_full_representation_length(
    version: NonNullConst<Version>,
) -> usize {
    (*BASE_INTERFACE.as_ptr()).version_get_full_representation_length(version)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_version_compare(
    lhs: NonNullConst<Version>,
    rhs: NonNullConst<Version>,
) -> i32 {
    (*BASE_INTERFACE.as_ptr()).version_compare(lhs, rhs)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_version_compare_weak(
    lhs: NonNullConst<Version>,
    rhs: NonNullConst<Version>,
) -> i32 {
    (*BASE_INTERFACE.as_ptr()).version_compare_weak(lhs, rhs)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_version_compare_strong(
    lhs: NonNullConst<Version>,
    rhs: NonNullConst<Version>,
) -> i32 {
    (*BASE_INTERFACE.as_ptr()).version_compare_strong(lhs, rhs)
}

#[must_use]
#[no_mangle]
unsafe extern "C" fn emf_cbase_version_is_compatible(
    lhs: NonNullConst<Version>,
    rhs: NonNullConst<Version>,
) -> Bool {
    (*BASE_INTERFACE.as_ptr()).version_is_compatible(lhs, rhs)
}
