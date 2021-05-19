//! Types and traits for denoting ownership.
use std::marker::PhantomData;

/// Trait for identifying ownership identifiers
pub trait AccessIdentifier: private::Sealed {}

/// Trait for identifying ownership identifiers with immutable access semantics.
pub trait ImmutableAccessIdentifier: AccessIdentifier {}

/// Trait for identifying ownership identifiers with unique access semantics.
pub trait MutableAccessIdentifier: ImmutableAccessIdentifier {}

/// Identifier for `owned` types.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Owned {}

/// Identifier for `mutably borrowed` types.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BorrowMutable<'a>(PhantomData<&'a ()>);

/// Identifier for `borrowed` types.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct BorrowImmutable<'a>(PhantomData<&'a ()>);

impl AccessIdentifier for Owned {}
impl AccessIdentifier for BorrowMutable<'_> {}
impl AccessIdentifier for BorrowImmutable<'_> {}

impl ImmutableAccessIdentifier for Owned {}
impl ImmutableAccessIdentifier for BorrowMutable<'_> {}
impl ImmutableAccessIdentifier for BorrowImmutable<'_> {}

impl MutableAccessIdentifier for Owned {}
impl MutableAccessIdentifier for BorrowMutable<'_> {}

mod private {
    pub trait Sealed {}

    impl Sealed for super::Owned {}
    impl Sealed for super::BorrowMutable<'_> {}
    impl Sealed for super::BorrowImmutable<'_> {}
}
