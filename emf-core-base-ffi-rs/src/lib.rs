mod boolean;
mod fn_id;
mod interface;

pub mod containers;
pub mod fn_ptr;
pub mod library;
pub mod module;
pub mod sys;
pub mod version;

pub use boolean::Bool;
pub use fn_id::FnId;
pub use interface::BaseInterface;

/// An opaque structure representing the emf-core-base interface.
#[repr(C)]
pub struct BaseT {
    _private: [u8; 0],
}
