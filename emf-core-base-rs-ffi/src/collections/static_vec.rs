use std::cmp::{min, Ordering};
use std::fmt::{Debug, Formatter};
use std::ops::{Deref, DerefMut};
use std::os::raw::c_char;
use std::slice::{Iter, IterMut};

/// A contiguous statically sized array type.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct StaticVec<T, const N: usize>
where
    T: Copy + Sized,
{
    data: [T; N],
    length: usize,
}

impl<T, const N: usize> StaticVec<T, N>
where
    T: Copy + Sized,
{
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
    ///
    /// # Safety
    ///
    /// Wrong usage may lead to reading uninitialized data.
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
                    self.as_ptr().add(index),
                    self.as_mut_ptr().add(index + 1),
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
                    self.as_ptr().add(index + 1),
                    self.as_mut_ptr().add(index),
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

    /// Constructs an immutable iterator.
    pub fn iter(&self) -> Iter<T> {
        self.as_ref().iter()
    }

    /// Constructs a mutable iterator.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.as_mut().iter_mut()
    }
}

impl<T, const N: usize> AsMut<[T]> for StaticVec<T, N>
where
    T: Copy + Sized,
{
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, const N: usize> AsRef<[T]> for StaticVec<T, N>
where
    T: Copy + Sized,
{
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, const N: usize> Debug for StaticVec<T, N>
where
    T: Copy + Sized + Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.as_slice().fmt(f)
    }
}

impl<T, const N: usize> Default for StaticVec<T, N>
where
    T: Copy + Sized,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<T, const N: usize> Deref for StaticVec<T, N>
where
    T: Copy + Sized,
{
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl<T, const N: usize> DerefMut for StaticVec<T, N>
where
    T: Copy + Sized,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_slice()
    }
}

impl<T, const N: usize> PartialEq for StaticVec<T, N>
where
    T: Copy + Sized + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl<T, const N: usize> Eq for StaticVec<T, N> where T: Copy + Sized + PartialEq + Eq {}

impl<T, const N: usize> PartialOrd for StaticVec<T, N>
where
    T: Copy + Sized + PartialEq + PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_ref().partial_cmp(other.as_ref())
    }
}

impl<T, const N: usize> Ord for StaticVec<T, N>
where
    T: Copy + Sized + PartialEq + Eq + PartialOrd + Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_ref().cmp(other.as_ref())
    }
}

impl<T, const N: usize> From<&'_ [T]> for StaticVec<T, N>
where
    T: Copy + Sized,
{
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

impl<T, const N: usize> From<&'_ mut [T]> for StaticVec<T, N>
where
    T: Copy + Sized,
{
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

impl<const N: usize> From<&'_ str> for StaticVec<c_char, N> {
    fn from(str: &str) -> Self {
        unsafe { Self::from(&*(str.as_bytes() as *const [u8] as *const [c_char])) }
    }
}

impl<T, const N: usize> From<&'_ Vec<T>> for StaticVec<T, N>
where
    T: Copy + Sized,
{
    fn from(vec: &Vec<T>) -> Self {
        Self::from(vec.as_slice())
    }
}

impl<T, const N: usize> From<&'_ mut Vec<T>> for StaticVec<T, N>
where
    T: Copy + Sized,
{
    fn from(vec: &mut Vec<T>) -> Self {
        Self::from(vec.as_slice())
    }
}

impl<T, const N: usize, const M: usize> From<[T; M]> for StaticVec<T, N>
where
    T: Copy + Sized,
{
    fn from(arr: [T; M]) -> Self {
        Self::from(&arr[..M])
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a StaticVec<T, N>
where
    T: Copy + Sized,
{
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, const N: usize> IntoIterator for &'a mut StaticVec<T, N>
where
    T: Copy + Sized,
{
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
