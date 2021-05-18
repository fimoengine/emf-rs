//! Global sys api.
use crate::fn_caster::FnCaster;
use crate::global::{get_interface, get_mut_interface, LockToken};
use crate::ownership::Owned;
use crate::sys::sync_handler::SyncHandlerAPI;
use crate::sys::{SysAPI, SysAPIMin};
use crate::Error;

/// Execution of the program is stopped abruptly. The error may be logged.
#[inline]
pub fn panic(error: Option<Error<Owned>>) -> ! {
    SysAPIMin::panic(get_interface(), error)
}

/// Sends a termination signal.
#[inline]
pub fn shutdown<L>(_token: &mut LockToken<L>) -> ! {
    SysAPI::shutdown(get_mut_interface())
}

/// Checks if a function is implemented.
///
/// # Return
///
/// Returns [true] if the function exists, [false] otherwise.
#[inline]
pub fn has_function<U>() -> bool
where
    U: FnCaster,
{
    SysAPIMin::has_function::<U>(get_interface())
}

/// Fetches a function from the interface.
///
/// # Return
///
/// Function casted to the appropriate type.
#[inline]
pub fn get_function<U>(caster: &U) -> Option<<U as FnCaster>::Type>
where
    U: FnCaster,
{
    SysAPIMin::get_function::<U>(get_interface(), caster)
}

/// Fetches the active synchronization handler.
///
/// # Return
///
/// The active synchronization handler.
#[inline]
pub fn get_sync_handler<L, U>(_token: &LockToken<L>) -> <U as SyncHandlerAPI<'static>>::Handler
where
    U: SyncHandlerAPI<'static>,
{
    SysAPI::get_sync_handler::<U>(get_interface())
}

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
#[inline]
pub unsafe fn set_sync_handler<L>(
    _token: &mut LockToken<L>,
    handler: Option<&impl SyncHandlerAPI<'static>>,
) {
    SysAPI::set_sync_handler(get_mut_interface(), handler)
}
