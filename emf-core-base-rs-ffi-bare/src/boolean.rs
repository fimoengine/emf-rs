//! Boolean data type
#![allow(dead_code)]

/// An enum describing a boolean value.
// TODO: Replace with `#[repr(i8)]` once https://github.com/rust-lang/rust/issues/80556 is fixed.
#[repr(u8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Bool {
    False = 0,
    True = 1,
}
