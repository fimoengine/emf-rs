//! Mutable and immutable span implementation.
#![allow(dead_code)]

use std::fmt::{Debug, Formatter};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::os::raw::c_char;
use std::ptr::{null, null_mut};
use std::slice::{Iter, IterMut};

/// An immutable slice.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Span<'a, T>
where
    T: Copy + Sized,
{
    data: *const T,
    length: usize,
    phantom: PhantomData<&'a T>,
}

impl<T> Span<'_, T>
where
    T: Copy + Sized,
{
    /// Create a new empty span.
    pub fn new() -> Self {
        Self {
            data: null(),
            length: 0,
            phantom: PhantomData,
        }
    }

    /// Creates a new span from a pointer and a length.
    ///
    /// # Safety
    ///
    /// Same restrictions as [from_raw_parts](std::slice::from_raw_parts) apply.
    pub unsafe fn from_raw_parts(ptr: *const T, length: usize) -> Self {
        Self {
            data: ptr,
            length,
            phantom: PhantomData,
        }
    }

    /// Creates a new span from a mutable pointer and a length.
    ///
    /// # Safety
    ///
    /// Same restrictions as [from_raw_parts_mut](std::slice::from_raw_parts_mut) apply.
    pub unsafe fn from_raw_parts_mut(ptr: *mut T, length: usize) -> Self {
        Self {
            data: ptr,
            length,
            phantom: PhantomData,
        }
    }

    /// Fetches an immutable pointer of the elements the span points to.
    pub fn as_ptr(&self) -> *const T {
        self.data
    }

    /// Retrieves the length of the span.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Checks if the span is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_null() || self.length == 0
    }
}

impl<'a> Span<'a, u8> {
    /// Transforms a `Span<'a, u8>` into a `Span<'a, c_char>`.
    pub fn as_c_char_span(self) -> Span<'a, c_char> {
        Span {
            data: self.data as *const c_char,
            length: self.length,
            phantom: PhantomData,
        }
    }
}

impl<'a> Span<'a, c_char> {
    /// Transforms a `Span<'a, c_char>` into a `Span<'a, u8>`.
    pub fn as_rust_char_span(self) -> Span<'a, u8> {
        Span {
            data: self.data as *const u8,
            length: self.length,
            phantom: PhantomData,
        }
    }
}

impl<T> AsRef<[T]> for Span<'_, T>
where
    T: Copy + Sized,
{
    fn as_ref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data, self.length) }
    }
}

impl<T> Default for Span<'_, T>
where
    T: Copy + Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Debug for Span<'_, T>
where
    T: Copy + Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("data", &format_args!("{:p}", self.data))
            .field("length", &self.length)
            .finish()
    }
}

impl<T> Deref for Span<'_, T>
where
    T: Copy + Sized,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> PartialEq for Span<'_, T>
where
    T: Copy + Sized + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T> Eq for Span<'_, T> where T: Copy + Sized + PartialEq + Eq {}

