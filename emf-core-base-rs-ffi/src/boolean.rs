/// Boolean value.
#[repr(i8)]
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Copy, Clone)]
pub enum Bool {
    False = 0,
    True = 1,
}
