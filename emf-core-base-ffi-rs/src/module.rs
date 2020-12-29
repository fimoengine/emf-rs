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
//! can be added by constructing a [ModuleLoaderInterface] and calling the
//! [emf_cbase_module_register_loader()] function.
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
//! [MODULE_LOADER_DEFAULT_HANDLE] handle. The same restrictions of the native
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
//! (see [NATIVE_MODULE_INTERFACE_SYMBOL_NAME]) and the type [NativeModuleInterface].
//!
//! # Example
//!
//! ```no_run
//! # use emf_core_base_ffi_rs::sys::{emf_cbase_sys_lock, emf_cbase_sys_unlock, emf_cbase_sys_panic};
//! # use emf_core_base_ffi_rs::module::{emf_cbase_module_add_module, MODULE_LOADER_DEFAULT_HANDLE,
//! #     emf_cbase_module_load, emf_cbase_module_initialize, InterfaceDescriptor, ModuleName,
//! #     emf_cbase_module_export_interface};
//! # use emf_core_base_ffi_rs::containers::{NonNullConst, Span};
//! # use std::ffi::CString;
//! # use emf_core_base_ffi_rs::version::emf_cbase_version_construct_short;
//! # use std::os::raw::c_char;
//!
//! unsafe {
//!     const MODULE_PATH: &[u8; 14] = b"./jobs:module\0";
//!
//!     emf_cbase_sys_lock();
//!
//!     let module_handle = emf_cbase_module_add_module(
//!         MODULE_LOADER_DEFAULT_HANDLE,
//!         NonNullConst::new_unchecked(MODULE_PATH.as_ptr()).cast()
//!     );
//!
//!     let module_handle = match module_handle.to_native() {
//!         Ok(handle) => handle,
//!         Err(_) => {
//!             let error = CString::new("Unable to load the `./jobs_module` module.").unwrap();
//!             emf_cbase_sys_panic(error.as_ptr());
//!         }
//!     };
//!
//!     match emf_cbase_module_load(module_handle).to_native() {
//!         Some(_) => {
//!             let error = CString::new("Unable to load the `./jobs_module` module.").unwrap();
//!             emf_cbase_sys_panic(error.as_ptr());
//!         }
//!         None => {}
//!     };
//!
//!     match emf_cbase_module_initialize(module_handle).to_native() {
//!         Some(_) => {
//!             let error = CString::new("Unable to initialize the `./jobs_module` module.").unwrap();
//!             emf_cbase_sys_panic(error.as_ptr());
//!         }
//!         None => {}
//!     };
//!
//!     let interface_descriptor = InterfaceDescriptor {
//!         name: ModuleName::from("jobs_interface"),
//!         version: emf_cbase_version_construct_short(1, 0, 0),
//!         extensions: Span::new()
//!     };
//!
//!     match emf_cbase_module_export_interface(module_handle,
//!         NonNullConst::from(&interface_descriptor)).to_native() {
//!             Some(_) => {
//!                 let error = CString::new("Unable to export `jobs_interface` from
//!                     the `./jobs_module` module.").unwrap();
//!                 emf_cbase_sys_panic(error.as_ptr());
//!             },
//!             None => {}
//!     };
//!
//!     emf_cbase_sys_unlock();
//! }
//! ```

use crate::containers::{MutSpan, NonNullConst, Optional, Result, Span, StaticVec};
use crate::fn_ptr::{SysGetFunctionFn, SysHasFunctionFn};
use crate::library::OsPathChar;
use crate::version::Version;
use crate::{BaseT, Bool};
use std::os::raw::{c_char, c_void};
use std::ptr::NonNull;

/// Max length of a module name.
pub const MODULE_INFO_NAME_MAX_LENGTH: usize = 32;

/// Max length of a module version string.
pub const MODULE_INFO_VERSION_MAX_LENGTH: usize = 32;

/// Max length of an interface name string.
pub const INTERFACE_INFO_NAME_MAX_LENGTH: usize = 32;

/// Max length of an interface extension name.
pub const INTERFACE_EXTENSION_NAME_MAX_LENGTH: usize = 32;

