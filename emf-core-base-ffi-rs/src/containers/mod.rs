//! Containers used by the `emf-core-base` interface.

mod optional;
mod result;
mod span;
mod static_vec;

pub use optional::Optional;
pub use result::Result;
pub use span::{MutSpan, Span};
pub use static_vec::StaticVec;
