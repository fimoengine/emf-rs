///! Containers used by the `emf-core-base` interface.
mod span;
mod static_vec;
mod result;

pub use span::{MutSpan, Span};
pub use static_vec::StaticVec;
pub use result::Result;
