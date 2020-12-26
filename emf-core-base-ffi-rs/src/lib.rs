#![feature(min_const_generics)]

mod boolean;

pub mod containers;

pub use boolean::Bool;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
