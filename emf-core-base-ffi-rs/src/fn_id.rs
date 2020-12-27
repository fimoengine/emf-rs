//! Id's of the function types specified by the `emf-core-base` interface.

/// Id's of the exported functions.
///
/// The values `0-1000` are reserved for future use.
#[repr(i32)]
#[non_exhaustive]
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
}
