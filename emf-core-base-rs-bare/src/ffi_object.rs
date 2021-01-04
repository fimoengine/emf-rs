/// A trait that identifies types that can be constructed from ffi objects.
pub trait FromFFI<T: ?Sized> {
    /// Constructs itself from a `T`.
    ///
    /// # Safety
    ///
    /// The compiler can't check the lifetime of native ffi objects.
    unsafe fn from_ffi(v: T) -> Self;
}

/// A trait that identifies types that can be converted to ffi objects.
pub trait IntoFFI<T: ?Sized> {
    /// Constructs a `T` from itself.
    ///
    /// # Safety
    ///
    /// This function can be used to elide lifetimes.
    unsafe fn into_ffi(self) -> T;
}

/// A trait to deal with objects stemming from an ffi call.
pub trait FFIObject<T: ?Sized> {
    /// Constructs the native object from itself.
    fn as_native(&self) -> T;

    /// Constructs itself using the native object.
    ///
    /// # Safety
    ///
    /// This function is usually unsafe because it bypasses lifetimes.
    unsafe fn from_native(val: T) -> Self;
}

impl<T, U> IntoFFI<U> for T
where
    U: FromFFI<T>,
{
    #[inline]
    unsafe fn into_ffi(self) -> U {
        U::from_ffi(self)
    }
}

impl<T, U> FromFFI<U> for T
where
    T: From<U>,
{
    #[inline]
    unsafe fn from_ffi(v: U) -> Self {
        T::from(v)
    }
}

impl<T, U: Sized + Copy> FFIObject<U> for T
where
    T: AsRef<U> + FromFFI<U>,
{
    fn as_native(&self) -> U {
        *self.as_ref()
    }

    unsafe fn from_native(val: U) -> Self {
        T::from_ffi(val)
    }
}
