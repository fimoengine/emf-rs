//! Collections used by the `emf-core-base` interface.

mod non_null_const;
mod optional;
mod result;
mod span;
mod static_vec;

pub use non_null_const::NonNullConst;
pub use optional::Optional;
pub use result::Result;
pub use span::{ConstSpan, MutSpan, Span};
pub use static_vec::StaticVec;
