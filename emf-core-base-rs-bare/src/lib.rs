//! Idiomatic rust wrapper for the `emf-core-base` interface.
//!
//! This crate provides the function and type definitions specified by the
//! [emf-core-base](https://fimoengine.github.io/emf/emf-core-base/index.html) interface.
//! No implementation for those functions is provided.
//!
//! # Note
//!
//! This crate is intended by the implementors of the `emf-core-base` interface.
#![feature(const_generics)]
#![allow(incomplete_features)]

pub use emf_core_base_rs_ffi_bare as ffi;

mod base_interface_fn;
mod ffi_object;

pub mod library;
pub mod sys;
pub mod version;

pub use base_interface_fn::BaseInterfaceFn;
pub use ffi::FnId;
pub use ffi_object::{FFIObject, FromFFI, IntoFFI};

#[cfg(feature = "global_api")]
pub use sys::GlobalToken;
pub use sys::LocalToken;
