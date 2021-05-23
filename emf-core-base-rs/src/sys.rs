//! Sys api.
//!
//! The sys api is exposed by the [SysAPI] trait.
use crate::ffi::sys::api::SysBinding;
use crate::ffi::Bool;
use crate::fn_caster::FnCaster;
use crate::ownership::Owned;
use crate::sys::sync_handler::SyncHandlerAPI;
use crate::Error;

pub mod sync_handler;

/// Minimal sys api.
pub trait SysAPIMin<'interface> {
    /// Execution of the program is stopped abruptly. The error may be logged.
    fn panic(&self, error: Option<Error<Owned>>) -> !;

    /// Checks if a function is implemented.
    ///
    /// # Return
    ///
    /// Returns [true] if the function exists, [false] otherwise.
    fn has_function<U>(&self) -> bool
    where
        U: FnCaster;

    /// Fetches a function from the interface.
    ///
    /// # Return
    ///
    /// Function casted to the appropriate type.
    fn get_function<U>(&self, caster: &U) -> Option<<U as FnCaster>::Type>
    where
        U: FnCaster;
}

/// Idiomatic sys api.
pub trait SysAPI<'interface>: SysAPIMin<'interface> {
    /// Sends a termination signal.
    fn shutdown(&mut self) -> !;

    /// Fetches the active synchronization handler.
    ///
    /// # Return
    ///
    /// The active synchronization handler.
    fn get_sync_handler<U>(&self) -> <U as SyncHandlerAPI<'interface>>::Handler
    where
        U: SyncHandlerAPI<'interface>;

    /// Sets a new synchronization handler.
    ///
    /// The default synchronization handler is used, if `handler` is [Option::None].
    ///
    /// # Uses
    ///
    /// This function can be used by modules, that want to provide a more complex
    /// synchronization mechanism than the one presented by the default handler.
    ///
    /// # Swapping
    ///
    /// The swapping occurs in three steps:
    ///
    /// 1. The new synchronization handler is locked.
    /// 2. The new synchronization handler is set as the active synchronization handler.
    /// 3. The old synchronization handler is unlocked.
    ///
    /// # Safety
    ///
    /// Changing the synchronization handler may break some modules,
    /// if they depend on a specific synchronization handler.
    unsafe fn set_sync_handler(&mut self, handler: Option<&impl SyncHandlerAPI<'interface>>);
}

impl<'interface, T> SysAPIMin<'interface> for T
where
    T: SysBinding,
{
    #[inline]
    fn panic(&self, error: Option<Error<Owned>>) -> ! {
        unsafe { <T as SysBinding>::panic(self, From::from(error.map(|e| e.into_inner()))) }
    }

    #[inline]
    fn has_function<U>(&self) -> bool
    where
        U: FnCaster,
    {
        unsafe { <T as SysBinding>::has_function(self, U::ID) == Bool::True }
    }

    #[inline]
    fn get_function<U>(&self, caster: &U) -> Option<<U as FnCaster>::Type>
    where
        U: FnCaster,
    {
        unsafe {
            <T as SysBinding>::get_function(self, U::ID)
                .into_rust()
                .map(|func| caster.cast(func))
        }
    }
}

impl<'interface, T> SysAPI<'interface> for T
where
    T: SysBinding,
{
    #[inline]
    fn shutdown(&mut self) -> ! {
        unsafe { <T as SysBinding>::shutdown(self) }
    }

    #[inline]
    fn get_sync_handler<U>(&self) -> <U as SyncHandlerAPI<'interface>>::Handler
    where
        U: SyncHandlerAPI<'interface>,
    {
        unsafe { U::from_raw(<T as SysBinding>::get_sync_handler(self)) }
    }

    #[inline]
    unsafe fn set_sync_handler(&mut self, handler: Option<&impl SyncHandlerAPI<'interface>>) {
        <T as SysBinding>::set_sync_handler(
            self,
            From::from(handler.map(|handler| handler.to_raw())),
        )
    }
}
