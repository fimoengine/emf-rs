use crate::module::ModuleLoaderWrapper;
use crate::{ffi, FromFFI};
use std::marker::PhantomData;

/// A borrowed internal handle to a module.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct LoaderModuleHandleRef<'a, 'b: 'a, T: ModuleLoaderWrapper<'b>> {
    handle: ffi::module::LoaderModuleHandle,
    phantom: PhantomData<&'a T>,
    phantom_b: PhantomData<&'b ()>,
}

impl<'a, 'b: 'a, T: ModuleLoaderWrapper<'b>> LoaderModuleHandleRef<'a, 'b, T> {
    /// Extends the lifetime of the handle.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the handle lives long enough.
    #[inline]
    pub unsafe fn extend_lifetime<'c>(self) -> LoaderModuleHandleRef<'c, 'b, T> {
        LoaderModuleHandleRef {
            handle: self.handle,
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }

    /// Casts the type of the loader to another type.
    pub fn cast<U: ModuleLoaderWrapper<'b> + From<T>>(self) -> LoaderModuleHandleRef<'a, 'b, U> {
        LoaderModuleHandleRef {
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
    pub unsafe fn cast_ref<U: ModuleLoaderWrapper<'b> + From<T>>(
        &self,
    ) -> LoaderModuleHandleRef<'a, 'b, U> {
        LoaderModuleHandleRef {
            handle: self.handle,
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }
}

impl<'a, 'b: 'a, T: ModuleLoaderWrapper<'b>> AsRef<ffi::module::LoaderModuleHandle>
    for LoaderModuleHandleRef<'a, 'b, T>
{
    fn as_ref(&self) -> &ffi::module::LoaderModuleHandle {
        &self.handle
    }
}

impl<'a, 'b: 'a, T: ModuleLoaderWrapper<'b>> FromFFI<ffi::module::LoaderModuleHandle>
    for LoaderModuleHandleRef<'a, 'b, T>
{
    unsafe fn from_ffi(handle: ffi::module::LoaderModuleHandle) -> Self {
        Self {
            handle,
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }
}

/// The internal handle to a module.
#[derive(Debug, Eq, PartialEq)]
pub struct LoaderModuleHandle<'a, 'b: 'a, T: ModuleLoaderWrapper<'b>> {
    handle: LoaderModuleHandleRef<'a, 'b, T>,
    phantom: PhantomData<&'a T>,
    phantom_b: PhantomData<&'b ()>,
}

impl<'a, 'b: 'a, T: ModuleLoaderWrapper<'b>> LoaderModuleHandle<'a, 'b, T> {
    /// Extends the lifetime of the handle.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the handle lives long enough.
    #[inline]
    pub unsafe fn extend_lifetime<'c>(self) -> LoaderModuleHandle<'c, 'b, T> {
        LoaderModuleHandle {
            handle: self.handle.extend_lifetime::<'c>(),
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }

    /// Casts the type of the loader to another type.
    pub fn cast<U: ModuleLoaderWrapper<'b> + From<T>>(self) -> LoaderModuleHandle<'a, 'b, U> {
        LoaderModuleHandle {
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
    pub unsafe fn cast_ref<U: ModuleLoaderWrapper<'b> + From<T>>(
        &self,
    ) -> LoaderModuleHandle<'a, 'b, U> {
        LoaderModuleHandle {
            handle: self.handle.cast_ref(),
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }
}

impl<'a, 'b: 'a, T: ModuleLoaderWrapper<'b>> AsRef<ffi::module::LoaderModuleHandle>
    for LoaderModuleHandle<'a, 'b, T>
{
    fn as_ref(&self) -> &ffi::module::LoaderModuleHandle {
        self.handle.as_ref()
    }
}

impl<'a, 'b: 'a, T: ModuleLoaderWrapper<'b>> AsRef<LoaderModuleHandleRef<'a, 'b, T>>
    for LoaderModuleHandle<'a, 'b, T>
{
    fn as_ref(&self) -> &LoaderModuleHandleRef<'a, 'b, T> {
        &self.handle
    }
}

impl<'a, 'b: 'a, T: ModuleLoaderWrapper<'b>> FromFFI<ffi::module::LoaderModuleHandle>
    for LoaderModuleHandle<'a, 'b, T>
{
    unsafe fn from_ffi(handle: ffi::module::LoaderModuleHandle) -> Self {
        Self {
            handle: LoaderModuleHandleRef::from_ffi(handle),
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }
}

impl<'a, 'b: 'a, T: ModuleLoaderWrapper<'b>> FromFFI<LoaderModuleHandleRef<'a, 'b, T>>
    for LoaderModuleHandle<'a, 'b, T>
{
    unsafe fn from_ffi(handle: LoaderModuleHandleRef<'a, 'b, T>) -> Self {
        Self {
            handle,
            phantom: PhantomData,
            phantom_b: PhantomData,
        }
    }
}
