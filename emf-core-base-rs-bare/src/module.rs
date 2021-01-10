//! A collection of utilities for loading modules.
//!
//! The `module` api is built on top of the `library` api and specifies the loading and unloading
//! of `modules`. A `module` is a collection libraries and resources, that together form an
//! independent unit. Similarly to the `library` api, the loading and unloading of modules is
//! implemented by `module loaders`, which are associated to one `module type`.
//!
//! ## Loaders
//!
//! `Module loaders` implement the actual loading and unloading of modules. Custom `module loaders`
//! can be added by constructing a [ModuleLoaderWrapper] and calling the
//! [ModuleToken::register_loader()] function.
//!
//! ## Module types
//!
//! The type of a module identifies a `module loader` capable of loading a specific module.
//! A `module type` is represented by a [ModuleType].
//!
//! # Module structure
//!
//! > module/module.json\
//! > module/...\
//! > ...\
//!
//! A `module` is a simple directory containing all the required resources.
//! The internal structure of the `module` is defined by the respective `module loader`.
//! The only requirement is the existence of the `module.json` file at the root of the module.
//! This file is the `module manifest` and specifies that the directory is indeed a module.
//!
//! # Module manifest
//!
//! The `module manifest` identifies a module and specifies how a module can be loaded.
//! To allow for backwards (and forwards) compatibility, the version of the `manifest schema`
//! is saved in the manifest with the key `schema`.
//!
//! ## Version 0
//!
//! The version `0` introduces several required and optional fields:
//!
//! - `name`: A `string` describing the name of the module.
//!     Has a maximum length of 32 ASCII characters. Is required.
//! - `type`: A `string` describing the `module type` of the module.
//!     Has a maximum length of 64 ASCII characters. Is required.
//! - `version`: A `string` describing the version of the module.
//!     Has a maximum length of 32 ASCII characters. Is required.
//! - `load_dependencies`: An `array` of `<interface_descriptors>` describing the load dependencies.
//! - `runtime_dependencies`: An `array` of `<interface_descriptors>` describing the runtime dependencies.
//! - `exports`: An `array` of `<interface_descriptors>` describing the exportable interfaces.
//!
//! The definition of the custom types can be found below:
//!
//! - `<interface_descriptor>`: The descriptor of an interface.
//!   - `name`: A `string` describing the name of the interface.
//!     Has a maximum length of 32 ASCII characters. Is required.
//!   - `version`: A `<version>` describing the version of the interface. Is required.
//!   - `extensions`: An `array` of strings describing the extensions of the interface.
//!     Each extension has a maximum length of 32 ASCII characters.
//!
//! - `<version>`: The `string` representation of a version.
//!     See [`Versions`](https://fimoengine.github.io/emf/conventions.html#versions).
//!
//! Example:
//!
//! ```json
//! {
//!     "schema": 0,
//!     "name": "jobs",
//!     "type": "emf::core_base::native",
//!     "version": "0.1.5-rc.7-a",
//!     "load_dependencies": [{
//!         "name": "memory",
//!         "version": "0.1.0"
//!     }],
//!     "runtime_dependencies": [{
//!         "name": "logging",
//!         "version": "1.0.0"
//!     }],
//!     "exports": [{
//!         "name": "jobs_interface",
//!         "version": "1.0.0",
//!         "extensions": [
//!             "schedulers",
//!             "fibers"
//!         ]
//!     }]
//! }
//! ```
//!
//! # Predefined Loaders
//!
//! Some `module loaders` are always present and can not be removed at runtime.
//!
//! ## Native
//!
//! The native `module loader` is built on top of the native `library loader` and is able to
//! load modules consisting of native libraries. It is reachable with the
//! [DEFAULT_MODULE_LOADER] handle. The same restrictions of the native
//! `library loader` apply to the native `module loader`. The native `module loader` requires
//! the presence of a `native module manifest` file named `native_module.json` at the root
//! of the module.
//!
//! ### Native module manifest
//!
//! The `native module manifest` specifies which library implements the module.
//! To allow for backwards (and forwards) compatibility, the version of the `manifest schema`
//! is saved in the manifest with the key `schema`.
//!
//! #### Native module manifest version 0
//!
//! The version `0` requires one field:
//!
//! - `library_path`: A `string` describing the relative path to the library. Is required.
//!
//! Example:
//!
//! ```json
//! {
//!     "schema": 0,
//!     "library_path": "./lib/jobs.so"
//! }
//! ```
//!
//! ### Native module interface
//!
//! Once the module library has been loaded by the native `library loader`, the native
//! `module loader` searches for a symbol with the name `emf_cbase_native_module_interface`
//! (see [NATIVE_MODULE_INTERFACE_SYMBOL_NAME]) and the type [NativeModuleLoader].
//!
//! # Example
//!
//! ```no_run
//! # use emf_core_base_rs_bare::GlobalToken;
//! # use emf_core_base_rs_bare::module::{ModuleToken, DEFAULT_MODULE_LOADER, InterfaceDescriptor, ModuleName};
//! # use std::ffi::CString;
//! # use emf_core_base_rs_bare::sys::SysToken;
//! # use emf_core_base_rs_bare::version::{Version, VersionImplGlobal};
//! # use emf_core_base_rs_bare::ffi::containers::Span;
//! #
//! let token = GlobalToken::new();
//! let mod_handle = match ModuleToken::add_module(
//!     &token,
//!     &DEFAULT_MODULE_LOADER,
//!     &"`./jobs") {
//!     Ok(h) => h,
//!     Err(_) => {
//!         let error = CString::new("Could not load the load the `./jobs` module.").unwrap();
//!         SysToken::panic(&token, Some(&error));
//!     }
//! };
//!
//! match ModuleToken::load(&token, &mod_handle) {
//!     None => {}
//!     Some(_) => {
//!         let error = CString::new("Could not load the load the `./jobs` module.").unwrap();
//!         SysToken::panic(&token, Some(&error));
//!     }
//! };
//!
//! match ModuleToken::initialize(&token, &mod_handle) {
//!     None => {}
//!     Some(_) => {
//!         let error = CString::new("Could not initialize the load the `./jobs` module.").unwrap();
//!         SysToken::panic(&token, Some(&error));
//!     }
//! };
//!
//!
//! let interface = InterfaceDescriptor {
//!     name: ModuleName::from("jobs_interface"),
//!     version: Version::new(1, 0, 0),
//!     extensions: Span::new()
//! };
//!
//! match ModuleToken::export_interface(&token, &mod_handle, &interface) {
//!     None => {},
//!     Some(_) => {
//!         let error = CString::new("Unable to export `jobs_interface` from
//!             the `./jobs` module.").unwrap();
//!         SysToken::panic(&token, Some(&error));
//!     }
//! };
//! ```

