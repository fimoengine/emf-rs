//! Implementation of the `NonNullConst<T>` type.

use std::ptr::NonNull;

/// A type representing a `*const T` but non-zero.
///
/// Uses [NonNull] internally, as such, the same restrictions apply.
#[repr(transparent)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct NonNullConst<T>
where
    T: ?Sized,
{
    ptr: NonNull<T>,
}

impl<T> NonNullConst<T> {
    /// Creates a new `NonNullConst` that is dangling, but well-aligned.
    #[inline]
    pub const fn dangling() -> NonNullConst<T> {
        Self {
            ptr: NonNull::dangling(),
        }
    }
}

impl<T> NonNullConst<T>
where
    T: ?Sized,
{
    /// Creates a new `NonNull`.
    #[inline]
    pub const unsafe fn new_unchecked(ptr: *const T) -> NonNullConst<T> {
        Self {
            ptr: NonNull::new_unchecked(ptr as *mut T),
        }
    }

    /// Creates a new `NonNullConst` if `ptr` is non-null.
    #[inline]
    pub fn new(ptr: *const T) -> Option<NonNullConst<T>> {
        let ptr = NonNull::new(ptr as *mut T);

        match ptr {
            Some(ptr) => Some(Self { ptr }),
            None => None,
        }
    }

    /// Acquires the underlying `*const` pointer.
    #[inline]
    pub const fn as_ptr(self) -> *const T {
        self.ptr.as_ptr() as *const T
    }

    /// Returns a shared reference to the value.
    #[inline]
    pub unsafe fn as_ref(&self) -> &T {
        self.ptr.as_ref()
    }

    /// Casts to a pointer of another type.
    #[inline]
    pub const fn cast<U>(self) -> NonNullConst<U> {
        NonNullConst {
            ptr: self.ptr.cast(),
        }
    }
}
