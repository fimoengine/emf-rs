//! Global module api.
//!
//! # Example
//!
//! ```no_run
//! use emf_core_base_rs::global::{LockToken, Unlock, module, version};
//! use emf_core_base_rs::module::{DEFAULT_HANDLE, InterfaceDescriptor, InterfaceName};
//! use emf_core_base_rs::ffi::collections::ConstSpan;
//! use std::path::Path;
//!
//! # use emf_core_base_rs::module::Error;
//! # fn main() -> Result<(), Error> {
//! let mut lock = LockToken::<Unlock>::lock();
//!
//! let module_path = Path::new("path to a module");
//! let interface_desc = InterfaceDescriptor {
//!     name: InterfaceName::from("my_interface"),
//!     version: version::new_short(1, 0, 0),
//!     extensions: ConstSpan::new()
//! };
//!
//! let mut module = module::add_module(&DEFAULT_HANDLE, &module_path)?;
//! module::load(&mut module)?;
//! module::initialize(&mut module)?;
//! module::export_interface(&mut module, &interface_desc)?;
//! # Ok(())
//! # }
//! ```
use crate::ffi::library::OSPathChar;
use crate::global::{get_interface as get_interface_glob, get_mut_interface};
use crate::module::module_loader::{ModuleLoader, ModuleLoaderABICompat, ModuleLoaderAPI};
use crate::module::{
    Error, Interface, InterfaceDescriptor, InternalModule, Loader, Module, ModuleAPI, ModuleInfo,
    ModuleStatus, ModuleType,
};
use crate::ownership::{
    BorrowImmutable, BorrowMutable, ImmutableAccessIdentifier, MutableAccessIdentifier, Owned,
};
use std::path::Path;

/// Registers a new module loader.
///
/// Module types starting with `__` are reserved for future use.
///
/// # Failure
///
/// The function fails if `mod_type` already exists.
///
/// # Return
///
/// Handle on success, error otherwise.
#[inline]
pub fn register_loader<'loader, LT, L>(
    loader: &'loader LT,
    mod_type: &impl AsRef<str>,
) -> Result<Loader<'static, Owned>, Error>
where
    L: ModuleLoaderAPI<'static>,
    ModuleLoader<L, Owned>: From<&'loader LT>,
{
    ModuleAPI::register_loader(get_mut_interface(), loader, mod_type)
}

/// Unregisters an existing module loader.
///
/// Unregistering a module loader also unloads the modules it loaded.
///
/// # Failure
///
/// The function fails if `loader` is invalid.
///
/// # Return
///
/// Error on failure.
#[inline]
pub fn unregister_loader(loader: Loader<'_, Owned>) -> Result<(), Error> {
    ModuleAPI::unregister_loader(get_mut_interface(), loader)
}

/// Fetches the interface of a module loader.
///
/// # Failure
///
/// The function fails if `loader` is invalid.
///
/// # Return
///
/// Interface on success, error otherwise.
#[inline]
pub fn get_loader_interface<'loader, O, L>(
    loader: &Loader<'loader, O>,
) -> Result<ModuleLoader<L, O>, Error>
where
    O: ImmutableAccessIdentifier,
    L: ModuleLoaderAPI<'loader> + ModuleLoaderABICompat,
{
    ModuleAPI::get_loader_interface(get_mut_interface(), loader)
}

/// Fetches the handle of the loader associated with a module type.
///
/// # Failure
///
/// The function fails if `mod_type` does not exist.
///
/// # Return
///
/// Handle on success, error otherwise.
#[inline]
pub fn get_loader_handle_from_type(
    mod_type: &impl AsRef<str>,
) -> Result<Loader<'static, BorrowMutable<'_>>, Error> {
    ModuleAPI::get_loader_handle_from_type(get_interface_glob(), mod_type)
}

/// Fetches the handle of the loader linked with the module handle.
///
/// # Failure
///
/// The function fails if `module` is invalid.
///
/// # Return
///
/// Handle on success, error otherwise.
#[inline]
pub fn get_loader_handle_from_module<'m, 'module, O>(
    module: &'m Module<'module, O>,
) -> Result<Loader<'module, BorrowMutable<'m>>, Error>
where
    O: ImmutableAccessIdentifier,
{
    ModuleAPI::get_loader_handle_from_module(get_interface_glob(), module)
}

