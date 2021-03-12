//! Utilities for casting interface functions.
use crate::ffi::{CBaseFn, FnId};

/// Interface of a function caster.
pub trait FnCaster {
    /// Type of the function.
    type Type;

    /// Id of the function
    const ID: FnId;

    /// Cast unsafe the function to a safer type.
    fn cast(&self, func: CBaseFn) -> Self::Type;
}

macro_rules! transmute_caster {
    ($caster:ty, $target_fn:ty, $target_id:expr) => {
        impl FnCaster for $caster {
            type Type = $target_fn;
            const ID: FnId = $target_id;

            #[inline]
            fn cast(&self, func: CBaseFn) -> Self::Type {
                unsafe { std::mem::transmute(func) }
            }
        }
    };
}

pub mod library;
pub mod module;
pub mod sys;
pub mod version;

#[cfg(feature = "extensions")]
pub mod extensions;
