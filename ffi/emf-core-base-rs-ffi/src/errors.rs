//! Error api
use std::fmt::{Debug, Display, Formatter};

pub mod error;
pub mod error_info;

pub use error::Error;
pub use error_info::ErrorInfo;

/// Static error.
pub struct StaticError<T: ?Sized + 'static + Display + Debug> {
    error: &'static T,
}

impl<T: ?Sized + 'static + Display + Debug + Sync> StaticError<T> {
    /// Constructs a new error.
    pub fn new(error: &'static T) -> Self {
        Self { error }
    }
}

impl<T: ?Sized + 'static + Display + Debug + Sync> Display for StaticError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self.error, f)
    }
}

impl<T: ?Sized + 'static + Display + Debug + Sync> Debug for StaticError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self.error, f)
    }
}

impl<T: ?Sized + 'static + Display + Debug + Sync> std::error::Error for StaticError<T> {}

/// Simple error.
pub struct SimpleError<T: Display + Debug + Send> {
    error: T,
}

impl<T: Display + Debug + Send> SimpleError<T> {
    /// Constructs a new error.
    pub fn new(error: T) -> Self {
        Self { error }
    }
}

impl<T: Display + Debug + Send> Display for SimpleError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.error, f)
    }
}

impl<T: Display + Debug + Send> Debug for SimpleError<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(&self.error, f)
    }
}

impl<T: Display + Debug + Send> std::error::Error for SimpleError<T> {}
