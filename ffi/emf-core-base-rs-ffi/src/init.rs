use crate::collections::{ConstSpan, NonNullConst};
use crate::module::{api as mod_api, InterfaceDescriptor, InterfaceName};
use crate::sys::api as sys_api;
use crate::version::{
    Version, VERSION_BUILD, VERSION_MAJOR, VERSION_MINOR, VERSION_PATCH, VERSION_RELEASE_NUMBER,
    VERSION_RELEASE_TYPE,
};
use crate::{CBase, CBaseBinding, CBaseInterface, FnId, CBASE_INTERFACE_NAME};
use std::ptr::NonNull;

/// Types that can fetch an `emf-core-base` interface.
pub trait CBaseLoader {
    /// Type of the interface.
    type Interface: CBaseBinding;

    /// Fetches the `emf-core-base` interface.
    ///
    /// # Safety
    ///
    /// The parameter `get_function_fn` must be able to accept `base_module`.
    ///
    /// # Panics
    ///
    /// This function panics if it can not fetch the interface.
    unsafe fn fetch_interface(
        base_module: Option<NonNull<CBase>>,
        get_function_fn: sys_api::GetFunctionFn,
    ) -> NonNullConst<Self::Interface>;
}

impl CBaseLoader for CBaseInterface {
    type Interface = Self;

    unsafe fn fetch_interface(
        base_module: Option<NonNull<CBase>>,
        get_function_fn: sys_api::GetFunctionFn,
    ) -> NonNullConst<Self::Interface> {
        let panic_fn: sys_api::PanicFn =
            match get_function_fn(base_module, FnId::SysPanic).to_option() {
                None => panic!("Unable to fetch the interface"),
                Some(func) => std::mem::transmute(func),
            };

        let get_exported_interface_handle_fn: mod_api::GetExportedInterfaceHandleFn =
            match get_function_fn(base_module, FnId::ModuleGetExportedInterfaceHandle).to_option() {
                None => {
                    let error = b"Could not fetch the function pointer to `FnId::ModuleGetExportedInterfaceHandle`\0";
                    panic_fn(base_module, Some(NonNullConst::from(error)))
                }
                Some(func) => std::mem::transmute(func),
            };

        let get_interface_fn: mod_api::GetInterfaceFn =
            match get_function_fn(base_module, FnId::ModuleGetInterface).to_option() {
                None => {
                    let error =
                        b"Could not fetch the function pointer to `FnId::ModuleGetInterface`\0";
                    panic_fn(base_module, Some(NonNullConst::from(error)))
                }
                Some(func) => std::mem::transmute(func),
            };

        let cbase_interface_desc = InterfaceDescriptor {
            name: InterfaceName::from(CBASE_INTERFACE_NAME),
            version: Version {
                major: VERSION_MAJOR,
                minor: VERSION_MINOR,
                patch: VERSION_PATCH,
                build: VERSION_BUILD,
                release_number: VERSION_RELEASE_NUMBER,
                release_type: VERSION_RELEASE_TYPE,
            },
            extensions: ConstSpan::new(),
        };

        let module_handle = match get_exported_interface_handle_fn(
            base_module,
            NonNullConst::from(&cbase_interface_desc),
        )
        .to_result()
        {
            Ok(handle) => handle,
            Err(_) => {
                let error = b"Could not fetch the handle to the interface module.\0";
                panic_fn(base_module, Some(NonNullConst::from(error)))
            }
        };

        match get_interface_fn(
            base_module,
            module_handle,
            NonNullConst::from(&cbase_interface_desc),
        )
        .to_result()
        {
            Ok(interface) => NonNullConst::from(interface.interface).cast(),
            Err(_) => {
                let error = b"Could not fetch the interface from the module.\0";
                panic_fn(base_module, Some(NonNullConst::from(error)))
            }
        }
    }
}
