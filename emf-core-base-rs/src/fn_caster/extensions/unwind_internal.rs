//! Casters for the `unwind_internal` extension.
use crate::ffi::extensions::unwind_internal;
use crate::ffi::{CBaseFn, FnId};
use crate::fn_caster::FnCaster;

/// Caster for [FnId::ExtGetUnwindInternalInterface]
#[derive(Debug)]
pub struct GetUnwindInternalInterfaceCaster {}

transmute_caster!(
    GetUnwindInternalInterfaceCaster,
    unwind_internal::GetUnwindInternalInterfaceFn,
    FnId::ExtGetUnwindInternalInterface
);
