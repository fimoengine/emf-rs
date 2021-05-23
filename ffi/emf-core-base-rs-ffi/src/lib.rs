//! Rust bindings to the `emf-core-base` interface.
//!
//! This crate provides the function and type definitions of the
//! [emf-core-base](https://github.com/fimoengine/emf/tree/main/emf_core_base) interface.
//!
//! # Multithreading
//!
//! Most of the interface is not thread-safe and must be manually synchronised with
//! [sys::api::SysBinding::lock] or [sys::api::SysBinding::try_lock].
#![feature(c_unwind)]
mod boolean;
mod cbase;
mod fn_id;
mod type_wrapper;

#[cfg(feature = "init")]
mod init;

#[cfg(feature = "extensions")]
pub mod extensions;

pub mod collections;
pub mod errors;
pub mod library;
pub mod module;
pub mod sys;
pub mod version;
pub use boolean::Bool;
pub use cbase::{
    CBase, CBaseBinding, CBaseFn, CBaseInterface, CBaseInterfaceVTable, CBASE_INTERFACE_NAME,
};
pub use fn_id::FnId;
pub use type_wrapper::TypeWrapper;

#[cfg(feature = "init")]
pub use init::CBaseLoader;
