use crate::ffi::errors::error::ErrorRef;
use crate::ffi::errors::Error as ErrorFFI;
use crate::ownership::{BorrowImmutable, Owned};
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::mem::ManuallyDrop;

/// Error info.
#[derive(Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ErrorInfo {
    info: crate::ffi::errors::ErrorInfo,
}

impl ErrorInfo {
    /// Constructs a new instance.
    #[inline]
    pub fn new(info: crate::ffi::errors::ErrorInfo) -> Self {
        Self { info }
    }

    /// Gets the internal info.
    #[inline]
    pub fn into_inner(self) -> crate::ffi::errors::ErrorInfo {
        self.info
    }

    /// Fetches a reference to the error string.
    #[inline]
    pub fn as_str(&self) -> &str {
        self.info.as_ref()
    }
}

impl Debug for ErrorInfo {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.info, f)
    }
}

impl Display for ErrorInfo {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.info, f)
    }
}

impl AsRef<str> for ErrorInfo {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl From<crate::ffi::errors::ErrorInfo> for ErrorInfo {
    #[inline]
    fn from(val: crate::ffi::errors::ErrorInfo) -> Self {
        Self::new(val)
    }
}

/// An error type.
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Error<O> {
    _error: ManuallyDrop<ErrorFFI>,
    _ownership: PhantomData<fn() -> O>,
}

impl<O> Drop for Error<O> {
    default fn drop(&mut self) {}
}

impl<O> Display for Error<O> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&*self._error, f)
    }
}

impl<O> Debug for Error<O> {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&*self._error, f)
    }
}

impl Error<Owned> {
    /// Constructs a new instance.
    #[inline]
    pub const fn new(error: ErrorFFI) -> Self {
        Self {
            _error: ManuallyDrop::new(error),
            _ownership: PhantomData,
        }
    }

    /// Gets the internal error.
    #[inline]
    pub fn into_inner(mut self) -> ErrorFFI {
        let err = unsafe { ManuallyDrop::take(&mut self._error) };
        std::mem::forget(self);
        err
    }

    /// Lower-level source, if it exists.
    #[inline]
    pub fn source(&self) -> Option<Error<BorrowImmutable<'_>>> {
        unsafe {
            self._error.source().into_rust().map(|e| Error {
                _error: ManuallyDrop::new(ErrorFFI { internal: e }),
                _ownership: PhantomData,
            })
        }
    }

    /// Display error info.
    #[inline]
    pub fn display_info(&self) -> ErrorInfo {
        ErrorInfo::new(self._error.display_info())
    }

    /// Display error info.
    #[inline]
    pub fn debug_info(&self) -> ErrorInfo {
        ErrorInfo::new(self._error.debug_info())
    }
}

impl Drop for Error<Owned> {
    fn drop(&mut self) {
        unsafe { ManuallyDrop::drop(&mut self._error) }
    }
}

impl<T> From<T> for Error<Owned>
where
    ErrorFFI: From<T>,
{
    default fn from(val: T) -> Self {
        Self::from(ErrorFFI::from(val))
    }
}

impl From<ErrorFFI> for Error<Owned> {
    #[inline]
    fn from(val: ErrorFFI) -> Self {
        Self::new(val)
    }
}

impl<'a> Error<BorrowImmutable<'a>> {
    /// Gets the internal error.
    ///
    /// # Safety
    ///
    /// The resulting error may not outlive `'a`.
    #[inline]
    pub unsafe fn into_inner(self) -> ErrorRef {
        self._error.internal
    }

    /// Lower-level source, if it exists.
    #[inline]
    pub fn source(&self) -> Option<Error<BorrowImmutable<'a>>> {
        unsafe {
            self._error.source().into_rust().map(|e| Error {
                _error: ManuallyDrop::new(ErrorFFI { internal: e }),
                _ownership: PhantomData,
            })
        }
    }

    /// Display error info.
    #[inline]
    pub fn display_info(&self) -> ErrorInfo {
        ErrorInfo::new(self._error.display_info())
    }

    /// Display error info.
    #[inline]
    pub fn debug_info(&self) -> ErrorInfo {
        ErrorInfo::new(self._error.debug_info())
    }
}
