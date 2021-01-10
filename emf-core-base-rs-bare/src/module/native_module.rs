use crate::ffi::module::NativeModuleInterfaceBinding;
use crate::module::{InterfaceDescriptor, ModuleHandleRef, ModuleInterface};
use crate::module::{ModuleError, ModuleInfo};
use crate::{ffi, BaseInterface, FFIObject, FromFFI};
use emf_core_base_rs_ffi_bare::containers::NonNullConst;
use std::marker::PhantomData;

/// A trait providing the interface of a `NativeModule`.
pub trait NativeModuleWrapper<'a>:
    AsRef<&'a ffi::module::NativeModuleInterface>
    + FromFFI<&'a ffi::module::NativeModuleInterface>
    + Sized
{
    /// Loads the module.
    ///
    /// The function loads a module and returns a reference to the newly created instance.
    ///
    /// # Safety
    ///
    /// Direct usage of a `NativeModule` circumvents the safety of the `module` api.
    unsafe fn load<'b: 'a>(
        &self,
        module: &ModuleHandleRef<'a>,
        interface: &BaseInterface<'b>,
    ) -> Result<NativeModuleInstance<'a>, ModuleError>;

    /// Unloads the module.
    ///
    /// # Safety
    ///
    /// Direct usage of a `NativeModule` circumvents the safety of the `module` api.
    unsafe fn unload(&self, module: NativeModuleInstance<'a>) -> Option<ModuleError>;

    /// Initializes the module.
    ///
    /// # Safety
    ///
    /// Direct usage of a `NativeModule` circumvents the safety of the `module` api.
    unsafe fn initialize(&self, module: &NativeModuleInstance<'a>) -> Option<ModuleError>;

    /// Terminates the module.
    ///
    /// # Safety
    ///
    /// Direct usage of a `NativeModule` circumvents the safety of the `module` api.
    unsafe fn terminate(&self, module: &NativeModuleInstance<'a>) -> Option<ModuleError>;

    /// Fetches the module info from the module.
    ///
    /// # Safety
    ///
    /// Direct usage of a `NativeModule` circumvents the safety of the `module` api.
    unsafe fn get_module_info(
        &self,
        module: &NativeModuleInstance<'a>,
    ) -> Result<&'a ModuleInfo, ModuleError>;

    /// Fetches the load dependencies from the module.
    ///
    /// # Safety
    ///
    /// Direct usage of a `NativeModule` circumvents the safety of the `module` api.
    unsafe fn get_load_dependencies(&self) -> &'a [InterfaceDescriptor<'a>];

    /// Fetches the runtime dependencies from the module.
    ///
    /// # Safety
    ///
    /// Direct usage of a `NativeModule` circumvents the safety of the `module` api.
    unsafe fn get_runtime_dependencies(
        &self,
        module: &NativeModuleInstance<'a>,
    ) -> Result<&'a [InterfaceDescriptor<'a>], ModuleError>;

    /// Fetches the exportable interfaces from the module.
    ///
    /// # Safety
    ///
    /// Direct usage of a `NativeModule` circumvents the safety of the `module` api.
    unsafe fn get_exportable_interfaces(
        &self,
        module: &NativeModuleInstance<'a>,
    ) -> Result<&'a [InterfaceDescriptor<'a>], ModuleError>;

    /// Fetches an interface from the module.
    ///
    /// # Safety
    ///
    /// Direct usage of a `NativeModule` circumvents the safety of the `module` api.
    unsafe fn get_interface<T: Sized + FFIObject<ffi::module::ModuleInterface>>(
        &self,
        module: &NativeModuleInstance<'a>,
        interface: &InterfaceDescriptor,
    ) -> Result<ModuleInterface<'a, T>, ModuleError>;
}

/// A native module.
pub struct NativeModule<'a> {
    interface: &'a ffi::module::NativeModuleInterface,
}

impl<'a> NativeModule<'a> {
    /// Extends the lifetime of the `NativeModule`.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the `NativeModule` lives long enough.
    #[inline]
    pub unsafe fn extend_lifetime<'b>(self) -> NativeModule<'b> {
        NativeModule {
            interface: std::mem::transmute(self.interface),
        }
    }
}

