//! Bootstrapping of the binding.
#![allow(dead_code)]

use crate::containers::{NonNullConst, Span};
use crate::module::{InterfaceDescriptor, InterfaceExtension, ModuleName};
use crate::version::Version;
use crate::{BaseInterface, FnId, BASE_INTERFACE_NAME};
use emf_core_base_rs_ffi_bare::fn_ptr::{
    ModuleGetExportedInterfaceHandleFn, ModuleGetInterfaceFn, SysGetFunctionFn, SysPanicFn,
};
use emf_core_base_rs_ffi_bare::BaseT;
use std::ffi::CString;
use std::marker::PhantomData;
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::ptr::null;

#[cfg(test)]
mod tests;

#[repr(C)]
#[derive(Copy, Clone)]
struct ModuleNameInternal {
    pub data: [c_char; 32],
    pub length: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct InterfaceExtensionSpanInternal {
    pub data: *const InterfaceExtension,
    pub length: usize,
    pub phantom: PhantomData<&'static InterfaceExtension>,
}

const fn recursive_copy(buff: &mut [c_char; 32], i: usize) {
    if i < BASE_INTERFACE_NAME.len() {
        buff[i] = BASE_INTERFACE_NAME.as_bytes()[i] as c_char;
        recursive_copy(buff, i + 1);
    }
}

const fn generate_const_module_name() -> ModuleName {
    unsafe {
        let mut name = ModuleNameInternal {
            data: [0; 32],
            length: 0,
        };

        recursive_copy(&mut name.data, 0);
        name.length = BASE_INTERFACE_NAME.len();
        std::mem::transmute(name)
    }
}

const fn generate_const_empty_extensions() -> Span<'static, InterfaceExtension> {
    unsafe {
        let ext_span = InterfaceExtensionSpanInternal {
            data: null(),
            length: 0,
            phantom: PhantomData,
        };
        std::mem::transmute(ext_span)
    }
}

const BASE_VERSION: Version = Version {
    major: crate::VERSION_MAJOR,
    minor: crate::VERSION_MINOR,
    patch: crate::VERSION_PATCH,
    build_number: crate::VERSION_BUILD_NUMBER,
    release_number: crate::VERSION_RELEASE_NUMBER,
    release_type: crate::VERSION_RELEASE_TYPE,
};

const BASE_INTERFACE_DESC: InterfaceDescriptor<'static> = InterfaceDescriptor {
    name: generate_const_module_name(),
    version: BASE_VERSION,
    extensions: generate_const_empty_extensions(),
};

pub static mut BASE_INTERFACE: MaybeUninit<&mut BaseInterface> = MaybeUninit::uninit();

/// Initializes the binding to the `emf-core-base` interface.
pub fn initialize_base_binding(base_module: *mut BaseT, get_function_fn: SysGetFunctionFn) {
    unsafe {
        let panic_fn: SysPanicFn = match get_function_fn(base_module, FnId::SysPanic).to_native() {
            Some(func) => std::mem::transmute(func),
            None => panic!(),
        };

        let module_get_exported_interface_handle_fn: ModuleGetExportedInterfaceHandleFn =
            match get_function_fn(base_module, FnId::ModuleGetExportedInterfaceHandle).to_native() {
                Some(func) => std::mem::transmute(func),
                None => {
                    let error = CString::new(
                        "Could not fetch the function pointer to `emf_cbase_module_get_exported_interface_handle`"
                    ).unwrap();
                    panic_fn(base_module, error.as_ptr());
                }
            };

        let module_get_interface_fn: ModuleGetInterfaceFn =
            match get_function_fn(base_module, FnId::ModuleGetInterface).to_native() {
                Some(func) => std::mem::transmute(func),
                None => {
                    let error = CString::new(
                        "Could not fetch the function pointer to `emf_cbase_module_get_interface`",
                    )
                    .unwrap();
                    panic_fn(base_module, error.as_ptr());
                }
            };

        let interface_handle = match module_get_exported_interface_handle_fn(
            base_module,
            NonNullConst::from(&BASE_INTERFACE_DESC),
        )
        .to_native()
        {
            Ok(handle) => handle,
            Err(_) => {
                let error = CString::new(format!(
                    "Could not fetch the handle to the '{}' interface module",
                    BASE_INTERFACE_NAME
                ))
                .unwrap();
                panic_fn(base_module, error.as_ptr());
            }
        };

        match module_get_interface_fn(
            base_module,
            interface_handle,
            NonNullConst::from(&BASE_INTERFACE_DESC),
        )
        .to_native()
        {
            Ok(interface) => {
                let base_interface: &'static mut BaseInterface =
                    std::mem::transmute(interface.interface.cast::<BaseInterface>().as_mut());
                BASE_INTERFACE = MaybeUninit::new(base_interface);
            }
            Err(_) => {
                let error = CString::new(format!(
                    "Could not initialize the bindings to the '{}' interface",
                    BASE_INTERFACE_NAME
                ))
                .unwrap();
                panic_fn(base_module, error.as_ptr());
            }
        };
    }
}
