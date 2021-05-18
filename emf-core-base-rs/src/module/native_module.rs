//! API of a native module.
use crate::ffi::collections::NonNullConst;
use crate::ffi::module::native_module::{
    NativeModule as NativeModuleFFI, NativeModuleBinding,
    NativeModuleInterface as NativeModuleInterfaceFFI,
};
use crate::ffi::CBaseBinding;
use crate::module::{Interface, InterfaceDescriptor, Module, ModuleInfo};
use crate::ownership::{
    AccessIdentifier, BorrowImmutable, BorrowMutable, ImmutableAccessIdentifier,
    MutableAccessIdentifier, Owned,
};
use crate::CBaseInterfaceInfo;
use crate::Error;
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

/// An instance from a native module.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NativeModuleInstance<'a, O> {
    _handle: Option<NonNull<NativeModuleFFI>>,
    _phantom: PhantomData<&'a ()>,
    _ownership: PhantomData<*const O>,
}

unsafe impl<O> Send for NativeModuleInstance<'_, O> {}
unsafe impl<O> Sync for NativeModuleInstance<'_, O> {}

impl<O> NativeModuleInstance<'_, O>
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
    pub const unsafe fn new(handle: Option<NonNull<NativeModuleFFI>>) -> Self {
        Self {
            _handle: handle,
            _phantom: PhantomData,
            _ownership: PhantomData,
        }
    }

    /// Fetches the internal handle.
    #[inline]
    pub const fn as_handle(&self) -> Option<NonNull<NativeModuleFFI>> {
        self._handle
    }
}

impl NativeModuleInstance<'_, Owned> {
    /// Borrows the instance.
    #[inline]
    pub const fn as_borrowed(&self) -> NativeModuleInstance<'_, BorrowImmutable<'_>> {
        unsafe { NativeModuleInstance::<BorrowImmutable<'_>>::new(self._handle) }
    }

    /// Borrows the instance mutably.
    #[inline]
    pub fn as_borrowed_mut(&mut self) -> NativeModuleInstance<'_, BorrowMutable<'_>> {
        unsafe { NativeModuleInstance::<BorrowMutable<'_>>::new(self._handle) }
    }
}

/// A native module.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NativeModule<'a, O> {
    _interface: NonNullConst<NativeModuleInterfaceFFI>,
    _phantom: PhantomData<&'a ()>,
    _ownership: PhantomData<*const O>,
}

unsafe impl<O> Send for NativeModule<'_, O> {}
unsafe impl<O> Sync for NativeModule<'_, O> {}

impl<O> Deref for NativeModule<'_, O> {
    type Target = NonNullConst<NativeModuleInterfaceFFI>;

    fn deref(&self) -> &Self::Target {
        &self._interface
    }
}

impl<O> DerefMut for NativeModule<'_, O> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self._interface
    }
}

impl<O> NativeModule<'_, O>
where
    O: AccessIdentifier,
{
    /// Construct a new instance from an interface.
    ///
    /// # Safety
    ///
    /// This function allows the creation of invalid modules
    /// by bypassing lifetimes.
    #[inline]
    pub const unsafe fn new(interface: NonNullConst<NativeModuleInterfaceFFI>) -> Self {
        Self {
            _interface: interface,
            _phantom: PhantomData,
            _ownership: PhantomData,
        }
    }
}

impl NativeModule<'_, Owned> {
    /// Borrows the module interface.
    #[inline]
    pub const fn as_borrowed(&self) -> NativeModule<'_, BorrowImmutable<'_>> {
        unsafe { NativeModule::<'_, BorrowImmutable<'_>>::new(self._interface) }
    }

    /// Borrows the module interface mutably.
    #[inline]
    pub fn as_borrowed_mut(&mut self) -> NativeModule<'_, BorrowMutable<'_>> {
        unsafe { NativeModule::<'_, BorrowMutable<'_>>::new(self._interface) }
    }
}

impl<'a, O> NativeModule<'a, O>
where
    O: MutableAccessIdentifier,
{
    /// Loads the module.
    ///
    /// # Failure
    ///
    /// The function can fail if some module invariant is not met.
    ///
    /// # Return
    ///
    /// Handle on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeModule] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn load<MO>(
        &mut self,
        module: &Module<'_, MO>,
        interface: &impl CBaseInterfaceInfo,
    ) -> Result<NativeModuleInstance<'a, Owned>, Error<Owned>>
    where
        MO: AccessIdentifier,
    {
        let internal = interface.internal_interface();
        let interface_handle = internal.base_module();
        let has_fn_fn = internal.fetch_has_function_fn();
        let get_fn_fn = internal.fetch_get_function_fn();

        self._interface
            .into_mut()
            .as_mut()
            .load(module.as_handle(), interface_handle, has_fn_fn, get_fn_fn)
            .map_or_else(
                |e| Err(Error::from(e)),
                |v| Ok(NativeModuleInstance::new(v)),
            )
    }

    /// Unloads the module.
    ///
    /// # Failure
    ///
    /// The function can fail if some module invariant is not met or `instance` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeModule] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn unload(
        &mut self,
        instance: NativeModuleInstance<'_, Owned>,
    ) -> Result<(), Error<Owned>> {
        self._interface
            .into_mut()
            .as_mut()
            .unload(instance.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
    }

    /// Initializes the module.
    ///
    /// # Failure
    ///
    /// The function can fail if some module invariant is not met or `instance` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeModule] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn initialize(
        &mut self,
        instance: &mut NativeModuleInstance<'_, Owned>,
    ) -> Result<(), Error<Owned>> {
        self._interface
            .into_mut()
            .as_mut()
            .initialize(instance.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
    }

    /// Terminates the module.
    ///
    /// # Failure
    ///
    /// The function can fail if some module invariant is not met or `instance` is invalid.
    ///
    /// # Return
    ///
    /// Error on failure.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeModule] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn terminate(
        &mut self,
        instance: &mut NativeModuleInstance<'_, Owned>,
    ) -> Result<(), Error<Owned>> {
        self._interface
            .into_mut()
            .as_mut()
            .terminate(instance.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |_v| Ok(()))
    }
}

