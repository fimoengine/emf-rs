//! Rust wrapper for the `emf-core-base` interface.
//!
//! This crate provides the function and type definitions specified by the
//! [emf-core-base](https://fimoengine.github.io/emf/emf-core-base/index.html) interface.
//! No implementation for those functions is provided.
//!
//! # Note
//!
//! This crate is intended to be used by the implementors of the `emf-core-base` interface.

pub use boolean::Bool;
pub use fn_id::FnId;
pub use interface::{BaseInterface, BASE_INTERFACE_NAME};
pub use interface_binding::InterfaceBinding;

mod boolean;
mod fn_id;
mod interface;
mod interface_binding;

pub mod containers;
pub mod fn_ptr;
pub mod library;
pub mod module;
pub mod sys;
pub mod version;

/// An opaque structure representing the `emf-core-base` interface.
#[repr(C)]
pub struct BaseT {
    _private: [u8; 0],
}

include!(concat!(env!("OUT_DIR"), "/versions.rs"));
