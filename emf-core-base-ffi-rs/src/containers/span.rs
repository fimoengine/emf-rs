//! Mutable and immutable span implementation.
#![allow(dead_code)]

use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
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
    pub unsafe fn from_raw_parts(ptr: *const T, length: usize) -> Self {
        Self {
            data: ptr,
            length,
            phantom: PhantomData,
        }
    }

    /// Creates a new span from a mutable pointer and a length.
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
        self.data == null() || self.length == 0
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

impl<'a, T> IntoIterator for &'a Span<'_, T>
where
    T: Copy + Sized,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().into_iter()
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
        self.data == null_mut() || self.length == 0
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

impl<'a, T> IntoIterator for &'a MutSpan<'_, T>
where
    T: Copy + Sized,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_ref().into_iter()
    }
}

impl<'a, T> IntoIterator for &'a mut MutSpan<'_, T>
where
    T: Copy + Sized,
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut().into_iter()
    }
}
