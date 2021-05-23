use crate::ffi::collections::{ConstSpan, MutSpan, NonNullConst, Optional, Result};
use crate::ffi::errors::Error;
use crate::ffi::library::api::LibraryBinding;
use crate::ffi::library::library_loader::LibraryLoaderInterface;
use crate::ffi::library::OSPathString;
use crate::ffi::module;
use crate::ffi::module::api::ModuleBinding;
use crate::ffi::module::module_loader::ModuleLoaderInterface;
use crate::ffi::sys::api::SysBinding;
use crate::ffi::sys::sync_handler::SyncHandlerInterface;
use crate::ffi::version;
use crate::ffi::version::api::VersionBinding;
use crate::ffi::version::ReleaseType;
use crate::ffi::{library, CBaseBinding};
use crate::ffi::{Bool, CBaseInterface, FnId};
use crate::fn_caster::FnCaster;
use crate::library::LibraryAPI;
use crate::module::ModuleAPI;
use crate::ownership::Owned;
use crate::sys::{SysAPI, SysAPIMin};
use crate::version::{Version, VersionAPI};
use std::cell::UnsafeCell;
use std::ffi::c_void;
use std::marker::PhantomData;

/// Borrowed `emf-core-base` interface.
#[derive(Debug)]
pub struct CBaseRef<'interface> {
    _interface: CBaseInterface,
    _phantom: PhantomData<&'interface CBaseInterface>,
}

unsafe impl Send for CBaseRef<'_> {}

/// Owned `emf-core-base` interface.
#[derive(Debug)]
pub struct CBase<'interface> {
    _interface: UnsafeCell<CBaseRef<'interface>>,
}

unsafe impl Send for CBase<'_> {}
unsafe impl Sync for CBase<'_> {}

impl CBaseRef<'_> {
    /// Constructs itself using the native interface.
    ///
    /// # Safety
    ///
    /// This function is unsafe, as it enables bypassing the locking mechanism.
    pub unsafe fn new(interface: CBaseInterface) -> Self {
        Self {
            _interface: interface,
            _phantom: PhantomData,
        }
    }
}

impl<'interface> CBase<'interface> {
    /// Constructs itself using the borrowed interface.
    ///
    /// # Safety
    ///
    /// This function is unsafe, as it enables bypassing the locking mechanism.
    pub unsafe fn new(interface: CBaseRef<'interface>) -> Self {
        Self {
            _interface: UnsafeCell::new(interface),
        }
    }
}

/// A trait for providing information about the loaded interface.
pub trait CBaseInterfaceInfo {
    /// Type of the interface.
    type Interface: CBaseBinding;

    /// Returns the version of the interface.
    fn interface_version(&self) -> Version;

    /// Fetches the internal low-level interface.
    fn internal_interface(&self) -> &Self::Interface;
}

/// A collection of traits providing access to the entire interface.
pub trait CBaseAPI<'interface>:
    CBaseInterfaceInfo
    + SysAPI<'interface>
    + VersionAPI
    + LibraryAPI<'interface>
    + ModuleAPI<'interface>
{
}

impl<'interface, T> CBaseAPI<'interface> for T where
    T: CBaseInterfaceInfo
        + SysAPI<'interface>
        + VersionAPI
        + LibraryAPI<'interface>
        + ModuleAPI<'interface>
{
}

