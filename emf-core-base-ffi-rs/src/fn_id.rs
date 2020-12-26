//! Id's of the function types specified by the `emf-core-base` interface.

/// Id's of the exported functions.
///
/// The values `0-1000` are reserved for future use.
#[repr(i32)]
#[non_exhaustive]
pub enum FnId {
    SysLock = 1,
}
