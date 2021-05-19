//! Module api
//!
//! # Example
//!
//! ```no_run
//! # use emf_core_base_rs::{CBaseAccess, CBase};
//! # let base_interface: &mut CBase = unsafe { std::mem::MaybeUninit::uninit().assume_init() };
//! use emf_core_base_rs::CBaseAPI;
//! use emf_core_base_rs::version::VersionAPI;
//! use emf_core_base_rs::module::{
//!     ModuleAPI, DEFAULT_HANDLE, InterfaceDescriptor, InterfaceName, Module
//! };
//! use emf_core_base_rs::ffi::collections::ConstSpan;
//! use emf_core_base_rs::Error;
//! use std::path::Path;
//!
//! let result = CBaseAccess::lock(base_interface, |interface| -> Result<Module<'_, _>, Error<_>> {
//!     let module_path = Path::new("path to a module");
//!     let interface_desc = InterfaceDescriptor {
//!         name: InterfaceName::from("my_interface"),
//!         version: VersionAPI::new_short(interface, 1, 0, 0),
//!         extensions: ConstSpan::new()
//!     };
//!
//!     let mut module = ModuleAPI::add_module(interface, &DEFAULT_HANDLE, &module_path)?;
//!     ModuleAPI::load(interface, &mut module)?;
//!     ModuleAPI::initialize(interface, &mut module)?;
//!     ModuleAPI::export_interface(interface, &mut module, &interface_desc)?;
//!     Ok(module)
//! });
//!
//! assert_eq!(result.is_ok(), true);
//! ```
use crate::ffi::module::{InternalHandle, LoaderHandle, ModuleHandle};
use crate::ownership::{AccessIdentifier, BorrowImmutable, BorrowMutable, Owned};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};

mod api;
pub mod module_loader;
pub mod native_module;

pub use crate::ffi::module::InterfaceDescriptor;
pub use crate::ffi::module::InterfaceExtension;
pub use crate::ffi::module::InterfaceName;
pub use crate::ffi::module::ModuleInfo;
pub use crate::ffi::module::ModuleName;
pub use crate::ffi::module::ModuleStatus;
pub use crate::ffi::module::ModuleType;
pub use crate::ffi::module::ModuleVersion;
pub use crate::ffi::module::INTERFACE_EXTENSION_NAME_MAX_LENGTH;
pub use crate::ffi::module::INTERFACE_INFO_NAME_MAX_LENGTH;
pub use crate::ffi::module::MODULE_INFO_NAME_MAX_LENGTH;
pub use crate::ffi::module::MODULE_INFO_VERSION_MAX_LENGTH;
pub use crate::ffi::module::MODULE_LOADER_TYPE_MAX_LENGTH;
pub use crate::ffi::module::NATIVE_MODULE_INTERFACE_SYMBOL_NAME;
pub use crate::ffi::module::NATIVE_MODULE_TYPE_NAME;

pub use api::ModuleAPI;
use std::fmt::{Display, Formatter};

/// Handle of the default loader.
pub const DEFAULT_HANDLE: Loader<'static, BorrowMutable<'static>> =
    unsafe { Loader::new(crate::ffi::module::MODULE_LOADER_DEFAULT_HANDLE) };

/// A module handle.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Module<'a, O> {
    _handle: ModuleHandle,
    _lifetime: PhantomData<&'a ModuleHandle>,
    _ownership: PhantomData<fn() -> O>,
}

impl<'a, O> Module<'a, O>
where
    O: AccessIdentifier,
{
    /// Construct a new instance from a handle.
    ///
    /// # Safety
    ///
    /// This function allows the creation of invalid handles
    /// by bypassing lifetimes.
    #[inline]
    pub const unsafe fn new(handle: ModuleHandle) -> Self {
        Self {
            _handle: handle,
            _lifetime: PhantomData,
            _ownership: PhantomData,
        }
    }

    /// Fetches the internal handle.
    #[inline]
    pub const fn as_handle(&self) -> ModuleHandle {
        self._handle
    }
}

