//! Error type.
use crate::collections::{NonNullConst, Optional};
use crate::errors::ErrorInfo;
use crate::TypeWrapper;
use std::error::Error as ErrorStd;
use std::fmt::{Debug, Display, Formatter};
use std::ptr::NonNull;

/// Opaque structure representing an error.
#[repr(C)]
#[derive(Debug)]
pub struct ErrorData {
    _dummy: [u8; 0],
}

pub type CleanupFn = TypeWrapper<unsafe extern "C-unwind" fn(Option<NonNull<ErrorData>>)>;
pub type SourceFn =
    TypeWrapper<unsafe extern "C-unwind" fn(Option<NonNullConst<ErrorData>>) -> Optional<ErrorRef>>;
pub type DisplayInfoFn =
    TypeWrapper<unsafe extern "C-unwind" fn(Option<NonNullConst<ErrorData>>) -> ErrorInfo>;
pub type DebugInfoFn =
    TypeWrapper<unsafe extern "C-unwind" fn(Option<NonNullConst<ErrorData>>) -> ErrorInfo>;

/// Error vtable.
#[repr(C)]
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ErrorVTable {
    pub cleanup_fn: CleanupFn,
    pub source_fn: SourceFn,
    pub display_info_fn: DisplayInfoFn,
    pub debug_info_fn: DebugInfoFn,
}

/// Unowned error value.
#[repr(C)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct ErrorRef {
    pub data: Option<NonNullConst<ErrorData>>,
    pub vtable: NonNullConst<ErrorVTable>,
}

impl ErrorRef {
    /// Lower-level source, if it exists.
    ///
    /// # Safety
    ///
    /// The resulting error may not outlive self.
    #[inline]
    pub unsafe fn source(&self) -> Optional<ErrorRef> {
        (self.vtable.as_ref().source_fn)(self.data)
    }

    /// Display error info.
    #[inline]
    pub fn display_info(&self) -> ErrorInfo {
        unsafe { (self.vtable.as_ref().display_info_fn)(self.data) }
    }

    /// Display error info.
    #[inline]
    pub fn debug_info(&self) -> ErrorInfo {
        unsafe { (self.vtable.as_ref().debug_info_fn)(self.data) }
    }
}

impl Display for ErrorRef {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.display_info(), f)
    }
}

impl Debug for ErrorRef {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.debug_info(), f)
    }
}

/// Owned error value.
#[repr(C)]
#[derive(Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Error {
    pub internal: ErrorRef,
}

impl Error {
    /// Lower-level source, if it exists.
    ///
    /// # Safety
    ///
    /// The resulting error may not outlive self.
    #[inline]
    pub unsafe fn source(&self) -> Optional<ErrorRef> {
        self.internal.source()
    }

    /// Display error info.
    #[inline]
    pub fn display_info(&self) -> ErrorInfo {
        self.internal.display_info()
    }

    /// Display error info.
    #[inline]
    pub fn debug_info(&self) -> ErrorInfo {
        self.internal.debug_info()
    }
}

impl Drop for Error {
    fn drop(&mut self) {
        unsafe {
            (self.internal.vtable.as_ref().cleanup_fn)(self.internal.data.map(|v| v.into_mut()))
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.internal, f)
    }
}

impl Debug for Error {
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.internal, f)
    }
}

impl<T> From<T> for Error
where
    T: ErrorStd + Sized,
{
    fn from(error: T) -> Self {
        Self::from(Box::new(box_error::OwnedError::new(error)))
    }
}

/// Types with available error vtables.
pub trait AsErrorVTable {
    /// VTable for the type.
    const VTABLE: ErrorVTable;

    /// Cleanup function.
    ///
    /// # Safety
    ///
    /// Passes the ffi boundary.
    unsafe extern "C-unwind" fn cleanup_fn(data: Option<NonNull<ErrorData>>);

    /// Source function.
    ///
    /// # Safety
    ///
    /// Passes the ffi boundary.
    unsafe extern "C-unwind" fn source_fn(
        data: Option<NonNullConst<ErrorData>>,
    ) -> Optional<ErrorRef>;