/// Max length of a module type name.
pub const MODULE_LOADER_TYPE_MAX_LENGTH: usize = 64;

/// Name of the native module type.
pub const NATIVE_MODULE_TYPE_NAME: &str = "emf::core_base::native";

pub const NATIVE_MODULE_INTERFACE_SYMBOL_NAME: &str = "emf_cbase_native_module_interface";

/// Handle to the default module loader.
pub const MODULE_LOADER_DEFAULT_HANDLE: LoaderHandle = LoaderHandle {
    id: ModulePredefinedHandles::Native as i32,
};

/// An enum describing all possible module states.
#[repr(i32)]
#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum ModuleStatus {
    Unloaded = 0,
    Terminated = 1,
    Ready = 2,
}

/// A handle to a module
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModuleHandle {
    pub id: i32,
}

/// The name of a module.
///
/// A module name is modelled as an `UTF-8` encoded string, without a `\0` terminator.
pub type ModuleName = StaticVec<c_char, 32>;

/// The version string of a module.
///
/// A version string is modelled as an `UTF-8` encoded string, without a `\0` terminator.
pub type ModuleVersion = StaticVec<c_char, 32>;

/// Module info.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModuleInfo {
    pub name: ModuleName,
    pub version: ModuleVersion,
}

/// An extension of an interface.
///
/// An interface extension is modelled as an `UTF-8` encoded string, without a `\0` terminator.
pub type InterfaceExtension = StaticVec<c_char, 32>;

/// The name of an interface.
///
/// An interface extension is modelled as an `UTF-8` encoded string, without a `\0` terminator.
pub type InterfaceName = StaticVec<c_char, 32>;

/// An interface.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModuleInterface {
    pub interface: NonNull<c_void>,
}

/// A handle to a module loader.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LoaderHandle {
    pub id: i32,
}

/// An internal module handle used by module loaders.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct LoaderModuleHandle {
    pub id: isize,
}

/// The type of a module
///
/// A module type is modelled as an `UTF-8` encoded string, without a `\0` terminator.
pub type ModuleType = StaticVec<c_char, 64>;

/// An enum describing all predefined module loader handles.
///
/// The values `0-99` are reserved for future use.
#[repr(i32)]
#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum ModulePredefinedHandles {
    Native = 0,
}

/// A descriptor of an interface.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct InterfaceDescriptor<'a> {
    pub name: InterfaceName,
    pub version: Version,
    pub extensions: Span<'a, InterfaceExtension>,
}

/// An enum describing all defined error values.
///
/// The values `0-99` are reserved for future use.
#[repr(i32)]
#[non_exhaustive]
#[derive(Copy, Clone)]
pub enum ModuleError {
    PathInvalid = 0,
    ModuleStateInvalid = 1,
    ModuleHandleInvalid = 2,
    LoaderHandleInvalid = 3,
    LoaderModuleHandleInvalid = 4,
    ModuleTypeInvalid = 5,
    ModuleTypeNotFound = 6,
    DuplicateModuleType = 7,
    InterfaceNotFound = 8,
    DuplicateInterface = 9,
    ModuleDependencyNotFound = 10,
    BufferOverflow = 11,
}

/// An opaque structure representing a module loader.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModuleLoader {
    _private: [u8; 0],
}

/// A function pointer to a `add_module` function.
///
/// This function loads the module, which is located at `module_path`, and returns its handle.
/// The function must be thread-safe.
pub type ModuleLoaderInterfaceAddModuleFn =
    extern "C" fn(
        module_loader: *mut ModuleLoader,
        module_path: NonNullConst<OsPathChar>,
    ) -> Result<LoaderModuleHandle, ModuleError>;

/// A function pointer to a `remove_module` function.
///
/// This function unloads a module, that was loaded previously.
/// The function must be thread-safe.
pub type ModuleLoaderInterfaceRemoveModuleFn = extern "C" fn(
    module_loader: *mut ModuleLoader,
    module_handle: LoaderModuleHandle,
) -> Optional<ModuleError>;

