use crate::{ffi, FromFFI};
use std::marker::PhantomData;

/// A borrowed handle to a module.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct ModuleHandleRef<'a> {
    handle: ffi::module::ModuleHandle,
    phantom: PhantomData<&'a ()>,
}

impl<'a> ModuleHandleRef<'a> {
    /// Extends the lifetime of the handle.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the handle lives long enough.
    pub unsafe fn extend_lifetime<'b>(self) -> ModuleHandleRef<'b> {
        ModuleHandleRef {
            handle: self.handle,
            phantom: PhantomData,
        }
    }
}

impl<'a> AsRef<ffi::module::ModuleHandle> for ModuleHandleRef<'a> {
    fn as_ref(&self) -> &ffi::module::ModuleHandle {
        &self.handle
    }
}

impl<'a> FromFFI<ffi::module::ModuleHandle> for ModuleHandleRef<'a> {
    unsafe fn from_ffi(handle: ffi::module::ModuleHandle) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

/// A handle to a module.
#[derive(Debug, Eq, PartialEq)]
pub struct ModuleHandle<'a> {
    handle: ModuleHandleRef<'a>,
    phantom: PhantomData<&'a ()>,
}

impl<'a> ModuleHandle<'a> {
    /// Extends the lifetime of the handle.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the handle lives long enough.
    pub unsafe fn extend_lifetime<'b>(self) -> ModuleHandle<'b> {
        ModuleHandle {
            handle: self.handle.extend_lifetime(),
            phantom: PhantomData,
        }
    }
}

impl<'a> AsRef<ffi::module::ModuleHandle> for ModuleHandle<'a> {
    fn as_ref(&self) -> &ffi::module::ModuleHandle {
        self.handle.as_ref()
    }
}

impl<'a> AsRef<ModuleHandleRef<'a>> for ModuleHandle<'a> {
    fn as_ref(&self) -> &ModuleHandleRef<'a> {
        &self.handle
    }
}

impl<'a> FromFFI<ffi::module::ModuleHandle> for ModuleHandle<'a> {
    unsafe fn from_ffi(handle: ffi::module::ModuleHandle) -> Self {
        Self {
            handle: ModuleHandleRef::from_ffi(handle),
            phantom: PhantomData,
        }
    }
}

impl<'a> FromFFI<ModuleHandleRef<'a>> for ModuleHandle<'a> {
    unsafe fn from_ffi(handle: ModuleHandleRef<'a>) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}
