//! Idiomatic rust wrapper for the `emf-core-base` interface.
//!
//! This crate provides the function and type definitions specified by the
//! [emf-core-base](https://fimoengine.github.io/emf/emf-core-base/index.html) interface.
//! No implementation for those functions is provided.
//!
//! # Note
//!
//! This crate is intended by the implementors of the `emf-core-base` interface.

pub use emf_core_base_rs_ffi_bare as ffi;

pub mod version;