/// A function pointer to a `fetch_status` function.
///
/// This function returns the status of an already loaded module.
/// The function must be thread-safe.
pub type ModuleLoaderInterfaceFetchStatusFn = extern "C" fn(
    module_loader: *mut ModuleLoader,
    module_handle: LoaderModuleHandle,
) -> Result<ModuleStatus, ModuleError>;

/// A function pointer to a `load` function.
///
/// This function executes the loading procedure of the module.
/// The function must be thread-safe.
pub type ModuleLoaderInterfaceLoadFn = extern "C" fn(
    module_loader: *mut ModuleLoader,
    module_handle: LoaderModuleHandle,
) -> Optional<ModuleError>;

/// A function pointer to a `unload` function.
///
/// This function executes the unloading procedure of the module.
/// The function must be thread-safe.
pub type ModuleLoaderInterfaceUnloadFn = extern "C" fn(
    module_loader: *mut ModuleLoader,
    module_handle: LoaderModuleHandle,
) -> Optional<ModuleError>;

/// A function pointer to a `initialize` function.
///
/// This function executes the initialization procedure of the module.
/// The function must be thread-safe.
pub type ModuleLoaderInterfaceInitializeFn = extern "C" fn(
    module_loader: *mut ModuleLoader,
    module_handle: LoaderModuleHandle,
) -> Optional<ModuleError>;

/// A function pointer to a `terminate` function.
///
/// This function executes the termination procedure of the module.
/// The function must be thread-safe.
pub type ModuleLoaderInterfaceTerminateFn = extern "C" fn(
    module_loader: *mut ModuleLoader,
    module_handle: LoaderModuleHandle,
) -> Optional<ModuleError>;

/// A function pointer to a `get_module_info` function.
///
/// This function fetches the module info from a module.
/// The function must be thread-safe.
pub type ModuleLoaderInterfaceGetModuleInfoFn =
    extern "C" fn(
        module_loader: *mut ModuleLoader,
        module_handle: LoaderModuleHandle,
    ) -> Result<NonNullConst<ModuleInfo>, ModuleError>;

