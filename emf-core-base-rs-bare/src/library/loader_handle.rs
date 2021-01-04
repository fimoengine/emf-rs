use crate::{ffi, FromFFI};
use emf_core_base_rs_ffi_bare::library::LoaderHandle;
use std::marker::PhantomData;

/// The handle to the default library loader.
pub const DEFAULT_LIBRARY_LOADER: LibraryLoaderHandle<'static> =
    unsafe { LibraryLoaderHandle::from_native_const(ffi::library::LIBRARY_LOADER_DEFAULT_HANDLE) };

/// The handle to a library loader.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LibraryLoaderHandle<'a> {
    handle: ffi::library::LoaderHandle,
    phantom: PhantomData<&'a ()>,
}

impl<'a> LibraryLoaderHandle<'a> {
    /// Extends the lifetime of the handle.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the handle lives long enough.
    pub unsafe fn extend_lifetime<'b>(self) -> LibraryLoaderHandle<'b> {
        LibraryLoaderHandle {
            handle: self.handle,
            phantom: PhantomData,
        }
    }

    /// Constructs itself from a [ffi::library::LoaderHandle]
    ///
    /// # Safety
    ///
    /// Handling ffi objects elides lifetimes.
    pub(self) const unsafe fn from_native_const(handle: ffi::library::LoaderHandle) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}

impl<'a> AsRef<ffi::library::LoaderHandle> for LibraryLoaderHandle<'a> {
    fn as_ref(&self) -> &ffi::library::LoaderHandle {
        &self.handle
    }
}

impl<'a> FromFFI<ffi::library::LoaderHandle> for LibraryLoaderHandle<'a> {
    unsafe fn from_ffi(handle: LoaderHandle) -> Self {
        Self {
            handle,
            phantom: PhantomData,
        }
    }
}
