use std::fmt::{Display, Formatter};

/// Boolean value.
#[repr(i8)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Hash)]
pub enum Bool {
    False = 0,
    True = 1,
}

impl Display for Bool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Bool::False => write!(f, "false"),
            Bool::True => write!(f, "true"),
        }
    }
}

impl From<bool> for Bool {
    fn from(val: bool) -> Self {
        if val {
            Bool::True
        } else {
            Bool::False
        }
    }
}

impl From<Bool> for bool {
    fn from(val: Bool) -> Self {
        match val {
            Bool::False => false,
            Bool::True => true,
        }
    }
}
