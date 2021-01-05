use crate::library::library_loader::LibraryLoaderWrapper;
use crate::{ffi, FromFFI};
use std::marker::PhantomData;

/// A borrowed internal handle to a library.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LoaderLibraryHandleRef<'a, 'b: 'a, T: LibraryLoaderWrapper<'b>> {
    handle: ffi::library::LoaderLibraryHandle,
    phantom: PhantomData<&'a T>,
    phantom_b: PhantomData<&'b ()>,
}

impl<'a, 'b: 'a, T: LibraryLoaderWrapper<'b>> LoaderLibraryHandleRef<'a, 'b, T> {
    /// Extends the lifetime of the handle.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the handle lives long enough.
    #[inline]
    pub unsafe fn extend_lifetime<'c>(self) -> LoaderLibraryHandleRef<'c, 'b, T> {
        LoaderLibraryHandleRef {
            handle: self.handle,
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }

    /// Casts the type of the loader to another type.
    pub fn cast<U: LibraryLoaderWrapper<'b> + From<T>>(self) -> LoaderLibraryHandleRef<'a, 'b, U> {
        LoaderLibraryHandleRef {
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
    ) -> LoaderLibraryHandleRef<'a, 'b, U> {
        LoaderLibraryHandleRef {
            handle: self.handle,
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }
}

impl<'a, 'b: 'a, T: LibraryLoaderWrapper<'b>> AsRef<ffi::library::LoaderLibraryHandle>
    for LoaderLibraryHandleRef<'a, 'b, T>
{
    fn as_ref(&self) -> &ffi::library::LoaderLibraryHandle {
        &self.handle
    }
}

impl<'a, 'b: 'a, T: LibraryLoaderWrapper<'b>> FromFFI<ffi::library::LoaderLibraryHandle>
    for LoaderLibraryHandleRef<'a, 'b, T>
{
    unsafe fn from_ffi(handle: ffi::library::LoaderLibraryHandle) -> Self {
        Self {
            handle,
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }
}

/// The internal handle to a library.
#[derive(Debug, Eq, PartialEq)]
pub struct LoaderLibraryHandle<'a, 'b: 'a, T: LibraryLoaderWrapper<'b>> {
    handle: LoaderLibraryHandleRef<'a, 'b, T>,
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
            handle: self.handle.extend_lifetime::<'c>(),
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }

    /// Casts the type of the loader to another type.
    pub fn cast<U: LibraryLoaderWrapper<'b> + From<T>>(self) -> LoaderLibraryHandle<'a, 'b, U> {
        LoaderLibraryHandle {
            handle: self.handle.cast(),
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
            handle: self.handle.cast_ref(),
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }
}

impl<'a, 'b: 'a, T: LibraryLoaderWrapper<'b>> AsRef<ffi::library::LoaderLibraryHandle>
    for LoaderLibraryHandle<'a, 'b, T>
{
    fn as_ref(&self) -> &ffi::library::LoaderLibraryHandle {
        self.handle.as_ref()
    }
}

impl<'a, 'b: 'a, T: LibraryLoaderWrapper<'b>> AsRef<LoaderLibraryHandleRef<'a, 'b, T>>
    for LoaderLibraryHandle<'a, 'b, T>
{
    fn as_ref(&self) -> &LoaderLibraryHandleRef<'a, 'b, T> {
        &self.handle
    }
}

impl<'a, 'b: 'a, T: LibraryLoaderWrapper<'b>> FromFFI<ffi::library::LoaderLibraryHandle>
    for LoaderLibraryHandle<'a, 'b, T>
{
    unsafe fn from_ffi(handle: ffi::library::LoaderLibraryHandle) -> Self {
        Self {
            handle: LoaderLibraryHandleRef::from_ffi(handle),
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }
}

impl<'a, 'b: 'a, T: LibraryLoaderWrapper<'b>> FromFFI<LoaderLibraryHandleRef<'a, 'b, T>>
    for LoaderLibraryHandle<'a, 'b, T>
{
    unsafe fn from_ffi(handle: LoaderLibraryHandleRef<'a, 'b, T>) -> Self {
        Self {
            handle,
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }
}
