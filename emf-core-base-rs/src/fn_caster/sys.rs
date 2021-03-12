//! Casters for the sys api.
use crate::ffi::sys::api;
use crate::ffi::{CBaseFn, FnId};
use crate::fn_caster::FnCaster;

/// Caster for [FnId::SysShutdown]
#[derive(Debug)]
pub struct ShutdownCaster {}

/// Caster for [FnId::SysPanic]
#[derive(Debug)]
pub struct PanicCaster {}

/// Caster for [FnId::SysHasFunction]
#[derive(Debug)]
pub struct HasFunctionCaster {}

/// Caster for [FnId::SysGetFunction]
#[derive(Debug)]
pub struct GetFunctionCaster {}

/// Caster for [FnId::SysLock]
#[derive(Debug)]
pub struct LockCaster {}

/// Caster for [FnId::SysTryLock]
#[derive(Debug)]
pub struct TryLockCaster {}

/// Caster for [FnId::SysUnlock]
#[derive(Debug)]
pub struct UnlockCaster {}

/// Caster for [FnId::SysGetSyncHandler]
#[derive(Debug)]
pub struct GetSyncHandlerCaster {}

/// Caster for [FnId::SysSetSyncHandler]
#[derive(Debug)]
pub struct SetSyncHandlerCaster {}

transmute_caster!(ShutdownCaster, api::ShutdownFn, FnId::SysShutdown);

transmute_caster!(PanicCaster, api::PanicFn, FnId::SysPanic);

transmute_caster!(HasFunctionCaster, api::HasFunctionFn, FnId::SysHasFunction);

transmute_caster!(GetFunctionCaster, api::GetFunctionFn, FnId::SysGetFunction);

transmute_caster!(LockCaster, api::LockFn, FnId::SysLock);

transmute_caster!(TryLockCaster, api::TryLockFn, FnId::SysTryLock);

transmute_caster!(UnlockCaster, api::UnlockFn, FnId::SysUnlock);

transmute_caster!(
    GetSyncHandlerCaster,
    api::GetSyncHandlerFn,
    FnId::SysGetSyncHandler
);

transmute_caster!(
    SetSyncHandlerCaster,
    api::SetSyncHandlerFn,
    FnId::SysSetSyncHandler
);