/// A trait that provides access to the interface.
pub trait CBaseAccess<'interface> {
    /// Type of the interface.
    type Interface: CBaseAPI<'interface>;

    /// Enters the critical section with the provided function.
    ///
    /// The calling thread will wait until it can acquire the lock.
    ///
    /// # Return
    ///
    /// Return value from the provided function.
    fn lock<U>(&self, f: impl FnOnce(&mut Self::Interface) -> U) -> U;

    /// Enters the critical section with the provided function.
    ///
    /// The function does nothing if the lock could not be acquired.
    ///
    /// # Return
    ///
    /// Return value from the provided function or [Option::None] if
    /// the lock could not be acquired.
    fn try_lock<U>(&self, f: impl FnOnce(&mut Self::Interface) -> U) -> Option<U>;

    /// Enters the critical section with the provided function without locking.
    ///
    /// # Return
    ///
    /// Return value from the provided function.
    ///
    /// # Safety
    ///
    /// Most of the interface assumes that the caller has unique access to the interface.
    /// This function can be used to bypass this restriction, if the user can guarantee
    /// that no data-races will occur.
    unsafe fn assume_locked<U>(&self, f: impl FnOnce(&mut Self::Interface) -> U) -> U;
}

impl SysBinding for CBaseRef<'_> {
    #[inline]
    unsafe fn shutdown(&mut self) -> ! {
        SysBinding::shutdown(&mut self._interface)
    }

    #[inline]
    unsafe fn panic(&self, error: Optional<Error>) -> ! {
        SysBinding::panic(&self._interface, error)
    }

    #[inline]
    unsafe fn has_function(&self, id: FnId) -> Bool {
        SysBinding::has_function(&self._interface, id)
    }

    #[inline]
    unsafe fn get_function(&self, id: FnId) -> Optional<fn()> {
        SysBinding::get_function(&self._interface, id)
    }

    #[inline]
    unsafe fn lock(&self) {
        SysBinding::lock(&self._interface)
    }

    #[inline]
    unsafe fn try_lock(&self) -> Bool {
        SysBinding::try_lock(&self._interface)
    }

    #[inline]
    unsafe fn unlock(&self) {
        SysBinding::unlock(&self._interface)
    }

    #[inline]
    unsafe fn get_sync_handler(&self) -> SyncHandlerInterface {
        SysBinding::get_sync_handler(&self._interface)
    }

    #[inline]
    unsafe fn set_sync_handler(&mut self, handler: Optional<SyncHandlerInterface>) {
        SysBinding::set_sync_handler(&mut self._interface, handler)
    }
}

