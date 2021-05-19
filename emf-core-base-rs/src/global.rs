//! Global api that can be used instead of the local api.
//!
//! The global api is the preferred way of interfacing with the interface.
use crate::ffi::{
    collections::NonNullConst,
    sys::api::{GetFunctionFn as GetFunctionFnFFI, SysBinding},
    Bool, CBase as CBaseFFI,
};
use crate::init::CBaseAPILoader;
use crate::{CBase, CBaseInterfaceInfo, CBaseRef};
use std::mem::MaybeUninit;
use std::ptr::NonNull;

pub mod library;
pub mod module;
pub mod sys;
pub mod version;

#[cfg(feature = "extensions")]
pub mod extensions;

static mut INTERFACE: MaybeUninit<CBaseRef<'static>> = MaybeUninit::uninit();

/// Type indicating that dropping unlocks the interface.
#[derive(Debug, Hash)]
pub struct Unlock {}

/// Type indicating that dropping does not unlock the interface.
#[derive(Debug, Hash)]
pub struct ForgetUnlock {}

impl Drop for Unlock {
    fn drop(&mut self) {
        unsafe { SysBinding::unlock(get_interface()) }
    }
}

/// A token indicating a locked interface.
#[derive(Debug, Hash)]
pub struct LockToken<T> {
    _phantom: T,
}

impl LockToken<Unlock> {
    /// Takes ownership of the token and exchanges it with
    /// a token which does not unlock the interface.
    ///
    /// # Safety
    ///
    /// Improper usage can leave the interface in a locked state.
    pub unsafe fn relinquish_locking(self) -> LockToken<ForgetUnlock> {
        std::mem::forget(self);
        LockToken {
            _phantom: ForgetUnlock {},
        }
    }
}

impl LockToken<ForgetUnlock> {
    /// Takes ownership of the token and exchanges it with
    /// a token which unlocks the interface.
    ///
    /// # Safety
    ///
    /// Improper usage can unlock the interface multiple times.
    pub unsafe fn take_ownership(self) -> LockToken<Unlock> {
        std::mem::forget(self);
        LockToken {
            _phantom: Unlock {},
        }
    }
}

impl<T> LockToken<T> {
    /// Constructs a new token by locking the interface.
    ///
    /// The calling thread is stalled until the lock can be acquired.
    /// Only one thread can hold the lock at a time.
    ///
    /// # Return
    ///
    /// A token.
    #[inline]
    #[must_use]
    pub fn lock() -> LockToken<Unlock> {
        unsafe {
            SysBinding::lock(get_interface());
            LockToken {
                _phantom: Unlock {},
            }
        }
    }

    /// Tries to lock the interface.
    ///
    /// The function fails if another thread already holds the lock.
    ///
    /// # Return
    ///
    /// [LockToken] on success and [None] otherwise.
    #[inline]
    #[must_use]
    pub fn try_lock() -> Option<LockToken<Unlock>> {
        unsafe {
            match SysBinding::try_lock(get_interface()) {
                Bool::False => None,
                Bool::True => Some(LockToken {
                    _phantom: Unlock {},
                }),
            }
        }
    }

    /// Constructs a new token without locking.
    ///
    /// # Return
    ///
    /// A token.
    ///
    /// # Safety
    ///
    /// Most of the interface assumes that the caller has unique access to the interface.
    /// This function can be used to bypass this restriction, if the user can guarantee
    /// that no data-races will occur.
    #[inline]
    pub unsafe fn assume_locked() -> LockToken<ForgetUnlock> {
        LockToken {
            _phantom: ForgetUnlock {},
        }
    }
}

/// Initializes the interface.
#[inline]
pub fn initialize(base_module: Option<NonNull<CBaseFFI>>, get_function_fn: GetFunctionFnFFI) {
    unsafe {
        INTERFACE = MaybeUninit::new(CBaseRef::new(NonNullConst::from(
            CBase::fetch_interface(base_module, get_function_fn).internal_interface(),
        )));
    }

    #[cfg(feature = "unwind_internal")]
    extensions::unwind_internal::initialize();
}

/// Fetches a reference to the interface.
///
/// Using the interface is safe, as long as a [LockToken] is constructed.
#[inline]
pub fn get_interface<'a>() -> &'a CBaseRef<'static> {
    unsafe { &*INTERFACE.as_ptr() }
}

/// Fetches a mutable reference to the interface.
///
/// Using the interface is safe, as long as a [LockToken] is constructed.
#[inline]
pub fn get_mut_interface<'a>() -> &'a mut CBaseRef<'static> {
    unsafe { &mut *INTERFACE.as_mut_ptr() }
}
