#[cfg(feature = "global_api")]
mod bootstrap_bindings;

#[cfg(feature = "global_api")]
mod library;

#[cfg(feature = "global_api")]
mod module;

#[cfg(feature = "global_api")]
mod sys;

#[cfg(feature = "global_api")]
mod version;

#[cfg(test)]
#[cfg(feature = "global_api")]
mod closure_wrapper;

#[cfg(feature = "global_api")]
pub use bootstrap_bindings::initialize_base_binding;

#[cfg(feature = "global_api")]
pub use bootstrap_bindings::BASE_INTERFACE;
