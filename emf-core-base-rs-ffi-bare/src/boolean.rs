//! Boolean data type
#![allow(dead_code)]

/// An enum describing a boolean value.
#[repr(i8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Bool {
    False = 0,
    True = 1,
}