    /// Display function.
    ///
    /// # Safety
    ///
    /// Passes the ffi boundary.
    unsafe extern "C-unwind" fn display_info_fn(data: Option<NonNullConst<ErrorData>>)
        -> ErrorInfo;

    /// Debug function.
    ///
    /// # Safety
    ///
    /// Passes the ffi boundary.
    unsafe extern "C-unwind" fn debug_info_fn(data: Option<NonNullConst<ErrorData>>) -> ErrorInfo;
}

mod box_error {
    use crate::collections::{NonNullConst, Optional};
    use crate::errors::error::{AsErrorVTable, Error as Err, ErrorData, ErrorRef, ErrorVTable};
    use crate::errors::ErrorInfo;
    use crate::TypeWrapper;
    use std::error::Error;
    use std::ptr::NonNull;

    pub struct InternalError {
        error: NonNullConst<dyn Error + 'static>,
        source: Option<Box<InternalError>>,
    }

    impl InternalError {
        #[inline]
        pub fn new(error: &(dyn Error + 'static)) -> Self {
            let source = error.source();
            Self {
                error: NonNullConst::from(error),
                source: source.map(|e| Box::new(Self::new(e))),
            }
        }

        #[inline]
        pub fn source(&self) -> &Option<Box<InternalError>> {
            &self.source
        }

        #[inline]
        pub fn display_info(&self) -> Box<String> {
            Box::new(format!("{}", unsafe { self.error.as_ref() }))
        }

        #[inline]
        pub fn debug_info(&self) -> Box<String> {
            Box::new(format!("{:?}", unsafe { self.error.as_ref() }))
        }
    }

    impl From<&InternalError> for ErrorRef {
        fn from(error: &InternalError) -> Self {
            Self {
                data: Some(NonNullConst::from(error).cast()),
                vtable: NonNullConst::from(&<&InternalError>::VTABLE),
            }
        }
    }

    impl AsErrorVTable for &InternalError {
        const VTABLE: ErrorVTable = ErrorVTable {
            cleanup_fn: TypeWrapper(Self::cleanup_fn),
            source_fn: TypeWrapper(Self::source_fn),
            display_info_fn: TypeWrapper(Self::display_info_fn),
            debug_info_fn: TypeWrapper(Self::debug_info_fn),
        };

        unsafe extern "C-unwind" fn cleanup_fn(_data: Option<NonNull<ErrorData>>) {}

        unsafe extern "C-unwind" fn source_fn(
            data: Option<NonNullConst<ErrorData>>,
        ) -> Optional<ErrorRef> {
            data.unwrap()
                .cast::<InternalError>()
                .as_ref()
                .source()
                .as_ref()
                .map_or(Optional::none(), |error| {
                    Optional::some(ErrorRef::from(error.as_ref()))
                })
        }

        unsafe extern "C-unwind" fn display_info_fn(
            data: Option<NonNullConst<ErrorData>>,
        ) -> ErrorInfo {
            ErrorInfo::from(
                data.unwrap()
                    .cast::<InternalError>()
                    .as_ref()
                    .display_info(),
            )
        }

        unsafe extern "C-unwind" fn debug_info_fn(
            data: Option<NonNullConst<ErrorData>>,
        ) -> ErrorInfo {
            ErrorInfo::from(data.unwrap().cast::<InternalError>().as_ref().debug_info())
        }
    }

    pub struct OwnedError<T: Error> {
        error: Box<T>,
        source: Option<Box<InternalError>>,
    }

    impl<T: Error> OwnedError<T> {
        #[inline]
        pub fn new(error: T) -> Self {
            let mut err = Self {
                error: Box::new(error),
                source: None,
            };

            err.source = err.error.source().map(|e| Box::new(InternalError::new(e)));
            err
        }

        #[inline]
        pub fn source(&self) -> &Option<Box<InternalError>> {
            &self.source
        }