impl VersionBinding for CBaseRef<'_> {
    #[inline]
    unsafe fn new_short(&self, major: i32, minor: i32, patch: i32) -> Version {
        VersionBinding::new_short(&self._interface, major, minor, patch)
    }

    #[inline]
    unsafe fn new_long(
        &self,
        major: i32,
        minor: i32,
        patch: i32,
        release_type: version::ReleaseType,
        release_number: i8,
    ) -> Version {
        VersionBinding::new_long(
            &self._interface,
            major,
            minor,
            patch,
            release_type,
            release_number,
        )
    }

    #[inline]
    unsafe fn new_full(
        &self,
        major: i32,
        minor: i32,
        patch: i32,
        release_type: version::ReleaseType,
        release_number: i8,
        build: i64,
    ) -> Version {
        VersionBinding::new_full(
            &self._interface,
            major,
            minor,
            patch,
            release_type,
            release_number,
            build,
        )
    }

    #[inline]
    unsafe fn from_string(&self, buffer: ConstSpan<u8>) -> Result<Version, Error> {
        VersionBinding::from_string(&self._interface, buffer)
    }

    #[inline]
    unsafe fn string_length_short(&self, version: NonNullConst<Version>) -> usize {
        VersionBinding::string_length_short(&self._interface, version)
    }

    #[inline]
    unsafe fn string_length_long(&self, version: NonNullConst<Version>) -> usize {
        VersionBinding::string_length_long(&self._interface, version)
    }

    #[inline]
    unsafe fn string_length_full(&self, version: NonNullConst<Version>) -> usize {
        VersionBinding::string_length_full(&self._interface, version)
    }

    #[inline]
    unsafe fn as_string_short(
        &self,
        version: NonNullConst<Version>,
        buffer: MutSpan<u8>,
    ) -> Result<usize, Error> {
        VersionBinding::as_string_short(&self._interface, version, buffer)
    }

    #[inline]
    unsafe fn as_string_long(
        &self,
        version: NonNullConst<Version>,
        buffer: MutSpan<u8>,
    ) -> Result<usize, Error> {
        VersionBinding::as_string_long(&self._interface, version, buffer)
    }

    #[inline]
    unsafe fn as_string_full(
        &self,
        version: NonNullConst<Version>,
        buffer: MutSpan<u8>,
    ) -> Result<usize, Error> {
        VersionBinding::as_string_full(&self._interface, version, buffer)
    }

    #[inline]
    unsafe fn string_is_valid(&self, version_string: ConstSpan<u8>) -> Bool {
        VersionBinding::string_is_valid(&self._interface, version_string)
    }

    #[inline]
    unsafe fn compare(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> i32 {
        VersionBinding::compare(&self._interface, lhs, rhs)
    }

    #[inline]
    unsafe fn compare_weak(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> i32 {
        VersionBinding::compare_weak(&self._interface, lhs, rhs)
    }

    #[inline]
    unsafe fn compare_strong(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> i32 {
        VersionBinding::compare_strong(&self._interface, lhs, rhs)
    }

    #[inline]
    unsafe fn is_compatible(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> Bool {
        VersionBinding::is_compatible(&self._interface, lhs, rhs)
    }
}

impl LibraryBinding for CBaseRef<'_> {
    #[inline]
    unsafe fn register_loader(
        &mut self,
        loader: LibraryLoaderInterface,
        lib_type: NonNullConst<library::LibraryType>,
    ) -> Result<library::LoaderHandle, Error> {
        LibraryBinding::register_loader(&mut self._interface, loader, lib_type)
    }

    #[inline]
    unsafe fn unregister_loader(&mut self, handle: library::LoaderHandle) -> Result<i8, Error> {
        LibraryBinding::unregister_loader(&mut self._interface, handle)
    }

    #[inline]
    unsafe fn get_loader_interface(
        &self,
        handle: library::LoaderHandle,
    ) -> Result<LibraryLoaderInterface, Error> {
        LibraryBinding::get_loader_interface(&self._interface, handle)
    }

    #[inline]
    unsafe fn get_loader_handle_from_type(
        &self,
        lib_type: NonNullConst<library::LibraryType>,
    ) -> Result<library::LoaderHandle, Error> {
        LibraryBinding::get_loader_handle_from_type(&self._interface, lib_type)
    }

    #[inline]
    unsafe fn get_loader_handle_from_library(
        &self,
        handle: library::LibraryHandle,
    ) -> Result<library::LoaderHandle, Error> {
        LibraryBinding::get_loader_handle_from_library(&self._interface, handle)
    }

    #[inline]
    unsafe fn get_num_loaders(&self) -> usize {
        LibraryBinding::get_num_loaders(&self._interface)
    }

    #[inline]
    unsafe fn library_exists(&self, handle: library::LibraryHandle) -> Bool {
        LibraryBinding::library_exists(&self._interface, handle)
    }

    #[inline]
    unsafe fn type_exists(&self, lib_type: NonNullConst<library::LibraryType>) -> Bool {
        LibraryBinding::type_exists(&self._interface, lib_type)
    }

    #[inline]
    unsafe fn get_library_types(
        &self,
        buffer: MutSpan<library::LibraryType>,
    ) -> Result<usize, Error> {
        LibraryBinding::get_library_types(&self._interface, buffer)
    }

    #[inline]
    unsafe fn create_library_handle(&mut self) -> library::LibraryHandle {
        LibraryBinding::create_library_handle(&mut self._interface)
    }

    #[inline]
    unsafe fn remove_library_handle(
        &mut self,
        handle: library::LibraryHandle,
    ) -> Result<i8, Error> {
        LibraryBinding::remove_library_handle(&mut self._interface, handle)
    }

    #[inline]
    unsafe fn link_library(
        &mut self,
        handle: library::LibraryHandle,
        loader: library::LoaderHandle,
        internal: library::InternalHandle,
    ) -> Result<i8, Error> {
        LibraryBinding::link_library(&mut self._interface, handle, loader, internal)
    }

    #[inline]
    unsafe fn get_internal_library_handle(
        &self,
        handle: library::LibraryHandle,
    ) -> Result<library::InternalHandle, Error> {
        LibraryBinding::get_internal_library_handle(&self._interface, handle)
    }

    #[inline]
    unsafe fn load(
        &mut self,
        loader: library::LoaderHandle,
        path: OSPathString,
    ) -> Result<library::LibraryHandle, Error> {
        LibraryBinding::load(&mut self._interface, loader, path)
    }

    #[inline]
    unsafe fn unload(&mut self, handle: library::LibraryHandle) -> Result<i8, Error> {
        LibraryBinding::unload(&mut self._interface, handle)
    }

    #[inline]
    unsafe fn get_data_symbol(
        &self,
        handle: library::LibraryHandle,
        symbol: NonNullConst<u8>,
    ) -> Result<library::Symbol<NonNullConst<c_void>>, Error> {
        LibraryBinding::get_data_symbol(&self._interface, handle, symbol)
    }

    #[inline]
    unsafe fn get_function_symbol(
        &self,
        handle: library::LibraryHandle,
        symbol: NonNullConst<u8>,
    ) -> Result<library::Symbol<fn()>, Error> {
        LibraryBinding::get_function_symbol(&self._interface, handle, symbol)
    }
}

impl ModuleBinding for CBaseRef<'_> {
    #[inline]
    unsafe fn register_loader(
        &mut self,
        loader: ModuleLoaderInterface,
        mod_type: NonNullConst<module::ModuleType>,
    ) -> Result<module::LoaderHandle, Error> {
        ModuleBinding::register_loader(&mut self._interface, loader, mod_type)
    }

    #[inline]
    unsafe fn unregister_loader(&mut self, loader: module::LoaderHandle) -> Result<i8, Error> {
        ModuleBinding::unregister_loader(&mut self._interface, loader)
    }

    #[inline]
    unsafe fn get_loader_interface(
        &self,
        loader: module::LoaderHandle,
    ) -> Result<ModuleLoaderInterface, Error> {
        ModuleBinding::get_loader_interface(&self._interface, loader)
    }

    #[inline]
    unsafe fn get_loader_handle_from_type(
        &self,
        mod_type: NonNullConst<module::ModuleType>,
    ) -> Result<module::LoaderHandle, Error> {
        ModuleBinding::get_loader_handle_from_type(&self._interface, mod_type)
    }

    #[inline]
    unsafe fn get_loader_handle_from_module(
        &self,
        handle: module::ModuleHandle,
    ) -> Result<module::LoaderHandle, Error> {
        ModuleBinding::get_loader_handle_from_module(&self._interface, handle)
    }

    #[inline]
    unsafe fn get_num_modules(&self) -> usize {
        ModuleBinding::get_num_modules(&self._interface)
    }

    #[inline]
    unsafe fn get_num_loaders(&self) -> usize {
        ModuleBinding::get_num_loaders(&self._interface)
    }

    #[inline]
    unsafe fn get_num_exported_interfaces(&self) -> usize {
        ModuleBinding::get_num_exported_interfaces(&self._interface)
    }

    #[inline]
    unsafe fn module_exists(&self, handle: module::ModuleHandle) -> Bool {
        ModuleBinding::module_exists(&self._interface, handle)
    }

    #[inline]
    unsafe fn type_exists(&self, mod_type: NonNullConst<module::ModuleType>) -> Bool {
        ModuleBinding::type_exists(&self._interface, mod_type)
    }

    #[inline]
    unsafe fn exported_interface_exists(
        &self,
        interface: NonNullConst<module::InterfaceDescriptor>,
    ) -> Bool {
        ModuleBinding::exported_interface_exists(&self._interface, interface)
    }

    #[inline]
    unsafe fn get_modules(&self, buffer: MutSpan<module::ModuleInfo>) -> Result<usize, Error> {
        ModuleBinding::get_modules(&self._interface, buffer)
    }

    #[inline]
    unsafe fn get_module_types(&self, buffer: MutSpan<module::ModuleType>) -> Result<usize, Error> {
        ModuleBinding::get_module_types(&self._interface, buffer)
    }

    #[inline]
    unsafe fn get_exported_interfaces(
        &self,
        buffer: MutSpan<module::InterfaceDescriptor>,
    ) -> Result<usize, Error> {
        ModuleBinding::get_exported_interfaces(&self._interface, buffer)
    }

    #[inline]
    unsafe fn get_exported_interface_handle(
        &self,
        interface: NonNullConst<module::InterfaceDescriptor>,
    ) -> Result<module::ModuleHandle, Error> {
        ModuleBinding::get_exported_interface_handle(&self._interface, interface)
    }

    #[inline]
    unsafe fn create_module_handle(&mut self) -> module::ModuleHandle {
        ModuleBinding::create_module_handle(&mut self._interface)
    }

    #[inline]
    unsafe fn remove_module_handle(&mut self, handle: module::ModuleHandle) -> Result<i8, Error> {
        ModuleBinding::remove_module_handle(&mut self._interface, handle)
    }

    #[inline]
    unsafe fn link_module(
        &mut self,
        handle: module::ModuleHandle,
        loader: module::LoaderHandle,
        internal: module::InternalHandle,
    ) -> Result<i8, Error> {
        ModuleBinding::link_module(&mut self._interface, handle, loader, internal)
    }

    #[inline]
    unsafe fn get_internal_module_handle(
        &self,
        handle: module::ModuleHandle,
    ) -> Result<module::InternalHandle, Error> {
        ModuleBinding::get_internal_module_handle(&self._interface, handle)
    }

    #[inline]
    unsafe fn add_module(
        &mut self,
        loader: module::LoaderHandle,
        path: OSPathString,
    ) -> Result<module::ModuleHandle, Error> {
        ModuleBinding::add_module(&mut self._interface, loader, path)
    }

    #[inline]
    unsafe fn remove_module(&mut self, handle: module::ModuleHandle) -> Result<i8, Error> {
        ModuleBinding::remove_module(&mut self._interface, handle)
    }

    #[inline]
    unsafe fn load(&mut self, handle: module::ModuleHandle) -> Result<i8, Error> {
        ModuleBinding::load(&mut self._interface, handle)
    }

    #[inline]
    unsafe fn unload(&mut self, handle: module::ModuleHandle) -> Result<i8, Error> {
        ModuleBinding::unload(&mut self._interface, handle)
    }

    #[inline]
    unsafe fn initialize(&mut self, handle: module::ModuleHandle) -> Result<i8, Error> {
        ModuleBinding::initialize(&mut self._interface, handle)
    }

    #[inline]
    unsafe fn terminate(&mut self, handle: module::ModuleHandle) -> Result<i8, Error> {
        ModuleBinding::terminate(&mut self._interface, handle)
    }

    #[inline]
    unsafe fn add_dependency(
        &mut self,
        handle: module::ModuleHandle,
        interface: NonNullConst<module::InterfaceDescriptor>,
    ) -> Result<i8, Error> {
        ModuleBinding::add_dependency(&mut self._interface, handle, interface)
    }

    #[inline]
    unsafe fn remove_dependency(
        &mut self,
        handle: module::ModuleHandle,
        interface: NonNullConst<module::InterfaceDescriptor>,
    ) -> Result<i8, Error> {
        ModuleBinding::remove_dependency(&mut self._interface, handle, interface)
    }

    #[inline]
    unsafe fn export_interface(
        &mut self,
        handle: module::ModuleHandle,
        interface: NonNullConst<module::InterfaceDescriptor>,
    ) -> Result<i8, Error> {
        ModuleBinding::export_interface(&mut self._interface, handle, interface)
    }

    #[inline]
    unsafe fn get_load_dependencies(
        &self,
        handle: module::ModuleHandle,
    ) -> Result<ConstSpan<module::InterfaceDescriptor>, Error> {
        ModuleBinding::get_load_dependencies(&self._interface, handle)
    }

    #[inline]
    unsafe fn get_runtime_dependencies(
        &self,
        handle: module::ModuleHandle,
    ) -> Result<ConstSpan<module::InterfaceDescriptor>, Error> {
        ModuleBinding::get_runtime_dependencies(&self._interface, handle)
    }

    #[inline]
    unsafe fn get_exportable_interfaces(
        &self,
        handle: module::ModuleHandle,
    ) -> Result<ConstSpan<module::InterfaceDescriptor>, Error> {
        ModuleBinding::get_exportable_interfaces(&self._interface, handle)
    }

    #[inline]
    unsafe fn fetch_status(
        &self,
        handle: module::ModuleHandle,
    ) -> Result<module::ModuleStatus, Error> {
        ModuleBinding::fetch_status(&self._interface, handle)
    }

    #[inline]
    unsafe fn get_module_path(&self, handle: module::ModuleHandle) -> Result<OSPathString, Error> {
        ModuleBinding::get_module_path(&self._interface, handle)
    }

    #[inline]
    unsafe fn get_module_info(
        &self,
        handle: module::ModuleHandle,
    ) -> Result<NonNullConst<module::ModuleInfo>, Error> {
        ModuleBinding::get_module_info(&self._interface, handle)
    }

    #[inline]
    unsafe fn get_interface(
        &self,
        handle: module::ModuleHandle,
        interface: NonNullConst<module::InterfaceDescriptor>,
    ) -> Result<module::Interface, Error> {
        ModuleBinding::get_interface(&self._interface, handle, interface)
    }
}

impl<'interface> CBaseInterfaceInfo for CBaseRef<'interface> {
    type Interface = CBaseInterface;

    #[inline]
    fn interface_version(&self) -> Version {
        self._interface.interface_version()
    }

    #[inline]
    fn internal_interface(&self) -> &Self::Interface {
        &self._interface
    }
}

impl<'interface> CBaseAccess<'interface> for CBase<'interface> {
    type Interface = CBaseRef<'interface>;

    #[inline]
    fn lock<U>(&self, f: impl FnOnce(&mut Self::Interface) -> U) -> U {
        unsafe {
            SysBinding::lock(&*self._interface.get());
            let result = self.assume_locked(f);
            SysBinding::unlock(&*self._interface.get());
            result
        }
    }

    #[inline]
    fn try_lock<U>(&self, f: impl FnOnce(&mut Self::Interface) -> U) -> Option<U> {
        unsafe {
            if SysBinding::try_lock(&*self._interface.get()) == Bool::False {
                Option::None
            } else {
                let result = self.assume_locked(f);
                SysBinding::unlock(&*self._interface.get());
                Some(result)
            }
        }
    }

    #[inline]
    unsafe fn assume_locked<U>(&self, f: impl FnOnce(&mut Self::Interface) -> U) -> U {
        f(&mut *self._interface.get())
    }
}

