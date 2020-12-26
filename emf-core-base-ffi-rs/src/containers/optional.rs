//! Module implementing the `Optional` type.
#![allow(dead_code)]

use crate::Bool;
use std::fmt::{Debug, Formatter};

#[repr(C)]
#[derive(Copy, Clone)]
union OptionalImpl<T>
where
    T: Copy + Sized,
{
    val: T,
    dummy: Bool,
}

impl<T> OptionalImpl<T>
where
    T: Copy + Sized,
{
    pub fn new_with_val(val: T) -> Self {
        Self { val }
    }

    pub fn new_empty() -> Self {
        Self { dummy: Bool::False }
    }

    pub fn value(&self) -> T {
        unsafe { self.val }
    }

    pub fn value_ref(&self) -> &T {
        unsafe { &self.val }
    }
}

/// A type containing an optional value.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Optional<T>
where
    T: Copy + Sized,
{
    data: OptionalImpl<T>,
    has_value: Bool,
}

impl<T> Optional<T>
where
    T: Copy + Sized,
{
    /// Creates a new `Optional<T>` containing the value.
    pub fn some(val: T) -> Self {
        Self {
            data: OptionalImpl::new_with_val(val),
            has_value: Bool::True,
        }
    }

    /// Creates an empty `Optional<T>`.
    pub fn none() -> Self {
        Self {
            data: OptionalImpl::new_empty(),
            has_value: Bool::False,
        }
    }

    /// Returns `true` if the optional contains a value.
    pub fn is_some(&self) -> bool {
        self.has_value == Bool::True
    }

    /// Returns `true` if the optional is empty.
    pub fn is_none(&self) -> bool {
        self.has_value == Bool::False
    }

    /// Maps the `Optional<T>` to `Optional<&T>`.
    pub fn as_ref(&self) -> Optional<&T> {
        match self.is_some() {
            true => Optional::some(self.data.value_ref()),
            false => Optional::none(),
        }
    }

    /// Maps the `Optional<T>` to `Option<T>`.
    pub fn to_native(self) -> Option<T> {
        match self.is_some() {
            true => Some(self.data.value()),
            false => None,
        }
    }

    /// Returns the contained value.
    ///
    /// # Panics
    ///
    /// Panics if no value is contained with a custom panic message provided by `msg`.
    pub fn expect(self, msg: &str) -> T {
        match self.is_some() {
            true => self.data.value(),
            false => panic!("{}", msg),
        }
    }

    /// Returns the contained value.
    ///
    /// # Panics
    ///
    /// Panics if no value is contained.
    pub fn unwrap(self) -> T {
        match self.is_some() {
            true => self.data.value(),
            false => panic!("called `Optional::unwrap()` on an empty optional"),
        }
    }

    /// Returns the contained value or a default.
    pub fn unwrap_or(self, default: T) -> T {
        match self.is_some() {
            true => self.data.value(),
            false => default,
        }
    }

    /// Returns the contained value or computes it from a closure.
    pub fn unwrap_or_else<F>(self, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        match self.is_some() {
            true => self.data.value(),
            false => f(),
        }
    }

    /// Maps an `Optional<T>` to `Optional<U>` by applying a function to the contained value.
    pub fn map<U, F>(self, f: F) -> Optional<U>
    where
        U: Copy + Sized,
        F: FnOnce(T) -> U,
    {
        match self.is_some() {
            true => Optional::some(f(self.data.value())),
            false => Optional::none(),
        }
    }

    /// Returns the application of the closure to the contained value or a default value.
    pub fn map_or<U, F>(self, default: U, f: F) -> U
    where
        F: FnOnce(T) -> U,
    {
        match self.is_some() {
            true => f(self.data.value()),
            false => default,
        }
    }

    /// Applies a function to the contained value (if any), or computes a default (if not).
    pub fn map_or_else<U, D, F>(self, default: D, f: F) -> U
    where
        D: FnOnce() -> U,
        F: FnOnce(T) -> U,
    {
        match self.is_some() {
            true => f(self.data.value()),
            false => default(),
        }
    }

    /// Transforms the `Optional<T>` into a `Result<T, E>`.
    pub fn ok_or<E>(self, err: E) -> crate::containers::Result<T, E>
    where
        E: Copy + Sized,
    {
        match self.is_some() {
            true => crate::containers::Result::new_ok(self.data.value()),
            false => crate::containers::Result::new_err(err),
        }
    }

    /// Transforms the `Optional<T>` into a `Result<T, E>` by mapping the contained value or
    /// computing an error value from a closure.
    pub fn ok_or_else<E, F>(self, f: F) -> crate::containers::Result<T, E>
    where
        E: Copy + Sized,
        F: FnOnce() -> E,
    {
        match self.is_some() {
            true => crate::containers::Result::new_ok(self.data.value()),
            false => crate::containers::Result::new_err(f()),
        }
    }
}

impl<T> Debug for Optional<T>
where
    T: Copy + Sized + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.is_some() {
            true => write!(f, "Some:{:?}", self.data.value()),
            false => write!(f, "None"),
        }
    }
}

impl<T> Default for Optional<T>
where
    T: Copy + Sized,
{
    fn default() -> Self {
        Self::none()
    }
}

impl<T> PartialEq for Optional<T>
where
    T: Copy + Sized + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.is_some() == other.is_some()
            && (self.is_some() == false || self.data.value_ref() == other.data.value_ref())
    }
}

impl<T> Eq for Optional<T> where T: Copy + Sized + PartialEq + Eq {}

impl<T> From<Option<T>> for Optional<T>
where
    T: Copy + Sized,
{
    fn from(opt: Option<T>) -> Self {
        opt.map_or_else(|| Optional::none(), |val| Optional::some(val))
    }
}

impl<T> From<Optional<T>> for Option<T>
where
    T: Copy + Sized,
{
    fn from(opt: Optional<T>) -> Self {
        opt.to_native()
    }
}
