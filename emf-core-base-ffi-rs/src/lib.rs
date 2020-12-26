#![feature(min_const_generics)]

mod boolean;
mod fn_id;

pub mod containers;
pub mod fn_ptr;
pub mod sys;

pub use boolean::Bool;
pub use fn_id::FnId;

/// An opaque structure representing the emf-core-base interface.
#[repr(C)]
pub struct BaseT {
    _private: [u8; 0],
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
