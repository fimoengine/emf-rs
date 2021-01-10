pub use emf_core_base_rs_bare;

pub mod ffi_bindings {
    #[cfg(feature = "global_api")]
    pub use emf_core_base_rs_ffi::initialize_base_binding;
    pub use emf_core_base_rs_ffi::InterfaceLoader;
}
