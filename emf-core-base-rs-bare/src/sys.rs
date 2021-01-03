//! Utilities for managing the system.

use crate::ffi::sys::{SyncHandlerInterface, SyncHandlerInterfaceBinding};
use crate::{ffi, BaseInterfaceFn, FnId};
use std::ffi::CStr;

#[cfg(feature = "global_api")]
mod global_sys_token;
mod local_sys_token;

#[cfg(feature = "global_api")]
pub use global_sys_token::GlobalSysToken;
pub use local_sys_token::LocalSysToken;

/// A trait describing the functionality of a `SyncHandler`.
pub trait SyncHandlerWrapper<'a>:
    AsRef<ffi::sys::SyncHandlerInterface> + From<&'a ffi::sys::SyncHandlerInterface>
{
    /// Locks the interface using the `SyncHandler`.
    ///
    /// # Safety
    ///
    /// Manual locking is discouraged as it can lead to deadlocks.
    unsafe fn lock(&self);

    /// Locks the interface using the `SyncHandler` without blocking.
    ///
    /// # Safety
    ///
    /// Manual locking is discouraged as it can lead to deadlocks.
    unsafe fn try_lock(&self) -> bool;

    /// Unlocks the interface using the `SyncHandler`.
    ///
    /// # Safety
    ///
    /// Trying to unlock the interface when it is not locked leads to undefined behaviour.
    unsafe fn unlock(&self);

    /// Extends the lifetime of the `SyncHandler`.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the `SyncHandler` lives long enough.
    unsafe fn extend_lifetime<'b>(self) -> &'b Self;
}

/// A `SyncHandler`
#[derive(Debug, Copy, Clone)]
pub struct SyncHandler<'a> {
    handler: &'a ffi::sys::SyncHandlerInterface,
}

impl<'a> SyncHandlerWrapper<'a> for SyncHandler<'a> {
    #[inline]
    unsafe fn lock(&self) {
        self.handler.lock()
    }

    unsafe fn try_lock(&self) -> bool {
        self.handler.try_lock().into()
    }

    unsafe fn unlock(&self) {
        self.handler.unlock()
    }

    unsafe fn extend_lifetime<'b>(self) -> &'b Self {
        std::mem::transmute(self)
    }
}

impl<'a> AsRef<ffi::sys::SyncHandlerInterface> for SyncHandler<'a> {
    fn as_ref(&self) -> &SyncHandlerInterface {
        self.handler
    }
}

impl<'a> From<&'a ffi::sys::SyncHandlerInterface> for SyncHandler<'a> {
    fn from(handler: &'a SyncHandlerInterface) -> Self {
        Self { handler }
    }
}

pub trait SysToken<'a>
where
    Self: Sized,
{
    /// Terminates the execution.
    fn shutdown(&self) -> !;

    /// Panics.
    ///
    /// Execution of the program is stopped abruptly after the error is logged.
    fn panic<T: AsRef<CStr>>(&self, err: Option<&T>) -> !;

    /// Checks if a function is implemented.
    fn has_function<const ID: FnId, T: BaseInterfaceFn<{ ID }>>(&self) -> bool;

    /// Retrieves an implemented function.
    fn get_function<const ID: FnId, T: BaseInterfaceFn<{ ID }>>(&self) -> Option<T::Type>;

    /// Fetches the current synchronisation handler.
    fn get_sync_handler<T: SyncHandlerWrapper<'a>>(&self) -> T;

    /// Sets a new synchronisation handler.
    ///
    /// The default synchronisation handler is used, if `handler` is [Option::None].
    ///
    /// # Uses
    ///
    /// This function can be used by modules, that want to provide a more complex
    /// synchronisation mechanism than the one presented by the default handler.
    ///
    /// # Swapping
    ///
    /// The swapping occurs in three steps:
    ///
    /// 1. The new synchronisation handler is locked.
    /// 2. The new synchronisation handler is set as the main synchronisation handler.
    /// 3. The old synchronisation handler is unlocked.
    ///
    /// # Safety
    ///
    /// Changing the synchronisation handler may break some modules, if
    /// they depend on a specific synchronisation handler.
    unsafe fn set_sync_handler<T: SyncHandlerWrapper<'static>>(&self, handler: Option<T>);
}
