//! Boolean data type
#![allow(dead_code)]

/// An enum describing a boolean value.
#[repr(i8)]
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Debug)]
pub enum Bool {
    False = 0,
    True = 1,
}

impl From<bool> for Bool {
    fn from(v: bool) -> Self {
        match v {
            true => Bool::True,
            false => Bool::False,
        }
    }
}

impl From<Bool> for bool {
    fn from(v: Bool) -> Self {
        match v {
            Bool::False => false,
            Bool::True => true,
        }
    }
}
