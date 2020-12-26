#![allow(dead_code)]
///! The `Result` data type.
use crate::Bool;
use std::cmp::Ordering;
use std::cmp::Ordering::{Greater, Less};
use std::fmt::{Debug, Formatter};

#[repr(C)]
#[derive(Copy, Clone)]
union ResultImpl<T, E>
where
    T: Copy + Sized,
    E: Copy + Sized,
{
    result: T,
    error: E,
}

/// A type that represents either a result or an error.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Result<T, E>
where
    T: Copy + Sized,
    E: Copy + Sized,
{
    data: ResultImpl<T, E>,
    has_error: Bool,
}

impl<T, E> Result<T, E>
where
    T: Copy + Sized,
    E: Copy + Sized,
{
    /// Creates a new result containing a value.
    pub fn new_ok(value: T) -> Self {
        Self {
            data: ResultImpl { result: value },
            has_error: Bool::False,
        }
    }

    /// Creates a new result containing an error.
    pub fn new_err(err: E) -> Self {
        Self {
            data: ResultImpl { error: err },
            has_error: Bool::True,
        }
    }

    /// Returns `true` if the result contains a value.
    pub fn is_ok(&self) -> bool {
        self.has_error == Bool::False
    }

    /// Returns `true` if the result contains an error.
    pub fn is_err(&self) -> bool {
        self.has_error == Bool::True
    }

    /// Converts from `Result<T, E>` to `Option<T>` discarding any error.
    pub fn ok(self) -> Option<T> {
        match self.has_error {
            Bool::False => Some(unsafe { self.data.result }),
            Bool::True => None,
        }
    }

    /// Converts from `Result<T, E>` to `Option<E>` discarding any value.
    pub fn err(self) -> Option<E> {
        match self.has_error {
            Bool::False => None,
            Bool::True => Some(unsafe { self.data.error }),
        }
    }

    /// Converts from `&Result<T, E>` to `Result<&T, &E>`.
    pub fn as_ref(&self) -> Result<&T, &E> {
        match self.is_ok() {
            true => Result::new_ok(unsafe { &self.data.result }),
            false => Result::new_err(unsafe { &self.data.error }),
        }
    }

    /// Maps the `Result<T, E>` to `Result<U, E>` by mapping the ok value.
    pub fn map<U, F>(self, op: F) -> Result<U, E>
    where
        U: Copy + Sized,
        F: FnOnce(T) -> U,
    {
        match self.is_ok() {
            true => Result::new_ok(op(unsafe { self.data.result })),
            false => Result::new_err(unsafe { self.data.error }),
        }
    }

    /// Maps the ok value of the result by applying f or returning the default value.
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        match self.is_ok() {
            true => f(unsafe { self.data.result }),
            false => default,
        }
    }

    /// Maps the `Result<T, E>` to `U` by either applying f to the ok value or
    /// applying default to the error value.
    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: FnOnce(E) -> U,
        F: FnOnce(T) -> U,
    {
        match self.is_ok() {
            true => f(unsafe { self.data.result }),
            false => default(unsafe { self.data.error }),
        }
    }

    /// Maps the `Result<T, E>` to `Result<T,F>` by mapping the error value.
    pub fn map_err<F, O>(self, op: O) -> Result<T, F>
    where
        F: Copy + Sized,
        O: FnOnce(E) -> F,
    {
        match self.is_ok() {
            true => Result::new_ok(unsafe { self.data.result }),
            false => Result::new_err(op(unsafe { self.data.error })),
        }
    }

    /// Returns the contained ok value or a provided default.
    pub fn unwrap_or(self, default: T) -> T {
        match self.is_ok() {
            true => unsafe { self.data.result },
            false => default,
        }
    }

    /// Returns the contained ok value or computes it from a closure.
    pub fn unwrap_or_else<F>(self, op: F) -> T
    where
        F: FnOnce(E) -> T,
    {
        match self.is_ok() {
            true => unsafe { self.data.result },
            false => op(unsafe { self.data.error }),
        }
    }

    /// Maps a `Result<T, E>` to `std::result::Result<T, E>`.
    pub fn to_native(self) -> std::result::Result<T, E> {
        match self.is_ok() {
            true => Ok(unsafe { self.data.result }),
            false => Err(unsafe { self.data.error }),
        }
    }
}

