use crate::{ffi, FromFFI};
use std::marker::PhantomData;

/// The handle to the default module loader.
pub const DEFAULT_MODULE_LOADER: ModuleLoaderHandleRef<'static> =
    unsafe { ModuleLoaderHandleRef::from_native_const(ffi::module::MODULE_LOADER_DEFAULT_HANDLE) };

/// A borrowed handle to a module loader.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ModuleLoaderHandleRef<'a> {
    handle: ffi::module::LoaderHandle,
    phantom: PhantomData<&'a ()>,
}

impl<'a> ModuleLoaderHandleRef<'a> {
    /// Extends the lifetime of the handle.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the handle lives long enough.
    pub unsafe fn extend_lifetime<'b>(self) -> ModuleLoaderHandleRef<'b> {
        ModuleLoaderHandleRef {
            handle: self.handle,
            phantom: PhantomData,
        }
    }

    /// Constructs itself from a [ffi::module::LoaderHandle]
    ///
    /// # Safety
    ///
    /// Handling ffi objects elides lifetimes.
    pub(self) const unsafe fn from_native_const(handle: ffi::module::LoaderHandle) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<'a> AsRef<ffi::module::LoaderHandle> for ModuleLoaderHandleRef<'a> {
    fn as_ref(&self) -> &ffi::module::LoaderHandle {
        &self.handle
    }
}

impl<'a> FromFFI<ffi::module::LoaderHandle> for ModuleLoaderHandleRef<'a> {
    unsafe fn from_ffi(handle: ffi::module::LoaderHandle) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

/// The handle to a module loader.
#[derive(Debug, Eq, PartialEq)]
pub struct ModuleLoaderHandle<'a> {
    handle: ModuleLoaderHandleRef<'a>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> ModuleLoaderHandle<'a> {
    /// Extends the lifetime of the handle.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the handle lives long enough.
    pub unsafe fn extend_lifetime<'b>(self) -> ModuleLoaderHandle<'b> {
        ModuleLoaderHandle {
            handle: self.handle.extend_lifetime(),
            phantom: PhantomData,
        }
    }
}

impl<'a> AsRef<ffi::module::LoaderHandle> for ModuleLoaderHandle<'a> {
    fn as_ref(&self) -> &ffi::module::LoaderHandle {
        self.handle.as_ref()
    }
}

impl<'a> AsRef<ModuleLoaderHandleRef<'a>> for ModuleLoaderHandle<'a> {
    fn as_ref(&self) -> &ModuleLoaderHandleRef<'a> {
        &self.handle
    }
}

impl<'a> FromFFI<ffi::module::LoaderHandle> for ModuleLoaderHandle<'a> {
    unsafe fn from_ffi(handle: ffi::module::LoaderHandle) -> Self {
        Self {
            handle: ModuleLoaderHandleRef::from_ffi(handle),
            phantom: PhantomData,
        }
    }
}

impl<'a> FromFFI<ModuleLoaderHandleRef<'a>> for ModuleLoaderHandle<'a> {
    unsafe fn from_ffi(handle: ModuleLoaderHandleRef<'a>) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}
