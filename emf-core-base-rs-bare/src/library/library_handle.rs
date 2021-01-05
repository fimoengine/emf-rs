use crate::{ffi, FromFFI};
use std::marker::PhantomData;

/// A borrowed handle to a library.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LibraryHandleRef<'a> {
    handle: ffi::library::LibraryHandle,
    phantom: PhantomData<&'a ffi::library::LibraryHandle>,
}

impl<'a> LibraryHandleRef<'a> {
    /// Extends the lifetime of the handle.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the handle lives long enough.
    pub unsafe fn extend_lifetime<'b>(self) -> LibraryHandleRef<'b> {
        LibraryHandleRef {
            handle: self.handle,
            phantom: PhantomData,
        }
    }
}

impl<'a> AsRef<ffi::library::LibraryHandle> for LibraryHandleRef<'a> {
    fn as_ref(&self) -> &ffi::library::LibraryHandle {
        &self.handle
    }
}

impl<'a> FromFFI<ffi::library::LibraryHandle> for LibraryHandleRef<'a> {
    unsafe fn from_ffi(handle: ffi::library::LibraryHandle) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

/// The handle to a library.
#[derive(Debug, Eq, PartialEq)]
pub struct LibraryHandle<'a> {
    handle: LibraryHandleRef<'a>,
    phantom: PhantomData<&'a ffi::library::LibraryHandle>,
}

impl<'a> LibraryHandle<'a> {
    /// Extends the lifetime of the handle.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the handle lives long enough.
    pub unsafe fn extend_lifetime<'b>(self) -> LibraryHandle<'b> {
        LibraryHandle {
            handle: self.handle.extend_lifetime::<'b>(),
            phantom: PhantomData,
        }
    }
}

impl<'a> AsRef<ffi::library::LibraryHandle> for LibraryHandle<'a> {
    fn as_ref(&self) -> &ffi::library::LibraryHandle {
        self.handle.as_ref()
    }
}

impl<'a> AsRef<LibraryHandleRef<'a>> for LibraryHandle<'a> {
    fn as_ref(&self) -> &LibraryHandleRef<'a> {
        &self.handle
    }
}

impl<'a> FromFFI<ffi::library::LibraryHandle> for LibraryHandle<'a> {
    unsafe fn from_ffi(handle: ffi::library::LibraryHandle) -> Self {
        Self {
            handle: LibraryHandleRef::from_ffi(handle),
            phantom: PhantomData,
        }
    }
}

impl<'a> FromFFI<LibraryHandleRef<'a>> for LibraryHandle<'a> {
    unsafe fn from_ffi(handle: LibraryHandleRef<'a>) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}