use crate::ffi;

#[cfg(feature = "global_api")]
mod global_token;
mod local_token;

mod loader_module_handle;
mod module_handle;
mod module_interface;
mod module_loader;
mod module_loader_handle;
mod module_token;
mod native_module;

pub use ffi::module::InterfaceDescriptor;
pub use ffi::module::InterfaceExtension;
pub use ffi::module::InterfaceName;
pub use ffi::module::ModuleError;
pub use ffi::module::ModuleInfo;
pub use ffi::module::ModuleName;
pub use ffi::module::ModuleStatus;
pub use ffi::module::ModuleType;
pub use ffi::module::ModuleVersion;
pub use ffi::module::INTERFACE_EXTENSION_NAME_MAX_LENGTH;
pub use ffi::module::INTERFACE_INFO_NAME_MAX_LENGTH;
pub use ffi::module::MODULE_INFO_NAME_MAX_LENGTH;
pub use ffi::module::MODULE_INFO_VERSION_MAX_LENGTH;
pub use ffi::module::MODULE_LOADER_TYPE_MAX_LENGTH;
pub use ffi::module::NATIVE_MODULE_INTERFACE_SYMBOL_NAME;
pub use ffi::module::NATIVE_MODULE_TYPE_NAME;

pub use loader_module_handle::{LoaderModuleHandle, LoaderModuleHandleRef};
pub use module_handle::{ModuleHandle, ModuleHandleRef};
pub use module_interface::ModuleInterface;
pub use module_loader::{
    ModuleLoader, ModuleLoaderWrapper, NativeModuleLoader, NativeModuleLoaderWrapper,
};
pub use module_loader_handle::{ModuleLoaderHandle, ModuleLoaderHandleRef, DEFAULT_MODULE_LOADER};
pub use module_token::ModuleToken;
pub use native_module::{NativeModule, NativeModuleInstance, NativeModuleWrapper};
