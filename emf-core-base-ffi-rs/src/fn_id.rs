//! Id's of the function types specified by the `emf-core-base` interface.

/// Id's of the exported functions.
///
/// The values `0-1000` are reserved for future use.
#[repr(i32)]
#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum FnId {
    SysLock = 1,
    SysTryLock = 2,
    SysUnlock = 3,
    SysShutdown = 4,
    SysPanic = 5,
    SysHasFunction = 6,
    SysGetFunction = 7,
    SysGetSyncHandler = 8,
    SysSetSyncHandler = 9,

    VersionConstructShort = 101,
    VersionConstructLong = 102,
    VersionConstructFull = 103,
    VersionConstructFromString = 104,
    VersionRepresentationIsValid = 105,
    VersionGetShortRepresentation = 106,
    VersionGetShortRepresentationLength = 107,
    VersionGetLongRepresentation = 108,
    VersionGetLongRepresentationLength = 109,
    VersionGetFullRepresentation = 110,
    VersionGetFullRepresentationLength = 111,
    VersionCompare = 112,
    VersionCompareWeak = 113,
    VersionCompareStrong = 114,
    VersionIsCompatible = 115,
}
