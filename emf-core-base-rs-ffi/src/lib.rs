//! Rust bindings for the `emf-core-base` interface.
//!
//! This crate provides the function and type definitions specified by the
//! [emf-core-base](https://fimoengine.github.io/emf/emf-core-base/index.html) interface.
//! No implementation for those functions is provided.

#![feature(const_fn_transmute)]
#![feature(const_mut_refs)]

pub use bindings::initialize_base_binding;
pub use emf_core_base_rs_ffi_bare::*;
pub use interface_loader::InterfaceLoader;

mod bindings;
mod interface_loader;