impl<T, E> Result<T, E>
where
    T: Copy + Sized,
    E: Copy + Sized + Debug,
{
    /// Returns the contained ok value.
    ///
    /// # Panics
    ///
    /// Panics if no ok value is contained, with a panic message provided by the error value.
    pub fn expect(self, msg: &str) -> T {
        match self.is_ok() {
            true => unsafe { self.data.result },
            false => panic!("{}: {:?}", msg, unsafe { self.data.error }),
        }
    }

    /// Returns the contained ok value.
    ///
    /// # Panics
    ///
    /// Panics if no ok value is contained, with a panic message including the passed message,
    /// and the content of the error value.
    pub fn unwrap(self) -> T {
        match self.is_ok() {
            true => unsafe { self.data.result },
            false => panic!("{:?}", unsafe { self.data.error }),
        }
    }
}

impl<T, E> Result<T, E>
where
    T: Copy + Sized + Debug,
    E: Copy + Sized,
{
    /// Returns the contained error value.
    ///
    /// # Panics
    ///
    /// Panics if no error value is contained, with a panic message provided by the ok value.
    pub fn expect_err(self, msg: &str) -> E {
        match self.is_ok() {
            true => panic!("{}: {:?}", msg, unsafe { self.data.result }),
            false => unsafe { self.data.error },
        }
    }

    /// Returns the contained error value.
    ///
    /// # Panics
    ///
    /// Panics if no error value is contained, with a panic message including the passed message,
    /// and the content of the ok value.
    pub fn unwrap_err(self) -> E {
        match self.is_ok() {
            true => panic!("{:?}", unsafe { self.data.result }),
            false => unsafe { self.data.error },
        }
    }
}

impl<T, E> Result<T, E>
where
    T: Copy + Sized + Default,
    E: Copy + Sized,
{
    /// Returns the contained ok value or a default.
    pub fn unwrap_or_default(self) -> T {
        match self.is_ok() {
            true => unsafe { self.data.result },
            false => Default::default(),
        }
    }
}

impl<T, E> Debug for Result<T, E>
where
    T: Copy + Sized + Debug,
    E: Copy + Sized + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.is_ok() {
            true => write!(f, "Result: {:?}", unsafe { self.data.result }),
            false => write!(f, "Error: {:?}", unsafe { self.data.error }),
        }
    }
}

impl<T, E> PartialEq for Result<T, E>
where
    T: Copy + Sized + PartialEq,
    E: Copy + Sized + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        if self.is_ok() != other.is_ok() {
            return false;
        }

        match self.is_ok() {
            true => unsafe { self.data.result == other.data.result },
            false => unsafe { self.data.error == other.data.error },
        }
    }
}

impl<T, E> Eq for Result<T, E>
where
    T: Copy + Sized + PartialEq + Eq,
    E: Copy + Sized + PartialEq + Eq,
{
}

impl<T, E> PartialOrd for Result<T, E>
where
    T: Copy + Sized + PartialOrd,
    E: Copy + Sized + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_ok() != other.is_ok() {
            return match self.is_ok() {
                true => Some(Greater),
                false => Some(Less),
            };
        }

        match self.is_ok() {
            true => unsafe { self.data.result.partial_cmp(&other.data.result) },
            false => unsafe { self.data.error.partial_cmp(&other.data.error) },
        }
    }
}

impl<T, E> Ord for Result<T, E>
where
    T: Copy + Sized + PartialOrd + Ord,
    E: Copy + Sized + PartialOrd + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(&other).unwrap()
    }
}

impl<T, E> From<std::result::Result<T, E>> for Result<T, E>
where
    T: Copy + Sized,
    E: Copy + Sized,
{
    fn from(res: std::result::Result<T, E>) -> Self {
        res.map_or_else(|err| Self::new_err(err), |val| Self::new_ok(val))
    }
}

impl<T, E> From<Result<T, E>> for std::result::Result<T, E>
where
    T: Copy + Sized,
    E: Copy + Sized,
{
    fn from(res: Result<T, E>) -> Self {
        res.to_native()
    }
}
