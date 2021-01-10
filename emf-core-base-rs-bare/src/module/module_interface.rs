use crate::{ffi, FFIObject};
use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;

/// An interface contained in a module.
pub struct ModuleInterface<'a, T: 'a + Sized> {
    interface: T,
    phantom: PhantomData<&'a ()>,
}

impl<'a, T: Sized> ModuleInterface<'a, T> {
    /// Extends the lifetime of the handle.
    ///
    /// # Safety
    ///
    /// When using this function you must guarantee that the handle lives long enough.
    pub unsafe fn extend_lifetime<'b>(self) -> ModuleInterface<'b, T> {
        ModuleInterface {
            interface: self.interface,
            phantom: PhantomData,
        }
    }

    /// Casts the type of the interface to another type.
    pub fn cast<U: Sized + From<T>>(self) -> ModuleInterface<'a, U> {
        ModuleInterface {
            interface: self.interface.into(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Sized> AsRef<T> for ModuleInterface<'a, T> {
    fn as_ref(&self) -> &T {
        &self.interface
    }
}

impl<'a, T: Sized> AsMut<T> for ModuleInterface<'a, T> {
    fn as_mut(&mut self) -> &mut T {
        &mut self.interface
    }
}

impl<'a, T: Sized + Copy + Clone> Copy for ModuleInterface<'a, T> {}

impl<'a, T: Sized + Clone> Clone for ModuleInterface<'a, T> {
    fn clone(&self) -> Self {
        Self {
            interface: self.interface.clone(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T: Sized + Debug> Debug for ModuleInterface<'a, T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ModuleInterface")
            .field("interface", &self.interface)
            .finish()
    }
}

impl<'a, T: Sized + PartialEq> PartialEq for ModuleInterface<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.interface.eq(&other.interface)
    }
}

impl<'a, T: Sized + PartialEq + Eq> Eq for ModuleInterface<'a, T> {}

impl<'a, T: Sized + FFIObject<ffi::module::ModuleInterface>> FFIObject<ffi::module::ModuleInterface>
    for ModuleInterface<'a, T>
{
    fn as_native(&self) -> ffi::module::ModuleInterface {
        self.interface.as_native()
    }

    unsafe fn from_native(val: ffi::module::ModuleInterface) -> Self {
        Self {
            interface: T::from_native(val),
            phantom: PhantomData,
        }
    }
}
