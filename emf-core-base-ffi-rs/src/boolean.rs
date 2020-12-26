#![allow(dead_code)]
///! Boolean data type

#[repr(i8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Bool {
    False = 0,
    True = 1
}