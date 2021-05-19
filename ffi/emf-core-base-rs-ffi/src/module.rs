//! Module api.
//!
//! # Example
//!
//! ```no_run
//! # use emf_core_base_rs_ffi::CBaseBinding;
//! # let base_interface: &mut dyn CBaseBinding = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
//! use emf_core_base_rs_ffi::sys::api::SysBinding;
//! use emf_core_base_rs_ffi::version::api::VersionBinding;
//! use emf_core_base_rs_ffi::module::api::ModuleBinding;
//! use emf_core_base_rs_ffi::library::OSPathChar;
//! use emf_core_base_rs_ffi::collections::{NonNullConst, ConstSpan, Optional};
//! use emf_core_base_rs_ffi::module::{MODULE_LOADER_DEFAULT_HANDLE,
//!     InterfaceDescriptor, InterfaceName};
//!
//! unsafe {
//!     // `base_interface` has the type `&mut dyn CBaseBinding`.
//!     SysBinding::lock(base_interface);
//!
//!     // Path of the module. Platform dependent initialisation.
//!     let mod_path: OSPathChar = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
//!
//!     let handle = match ModuleBinding::add_module(
//!                     base_interface,
//!                     MODULE_LOADER_DEFAULT_HANDLE,
//!                     NonNullConst::from(&mod_path)
//!                     ).into_rust() {
//!         Ok(handle) => handle,
//!         Err(e) => {
//!             SysBinding::panic(
//!                 base_interface,
//!                 Optional::Some(e)
//!             );
//!         }
//!     };
//!
//!     if let Err(e) = ModuleBinding::load(base_interface, handle).into_rust() {
//!         SysBinding::panic(
//!             base_interface,
//!             Optional::Some(e)
//!         );
//!     }
//!
//!     if let Err(e) = ModuleBinding::initialize(base_interface, handle).into_rust() {
//!         SysBinding::panic(
//!             base_interface,
//!             Optional::Some(e)
//!         );
//!     }
//!
//!     let interface_desc = InterfaceDescriptor {
//!         name: InterfaceName::from("jobs_interface"),
//!         version: VersionBinding::new_short(base_interface, 1, 0, 0),
//!         extensions: ConstSpan::new()
//!     };
//!
//!     if let Err(e) = ModuleBinding::export_interface(
//!         base_interface,
//!         handle,
//!         NonNullConst::from(&interface_desc)
//!     ).into_rust() {
//!         SysBinding::panic(
//!             base_interface,
//!             Optional::Some(e)
//!         );
//!     }
//!
//!     SysBinding::unlock(base_interface);
//! }
//! ```
use crate::collections::{ConstSpan, StaticVec};
use crate::version::Version;
use std::ffi::c_void;
use std::fmt::{Display, Formatter};
use std::ptr::NonNull;

pub mod api;
pub mod module_loader;
pub mod native_module;

/// Maximum length of a module name.
pub const MODULE_INFO_NAME_MAX_LENGTH: usize = 32;

/// Maximum length of a module version.
pub const MODULE_INFO_VERSION_MAX_LENGTH: usize = 32;

/// Maximum length of an interface name.
pub const INTERFACE_INFO_NAME_MAX_LENGTH: usize = 32;

/// Maximum length of an extension name.
pub const INTERFACE_EXTENSION_NAME_MAX_LENGTH: usize = 32;

/// Maximum length of a module type.
pub const MODULE_LOADER_TYPE_MAX_LENGTH: usize = 64;

/// Type of a native module.
pub const NATIVE_MODULE_TYPE_NAME: &str = "emf::core_base::native";

/// Native module entry symbol.
pub const NATIVE_MODULE_INTERFACE_SYMBOL_NAME: &str = "emf_cbase_native_module_interface";

/// Default loader handle.
pub const MODULE_LOADER_DEFAULT_HANDLE: LoaderHandle = LoaderHandle {
    id: PredefinedHandles::Native as i32,
};

/// Predefined loader handles.
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum PredefinedHandles {
    Native = 0,
}

impl Display for PredefinedHandles {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PredefinedHandles::Native => write!(f, "Native"),
        }
    }
}

/// Status of a module.
#[repr(i32)]
#[non_exhaustive]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub enum ModuleStatus {
    Unloaded = 0,
    Terminated = 1,
    Ready = 2,
}

impl Display for ModuleStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ModuleStatus::Unloaded => write!(f, "Unloaded"),
            ModuleStatus::Terminated => write!(f, "Terminated"),
            ModuleStatus::Ready => write!(f, "Ready"),
        }
    }
}

/// Handle of a module.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ModuleHandle {
    pub id: i32,
}

impl Display for ModuleHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

/// Handle of a module loader.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct LoaderHandle {
    pub id: i32,
}

impl Display for LoaderHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

/// Internal handle of a module.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InternalHandle {
    pub id: isize,
}

impl Display for InternalHandle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.id)
    }
}

/// Interface from a module.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Interface {
    pub interface: NonNull<c_void>,
}

/// Name of a module.
pub type ModuleName = StaticVec<u8, MODULE_INFO_NAME_MAX_LENGTH>;

/// Type of a module.
pub type ModuleType = StaticVec<u8, MODULE_LOADER_TYPE_MAX_LENGTH>;

/// Version a module.
pub type ModuleVersion = StaticVec<u8, MODULE_INFO_VERSION_MAX_LENGTH>;

/// Name of an interface.
pub type InterfaceName = StaticVec<u8, INTERFACE_INFO_NAME_MAX_LENGTH>;

/// Extension of an interface.
pub type InterfaceExtension = StaticVec<u8, INTERFACE_EXTENSION_NAME_MAX_LENGTH>;

/// Information regarding a module.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ModuleInfo {
    pub name: ModuleName,
    pub version: ModuleVersion,
}

impl Display for ModuleInfo {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", &self.name, &self.version)
    }
}

/// Information regarding an interface.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InterfaceDescriptor {
    pub name: InterfaceName,
    pub version: Version,
    pub extensions: ConstSpan<InterfaceExtension>,
}

impl Display for InterfaceDescriptor {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let extensions: Vec<String> = self
            .extensions
            .as_ref()
            .iter()
            .map(|ext| format!("{}", ext))
            .collect();

        write!(f, "{}{:?}, {}", &self.name, extensions, &self.version)
    }
}
