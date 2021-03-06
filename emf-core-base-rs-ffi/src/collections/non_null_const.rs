//! Implementation of the `NonNullConst<T>` type.
use std::ptr::NonNull;

/// A type representing a `*const T` but non-zero.
///
/// Uses [NonNull] internally, as such, the same restrictions apply.
#[repr(transparent)]
#[derive(Ord, PartialOrd, Eq, PartialEq, Debug)]
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
    ///
    /// # Safety
    ///
    /// The same restrictions as [NonNull::new_unchecked](NonNull::new_unchecked) apply.
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
        ptr.map(|ptr| Self { ptr })
    }

    /// Acquires a mutable version of the pointer.
    ///
    /// # Safety
    ///
    /// It is undefined behavior if the underlying pointer is not already mutable.
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub unsafe fn into_mut(&self) -> NonNull<T> {
        self.ptr
    }

    /// Acquires the underlying `*const` pointer.
    #[inline]
    #[allow(clippy::wrong_self_convention)]
    pub const fn as_ptr(self) -> *const T {
        self.ptr.as_ptr() as *const T
    }

    /// Returns a shared reference to the value.
    ///
    /// # Safety
    ///
    /// The same restrictions as [NonNull::as_ref](NonNull::as_ref) apply.
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

impl<T> From<&T> for NonNullConst<T>
where
    T: ?Sized,
{
    fn from(value: &T) -> Self {
        Self { ptr: value.into() }
    }
}

impl<T> From<&mut T> for NonNullConst<T>
where
    T: ?Sized,
{
    fn from(value: &mut T) -> Self {
        Self { ptr: value.into() }
    }
}

impl<T> From<&[T]> for NonNullConst<T>
where
    T: Sized,
{
    fn from(slice: &[T]) -> Self {
        unsafe { Self::new_unchecked(slice.as_ptr()) }
    }
}

impl<T> From<&mut [T]> for NonNullConst<T>
where
    T: Sized,
{
    fn from(slice: &mut [T]) -> Self {
        unsafe { Self::new_unchecked(slice.as_ptr()) }
    }
}

impl<T, const N: usize> From<&[T; N]> for NonNullConst<T>
where
    T: Sized,
{
    fn from(array: &[T; N]) -> Self {
        unsafe { Self::new_unchecked(array.as_ptr()) }
    }
}

impl<T, const N: usize> From<&mut [T; N]> for NonNullConst<T>
where
    T: Sized,
{
    fn from(array: &mut [T; N]) -> Self {
        unsafe { Self::new_unchecked(array.as_ptr()) }
    }
}

impl<T> From<NonNull<T>> for NonNullConst<T>
where
    T: ?Sized,
{
    fn from(ptr: NonNull<T>) -> Self {
        Self { ptr }
    }
}

impl<T> Copy for NonNullConst<T> {}

impl<T> Clone for NonNullConst<T> {
    fn clone(&self) -> Self {
        unsafe { Self::new_unchecked(self.as_ptr()) }
    }
}
