use crate::{ffi, FromFFI};
use std::marker::PhantomData;

/// The handle to a library.
#[derive(Debug, Eq, PartialEq)]
pub struct LibraryHandle<'a> {
    handle: ffi::library::LibraryHandle,
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
            handle: self.handle,
            phantom: PhantomData,
        }
    }
}

impl<'a> AsRef<ffi::library::LibraryHandle> for LibraryHandle<'a> {
    fn as_ref(&self) -> &ffi::library::LibraryHandle {
        &self.handle
    }
}

impl<'a> FromFFI<ffi::library::LibraryHandle> for LibraryHandle<'a> {
    unsafe fn from_ffi(handle: ffi::library::LibraryHandle) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}
