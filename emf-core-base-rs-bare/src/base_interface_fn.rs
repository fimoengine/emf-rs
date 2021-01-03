//! Definition of `BaseInterfaceFn`

use crate::ffi;
use crate::FnId;

/// An utility trait to identify function pointer types.
pub trait BaseInterfaceFn<const ID: FnId> {
    type Type;

    /// Casts the function to the right type.
    ///
    /// # Safety
    ///
    /// Casting a pointer is inherently unsafe.
    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type;
}

// `Sys` api

impl BaseInterfaceFn<{ FnId::SysLock }> for ffi::fn_ptr::SysLockFn {
    type Type = ffi::fn_ptr::SysLockFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::SysTryLock }> for ffi::fn_ptr::SysTryLockFn {
    type Type = ffi::fn_ptr::SysTryLockFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::SysUnlock }> for ffi::fn_ptr::SysUnlockFn {
    type Type = ffi::fn_ptr::SysUnlockFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::SysShutdown }> for ffi::fn_ptr::SysShutdownFn {
    type Type = ffi::fn_ptr::SysShutdownFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::SysPanic }> for ffi::fn_ptr::SysPanicFn {
    type Type = ffi::fn_ptr::SysPanicFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::SysHasFunction }> for ffi::fn_ptr::SysHasFunctionFn {
    type Type = ffi::fn_ptr::SysHasFunctionFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::SysGetFunction }> for ffi::fn_ptr::SysGetFunctionFn {
    type Type = ffi::fn_ptr::SysGetFunctionFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::SysGetSyncHandler }> for ffi::fn_ptr::SysGetSyncHandlerFn {
    type Type = ffi::fn_ptr::SysGetSyncHandlerFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::SysSetSyncHandler }> for ffi::fn_ptr::SysSetSyncHandlerFn {
    type Type = ffi::fn_ptr::SysSetSyncHandlerFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

// `Version` api.

impl BaseInterfaceFn<{ FnId::VersionConstructShort }> for ffi::fn_ptr::VersionConstructShortFn {
    type Type = ffi::fn_ptr::VersionConstructShortFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::VersionConstructLong }> for ffi::fn_ptr::VersionConstructLongFn {
    type Type = ffi::fn_ptr::VersionConstructLongFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::VersionConstructFull }> for ffi::fn_ptr::VersionConstructFullFn {
    type Type = ffi::fn_ptr::VersionConstructFullFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::VersionConstructFromString }>
    for ffi::fn_ptr::VersionConstructFromStringFn
{
    type Type = ffi::fn_ptr::VersionConstructFromStringFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::VersionRepresentationIsValid }>
    for ffi::fn_ptr::VersionRepresentationIsValidFn
{
    type Type = ffi::fn_ptr::VersionRepresentationIsValidFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::VersionGetShortRepresentation }>
    for ffi::fn_ptr::VersionGetShortRepresentationFn
{
    type Type = ffi::fn_ptr::VersionGetShortRepresentationFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::VersionGetShortRepresentationLength }>
    for ffi::fn_ptr::VersionGetShortRepresentationLengthFn
{
    type Type = ffi::fn_ptr::VersionGetShortRepresentationLengthFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::VersionGetLongRepresentation }>
    for ffi::fn_ptr::VersionGetLongRepresentationFn
{
    type Type = ffi::fn_ptr::VersionGetLongRepresentationFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::VersionGetLongRepresentationLength }>
    for ffi::fn_ptr::VersionGetLongRepresentationLengthFn
{
    type Type = ffi::fn_ptr::VersionGetLongRepresentationLengthFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::VersionGetFullRepresentation }>
    for ffi::fn_ptr::VersionGetFullRepresentationFn
{
    type Type = ffi::fn_ptr::VersionGetFullRepresentationFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::VersionGetFullRepresentationLength }>
    for ffi::fn_ptr::VersionGetFullRepresentationLengthFn
{
    type Type = ffi::fn_ptr::VersionGetFullRepresentationLengthFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::VersionCompare }> for ffi::fn_ptr::VersionCompareFn {
    type Type = ffi::fn_ptr::VersionCompareFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::VersionCompareWeak }> for ffi::fn_ptr::VersionCompareWeakFn {
    type Type = ffi::fn_ptr::VersionCompareWeakFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::VersionCompareStrong }> for ffi::fn_ptr::VersionCompareStrongFn {
    type Type = ffi::fn_ptr::VersionCompareStrongFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::VersionIsCompatible }> for ffi::fn_ptr::VersionIsCompatibleFn {
    type Type = ffi::fn_ptr::VersionIsCompatibleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

// `Library` api.

impl BaseInterfaceFn<{ FnId::LibraryRegisterLoader }> for ffi::fn_ptr::LibraryRegisterLoaderFn {
    type Type = ffi::fn_ptr::LibraryRegisterLoaderFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryUnregisterLoader }> for ffi::fn_ptr::LibraryUnregisterLoaderFn {
    type Type = ffi::fn_ptr::LibraryUnregisterLoaderFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryGetNumLoaders }> for ffi::fn_ptr::LibraryGetNumLoadersFn {
    type Type = ffi::fn_ptr::LibraryGetNumLoadersFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryGetLibraryTypes }> for ffi::fn_ptr::LibraryGetLibraryTypesFn {
    type Type = ffi::fn_ptr::LibraryGetLibraryTypesFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryGetLoaderHandle }> for ffi::fn_ptr::LibraryGetLoaderHandleFn {
    type Type = ffi::fn_ptr::LibraryGetLoaderHandleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryTypeExists }> for ffi::fn_ptr::LibraryTypeExistsFn {
    type Type = ffi::fn_ptr::LibraryTypeExistsFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryLibraryExists }> for ffi::fn_ptr::LibraryLibraryExistsFn {
    type Type = ffi::fn_ptr::LibraryLibraryExistsFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryUnsafeCreateLibraryHandle }>
    for ffi::fn_ptr::LibraryUnsafeCreateLibraryHandleFn
{
    type Type = ffi::fn_ptr::LibraryUnsafeCreateLibraryHandleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryUnsafeRemoveLibraryHandle }>
    for ffi::fn_ptr::LibraryUnsafeRemoveLibraryHandleFn
{
    type Type = ffi::fn_ptr::LibraryUnsafeRemoveLibraryHandleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryUnsafeLinkLibrary }>
    for ffi::fn_ptr::LibraryUnsafeLinkLibraryFn
{
    type Type = ffi::fn_ptr::LibraryUnsafeLinkLibraryFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryUnsafeGetLoaderLibraryHandle }>
    for ffi::fn_ptr::LibraryUnsafeGetLoaderLibraryHandleFn
{
    type Type = ffi::fn_ptr::LibraryUnsafeGetLoaderLibraryHandleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryUnsafeGetLoaderHandle }>
    for ffi::fn_ptr::LibraryUnsafeGetLoaderHandleFn
{
    type Type = ffi::fn_ptr::LibraryUnsafeGetLoaderHandleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryUnsafeGetLoaderInterface }>
    for ffi::fn_ptr::LibraryUnsafeGetLoaderInterfaceFn
{
    type Type = ffi::fn_ptr::LibraryUnsafeGetLoaderInterfaceFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryLoad }> for ffi::fn_ptr::LibraryLoadFn {
    type Type = ffi::fn_ptr::LibraryLoadFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryUnload }> for ffi::fn_ptr::LibraryUnloadFn {
    type Type = ffi::fn_ptr::LibraryUnloadFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryGetDataSymbol }> for ffi::fn_ptr::LibraryGetDataSymbolFn {
    type Type = ffi::fn_ptr::LibraryGetDataSymbolFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::LibraryGetFunctionSymbol }>
    for ffi::fn_ptr::LibraryGetFunctionSymbolFn
{
    type Type = ffi::fn_ptr::LibraryGetFunctionSymbolFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

// `Module` api.

impl BaseInterfaceFn<{ FnId::ModuleRegisterLoader }> for ffi::fn_ptr::ModuleRegisterLoaderFn {
    type Type = ffi::fn_ptr::ModuleRegisterLoaderFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleUnregisterLoader }> for ffi::fn_ptr::ModuleUnregisterLoaderFn {
    type Type = ffi::fn_ptr::ModuleUnregisterLoaderFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleGetNumLoaders }> for ffi::fn_ptr::ModuleGetNumLoadersFn {
    type Type = ffi::fn_ptr::ModuleGetNumLoadersFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleGetModuleTypes }> for ffi::fn_ptr::ModuleGetModuleTypesFn {
    type Type = ffi::fn_ptr::ModuleGetModuleTypesFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleGetNumModules }> for ffi::fn_ptr::ModuleGetNumModulesFn {
    type Type = ffi::fn_ptr::ModuleGetNumModulesFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleGetModules }> for ffi::fn_ptr::ModuleGetModulesFn {
    type Type = ffi::fn_ptr::ModuleGetModulesFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleGetNumExportedInterfaces }>
    for ffi::fn_ptr::ModuleGetNumExportedInterfacesFn
{
    type Type = ffi::fn_ptr::ModuleGetNumExportedInterfacesFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleGetExportedInterfaces }>
    for ffi::fn_ptr::ModuleGetExportedInterfacesFn
{
    type Type = ffi::fn_ptr::ModuleGetExportedInterfacesFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleGetLoaderHandle }> for ffi::fn_ptr::ModuleGetLoaderHandleFn {
    type Type = ffi::fn_ptr::ModuleGetLoaderHandleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleTypeExists }> for ffi::fn_ptr::ModuleTypeExistsFn {
    type Type = ffi::fn_ptr::ModuleTypeExistsFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleModuleExists }> for ffi::fn_ptr::ModuleModuleExistsFn {
    type Type = ffi::fn_ptr::ModuleModuleExistsFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleGetExportedInterfaceHandle }>
    for ffi::fn_ptr::ModuleGetExportedInterfaceHandleFn
{
    type Type = ffi::fn_ptr::ModuleGetExportedInterfaceHandleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleExportedInterfaceExists }>
    for ffi::fn_ptr::ModuleExportedInterfaceExistsFn
{
    type Type = ffi::fn_ptr::ModuleExportedInterfaceExistsFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleUnsafeCreateModuleHandle }>
    for ffi::fn_ptr::ModuleUnsafeCreateModuleHandleFn
{
    type Type = ffi::fn_ptr::ModuleUnsafeCreateModuleHandleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleUnsafeRemoveModuleHandle }>
    for ffi::fn_ptr::ModuleUnsafeRemoveModuleHandleFn
{
    type Type = ffi::fn_ptr::ModuleUnsafeRemoveModuleHandleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleUnsafeLinkModule }> for ffi::fn_ptr::ModuleUnsafeLinkModuleFn {
    type Type = ffi::fn_ptr::ModuleUnsafeLinkModuleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleUnsafeGetLoaderModuleHandle }>
    for ffi::fn_ptr::ModuleUnsafeGetLoaderModuleHandleFn
{
    type Type = ffi::fn_ptr::ModuleUnsafeGetLoaderModuleHandleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleUnsafeGetLoaderHandle }>
    for ffi::fn_ptr::ModuleUnsafeGetLoaderHandleFn
{
    type Type = ffi::fn_ptr::ModuleUnsafeGetLoaderHandleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleUnsafeGetLoader }> for ffi::fn_ptr::ModuleUnsafeGetLoaderFn {
    type Type = ffi::fn_ptr::ModuleUnsafeGetLoaderFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleAddModule }> for ffi::fn_ptr::ModuleAddModuleFn {
    type Type = ffi::fn_ptr::ModuleAddModuleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleRemoveModule }> for ffi::fn_ptr::ModuleRemoveModuleFn {
    type Type = ffi::fn_ptr::ModuleRemoveModuleFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleGetLoadDependencies }>
    for ffi::fn_ptr::ModuleGetLoadDependenciesFn
{
    type Type = ffi::fn_ptr::ModuleGetLoadDependenciesFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleFetchStatus }> for ffi::fn_ptr::ModuleFetchStatusFn {
    type Type = ffi::fn_ptr::ModuleFetchStatusFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleAddDependency }> for ffi::fn_ptr::ModuleAddDependencyFn {
    type Type = ffi::fn_ptr::ModuleAddDependencyFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleRemoveDependency }> for ffi::fn_ptr::ModuleRemoveDependencyFn {
    type Type = ffi::fn_ptr::ModuleRemoveDependencyFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleExportInterface }> for ffi::fn_ptr::ModuleExportInterfaceFn {
    type Type = ffi::fn_ptr::ModuleExportInterfaceFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleLoad }> for ffi::fn_ptr::ModuleLoadFn {
    type Type = ffi::fn_ptr::ModuleLoadFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleUnload }> for ffi::fn_ptr::ModuleUnloadFn {
    type Type = ffi::fn_ptr::ModuleUnloadFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleInitialize }> for ffi::fn_ptr::ModuleInitializeFn {
    type Type = ffi::fn_ptr::ModuleInitializeFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleTerminate }> for ffi::fn_ptr::ModuleTerminateFn {
    type Type = ffi::fn_ptr::ModuleTerminateFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleGetModuleInfo }> for ffi::fn_ptr::ModuleGetModuleInfoFn {
    type Type = ffi::fn_ptr::ModuleGetModuleInfoFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleGetExportableInterfaces }>
    for ffi::fn_ptr::ModuleGetExportableInterfacesFn
{
    type Type = ffi::fn_ptr::ModuleGetExportableInterfacesFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleGetRuntimeDependencies }>
    for ffi::fn_ptr::ModuleGetRuntimeDependenciesFn
{
    type Type = ffi::fn_ptr::ModuleGetRuntimeDependenciesFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleGetInterface }> for ffi::fn_ptr::ModuleGetInterfaceFn {
    type Type = ffi::fn_ptr::ModuleGetInterfaceFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}

impl BaseInterfaceFn<{ FnId::ModuleGetModulePath }> for ffi::fn_ptr::ModuleGetModulePathFn {
    type Type = ffi::fn_ptr::ModuleGetModulePathFn;

    unsafe fn cast(f: ffi::fn_ptr::BaseFn) -> Self::Type {
        std::mem::transmute(f)
    }
}