impl<'a> Module<'a, Owned> {
    /// Borrows the library handle.
    #[inline]
    pub const fn as_borrowed(&self) -> Module<'a, BorrowImmutable<'_>> {
        unsafe { Module::<BorrowImmutable<'_>>::new(self._handle) }
    }

    /// Borrows the library handle mutably.
    #[inline]
    pub fn as_borrowed_mut(&mut self) -> Module<'a, BorrowMutable<'_>> {
        unsafe { Module::<BorrowMutable<'_>>::new(self._handle) }
    }
}

impl<O> Display for Module<'_, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self._handle)
    }
}

/// A loader handle.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Loader<'a, O> {
    _handle: LoaderHandle,
    _lifetime: PhantomData<&'a LoaderHandle>,
    _ownership: PhantomData<fn() -> O>,
}

impl<'a, O> Loader<'a, O>
where
    O: AccessIdentifier,
{
    /// Construct a new instance from a handle.
    ///
    /// # Safety
    ///
    /// This function allows the creation of invalid handles
    /// by bypassing lifetimes.
    #[inline]
    pub const unsafe fn new(handle: LoaderHandle) -> Self {
        Self {
            _handle: handle,
            _lifetime: PhantomData,
            _ownership: PhantomData,
        }
    }

    /// Fetches the internal handle.
    #[inline]
    pub const fn as_handle(&self) -> LoaderHandle {
        self._handle
    }
}

impl<'a> Loader<'a, Owned> {
    /// Borrows the loader handle.
    #[inline]
    pub const fn as_borrowed(&self) -> Loader<'a, BorrowImmutable<'_>> {
        unsafe { Loader::<BorrowImmutable<'_>>::new(self._handle) }
    }

    /// Borrows the loader handle mutably.
    #[inline]
    pub fn as_borrowed_mut(&mut self) -> Loader<'a, BorrowMutable<'_>> {
        unsafe { Loader::<BorrowMutable<'_>>::new(self._handle) }
    }
}

impl<O> Display for Loader<'_, O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self._handle)
    }
}

/// A loader handle.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct InternalModule<O> {
    _handle: InternalHandle,
    _ownership: PhantomData<fn() -> O>,
}

impl<O> InternalModule<O>
where
    O: AccessIdentifier,
{
    /// Construct a new instance from a handle.
    ///
    /// # Safety
    ///
    /// This function allows the creation of invalid handles
    /// by bypassing lifetimes.
    #[inline]
    pub const unsafe fn new(handle: InternalHandle) -> Self {
        Self {
            _handle: handle,
            _ownership: PhantomData,
        }
    }

    /// Fetches the internal handle.
    #[inline]
    pub const fn as_handle(&self) -> InternalHandle {
        self._handle
    }
}

impl InternalModule<Owned> {
    /// Borrows the loader handle.
    #[inline]
    pub const fn as_borrowed(&self) -> InternalModule<BorrowImmutable<'_>> {
        unsafe { InternalModule::<BorrowImmutable<'_>>::new(self._handle) }
    }

    /// Borrows the loader handle mutably.
    #[inline]
    pub fn as_borrowed_mut(&mut self) -> InternalModule<BorrowMutable<'_>> {
        unsafe { InternalModule::<BorrowMutable<'_>>::new(self._handle) }
    }
}

impl<O> Display for InternalModule<O> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self._handle)
    }
}

/// Interface from a module.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Interface<'a, T> {
    _interface: T,
    _phantom: PhantomData<&'a ()>,
}

impl<T> Interface<'_, T> {
    #[inline]
    fn new(interface: T) -> Self {
        Self {
            _interface: interface,
            _phantom: PhantomData,
        }
    }
}

impl<T> Deref for Interface<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self._interface
    }
}

impl<T> DerefMut for Interface<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._interface
    }
}

impl<T> Display for Interface<'_, T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self._interface)
    }
}