impl<'a> NativeModuleWrapper<'a> for NativeModule<'a> {
    #[inline]
    unsafe fn load<'b: 'a>(
        &self,
        module: &ModuleHandleRef<'a>,
        interface: &BaseInterface<'b>,
    ) -> Result<NativeModuleInstance<'a>, ModuleError> {
        let interface: &ffi::BaseInterface = interface.as_ref();
        self.interface
            .load(
                module.as_native(),
                interface.cbase_module,
                interface.sys_has_function_fn,
                interface.sys_get_function_fn,
            )
            .to_native()
            .map(|m| NativeModuleInstance::from_native(m))
    }

    #[inline]
    #[must_use]
    unsafe fn unload(&self, module: NativeModuleInstance<'a>) -> Option<ModuleError> {
        self.interface.unload(module.as_native()).to_native()
    }

    #[inline]
    #[must_use]
    unsafe fn initialize(&self, module: &NativeModuleInstance<'a>) -> Option<ModuleError> {
        self.interface.initialize(module.as_native()).to_native()
    }

    #[inline]
    #[must_use]
    unsafe fn terminate(&self, module: &NativeModuleInstance<'a>) -> Option<ModuleError> {
        self.interface.terminate(module.as_native()).to_native()
    }

    #[inline]
    unsafe fn get_module_info(
        &self,
        module: &NativeModuleInstance<'a>,
    ) -> Result<&'a ModuleInfo, ModuleError> {
        self.interface
            .get_module_info(module.as_native())
            .to_native()
            .map(|m| &*m.as_ptr())
    }

    #[inline]
    #[must_use]
    unsafe fn get_load_dependencies(&self) -> &'a [InterfaceDescriptor<'a>] {
        let dep = self.interface.get_load_dependencies();
        std::slice::from_raw_parts(dep.as_ptr(), dep.len())
    }

    #[inline]
    unsafe fn get_runtime_dependencies(
        &self,
        module: &NativeModuleInstance<'a>,
    ) -> Result<&'a [InterfaceDescriptor<'a>], ModuleError> {
        self.interface
            .get_runtime_dependencies(module.as_native())
            .to_native()
            .map(|dep| std::slice::from_raw_parts(dep.as_ptr(), dep.len()))
    }

    #[inline]
    unsafe fn get_exportable_interfaces(
        &self,
        module: &NativeModuleInstance<'a>,
    ) -> Result<&'a [InterfaceDescriptor<'a>], ModuleError> {
        self.interface
            .get_exportable_interfaces(module.as_native())
            .to_native()
            .map(|dep| std::slice::from_raw_parts(dep.as_ptr(), dep.len()))
    }

    #[inline]
    unsafe fn get_interface<T: Sized + FFIObject<ffi::module::ModuleInterface>>(
        &self,
        module: &NativeModuleInstance<'a>,
        interface: &InterfaceDescriptor,
    ) -> Result<ModuleInterface<'a, T>, ModuleError> {
        self.interface
            .get_interface(module.as_native(), NonNullConst::from(interface))
            .to_native()
            .map(|i| ModuleInterface::from_native(i))
    }
}

impl<'a> AsRef<&'a ffi::module::NativeModuleInterface> for NativeModule<'a> {
    fn as_ref(&self) -> &&'a ffi::module::NativeModuleInterface {
        &self.interface
    }
}

impl<'a> FromFFI<&'a ffi::module::NativeModuleInterface> for NativeModule<'a> {
    unsafe fn from_ffi(v: &'a ffi::module::NativeModuleInterface) -> Self {
        Self { interface: v }
    }
}

/// An instance of a native module.
#[derive(Debug)]
pub struct NativeModuleInstance<'a> {
    instance: *mut ffi::module::NativeModule,
    phantom: PhantomData<&'a ()>,
}

impl<'a> NativeModuleInstance<'a> {
    /// Extends the lifetime of the `NativeModuleInstance`.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the `NativeModuleInstance` lives long enough.
    #[inline]
    pub unsafe fn extend_lifetime<'b>(self) -> NativeModuleInstance<'b> {
        NativeModuleInstance {
            instance: self.instance,
            phantom: PhantomData,
        }
    }
}

impl<'a> FFIObject<*mut ffi::module::NativeModule> for NativeModuleInstance<'a> {
    fn as_native(&self) -> *mut ffi::module::NativeModule {
        self.instance
    }

    unsafe fn from_native(val: *mut ffi::module::NativeModule) -> Self {
        Self {
            instance: val,
            phantom: PhantomData,
        }
    }
}