/// Fetches the number of loaded modules.
///
/// # Return
///
/// Number of modules.
#[inline]
pub fn get_num_modules() -> usize {
    ModuleAPI::get_num_modules(get_interface_glob())
}

/// Fetches the number of loaders.
///
/// # Return
///
/// Number of module loaders.
#[inline]
pub fn get_num_loaders() -> usize {
    ModuleAPI::get_num_loaders(get_interface_glob())
}

/// Fetches the number of exported interfaces.
///
/// # Return
///
/// Number of exported interfaces.
#[inline]
pub fn get_num_exported_interfaces() -> usize {
    ModuleAPI::get_num_exported_interfaces(get_interface_glob())
}

/// Checks if a module exists.
///
/// # Return
///
/// [true] if it exists, [false] otherwise.
#[inline]
pub fn module_exists<O>(module: &Module<'_, O>) -> bool
where
    O: ImmutableAccessIdentifier,
{
    ModuleAPI::module_exists(get_interface_glob(), module)
}

/// Checks if a module type exists.
///
/// # Return
///
/// [true] if it exists, [false] otherwise.
#[inline]
pub fn type_exists(mod_type: &impl AsRef<str>) -> Result<bool, Error> {
    ModuleAPI::type_exists(get_interface_glob(), mod_type)
}

/// Checks whether an exported interface exists.
///
/// # Return
///
/// [true] if it exists, [false] otherwise.
#[inline]
pub fn exported_interface_exists(interface: &InterfaceDescriptor) -> bool {
    ModuleAPI::exported_interface_exists(get_interface_glob(), interface)
}

/// Copies the available module info into a buffer.
///
/// # Failure
///
/// Fails if `buffer.as_ref().len() < get_num_modules()`.
///
/// # Return
///
/// Number if written module info on success, error otherwise.
#[inline]
pub fn get_modules(buffer: &mut impl AsMut<[ModuleInfo]>) -> Result<usize, Error> {
    ModuleAPI::get_modules(get_interface_glob(), buffer)
}

/// Copies the available module types into a buffer.
///
/// # Failure
///
/// Fails if `buffer.as_ref().len() < get_num_loaders()`.
///
/// # Return
///
/// Number if written module types on success, error otherwise.
#[inline]
pub fn get_module_types(buffer: &mut impl AsMut<[ModuleType]>) -> Result<usize, Error> {
    ModuleAPI::get_module_types(get_interface_glob(), buffer)
}

/// Copies the descriptors of the exported interfaces into a buffer.
///
/// # Failure
///
/// Fails if `buffer.as_ref().len() < get_num_exported_interfaces()`.
///
/// # Return
///
/// Number if written descriptors on success, error otherwise.
#[inline]
pub fn get_exported_interfaces(
    buffer: &mut impl AsMut<[InterfaceDescriptor]>,
) -> Result<usize, Error> {
    ModuleAPI::get_exported_interfaces(get_interface_glob(), buffer)
}

/// Fetches the module handle of the exported interface.
///
/// # Failure
///
/// Fails if `interface` does not exist.
///
/// # Return
///
/// Module handle on success, error otherwise.
#[inline]
pub fn get_exported_interface_handle(
    interface: &InterfaceDescriptor,
) -> Result<Module<'static, BorrowImmutable<'_>>, Error> {
    ModuleAPI::get_exported_interface_handle(get_interface_glob(), interface)
}

/// Creates a new unlinked module handle.
///
/// # Return
///
/// Module handle.
///
/// # Safety
///
/// The handle remains invalid until it's linked with [link_module].
#[inline]
pub unsafe fn create_module_handle() -> Module<'static, Owned> {
    ModuleAPI::create_module_handle(get_mut_interface())
}

/// Links a module handle to an internal module handle.
///
/// # Failure
///
/// Fails if `module` or`loader` are invalid.
///
/// # Return
///
/// Error on failure.
///
/// # Safety
///
/// Removing the handle does not unload the module.
#[inline]
pub unsafe fn remove_module_handle(module: Module<'_, Owned>) -> Result<(), Error> {
    ModuleAPI::remove_module_handle(get_mut_interface(), module)
}