impl<'interface> CBaseInterfaceInfo for CBase<'interface> {
    type Interface = <CBaseRef<'interface> as CBaseInterfaceInfo>::Interface;

    fn interface_version(&self) -> Version {
        unsafe { self.assume_locked(|int| CBaseInterfaceInfo::interface_version(int)) }
    }

    fn internal_interface(&self) -> &Self::Interface {
        unsafe {
            self.assume_locked(|int| {
                std::mem::transmute(CBaseInterfaceInfo::internal_interface(int))
            })
        }
    }
}

impl<'interface> SysAPIMin<'interface> for CBase<'interface> {
    fn panic(&self, error: Option<crate::error::Error<Owned>>) -> ! {
        unsafe { self.assume_locked(|int| SysAPIMin::panic(int, error)) }
    }

    fn has_function<U>(&self) -> bool
    where
        U: FnCaster,
    {
        unsafe { self.assume_locked(|int| SysAPIMin::has_function::<U>(int)) }
    }

    fn get_function<U>(&self, caster: &U) -> Option<<U as FnCaster>::Type>
    where
        U: FnCaster,
    {
        unsafe { self.assume_locked(move |int| SysAPIMin::get_function(int, caster)) }
    }
}

impl VersionBinding for CBase<'_> {
    #[inline]
    unsafe fn new_short(&self, major: i32, minor: i32, patch: i32) -> Version {
        VersionBinding::new_short(&*self._interface.get(), major, minor, patch)
    }

    #[inline]
    unsafe fn new_long(
        &self,
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
    ) -> Version {
        VersionBinding::new_long(
            &*self._interface.get(),
            major,
            minor,
            patch,
            release_type,
            release_number,
        )
    }

    #[inline]
    unsafe fn new_full(
        &self,
        major: i32,
        minor: i32,
        patch: i32,
        release_type: ReleaseType,
        release_number: i8,
        build: i64,
    ) -> Version {
        VersionBinding::new_full(
            &*self._interface.get(),
            major,
            minor,
            patch,
            release_type,
            release_number,
            build,
        )
    }

    #[inline]
    unsafe fn from_string(&self, buffer: ConstSpan<u8>) -> Result<Version, Error> {
        VersionBinding::from_string(&*self._interface.get(), buffer)
    }

    #[inline]
    unsafe fn string_length_short(&self, version: NonNullConst<Version>) -> usize {
        VersionBinding::string_length_short(&*self._interface.get(), version)
    }

    #[inline]
    unsafe fn string_length_long(&self, version: NonNullConst<Version>) -> usize {
        VersionBinding::string_length_long(&*self._interface.get(), version)
    }

    #[inline]
    unsafe fn string_length_full(&self, version: NonNullConst<Version>) -> usize {
        VersionBinding::string_length_full(&*self._interface.get(), version)
    }

    #[inline]
    unsafe fn as_string_short(
        &self,
        version: NonNullConst<Version>,
        buffer: MutSpan<u8>,
    ) -> Result<usize, Error> {
        VersionBinding::as_string_short(&*self._interface.get(), version, buffer)
    }

    #[inline]
    unsafe fn as_string_long(
        &self,
        version: NonNullConst<Version>,
        buffer: MutSpan<u8>,
    ) -> Result<usize, Error> {
        VersionBinding::as_string_long(&*self._interface.get(), version, buffer)
    }

    #[inline]
    unsafe fn as_string_full(
        &self,
        version: NonNullConst<Version>,
        buffer: MutSpan<u8>,
    ) -> Result<usize, Error> {
        VersionBinding::as_string_full(&*self._interface.get(), version, buffer)
    }

    #[inline]
    unsafe fn string_is_valid(&self, version_string: ConstSpan<u8>) -> Bool {
        VersionBinding::string_is_valid(&*self._interface.get(), version_string)
    }

    #[inline]
    unsafe fn compare(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> i32 {
        VersionBinding::compare(&*self._interface.get(), lhs, rhs)
    }

    #[inline]
    unsafe fn compare_weak(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> i32 {
        VersionBinding::compare_weak(&*self._interface.get(), lhs, rhs)
    }

    #[inline]
    unsafe fn compare_strong(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> i32 {
        VersionBinding::compare_strong(&*self._interface.get(), lhs, rhs)
    }

    #[inline]
    unsafe fn is_compatible(&self, lhs: NonNullConst<Version>, rhs: NonNullConst<Version>) -> Bool {
        VersionBinding::is_compatible(&*self._interface.get(), lhs, rhs)
    }
}
