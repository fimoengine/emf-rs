#![feature(const_fn_transmute)]
#![feature(const_mut_refs)]

mod bindings;
mod interface_binding;

pub use bindings::initialize_base_binding;
pub use emf_core_base_rs_ffi_bare::*;
pub use interface_binding::InterfaceBinding;
