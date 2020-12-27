//! Function types specified by the `emf-core-base` interface.

use crate::containers::{MutSpan, Optional, Span};
use crate::sys::SyncHandlerInterface;
use crate::version::{ReleaseType, Version, VersionError};
use crate::{BaseT, Bool, FnId};
use std::os::raw::c_char;
use std::ptr::NonNull;

/// A type-erased function pointer.
pub type BaseFn = extern "C" fn();

/// Function pointer to the [emf_cbase_sys_lock](crate::sys::emf_cbase_sys_lock) function.
pub type SysLockFn = extern "C" fn(base_module: *mut BaseT);

/// Function pointer to the [emf_cbase_sys_try_lock](crate::sys::emf_cbase_sys_try_lock) function.
pub type SysTryLockFn = extern "C" fn(base_module: *mut BaseT) -> Bool;

/// Function pointer to the [emf_cbase_sys_unlock](crate::sys::emf_cbase_sys_unlock) function.
pub type SysUnlockFn = extern "C" fn(base_module: *mut BaseT);

/// Function pointer to the [emf_cbase_sys_shutdown](crate::sys::emf_cbase_sys_shutdown) function.
pub type SysShutdownFn = extern "C" fn(base_module: *mut BaseT) -> !;

/// Function pointer to the [emf_cbase_sys_panic](crate::sys::emf_cbase_sys_panic) function.
pub type SysPanicFn = extern "C" fn(base_module: *mut BaseT, error: *const c_char) -> !;

/// Function pointer to the
/// [emf_cbase_sys_has_function](crate::sys::emf_cbase_sys_has_function) function.
pub type SysHasFunctionFn = extern "C" fn(base_module: *mut BaseT, fn_id: FnId) -> Bool;

/// Function pointer to the
/// [emf_cbase_sys_get_function](crate::sys::emf_cbase_sys_get_function) function.
pub type SysGetFunctionFn = extern "C" fn(base_module: *mut BaseT, fn_id: FnId) -> Optional<BaseFn>;

/// Function pointer to the
/// [emf_cbase_sys_get_sync_handler](crate::sys::emf_cbase_sys_get_sync_handler) function.
pub type SysGetSyncHandlerFn =
    extern "C" fn(base_module: *mut BaseT) -> *const SyncHandlerInterface;

/// Function pointer to the
/// [emf_cbase_sys_set_sync_handler](crate::sys::emf_cbase_sys_set_sync_handler) function.
pub type SysSetSyncHandlerFn =
    extern "C" fn(base_module: *mut BaseT, sync_handler: *const SyncHandlerInterface);

/// Function pointer to the
/// [emf_cbase_version_construct_short](crate::version::emf_cbase_version_construct_short) function.
pub type VersionConstructShortFn =
    extern "C" fn(base_module: *mut BaseT, major: i32, minor: i32, patch: i32) -> Version;

/// Function pointer to the
/// [emf_cbase_version_construct_long](crate::version::emf_cbase_version_construct_long) function.
pub type VersionConstructLongFn = extern "C" fn(
    base_module: *mut BaseT,
    major: i32,
    minor: i32,
    patch: i32,
    release_type: ReleaseType,
    release_number: i8,
) -> Version;

/// Function pointer to the
/// [emf_cbase_version_construct_full](crate::version::emf_cbase_version_construct_full) function.
pub type VersionConstructFullFn = extern "C" fn(
    base_module: *mut BaseT,
    major: i32,
    minor: i32,
    patch: i32,
    release_type: ReleaseType,
    release_number: i8,
    build: i64,
) -> Version;

/// Function pointer to the
/// [emf_cbase_version_construct_from_string](crate::version::emf_cbase_version_construct_from_string)
/// function.
pub type VersionConstructFromStringFn = extern "C" fn(
    base_module: *mut BaseT,
    version_string: *const Span<'_, c_char>,
) -> Result<Version, VersionError>;

/// Function pointer to the
/// [emf_cbase_version_representation_is_valid](crate::version::emf_cbase_version_representation_is_valid)
/// function.
pub type VersionRepresentationIsValidFn =
    extern "C" fn(base_module: *mut BaseT, version_string: *const Span<'_, c_char>) -> Bool;

/// Function pointer to the
/// [emf_cbase_version_get_short_representation](crate::version::emf_cbase_version_get_short_representation)
/// function.
pub type VersionGetShortRepresentationFn = extern "C" fn(
    base_module: *mut BaseT,
    version: *const Version,
    buffer: NonNull<MutSpan<'_, c_char>>,
) -> Result<usize, VersionError>;

/// Function pointer to the
/// [emf_cbase_version_get_short_representation_length](crate::version::emf_cbase_version_get_short_representation_length)
/// function.
pub type VersionGetShortRepresentationLengthFn =
    extern "C" fn(base_module: *mut BaseT, version: *const Version) -> usize;

/// Function pointer to the
/// [emf_cbase_version_get_long_representation](crate::version::emf_cbase_version_get_long_representation)
/// function.
pub type VersionGetLongRepresentationFn = extern "C" fn(
    base_module: *mut BaseT,
    version: *const Version,
    buffer: NonNull<MutSpan<'_, c_char>>,
) -> Result<usize, VersionError>;

/// Function pointer to the
/// [emf_cbase_version_get_long_representation_length](crate::version::emf_cbase_version_get_long_representation_length)
/// function.
pub type VersionGetLongRepresentationLengthFn =
    extern "C" fn(base_module: *mut BaseT, version: *const Version) -> usize;

/// Function pointer to the
/// [emf_cbase_version_get_full_representation](crate::version::emf_cbase_version_get_full_representation)
/// function.
pub type VersionGetFullRepresentationFn = extern "C" fn(
    base_module: *mut BaseT,
    version: *const Version,
    buffer: NonNull<MutSpan<'_, c_char>>,
) -> Result<usize, VersionError>;

/// Function pointer to the
/// [emf_cbase_version_get_full_representation_length](crate::version::emf_cbase_version_get_full_representation_length)
/// function.
pub type VersionGetFullRepresentationLengthFn =
    extern "C" fn(base_module: *mut BaseT, version: *const Version) -> usize;

/// Function pointer to the
/// [emf_cbase_version_compare](crate::version::emf_cbase_version_compare) function.
pub type VersionCompareFn =
    extern "C" fn(base_module: *mut BaseT, lhs: *const Version, rhs: *const Version) -> i32;

/// Function pointer to the
/// [emf_cbase_version_compare_weak](crate::version::emf_cbase_version_compare_weak) function.
pub type VersionCompareWeakFn =
    extern "C" fn(base_module: *mut BaseT, lhs: *const Version, rhs: *const Version) -> i32;

/// Function pointer to the
/// [emf_cbase_version_compare_strong](crate::version::emf_cbase_version_compare_strong) function.
pub type VersionCompareStrongFn =
    extern "C" fn(base_module: *mut BaseT, lhs: *const Version, rhs: *const Version) -> i32;

/// Function pointer to the
/// [emf_cbase_version_is_compatible](crate::version::emf_cbase_version_is_compatible) function.
pub type VersionIsCompatibleFn =
    extern "C" fn(base_module: *mut BaseT, lhs: *const Version, rhs: *const Version) -> Bool;
