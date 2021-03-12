//! Casters for the version api.
use crate::ffi::version::api;
use crate::ffi::{CBaseFn, FnId};
use crate::fn_caster::FnCaster;

/// Caster for [FnId::VersionNewShort]
#[derive(Debug)]
pub struct NewShortCaster {}

/// Caster for [FnId::VersionNewLong]
#[derive(Debug)]
pub struct NewLongCaster {}

/// Caster for [FnId::VersionNewFull]
#[derive(Debug)]
pub struct NewFullCaster {}

/// Caster for [FnId::VersionFromString]
#[derive(Debug)]
pub struct FromStringCaster {}

/// Caster for [FnId::VersionStringLengthShort]
#[derive(Debug)]
pub struct StringLengthShortCaster {}

/// Caster for [FnId::VersionStringLengthLong]
#[derive(Debug)]
pub struct StringLengthLongCaster {}

/// Caster for [FnId::VersionStringLengthFull]
#[derive(Debug)]
pub struct StringLengthFullCaster {}

/// Caster for [FnId::VersionAsStringShort]
#[derive(Debug)]
pub struct AsStringShortCaster {}

/// Caster for [FnId::VersionAsStringLong]
#[derive(Debug)]
pub struct AsStringLongCaster {}

/// Caster for [FnId::VersionAsStringFull]
#[derive(Debug)]
pub struct AsStringFullCaster {}

/// Caster for [FnId::VersionStringIsValid]
#[derive(Debug)]
pub struct StringIsValidCaster {}

/// Caster for [FnId::VersionCompare]
#[derive(Debug)]
pub struct CompareCaster {}

/// Caster for [FnId::VersionCompareWeak]
#[derive(Debug)]
pub struct CompareWeakCaster {}

/// Caster for [FnId::VersionCompareStrong]
#[derive(Debug)]
pub struct CompareStrongCaster {}

/// Caster for [FnId::VersionIsCompatible]
#[derive(Debug)]
pub struct IsCompatibleCaster {}

transmute_caster!(NewShortCaster, api::NewShortFn, FnId::VersionNewShort);

transmute_caster!(NewLongCaster, api::NewLongFn, FnId::VersionNewLong);

transmute_caster!(NewFullCaster, api::NewFullFn, FnId::VersionNewFull);

transmute_caster!(FromStringCaster, api::FromStringFn, FnId::VersionFromString);

transmute_caster!(
    StringLengthShortCaster,
    api::StringLengthShortFn,
    FnId::VersionStringLengthShort
);

transmute_caster!(
    StringLengthLongCaster,
    api::StringLengthLongFn,
    FnId::VersionStringLengthLong
);

transmute_caster!(
    StringLengthFullCaster,
    api::StringLengthFullFn,
    FnId::VersionStringLengthFull
);

transmute_caster!(
    AsStringShortCaster,
    api::AsStringShortFn,
    FnId::VersionAsStringShort
);

transmute_caster!(
    AsStringLongCaster,
    api::AsStringLongFn,
    FnId::VersionAsStringLong
);

transmute_caster!(
    AsStringFullCaster,
    api::AsStringFullFn,
    FnId::VersionAsStringFull
);

transmute_caster!(
    StringIsValidCaster,
    api::StringIsValidFn,
    FnId::VersionStringIsValid
);

transmute_caster!(CompareCaster, api::CompareFn, FnId::VersionCompare);

transmute_caster!(
    CompareWeakCaster,
    api::CompareWeakFn,
    FnId::VersionCompareWeak
);

transmute_caster!(
    CompareStrongCaster,
    api::CompareStrongFn,
    FnId::VersionCompareStrong
);

transmute_caster!(
    IsCompatibleCaster,
    api::IsCompatibleFn,
    FnId::VersionIsCompatible
);
