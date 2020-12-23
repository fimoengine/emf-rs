#![allow(dead_code)]
///! Implementation of a statically sized vector.
use std::cmp::min;
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::slice::{Iter, IterMut};

/// A contiguous statically sized array type.
pub struct StaticVec<T: Copy + Sized, const N: usize> {
    data: [T; N],
    length: usize,
}

impl<T: Copy + Sized, const N: usize> StaticVec<T, N> {
    /// Returns an unsafe mutable pointer to the vector's buffer.
    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.data.as_mut_ptr()
    }

    /// Extracts a mutable slice containing the entire vector.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        &mut self.data[..self.length]
    }

    /// Returns an unsafe pointer to the vector's buffer.
    pub fn as_ptr(&self) -> *const T {
        self.data.as_ptr()
    }

    /// Extracts a slice containing the entire vector.
    pub fn as_slice(&self) -> &[T] {
        &self.data[..self.length]
    }

    /// Returns the capacity of the vector.
    pub fn capacity(&self) -> usize {
        N
    }

    /// Overwrites the length of the vector.
    pub unsafe fn set_len(&mut self, new_len: usize) {
        self.length = new_len;
    }

    /// Inserts an element at position `index` within the vector,
    /// shifting all elements after it to the right.
    pub fn insert(&mut self, index: usize, element: T) -> bool {
        if self.length == self.capacity() {
            return false;
        }

        if index < self.length {
            unsafe {
                std::ptr::copy(
                    self.as_ptr().offset(index as isize),
                    self.as_mut_ptr().offset(index as isize + 1),
                    self.length - index,
                );
            }
        }
        self.data[index] = element;
        self.length += 1;

        true
    }

    /// Creates a new empty vector.
    pub fn new() -> Self {
        unsafe { std::mem::zeroed() }
    }

    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left. `None` is returned if the position
    /// is invalid.
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if self.length <= index || self.length == 0 {
            return None;
        }

        if index < (self.length - 1) {
            unsafe {
                std::ptr::copy(
                    self.as_ptr().offset(index as isize + 1),
                    self.as_mut_ptr().offset(index as isize),
                    self.length - 1 - index,
                );
            }
        }
        self.length -= 1;
        Some(self.data[self.length + 1])
    }

    /// Appends the value to the end of the vector, returns `true` on success.
    pub fn push(&mut self, value: T) -> bool {
        if self.length == self.capacity() {
            return false;
        }
        self.data[self.length] = value;
        self.length += 1;

        true
    }

    /// Removes the last element from the vector and returns it, or `None` if it is empty.
    pub fn pop(&mut self) -> Option<T> {
        match self.length {
            0 => None,
            i => {
                self.length -= 1;
                Some(self.data[i])
            }
        }
    }

    /// Clears the vector.
    pub fn clear(&mut self) {
        self.length = 0;
    }

    /// Retrieves the length of the vector.
    pub fn len(&self) -> usize {
        self.length
    }

    /// Checks if the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl<T: Copy + Sized, const N: usize> AsMut<[T]> for StaticVec<T, N> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T: Copy + Sized, const N: usize> AsRef<[T]> for StaticVec<T, N> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T: Copy + Sized + Debug, const N: usize> Debug for StaticVec<T, N> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<T: Copy + Sized, const N: usize> Deref for StaticVec<T, N> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T: Copy + Sized, const N: usize> DerefMut for StaticVec<T, N> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T: Copy + Sized + PartialEq, const N: usize> PartialEq for StaticVec<T, N> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T: Copy + Sized + PartialEq + Eq, const N: usize> Eq for StaticVec<T, N> {}

impl<T: Copy + Sized, const N: usize> From<&'_ [T]> for StaticVec<T, N> {
    fn from(slice: &[T]) -> Self {
        let min_size = min(N, slice.len());
        unsafe {
            let mut vec: Self = std::mem::zeroed();
            std::ptr::copy_nonoverlapping(slice.as_ptr(), vec.as_mut_ptr(), min_size);
            vec.set_len(min_size);
            vec
        }
    }
}

impl<T: Copy + Sized, const N: usize> From<&'_ mut [T]> for StaticVec<T, N> {
    fn from(slice: &mut [T]) -> Self {
        let min_size = min(N, slice.len());
        unsafe {
            let mut vec: Self = std::mem::zeroed();
            std::ptr::copy_nonoverlapping(slice.as_ptr(), vec.as_mut_ptr(), min_size);
            vec.set_len(min_size);
            vec
        }
    }
}

impl<const N: usize> From<&'_ str> for StaticVec<u8, N> {
    fn from(str: &str) -> Self {
        Self::from(str.as_bytes())
    }
}

impl<T: Copy + Sized, const N: usize> From<&'_ Vec<T>> for StaticVec<T, N> {
    fn from(vec: &Vec<T>) -> Self {
        Self::from(vec.as_slice())
    }
}

impl<T: Copy + Sized, const N: usize> From<&'_ mut Vec<T>> for StaticVec<T, N> {
    fn from(vec: &mut Vec<T>) -> Self {
        Self::from(vec.as_slice())
    }
}

impl<T: Copy + Sized, const N: usize, const M: usize> From<[T; M]> for StaticVec<T, N> {
    fn from(arr: [T; M]) -> Self {
        Self::from(&arr[..M])
    }
}

impl<'a, T: Copy + Sized, const N: usize> IntoIterator for &'a StaticVec<T, N> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_slice().into_iter()
    }
}

impl<'a, T: Copy + Sized, const N: usize> IntoIterator for &'a mut StaticVec<T, N> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.as_mut_slice().iter_mut()
    }
}
