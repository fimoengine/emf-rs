#![feature(const_fn_transmute)]
#![feature(const_mut_refs)]

pub use bindings::initialize_base_binding;
pub use emf_core_base_rs_ffi_bare::*;
pub use interface_loader::InterfaceLoader;

mod bindings;
mod interface_loader;
