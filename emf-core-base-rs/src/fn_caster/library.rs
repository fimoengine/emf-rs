//! Casters for the library api.
use crate::ffi::library::api;
use crate::ffi::{CBaseFn, FnId};
use crate::fn_caster::FnCaster;

/// Caster for [FnId::LibraryRegisterLoader]
#[derive(Debug)]
pub struct RegisterLoaderCaster {}

/// Caster for [FnId::LibraryUnregisterLoader]
#[derive(Debug)]
pub struct UnregisterLoaderCaster {}

/// Caster for [FnId::LibraryGetLoaderInterface]
#[derive(Debug)]
pub struct GetLoaderInterfaceCaster {}

/// Caster for [FnId::LibraryGetLoaderHandleFromType]
#[derive(Debug)]
pub struct GetLoaderHandleFromTypeCaster {}

/// Caster for [FnId::LibraryGetLoaderHandleFromLibrary]
#[derive(Debug)]
pub struct GetLoaderHandleFromLibraryCaster {}

/// Caster for [FnId::LibraryGetNumLoaders]
#[derive(Debug)]
pub struct GetNumLoadersCaster {}

/// Caster for [FnId::LibraryLibraryExists]
#[derive(Debug)]
pub struct LibraryExistsCaster {}

/// Caster for [FnId::LibraryTypeExists]
#[derive(Debug)]
pub struct TypeExistsCaster {}

/// Caster for [FnId::LibraryGetLibraryTypes]
#[derive(Debug)]
pub struct GetLibraryTypesCaster {}

/// Caster for [FnId::LibraryCreateLibraryHandle]
#[derive(Debug)]
pub struct CreateLibraryHandleCaster {}

/// Caster for [FnId::LibraryRemoveLibraryHandle]
#[derive(Debug)]
pub struct RemoveLibraryHandleCaster {}

/// Caster for [FnId::LibraryLinkLibrary]
#[derive(Debug)]
pub struct LinkLibraryCaster {}

/// Caster for [FnId::LibraryGetInternalLibraryHandle]
#[derive(Debug)]
pub struct GetInternalLibraryHandleCaster {}

/// Caster for [FnId::LibraryLoad]
#[derive(Debug)]
pub struct LoadCaster {}

/// Caster for [FnId::LibraryUnload]
#[derive(Debug)]
pub struct UnloadCaster {}

/// Caster for [FnId::LibraryGetDataSymbol]
#[derive(Debug)]
pub struct GetDataSymbolCaster {}

/// Caster for [FnId::LibraryGetFunctionSymbol]
#[derive(Debug)]
pub struct GetFunctionSymbolCaster {}

transmute_caster!(
    RegisterLoaderCaster,
    api::RegisterLoaderFn,
    FnId::LibraryRegisterLoader
);

transmute_caster!(
    UnregisterLoaderCaster,
    api::UnregisterLoaderFn,
    FnId::LibraryUnregisterLoader
);

transmute_caster!(
    GetLoaderInterfaceCaster,
    api::GetLoaderInterfaceFn,
    FnId::LibraryGetLoaderInterface
);

transmute_caster!(
    GetLoaderHandleFromTypeCaster,
    api::GetLoaderHandleFromTypeFn,
    FnId::LibraryGetLoaderHandleFromType
);

transmute_caster!(
    GetLoaderHandleFromLibraryCaster,
    api::GetLoaderHandleFromLibraryFn,
    FnId::LibraryGetLoaderHandleFromLibrary
);

transmute_caster!(
    GetNumLoadersCaster,
    api::GetNumLoadersFn,
    FnId::LibraryGetNumLoaders
);

transmute_caster!(
    LibraryExistsCaster,
    api::LibraryExistsFn,
    FnId::LibraryLibraryExists
);

transmute_caster!(TypeExistsCaster, api::TypeExistsFn, FnId::LibraryTypeExists);

transmute_caster!(
    GetLibraryTypesCaster,
    api::GetLibraryTypesFn,
    FnId::LibraryGetLibraryTypes
);

transmute_caster!(
    CreateLibraryHandleCaster,
    api::CreateLibraryHandleFn,
    FnId::LibraryCreateLibraryHandle
);

transmute_caster!(
    RemoveLibraryHandleCaster,
    api::RemoveLibraryHandleFn,
    FnId::LibraryRemoveLibraryHandle
);

transmute_caster!(
    LinkLibraryCaster,
    api::LinkLibraryFn,
    FnId::LibraryLinkLibrary
);

transmute_caster!(
    GetInternalLibraryHandleCaster,
    api::GetInternalLibraryHandleFn,
    FnId::LibraryGetInternalLibraryHandle
);

transmute_caster!(LoadCaster, api::LoadFn, FnId::LibraryLoad);

transmute_caster!(UnloadCaster, api::UnloadFn, FnId::LibraryUnload);

transmute_caster!(
    GetDataSymbolCaster,
    api::GetDataSymbolFn,
    FnId::LibraryGetDataSymbol
);

transmute_caster!(
    GetFunctionSymbolCaster,
    api::GetFunctionSymbolFn,
    FnId::LibraryGetFunctionSymbol
);