/// A function pointer to a `get_exportable_interfaces_fn` function.
///
/// This function fetches the exportable interfaces from a module.
/// The function must be thread-safe.
pub type ModuleLoaderInterfaceGetExportableInterfacesFn =
    extern "C" fn(
        module_loader: *mut ModuleLoader,
        module_handle: LoaderModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError>;

/// A function pointer to a `get_runtime_dependencies` function.
///
/// This function fetches the runtime dependencies from a module.
/// The function must be thread-safe.
pub type ModuleLoaderInterfaceGetRuntimeDependenciesFn =
    extern "C" fn(
        module_loader: *mut ModuleLoader,
        module_handle: LoaderModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError>;

/// A function pointer to a `get_interface` function.
///
/// This function fetches an interface from a module.
/// The function must be thread-safe.
pub type ModuleLoaderInterfaceGetInterfaceFn =
    extern "C" fn(
        module_loader: *mut ModuleLoader,
        module_handle: LoaderModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Result<ModuleInterface, ModuleError>;

/// A function pointer to a `get_load_dependencies` function.
///
/// This function fetches a list of dependencies the module needs to allow its loading.
/// The function must be thread-safe.
pub type ModuleLoaderInterfaceGetLoadDependenciesFn =
    extern "C" fn(
        module_loader: *mut ModuleLoader,
        module_handle: LoaderModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError>;

/// A function pointer to a `get_module_path` function.
///
/// This function fetches the path a module was loaded from.
/// The function must be thread-safe.
pub type ModuleLoaderInterfaceGetModulePathFn =
    extern "C" fn(
        module_loader: *mut ModuleLoader,
        module_handle: LoaderModuleHandle,
    ) -> Result<NonNullConst<OsPathChar>, ModuleError>;

/// Interface of a module loader.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ModuleLoaderInterface {
    pub module_loader: *mut ModuleLoader,
    pub add_module_fn: ModuleLoaderInterfaceAddModuleFn,
    pub remove_module_fn: ModuleLoaderInterfaceRemoveModuleFn,
    pub fetch_status_fn: ModuleLoaderInterfaceFetchStatusFn,
    pub load_fn: ModuleLoaderInterfaceLoadFn,
    pub unload_fn: ModuleLoaderInterfaceUnloadFn,
    pub initialize_fn: ModuleLoaderInterfaceInitializeFn,
    pub terminate_fn: ModuleLoaderInterfaceTerminateFn,
    pub get_module_info_fn: ModuleLoaderInterfaceGetModuleInfoFn,
    pub get_exportable_interfaces_fn: ModuleLoaderInterfaceGetExportableInterfacesFn,
    pub get_runtime_dependencies_fn: ModuleLoaderInterfaceGetRuntimeDependenciesFn,
    pub get_interface_fn: ModuleLoaderInterfaceGetInterfaceFn,
    pub get_load_dependencies_fn: ModuleLoaderInterfaceGetLoadDependenciesFn,
    pub get_module_path_fn: ModuleLoaderInterfaceGetModulePathFn,
}

/// An opaque structure representing a native module.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NativeModule {
    _private: [u8; 0],
}

/// A function pointer to a `load` function.
///
/// The function loads a module and returns a pointer which represents the newly loaded module.
pub type NativeModuleInterfaceLoadFn = extern "C" fn(
    module_handle: ModuleHandle,
    base_module: BaseT,
    has_function_fn: SysHasFunctionFn,
    get_function_fn: SysGetFunctionFn,
) -> Result<*mut NativeModule, ModuleError>;

/// A function pointer to a `unload` function.
///
/// The function unloads the module and disposes of the pointer.
pub type NativeModuleInterfaceUnloadFn =
    extern "C" fn(module: *mut NativeModule) -> Optional<ModuleError>;

/// A function pointer to a `initialize` function.
///
/// The function initializes the module.
pub type NativeModuleInterfaceInitializeFn =
    extern "C" fn(module: *mut NativeModule) -> Optional<ModuleError>;

/// A function pointer to a `terminate` function.
///
/// The function terminates the module.
pub type NativeModuleInterfaceTerminateFn =
    extern "C" fn(module: *mut NativeModule) -> Optional<ModuleError>;

/// A function pointer to a `get_module_info` function.
///
/// The function fetches the module info from the module.
pub type NativeModuleInterfaceGetModuleInfoFn =
    extern "C" fn(module: *mut NativeModule) -> Result<NonNullConst<ModuleInfo>, ModuleError>;

/// A function pointer to a `get_exportable_interfaces` function.
///
/// The function fetches the exportable interfaces from the module.
pub type NativeModuleInterfaceGetExportableInterfacesFn =
    extern "C" fn(
        module: *mut NativeModule,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError>;

/// A function pointer to a `get_runtime_dependencies` function.
///
/// The function fetches the runtime dependencies from the module.
pub type NativeModuleInterfaceGetRuntimeDependenciesFn =
    extern "C" fn(
        module: *mut NativeModule,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError>;

/// A function pointer to a `get_interface` function.
///
/// The function fetches an interface from the module.
pub type NativeModuleInterfaceGetInterfaceFn =
    extern "C" fn(
        module: *mut NativeModule,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Result<ModuleInterface, ModuleError>;

/// A function pointer to a `get_load_dependencies` function.
///
/// The function fetches a list of dependencies the module needs, in order for it to be loaded.
pub type NativeModuleInterfaceGetLoadDependenciesFn =
    extern "C" fn() -> Span<'static, InterfaceDescriptor<'static>>;

/// Interface of a native module.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NativeModuleInterface {
    pub load_fn: NativeModuleInterfaceLoadFn,
    pub unload_fn: NativeModuleInterfaceUnloadFn,
    pub initialize_fn: NativeModuleInterfaceInitializeFn,
    pub terminate_fn: NativeModuleInterfaceTerminateFn,
    pub get_module_info_fn: NativeModuleInterfaceGetModuleInfoFn,
    pub get_exportable_interfaces_fn: NativeModuleInterfaceGetExportableInterfacesFn,
    pub get_runtime_dependencies_fn: NativeModuleInterfaceGetRuntimeDependenciesFn,
    pub get_interface_fn: NativeModuleInterfaceGetInterfaceFn,
    pub get_load_dependencies_fn: NativeModuleInterfaceGetLoadDependenciesFn,
}

/// A function pointer to a `get_native_module` function.
///
/// The function returns a pointer to the native module.
pub type NativeModuleLoaderInterfaceGetNativeModuleFn =
    extern "C" fn(
        module_loader: *mut ModuleLoader,
        module_handle: LoaderModuleHandle,
    ) -> Result<*mut NativeModule, ModuleError>;

/// Interface of the native module loader.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NativeModuleLoader {
    pub module_loader_interface: ModuleLoaderInterface,
    pub get_native_module_fn: NativeModuleLoaderInterfaceGetNativeModuleFn,
}

extern "C" {

    /// Registers a new module loader.
    ///
    /// Module names starting with `__` are reserved for future use.
    ///
    /// # Failure
    ///
    /// The function fails if `module_type` already exists.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_register_loader(
        loader_interface: NonNullConst<ModuleLoaderInterface>,
        module_type: NonNullConst<ModuleType>,
    ) -> Result<LoaderHandle, ModuleError>;

    /// Unregisters an existing module loader.
    ///
    /// Unregistering a module loader also unregisters the modules it loaded.
    ///
    /// # Failure
    ///
    /// The function fails if `loader_handle` is invalid.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_unregister_loader(loader_handle: LoaderHandle)
        -> Optional<ModuleError>;

    /// Fetches the number of loaders.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_get_num_loaders() -> usize;

    /// Copies the available module types.
    ///
    /// # Failure
    ///
    /// The function fails if `buffer` too small.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_get_module_types(
        buffer: NonNull<MutSpan<ModuleType>>,
    ) -> Result<usize, ModuleError>;

    /// Fetches the number of loaded modules.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_get_num_modules() -> usize;

    /// Copies the available module infos.
    ///
    /// # Failure
    ///
    /// The function fails if `buffer` too small.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_get_modules(
        buffer: NonNull<MutSpan<ModuleInfo>>,
    ) -> Result<usize, ModuleError>;

    /// Fetches the number of exported interfaces.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_get_num_exported_interfaces() -> usize;

    /// Copies the descriptors of the exported interfaces.
    ///
    /// # Failure
    ///
    /// The function fails if `buffer` too small.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_get_exported_interfaces(
        buffer: NonNull<MutSpan<InterfaceDescriptor>>,
    ) -> Result<usize, ModuleError>;

    /// Fetches the handle of the loader associated with a module type.
    ///
    /// # Failure
    ///
    /// The function fails if `module_type` does not exist.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_get_loader_handle(
        module_type: NonNullConst<ModuleType>,
    ) -> Result<LoaderHandle, ModuleError>;

    /// Checks if a module type exists.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_type_exists(module_type: NonNullConst<ModuleType>) -> Bool;

    /// Checks if a module exists.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_module_exists(module_handle: ModuleHandle) -> Bool;

    /// Fetches the module handle of the exported interface.
    ///
    /// # Failure
    ///
    /// The function fails if `interface` does not exist.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_get_exported_interface_handle(
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Result<ModuleHandle, ModuleError>;

    /// Checks whether an exported interface exists.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_exported_interface_exists(
        interface: NonNullConst<InterfaceDescriptor>,
    ) -> Bool;

    /// Creates a new unlinked module handle.
    ///
    /// # Warning
    ///
    /// The handle must be linked before use
    /// (See [emf_cbase_module_unsafe_link_module()]).
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_unsafe_create_module_handle() -> ModuleHandle;

    /// Removes an existing module handle.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid.
    ///
    /// # Warning
    ///
    /// Removing the handle does not unload the module.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_unsafe_remove_module_handle() -> Optional<ModuleError>;

    /// Links a module handle to an internal module handle.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` or`loader_handle` are invalid.
    ///
    /// # Warning
    ///
    /// Incorrect usage can lead to dangling handles or use-after-free errors.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_unsafe_link_module(
        module_handle: ModuleHandle,
        loader_handle: LoaderHandle,
        internal_handle: LoaderModuleHandle,
    ) -> Optional<ModuleError>;

    /// Fetches the internal handle linked with the module handle.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_unsafe_get_loader_module_handle(
        module_handle: ModuleHandle,
    ) -> Result<LoaderModuleHandle, ModuleError>;

    /// Fetches the loader handle linked with the module handle.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_unsafe_get_loader_handle(
        module_handle: ModuleHandle,
    ) -> Result<LoaderHandle, ModuleError>;

    /// Fetches the interface of a module loader.
    ///
    /// # Failure
    ///
    /// The function fails if `loader_handle` is invalid.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_unsafe_get_loader(
        loader_handle: LoaderHandle,
    ) -> Result<NonNullConst<ModuleLoaderInterface>, ModuleError>;

    /// Adds a new module.
    ///
    /// # Failure
    ///
    /// The function fails if `loader_handle` or `module_path` is invalid or the type of the module can not be loaded by the loader.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_add_module(
        loader_handle: LoaderHandle,
        module_path: NonNullConst<OsPathChar>,
    ) -> Result<ModuleHandle, ModuleError>;

    /// Removes a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid or the module is not in an unloaded state.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_remove_module(module_handle: ModuleHandle) -> Optional<ModuleError>;

    /// Fetches the load dependencies of a module.
    ///
    /// The load dependencies of a module must all be present at the time of loading.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_get_load_dependencies(
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError>;

    /// Fetches the load status of a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_fetch_status(
        module_handle: ModuleHandle,
    ) -> Result<ModuleStatus, ModuleError>;

    /// Registers a new runtime dependency of the module.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_add_dependency(
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Optional<ModuleError>;

    /// Removes an existing runtime dependency from the module.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_remove_dependency(
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Optional<ModuleError>;

    /// Exports an interface of a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid, `interface_descriptor` is already
    /// exported, `interface_descriptor` is not contained in the module or the
    /// module is not yet initialized.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_export_interface(
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Optional<ModuleError>;

    /// Loads a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid, the load dependencies
    /// of the module are not exported or the module is not in an unloaded state.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_load(module_handle: ModuleHandle) -> Optional<ModuleError>;

    /// Unloads a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid or the module is in an
    /// unloaded or ready state.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_unload(module_handle: ModuleHandle) -> Optional<ModuleError>;

    /// Initializes a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid, the runtime dependencies of
    /// the module are not exported or the module is not in a loaded state.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_initialize(module_handle: ModuleHandle) -> Optional<ModuleError>;

    /// Terminates a module.
    ///
    /// Terminating a module also removes the interfaces it exported.
    ///
    /// The modules that depend on the module are terminated.
    /// If they list the module as a load dependency, they are also unloaded.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid or the module is not in a ready state.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_terminate(module_handle: ModuleHandle) -> Optional<ModuleError>;

    /// Fetches the module info from a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid or the module is not yet loaded.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_get_module_info(
        module_handle: ModuleHandle,
    ) -> Result<NonNullConst<ModuleInfo>, ModuleError>;

    /// Fetches a list of the exportable interfaces from a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid or the module is not yet loaded.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_get_exportable_interfaces(
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError>;

    /// Fetches a list of the runtime dependencies from a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid or the module is not yet loaded.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_get_runtime_dependencies(
        module_handle: ModuleHandle,
    ) -> Result<Span<'static, InterfaceDescriptor<'static>>, ModuleError>;

    /// Fetches an interface from a module.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid, the module is not in a ready state or
    /// the interface is not contained in the module.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_get_interface(
        module_handle: ModuleHandle,
        interface_descriptor: NonNullConst<InterfaceDescriptor>,
    ) -> Result<ModuleInterface, ModuleError>;

    /// Fetches the path a module was loaded from.
    ///
    /// # Failure
    ///
    /// The function fails if `module_handle` is invalid or the module is not yet loaded.
    ///
    /// # Undefined Behaviour
    ///
    /// The callee expects that the caller holds a lock
    /// (See [emf_cbase_sys_lock()](crate::sys::emf_cbase_sys_lock)).
    #[must_use]
    pub fn emf_cbase_module_get_module_path(
        module_handle: ModuleHandle,
    ) -> Result<NonNullConst<OsPathChar>, ModuleError>;

}
