//! Casters for the module api.
use crate::ffi::module::api;
use crate::ffi::{CBaseFn, FnId};
use crate::fn_caster::FnCaster;

/// Caster for [FnId::ModuleRegisterLoader]
#[derive(Debug)]
pub struct RegisterLoaderCaster {}

/// Caster for [FnId::ModuleUnregisterLoader]
#[derive(Debug)]
pub struct UnregisterLoaderCaster {}

/// Caster for [FnId::ModuleGetLoaderInterface]
#[derive(Debug)]
pub struct GetLoaderInterfaceCaster {}

/// Caster for [FnId::ModuleGetLoaderHandleFromType]
#[derive(Debug)]
pub struct GetLoaderHandleFromTypeCaster {}

/// Caster for [FnId::ModuleGetLoaderHandleFromModule]
#[derive(Debug)]
pub struct GetLoaderHandleFromModuleCaster {}

/// Caster for [FnId::ModuleGetNumModules]
#[derive(Debug)]
pub struct GetNumModulesCaster {}

/// Caster for [FnId::ModuleGetNumLoaders]
#[derive(Debug)]
pub struct GetNumLoadersCaster {}

/// Caster for [FnId::ModuleGetNumExportedInterfaces]
#[derive(Debug)]
pub struct GetNumExportedInterfacesCaster {}

/// Caster for [FnId::ModuleModuleExists]
#[derive(Debug)]
pub struct ModuleExistsCaster {}

/// Caster for [FnId::ModuleTypeExists]
#[derive(Debug)]
pub struct TypeExistsCaster {}

/// Caster for [FnId::ModuleExportedInterfaceExists]
#[derive(Debug)]
pub struct ExportedInterfaceExistsCaster {}

/// Caster for [FnId::ModuleGetModules]
#[derive(Debug)]
pub struct GetModulesCaster {}

/// Caster for [FnId::ModuleGetModuleTypes]
#[derive(Debug)]
pub struct GetModuleTypesCaster {}

/// Caster for [FnId::ModuleGetExportedInterfaces]
#[derive(Debug)]
pub struct GetExportedInterfacesCaster {}

/// Caster for [FnId::ModuleGetExportedInterfaceHandle]
#[derive(Debug)]
pub struct GetExportedInterfaceHandleCaster {}

/// Caster for [FnId::ModuleCreateModuleHandle]
#[derive(Debug)]
pub struct CreateModuleHandleCaster {}

/// Caster for [FnId::ModuleRemoveModuleHandle]
#[derive(Debug)]
pub struct RemoveModuleHandleCaster {}

/// Caster for [FnId::ModuleLinkModule]
#[derive(Debug)]
pub struct LinkModuleCaster {}

/// Caster for [FnId::ModuleGetInternalModuleHandle]
#[derive(Debug)]
pub struct GetInternalModuleHandleCaster {}

/// Caster for [FnId::ModuleAddModule]
#[derive(Debug)]
pub struct AddModuleCaster {}

/// Caster for [FnId::ModuleRemoveModule]
#[derive(Debug)]
pub struct RemoveModuleCaster {}

/// Caster for [FnId::ModuleLoad]
#[derive(Debug)]
pub struct LoadCaster {}

/// Caster for [FnId::ModuleUnload]
#[derive(Debug)]
pub struct UnloadCaster {}

/// Caster for [FnId::ModuleInitialize]
#[derive(Debug)]
pub struct InitializeCaster {}

/// Caster for [FnId::ModuleTerminate]
#[derive(Debug)]
pub struct TerminateCaster {}

/// Caster for [FnId::ModuleAddDependency]
#[derive(Debug)]
pub struct AddDependencyCaster {}

/// Caster for [FnId::ModuleRemoveDependency]
#[derive(Debug)]
pub struct RemoveDependencyCaster {}

/// Caster for [FnId::ModuleExportInterface]
#[derive(Debug)]
pub struct ExportInterfaceCaster {}

/// Caster for [FnId::ModuleGetLoadDependencies]
#[derive(Debug)]
pub struct GetLoadDependenciesCaster {}

/// Caster for [FnId::ModuleGetRuntimeDependencies]
#[derive(Debug)]
pub struct GetRuntimeDependenciesCaster {}

/// Caster for [FnId::ModuleGetExportableInterfaces]
#[derive(Debug)]
pub struct GetExportableInterfacesCaster {}

/// Caster for [FnId::ModuleFetchStatus]
#[derive(Debug)]
pub struct FetchStatusCaster {}

/// Caster for [FnId::ModuleGetModulePath]
#[derive(Debug)]
pub struct GetModulePathCaster {}

/// Caster for [FnId::ModuleGetModuleInfo]
#[derive(Debug)]
pub struct GetModuleInfoCaster {}

/// Caster for [FnId::ModuleGetInterface]
#[derive(Debug)]
pub struct GetInterfaceCaster {}

