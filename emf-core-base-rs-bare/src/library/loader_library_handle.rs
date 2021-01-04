use crate::library::library_loader::LibraryLoaderWrapper;
use crate::{ffi, FromFFI};
use std::marker::PhantomData;

/// The internal handle to a library.
#[derive(Debug, Eq, PartialEq)]
pub struct LoaderLibraryHandle<'a, 'b: 'a, T: LibraryLoaderWrapper<'b>> {
    handle: ffi::library::LoaderLibraryHandle,
    phantom: PhantomData<&'a T>,
    phantom_b: PhantomData<&'b ()>,
}

impl<'a, 'b: 'a, T: LibraryLoaderWrapper<'b>> LoaderLibraryHandle<'a, 'b, T> {
    /// Extends the lifetime of the handle.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the handle lives long enough.
    #[inline]
    pub unsafe fn extend_lifetime<'c>(self) -> LoaderLibraryHandle<'c, 'b, T> {
        LoaderLibraryHandle {
            handle: self.handle,
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }

    /// Casts the type of the loader to another type.
    pub fn cast<U: LibraryLoaderWrapper<'b> + From<T>>(self) -> LoaderLibraryHandle<'a, 'b, U> {
        LoaderLibraryHandle {
            handle: self.handle,
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }

    /// Casts the type of the loader to another type.
    ///
    /// # Safety
    ///
    /// Does not consume the original.
    pub unsafe fn cast_ref<U: LibraryLoaderWrapper<'b> + From<T>>(
        &self,
    ) -> LoaderLibraryHandle<'a, 'b, U> {
        LoaderLibraryHandle {
            handle: self.handle,
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }
}

impl<'a, 'b: 'a, T: LibraryLoaderWrapper<'b>> AsRef<ffi::library::LoaderLibraryHandle>
    for LoaderLibraryHandle<'a, 'b, T>
{
    fn as_ref(&self) -> &ffi::library::LoaderLibraryHandle {
        &self.handle
    }
}

impl<'a, 'b: 'a, T: LibraryLoaderWrapper<'b>> FromFFI<ffi::library::LoaderLibraryHandle>
    for LoaderLibraryHandle<'a, 'b, T>
{
    unsafe fn from_ffi(handle: ffi::library::LoaderLibraryHandle) -> Self {
        Self {
            handle,
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }
}
