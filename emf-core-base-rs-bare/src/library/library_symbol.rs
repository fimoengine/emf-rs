use crate::library::LibraryHandle;
use crate::{ffi, FFIObject};
use std::marker::PhantomData;

///  A symbol contained in a library.
pub struct LibrarySymbol<'a, T: 'a + Sized> {
    symbol: T,
    phantom: PhantomData<&'a LibraryHandle<'a>>,
}

impl<'a, T: Sized> LibrarySymbol<'a, T> {
    /// Extends the lifetime of the handle.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the handle lives long enough.
    pub unsafe fn extend_lifetime<'b>(self) -> LibrarySymbol<'b, T> {
        LibrarySymbol {
            symbol: self.symbol,
            phantom: PhantomData,
        }
    }

    /// Casts the type of the symbol to another type.
    pub fn cast<U: Sized + From<T>>(self) -> LibrarySymbol<'a, U> {
        LibrarySymbol {
            symbol: self.symbol.into(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Sized> AsRef<T> for LibrarySymbol<'a, T> {
    fn as_ref(&self) -> &T {
        &self.symbol
    }
}

impl<'a, T: Sized> AsMut<T> for LibrarySymbol<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.symbol
    }
}

impl<'a, T: Sized + FFIObject<ffi::library::DataSymbol>> FFIObject<ffi::library::DataSymbol>
    for LibrarySymbol<'a, T>
{
    fn as_native(&self) -> ffi::library::DataSymbol {
        self.symbol.as_native()
    }

    unsafe fn from_native(val: ffi::library::DataSymbol) -> Self {
        Self {
            symbol: T::from_native(val),
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Sized + FFIObject<ffi::library::FnSymbol>> FFIObject<ffi::library::FnSymbol>
    for LibrarySymbol<'a, T>
{
    fn as_native(&self) -> ffi::library::FnSymbol {
        self.symbol.as_native()
    }

    unsafe fn from_native(val: ffi::library::FnSymbol) -> Self {
        Self {
            symbol: T::from_native(val),
            phantom: PhantomData,
        }
    }
}