transmute_caster!(
    RegisterLoaderCaster,
    api::RegisterLoaderFn,
    FnId::ModuleRegisterLoader
);

transmute_caster!(
    UnregisterLoaderCaster,
    api::UnregisterLoaderFn,
    FnId::ModuleUnregisterLoader
);

transmute_caster!(
    GetLoaderInterfaceCaster,
    api::GetLoaderInterfaceFn,
    FnId::ModuleGetLoaderInterface
);

transmute_caster!(
    GetLoaderHandleFromTypeCaster,
    api::GetLoaderHandleFromTypeFn,
    FnId::ModuleGetLoaderHandleFromType
);

transmute_caster!(
    GetLoaderHandleFromModuleCaster,
    api::GetLoaderHandleFromModuleFn,
    FnId::ModuleGetLoaderHandleFromModule
);

transmute_caster!(
    GetNumModulesCaster,
    api::GetNumModulesFn,
    FnId::ModuleGetNumModules
);

transmute_caster!(
    GetNumLoadersCaster,
    api::GetNumLoadersFn,
    FnId::ModuleGetNumLoaders
);

transmute_caster!(
    GetNumExportedInterfacesCaster,
    api::GetNumExportedInterfacesFn,
    FnId::ModuleGetNumExportedInterfaces
);

transmute_caster!(
    ModuleExistsCaster,
    api::ModuleExistsFn,
    FnId::ModuleModuleExists
);

transmute_caster!(TypeExistsCaster, api::TypeExistsFn, FnId::ModuleTypeExists);

transmute_caster!(
    ExportedInterfaceExistsCaster,
    api::ExportedInterfaceExistsFn,
    FnId::ModuleExportedInterfaceExists
);

transmute_caster!(GetModulesCaster, api::GetModulesFn, FnId::ModuleGetModules);

transmute_caster!(
    GetModuleTypesCaster,
    api::GetModuleTypesFn,
    FnId::ModuleGetModuleTypes
);

transmute_caster!(
    GetExportedInterfacesCaster,
    api::GetExportedInterfacesFn,
    FnId::ModuleGetExportedInterfaces
);

transmute_caster!(
    GetExportedInterfaceHandleCaster,
    api::GetExportedInterfaceHandleFn,
    FnId::ModuleGetExportedInterfaceHandle
);

transmute_caster!(
    CreateModuleHandleCaster,
    api::CreateModuleHandleFn,
    FnId::ModuleCreateModuleHandle
);

transmute_caster!(
    RemoveModuleHandleCaster,
    api::RemoveModuleHandleFn,
    FnId::ModuleRemoveModuleHandle
);

transmute_caster!(LinkModuleCaster, api::LinkModuleFn, FnId::ModuleLinkModule);

transmute_caster!(
    GetInternalModuleHandleCaster,
    api::GetInternalModuleHandleFn,
    FnId::ModuleGetInternalModuleHandle
);

transmute_caster!(AddModuleCaster, api::AddModuleFn, FnId::ModuleAddModule);

transmute_caster!(
    RemoveModuleCaster,
    api::RemoveModuleFn,
    FnId::ModuleRemoveModule
);

transmute_caster!(LoadCaster, api::LoadFn, FnId::ModuleLoad);

transmute_caster!(UnloadCaster, api::UnloadFn, FnId::ModuleUnload);

transmute_caster!(InitializeCaster, api::InitializeFn, FnId::ModuleInitialize);

transmute_caster!(TerminateCaster, api::TerminateFn, FnId::ModuleTerminate);

transmute_caster!(
    AddDependencyCaster,
    api::AddDependencyFn,
    FnId::ModuleAddDependency
);

transmute_caster!(
    RemoveDependencyCaster,
    api::RemoveDependencyFn,
    FnId::ModuleRemoveDependency
);

transmute_caster!(
    ExportInterfaceCaster,
    api::ExportInterfaceFn,
    FnId::ModuleExportInterface
);

transmute_caster!(
    GetLoadDependenciesCaster,
    api::GetLoadDependenciesFn,
    FnId::ModuleGetLoadDependencies
);

transmute_caster!(
    GetRuntimeDependenciesCaster,
    api::GetRuntimeDependenciesFn,
    FnId::ModuleGetRuntimeDependencies
);

transmute_caster!(
    GetExportableInterfacesCaster,
    api::GetExportableInterfacesFn,
    FnId::ModuleGetExportableInterfaces
);

transmute_caster!(
    FetchStatusCaster,
    api::FetchStatusFn,
    FnId::ModuleFetchStatus
);

transmute_caster!(
    GetModulePathCaster,
    api::GetModulePathFn,
    FnId::ModuleGetModulePath
);

transmute_caster!(
    GetModuleInfoCaster,
    api::GetModuleInfoFn,
    FnId::ModuleGetModuleInfo
);

transmute_caster!(
    GetInterfaceCaster,
    api::GetInterfaceFn,
    FnId::ModuleGetInterface
);