/// Links a module handle to an internal module handle.
///
/// # Failure
///
/// Fails if `module` or`loader` are invalid.
///
/// # Return
///
/// Error on failure.
///
/// # Safety
///
/// Incorrect usage can lead to dangling handles or use-after-free errors.
#[inline]
pub unsafe fn link_module<'module, 'loader, O, LO, IO>(
    module: &Module<'module, O>,
    loader: &Loader<'loader, LO>,
    internal: &InternalModule<IO>,
) -> Result<(), Error>
where
    'loader: 'module,
    O: MutableAccessIdentifier,
    LO: ImmutableAccessIdentifier,
    IO: ImmutableAccessIdentifier,
{
    ModuleAPI::link_module(get_mut_interface(), module, loader, internal)
}

/// Fetches the internal handle linked with the module handle.
///
/// # Failure
///
/// Fails if `module` is invalid.
///
/// # Return
///
/// Internal handle on success, error otherwise.
#[inline]
pub fn get_internal_module_handle<O>(module: &Module<'_, O>) -> Result<InternalModule<O>, Error>
where
    O: ImmutableAccessIdentifier,
{
    ModuleAPI::get_internal_module_handle(get_interface_glob(), module)
}

/// Adds a new module.
///
/// # Failure
///
/// Fails if `loader` or `path` is invalid or the type
/// of the module can not be loaded with the loader.
///
/// # Return
///
/// Module handle on success, error otherwise.
#[inline]
pub fn add_module<O>(
    loader: &Loader<'static, O>,
    path: &impl AsRef<Path>,
) -> Result<Module<'static, Owned>, Error>
where
    O: MutableAccessIdentifier,
{
    ModuleAPI::add_module(get_mut_interface(), loader, path)
}

/// Removes a module.
///
/// # Failure
///
/// Fails if `module` is invalid or the module is not in an unloaded state.
///
/// # Return
///
/// Error on failure.
#[inline]
pub fn remove_module(module: Module<'_, Owned>) -> Result<(), Error> {
    ModuleAPI::remove_module(get_mut_interface(), module)
}

/// Loads a module.
///
/// # Failure
///
/// Fails if `module` is invalid, the load dependencies of the module are
/// not exported or the module is not in an unloaded state.
///
/// # Return
///
/// Error on failure.
#[inline]
pub fn load<O>(module: &mut Module<'_, O>) -> Result<(), Error>
where
    O: MutableAccessIdentifier,
{
    ModuleAPI::load(get_mut_interface(), module)
}

/// Unloads a module.
///
/// # Failure
///
/// Fails if `module` is invalid or the module is in an unloaded or ready state.
///
/// # Return
///
/// Error on failure.
#[inline]
pub fn unload<O>(module: &mut Module<'_, O>) -> Result<(), Error>
where
    O: MutableAccessIdentifier,
{
    ModuleAPI::unload(get_mut_interface(), module)
}

/// Initializes a module.
///
/// # Failure
///
/// Fails if `module` is invalid, the runtime dependencies of the
/// module are not exported or the module is not in a loaded state.
///
/// # Return
///
/// Error on failure.
#[inline]
pub fn initialize<O>(module: &mut Module<'_, O>) -> Result<(), Error>
where
    O: MutableAccessIdentifier,
{
    ModuleAPI::initialize(get_mut_interface(), module)
}

/// Terminates a module.
///
/// Terminating a module also removes the interfaces it exported.
/// The modules that depend on the module are terminated.
/// If they list the module as a load dependency, they are also unloaded.
///
/// # Failure
///
/// Fails if `module` is invalid or the module is not in a ready state.
///
/// # Return
///
/// Error on failure.
#[inline]
pub fn terminate<O>(module: &mut Module<'_, O>) -> Result<(), Error>
where
    O: MutableAccessIdentifier,
{
    ModuleAPI::terminate(get_mut_interface(), module)
}

/// Registers a new runtime dependency of the module.
///
/// # Failure
///
/// Fails if `module` is invalid.
///
/// # Return
///
/// Error on failure.
#[inline]
pub fn add_dependency<O>(
    module: &mut Module<'_, O>,
    interface: &InterfaceDescriptor,
) -> Result<(), Error>
where
    O: MutableAccessIdentifier,
{
    ModuleAPI::add_dependency(get_mut_interface(), module, interface)
}

/// Removes an existing runtime dependency from the module.
///
/// # Failure
///
/// Fails if `module` is invalid.
///
/// # Return
///
/// Error on failure.
#[inline]
pub fn remove_dependency<O>(
    module: &mut Module<'_, O>,
    interface: &InterfaceDescriptor,
) -> Result<(), Error>
where
    O: MutableAccessIdentifier,
{
    ModuleAPI::remove_dependency(get_mut_interface(), module, interface)
}

/// Exports an interface of a module.
///
/// # Failure
///
/// Fails if `module` is invalid, `interface` is already exported,
/// `interface` is not contained in the module or the module is not yet initialized.
///
/// # Return
///
/// Error on failure.
#[inline]
pub fn export_interface<O>(
    module: &Module<'_, O>,
    interface: &InterfaceDescriptor,
) -> Result<(), Error>
where
    O: ImmutableAccessIdentifier,
{
    ModuleAPI::export_interface(get_mut_interface(), module, interface)
}

/// Fetches the load dependencies of a module.
///
/// # Failure
///
/// Fails if `module` is invalid.
///
/// # Return
///
/// Load dependencies on success, error otherwise.
#[inline]
pub fn get_load_dependencies<'module, O>(
    module: &Module<'module, O>,
) -> Result<&'module [InterfaceDescriptor], Error>
where
    O: ImmutableAccessIdentifier,
{
    ModuleAPI::get_load_dependencies(get_interface_glob(), module)
}

/// Fetches the runtime dependencies of a module.
///
/// # Failure
///
/// Fails if `module` is invalid or the module is not yet loaded.
///
/// # Return
///
/// Runtime dependencies on success, error otherwise.
#[inline]
pub fn get_runtime_dependencies<'module, O>(
    module: &Module<'module, O>,
) -> Result<&'module [InterfaceDescriptor], Error>
where
    O: ImmutableAccessIdentifier,
{
    ModuleAPI::get_runtime_dependencies(get_interface_glob(), module)
}

/// Fetches the exportable interfaces of a module.
///
/// # Failure
///
/// Fails if `module` is invalid or the module is not yet loaded.
///
/// # Return
///
/// Exportable interfaces on success, error otherwise.
#[inline]
pub fn get_exportable_interfaces<'module, O>(
    module: &Module<'module, O>,
) -> Result<&'module [InterfaceDescriptor], Error>
where
    O: ImmutableAccessIdentifier,
{
    ModuleAPI::get_exportable_interfaces(get_interface_glob(), module)
}

/// Fetches the load status of a module.
///
/// # Failure
///
/// Fails if `module` is invalid.
///
/// # Return
///
/// Module status on success, error otherwise.
#[inline]
pub fn fetch_status<O>(module: &Module<'_, O>) -> Result<ModuleStatus, Error>
where
    O: ImmutableAccessIdentifier,
{
    ModuleAPI::fetch_status(get_interface_glob(), module)
}

/// Fetches the path a module was loaded from.
///
/// # Failure
///
/// Fails if `module` is invalid or the module is not yet loaded.
///
/// # Return
///
/// Module path on success, error otherwise.
#[inline]
pub fn get_module_path<'module, O>(
    module: &Module<'module, O>,
) -> Result<&'module [OSPathChar], Error>
where
    O: ImmutableAccessIdentifier,
{
    ModuleAPI::get_module_path(get_interface_glob(), module)
}

/// Fetches the module info from a module.
///
/// # Failure
///
/// Fails if `module` is invalid or the module is not yet loaded.
///
/// # Return
///
/// Module info on success, error otherwise.
#[inline]
pub fn get_module_info<'module, O>(
    module: &Module<'module, O>,
) -> Result<&'module ModuleInfo, Error>
where
    O: ImmutableAccessIdentifier,
{
    ModuleAPI::get_module_info(get_interface_glob(), module)
}

/// Fetches an interface from a module.
///
/// # Failure
///
/// Fails if `module` is invalid, the module is not in a ready
/// state or the interface is not contained in the module.
///
/// # Return
///
/// Interface on success, error otherwise.
#[inline]
pub fn get_interface<'module, O, T>(
    module: &'module Module<'_, O>,
    interface: &InterfaceDescriptor,
    caster: impl FnOnce(crate::ffi::module::Interface) -> T,
) -> Result<Interface<'module, T>, Error>
where
    O: ImmutableAccessIdentifier,
{
    ModuleAPI::get_interface(get_interface_glob(), module, interface, caster)
}
