//! Implementation of a fat pointer.
use crate::ffi::collections::NonNullConst;
use std::fmt::{Debug, Formatter};
use std::hash::{Hash, Hasher};
use std::ptr::NonNull;

/// Fat pointer layout.
pub struct FatPtr<T, VTable> {
    pub data: Option<NonNull<T>>,
    pub vtable: NonNullConst<VTable>,
}

impl<T, VTable> FatPtr<T, VTable> {
    /// Constructs a new fat pointer.
    pub fn from_raw(data: Option<NonNull<T>>, vtable: NonNullConst<VTable>) -> Self {
        Self { data, vtable }
    }

    /// Constructs a new fat pointer with a different data pointer.
    pub fn exchange_data<D>(&self, data: Option<NonNull<D>>) -> FatPtr<D, VTable> {
        FatPtr::from_raw(data, self.vtable)
    }

    /// Constructs a new fat pointer with a different vtable.
    pub fn exchange_vtable<VT>(&self, vtable: NonNullConst<VT>) -> FatPtr<T, VT> {
        FatPtr::from_raw(self.data, vtable)
    }
}

impl<T, VTable> Debug for FatPtr<T, VTable> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FatPtr")
            .field("data", &self.data)
            .field("vtable", &self.vtable)
            .finish()
    }
}

impl<T, VTable> Copy for FatPtr<T, VTable> {}

impl<T, VTable> Clone for FatPtr<T, VTable> {
    fn clone(&self) -> Self {
        Self::from_raw(self.data, self.vtable)
    }
}

impl<T, VTable> Hash for FatPtr<T, VTable> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.data.hash(state);
        self.vtable.hash(state);
    }
}