impl<'a, T> From<&'a [T]> for Span<'a, T>
where
    T: Copy + Sized,
{
    fn from(slice: &[T]) -> Self {
        Self {
            data: slice.as_ptr(),
            length: slice.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T> From<&'a mut [T]> for Span<'a, T>
where
    T: Copy + Sized,
{
    fn from(slice: &mut [T]) -> Self {
        Self {
            data: slice.as_ptr(),
            length: slice.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T, const N: usize> From<&'a [T; N]> for Span<'a, T>
where
    T: Copy + Sized,
{
    fn from(array: &'a [T; N]) -> Self {
        Self {
            data: array.as_ptr(),
            length: N,
            phantom: PhantomData,
        }
    }
}

impl<'a, T, const N: usize> From<&'a mut [T; N]> for Span<'a, T>
where
    T: Copy + Sized,
{
    fn from(array: &'a mut [T; N]) -> Self {
        Self {
            data: array.as_ptr(),
            length: N,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> From<&'a Vec<T>> for Span<'a, T>
where
    T: Copy + Sized,
{
    fn from(vec: &'a Vec<T>) -> Self {
        Self {
            data: vec.as_ptr(),
            length: vec.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T> From<&'a mut Vec<T>> for Span<'a, T>
where
    T: Copy + Sized,
{
    fn from(vec: &'a mut Vec<T>) -> Self {
        Self {
            data: vec.as_ptr(),
            length: vec.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a> From<&'a str> for Span<'a, u8> {
    fn from(string: &'a str) -> Self {
        Self {
            data: string.as_ptr(),
            length: string.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a> From<&'a mut str> for Span<'a, u8> {
    fn from(string: &'a mut str) -> Self {
        Self {
            data: string.as_ptr(),
            length: string.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a> From<&'a String> for Span<'a, u8> {
    fn from(string: &'a String) -> Self {
        Self {
            data: string.as_ptr(),
            length: string.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a> From<&'a mut String> for Span<'a, u8> {
    fn from(string: &'a mut String) -> Self {
        Self {
            data: string.as_ptr(),
            length: string.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T> IntoIterator for &'a Span<'_, T>
where
    T: Copy + Sized,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter()
    }
}

/// A mutable slice.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MutSpan<'a, T>
where
    T: Copy + Sized,
{
    data: *mut T,
    length: usize,
    phantom: PhantomData<&'a T>,
}

impl<T> MutSpan<'_, T>
where
    T: Copy + Sized,
{
    /// Creates a new empty span.
    pub fn new() -> Self {
        Self {
            data: null_mut(),
            length: 0,
            phantom: PhantomData,
        }
    }

    /// Creates a new span from a pointer and a length.
    ///
    /// # Safety
    ///
    /// Same restrictions as [from_raw_parts_mut](std::slice::from_raw_parts_mut) apply.
    pub unsafe fn from_raw_parts(ptr: *mut T, length: usize) -> Self {
        Self {
            data: ptr,
            length,
            phantom: PhantomData,
        }
    }

    /// Fetches an immutable pointer of the elements the span points to.
    pub fn as_ptr(&self) -> *const T {
        self.data
    }

    /// Fetches a mutable pointer of the elements the span points to.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data
    }

    /// Retrieves the length of the span.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Checks if the span is empty.
    pub fn is_empty(&self) -> bool {
        self.data.is_null() || self.length == 0
    }
}

impl<'a> MutSpan<'a, u8> {
    /// Transforms a `MutSpan<'a, u8>` into a `MutSpan<'a, c_char>`.
    pub fn as_c_char_span(self) -> MutSpan<'a, c_char> {
        MutSpan {
            data: self.data as *mut c_char,
            length: self.length,
            phantom: PhantomData,
        }
    }
}

impl<'a> MutSpan<'a, c_char> {
    /// Transforms a `MutSpan<'a, c_char>` into a `MutSpan<'a, u8>`.
    pub fn as_rust_char_span(self) -> MutSpan<'a, u8> {
        MutSpan {
            data: self.data as *mut u8,
            length: self.length,
            phantom: PhantomData,
        }
    }
}

impl<T> AsMut<[T]> for MutSpan<'_, T>
where
    T: Copy + Sized,
{
    fn as_mut(&mut self) -> &mut [T] {
        unsafe { std::slice::from_raw_parts_mut(self.data, self.length) }
    }
}

impl<T> AsRef<[T]> for MutSpan<'_, T>
where
    T: Copy + Sized,
{
    fn as_ref(&self) -> &[T] {
        unsafe { std::slice::from_raw_parts(self.data, self.length) }
    }
}

impl<T> Default for MutSpan<'_, T>
where
    T: Copy + Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Debug for MutSpan<'_, T>
where
    T: Copy + Sized,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Span")
            .field("data", &format_args!("{:p}", self.data))
            .field("length", &self.length)
            .finish()
    }
}

impl<T> Deref for MutSpan<'_, T>
where
    T: Copy + Sized,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_ref()
    }
}

impl<T> DerefMut for MutSpan<'_, T>
where
    T: Copy + Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut()
    }
}

impl<T> PartialEq for MutSpan<'_, T>
where
    T: Copy + Sized + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_ref() == other.as_ref()
    }
}

impl<T> Eq for MutSpan<'_, T> where T: Copy + Sized + PartialEq + Eq {}

impl<'a, T> From<&'a mut [T]> for MutSpan<'a, T>
where
    T: Copy + Sized,
{
    fn from(slice: &mut [T]) -> Self {
        Self {
            data: slice.as_mut_ptr(),
            length: slice.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T, const N: usize> From<&'a mut [T; N]> for MutSpan<'a, T>
where
    T: Copy + Sized,
{
    fn from(array: &'a mut [T; N]) -> Self {
        Self {
            data: array.as_mut_ptr(),
            length: N,
            phantom: PhantomData,
        }
    }
}

impl<'a, T> From<&'a mut Vec<T>> for MutSpan<'a, T>
where
    T: Copy + Sized,
{
    fn from(vec: &'a mut Vec<T>) -> Self {
        Self {
            data: vec.as_mut_ptr(),
            length: vec.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a> From<&'a mut str> for MutSpan<'a, u8> {
    fn from(string: &'a mut str) -> Self {
        Self {
            data: string.as_mut_ptr(),
            length: string.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a> From<&'a mut String> for MutSpan<'a, u8> {
    fn from(string: &'a mut String) -> Self {
        Self {
            data: string.as_mut_ptr(),
            length: string.len(),
            phantom: PhantomData,
        }
    }
}

impl<'a, T> IntoIterator for &'a MutSpan<'_, T>
where
    T: Copy + Sized,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().iter()
    }
}

impl<'a, T> IntoIterator for &'a mut MutSpan<'_, T>
where
    T: Copy + Sized,
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut().iter_mut()
    }
}
