//! Idiomatic Rust bindings to the `emf-core-base` interface.
//!
//! This crate provides the function and type definitions of the
//! [emf-core-base](https://github.com/fimoengine/emf/tree/main/emf_core_base) interface.
#![feature(const_fn_fn_ptr_basics)]
#![feature(const_fn_trait_bound)]
#![feature(specialization)]
#![feature(c_unwind)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    missing_debug_implementations,
    broken_intra_doc_links
)]
#![allow(incomplete_features)]
pub use emf_core_base_rs_ffi as ffi;

mod cbase;
mod error;
mod fat_ptr;
mod to_os_path_buff;

#[cfg(feature = "init")]
mod init;

pub mod fn_caster;
pub mod library;
pub mod module;
pub mod ownership;
pub mod sys;
pub mod version;

#[cfg(feature = "global_api")]
pub mod global;

#[cfg(feature = "extensions")]
pub mod extensions;

pub use cbase::{CBase, CBaseAPI, CBaseAccess, CBaseInterfaceInfo, CBaseRef};
pub use error::{Error, ErrorInfo};
pub use init::CBaseAPILoader;
pub use to_os_path_buff::ToOsPathBuff;