        #[inline]
        pub fn display_info(&self) -> Box<String> {
            Box::new(format!("{}", &self.error))
        }

        #[inline]
        pub fn debug_info(&self) -> Box<String> {
            Box::new(format!("{:?}", &self.error))
        }
    }

    impl<T: Error> From<Box<OwnedError<T>>> for Err {
        fn from(error: Box<OwnedError<T>>) -> Self {
            Self {
                internal: ErrorRef {
                    data: Some(NonNullConst::from(Box::leak(error)).cast()),
                    vtable: NonNullConst::from(&<Box<OwnedError<T>>>::VTABLE),
                },
            }
        }
    }

    impl<T: Error> AsErrorVTable for Box<OwnedError<T>> {
        const VTABLE: ErrorVTable = ErrorVTable {
            cleanup_fn: TypeWrapper(Self::cleanup_fn),
            source_fn: TypeWrapper(Self::source_fn),
            display_info_fn: TypeWrapper(Self::display_info_fn),
            debug_info_fn: TypeWrapper(Self::debug_info_fn),
        };

        unsafe extern "C-unwind" fn cleanup_fn(data: Option<NonNull<ErrorData>>) {
            drop(Box::<OwnedError<T>>::from_raw(
                data.unwrap().cast().as_ptr(),
            ))
        }

        unsafe extern "C-unwind" fn source_fn(
            data: Option<NonNullConst<ErrorData>>,
        ) -> Optional<ErrorRef> {
            data.unwrap()
                .cast::<OwnedError<T>>()
                .as_ref()
                .source()
                .as_ref()
                .map_or(Optional::none(), |error| {
                    Optional::some(ErrorRef::from(error.as_ref()))
                })
        }

        unsafe extern "C-unwind" fn display_info_fn(
            data: Option<NonNullConst<ErrorData>>,
        ) -> ErrorInfo {
            ErrorInfo::from(
                data.unwrap()
                    .cast::<OwnedError<T>>()
                    .as_ref()
                    .display_info(),
            )
        }

        unsafe extern "C-unwind" fn debug_info_fn(
            data: Option<NonNullConst<ErrorData>>,
        ) -> ErrorInfo {
            ErrorInfo::from(data.unwrap().cast::<OwnedError<T>>().as_ref().debug_info())
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::collections::Optional;
    use crate::errors::Error as Err;
    use std::error::Error;
    use std::fmt::{Debug, Display, Formatter};

    #[derive(Copy, Clone)]
    struct MyError {
        internal: MyInternalError,
    }

    impl Debug for MyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str("my error debug!")
        }
    }

    impl Display for MyError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            f.write_str("my error display!")
        }
    }

    impl Error for MyError {
        fn source(&self) -> Option<&(dyn Error + 'static)> {
            Some(&self.internal)
        }
    }

    #[derive(Copy, Clone)]
    struct MyInternalError {
        error: &'static str,
    }

    impl Debug for MyInternalError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            Display::fmt(self.error, f)
        }
    }

    impl Display for MyInternalError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            Display::fmt(self.error, f)
        }
    }

    impl Error for MyInternalError {}

    #[test]
    fn custom_error() {
        let my_error = MyError {
            internal: MyInternalError { error: "my error!" },
        };

        let error = Err::from(my_error.clone());

        let error_dis_info = error.display_info();
        let error_dbg_info = error.debug_info();

        let error_dis = error_dis_info.as_ref();
        let error_dbg = error_dbg_info.as_ref();

        assert_eq!(format!("{}", &my_error), error_dis);
        assert_eq!(format!("{:?}", &my_error), error_dbg);

        let source = unsafe { error.source() }.unwrap();

        let source_dis_info = source.display_info();
        let source_dbg_info = source.debug_info();

        let source_dis = source_dis_info.as_ref();
        let source_dbg = source_dbg_info.as_ref();

        assert_eq!(format!("{}", &my_error.internal), source_dis);
        assert_eq!(format!("{:?}", &my_error.internal), source_dbg);

        assert_eq!(unsafe { source.source() }, Optional::none())
    }
}
