use crate::bindings::bootstrap_bindings::{BASE_INTERFACE_DESC, BASE_VERSION};
use crate::module::InterfaceName;
use crate::BASE_INTERFACE_NAME;
use emf_core_base_rs_ffi_bare::containers::Span;

#[test]
fn base_version() {
    assert_eq!(BASE_VERSION.major, crate::VERSION_MAJOR);
    assert_eq!(BASE_VERSION.minor, crate::VERSION_MINOR);
    assert_eq!(BASE_VERSION.patch, crate::VERSION_PATCH);
    assert_eq!(BASE_VERSION.build_number, crate::VERSION_BUILD_NUMBER);
    assert_eq!(BASE_VERSION.release_number, crate::VERSION_RELEASE_NUMBER);
    assert_eq!(BASE_VERSION.release_type, crate::VERSION_RELEASE_TYPE);
}

#[test]
fn base_interface_desc() {
    assert_eq!(
        BASE_INTERFACE_DESC.name,
        InterfaceName::from(BASE_INTERFACE_NAME)
    );
    assert_eq!(BASE_INTERFACE_DESC.version.major, BASE_VERSION.major);
    assert_eq!(BASE_INTERFACE_DESC.version.minor, BASE_VERSION.minor);
    assert_eq!(BASE_INTERFACE_DESC.version.patch, BASE_VERSION.patch);
    assert_eq!(
        BASE_INTERFACE_DESC.version.build_number,
        BASE_VERSION.build_number
    );
    assert_eq!(
        BASE_INTERFACE_DESC.version.release_number,
        BASE_VERSION.release_number
    );
    assert_eq!(
        BASE_INTERFACE_DESC.version.release_type,
        BASE_VERSION.release_type
    );
    assert_eq!(BASE_INTERFACE_DESC.extensions, Span::new());
}