impl<'a, O> NativeModule<'a, O>
where
    O: ImmutableAccessIdentifier,
{
    /// Fetches an interface from the module.
    ///
    /// # Failure
    ///
    /// The function fails if `instance` is invalid.
    ///
    /// # Return
    ///
    /// Interface on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    /// Direct usage of a [NativeModule] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn get_interface<'instance, IO, T>(
        &self,
        instance: &'instance NativeModuleInstance<'instance, IO>,
        interface: &InterfaceDescriptor,
        caster: impl FnOnce(crate::ffi::module::Interface) -> T,
    ) -> Result<Interface<'instance, T>, Error<Owned>>
    where
        IO: ImmutableAccessIdentifier,
    {
        self._interface
            .as_ref()
            .get_interface(instance.as_handle(), NonNullConst::from(interface))
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |v| Ok(Interface::new(caster(v))))
    }

    /// Fetches the module info of the module.
    ///
    /// # Failure
    ///
    /// The function fails if `instance` is invalid.
    ///
    /// # Return
    ///
    /// Module info on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    /// Direct usage of a [NativeModule] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn get_module_info<'instance, IO>(
        &self,
        instance: &'instance NativeModuleInstance<'instance, IO>,
    ) -> Result<&'instance ModuleInfo, Error<Owned>>
    where
        IO: ImmutableAccessIdentifier,
    {
        self._interface
            .as_ref()
            .get_module_info(instance.as_handle())
            .into_rust()
            .map_or_else(|e| Err(Error::from(e)), |v| Ok(&*v.as_ptr()))
    }

    /// Fetches the load dependencies of the module.
    ///
    /// # Return
    ///
    /// Load dependencies.
    ///
    /// # Safety
    ///
    /// The function crosses the ffi boundary.
    /// Direct usage of a [NativeModule] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn get_load_dependencies(&self) -> &'a [InterfaceDescriptor] {
        let span = self._interface.as_ref().get_load_dependencies();
        if span.is_empty() {
            <&[_]>::default()
        } else {
            std::slice::from_raw_parts(span.as_ptr(), span.len())
        }
    }

    /// Fetches the runtime dependencies of the module.
    ///
    /// # Failure
    ///
    /// The function fails if `instance` is invalid.
    ///
    /// # Return
    ///
    /// Runtime dependencies on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    /// Direct usage of a [NativeModule] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn get_runtime_dependencies<'instance, IO>(
        &self,
        instance: &'instance NativeModuleInstance<'instance, IO>,
    ) -> Result<&'instance [InterfaceDescriptor], Error<Owned>>
    where
        IO: ImmutableAccessIdentifier,
    {
        self._interface
            .as_ref()
            .get_runtime_dependencies(instance.as_handle())
            .into_rust()
            .map_or_else(
                |e| Err(Error::from(e)),
                |v| {
                    if v.is_empty() {
                        Ok(<&[_]>::default())
                    } else {
                        Ok(std::slice::from_raw_parts(v.as_ptr(), v.len()))
                    }
                },
            )
    }

    /// Fetches the exportable interfaces of the module.
    ///
    /// # Failure
    ///
    /// The function fails if `instance` is invalid.
    ///
    /// # Return
    ///
    /// Exportable interfaces on success, error otherwise.
    ///
    /// # Safety
    ///
    /// The function is not thread-safe and crosses the ffi boundary.
    /// Direct usage of a [NativeModule] may break some invariants
    /// of the module api, if not handled with care.
    #[inline]
    pub unsafe fn get_exportable_interfaces<'instance, IO>(
        &self,
        instance: &'instance NativeModuleInstance<'instance, IO>,
    ) -> Result<&'instance [InterfaceDescriptor], Error<Owned>>
    where
        IO: ImmutableAccessIdentifier,
    {
        self._interface
            .as_ref()
            .get_exportable_interfaces(instance.as_handle())
            .into_rust()
            .map_or_else(
                |e| Err(Error::from(e)),
                |v| {
                    if v.is_empty() {
                        Ok(<&[_]>::default())
                    } else {
                        Ok(std::slice::from_raw_parts(v.as_ptr(), v.len()))
                    }
                },
            )
    }
}
