use crate::bindings::closure_wrapper::{
    wrap_closure_1, wrap_closure_2, wrap_closure_3, wrap_closure_5, wrap_closure_6,
};
use crate::bindings::version::{
    emf_cbase_version_get_full_representation, emf_cbase_version_get_full_representation_length,
};
use crate::bindings::BASE_INTERFACE;
use crate::containers::{MutSpan, NonNullConst, Result, Span};
use crate::version::{
    emf_cbase_version_compare, emf_cbase_version_compare_strong, emf_cbase_version_compare_weak,
    emf_cbase_version_construct_from_string, emf_cbase_version_construct_full,
    emf_cbase_version_construct_long, emf_cbase_version_construct_short,
    emf_cbase_version_get_long_representation, emf_cbase_version_get_long_representation_length,
    emf_cbase_version_get_short_representation, emf_cbase_version_get_short_representation_length,
    emf_cbase_version_is_compatible, emf_cbase_version_representation_is_valid, ReleaseType,
    Version, VersionError,
};
use crate::{BaseInterface, BaseT, Bool};
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::ptr::NonNull;

#[test]
fn version_construct_short() {
    let version = Version {
        major: 0,
        minor: 0,
        patch: 0,
        release_type: ReleaseType::Gold,
        release_number: 0,
        build_number: 0,
    };
    let mut val = false;
    let mut closure = |_: i32, _: i32, _: i32| {
        val = true;
        version
    };
    let wrapper = wrap_closure_3(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_construct_short_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_version_construct_short(0, 0, 0);
        assert_eq!(result, version);
        assert_eq!(val, true);
    }
}

#[test]
fn version_construct_long() {
    let version = Version {
        major: 0,
        minor: 0,
        patch: 0,
        release_type: ReleaseType::Gold,
        release_number: 0,
        build_number: 0,
    };
    let mut val = false;
    let mut closure = |_: i32, _: i32, _: i32, _: ReleaseType, _: i8| {
        val = true;
        version
    };
    let wrapper = wrap_closure_5(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_construct_long_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_version_construct_long(0, 0, 0, ReleaseType::Gold, 0);
        assert_eq!(result, version);
        assert_eq!(val, true);
    }
}

#[test]
fn version_construct_full() {
    let version = Version {
        major: 0,
        minor: 0,
        patch: 0,
        release_type: ReleaseType::Gold,
        release_number: 0,
        build_number: 0,
    };
    let mut val = false;
    let mut closure = |_: i32, _: i32, _: i32, _: ReleaseType, _: i8, _: i64| {
        val = true;
        version
    };
    let wrapper = wrap_closure_6(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_construct_full_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_version_construct_full(0, 0, 0, ReleaseType::Gold, 0, 0);
        assert_eq!(result, version);
        assert_eq!(val, true);
    }
}

#[test]
fn version_construct_from_string() {
    let mut val = false;
    let mut closure = |_: NonNullConst<Span<c_char>>| {
        val = true;
        Result::<Version, _>::new_err(VersionError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_construct_from_string_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_version_construct_from_string(NonNullConst::dangling());
        assert_eq!(result, Result::new_err(VersionError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn version_representation_is_valid() {
    let mut val = false;
    let mut closure = |_: NonNullConst<Span<c_char>>| {
        val = true;
        Bool::False
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_representation_is_valid_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_version_representation_is_valid(NonNullConst::dangling());
        assert_eq!(result, Bool::False);
        assert_eq!(val, true);
    }
}

#[test]
fn version_get_short_representation() {
    let mut val = false;
    let mut closure = |_: NonNullConst<Version>, _: NonNullConst<MutSpan<c_char>>| {
        val = true;
        Result::<usize, _>::new_err(VersionError::BufferOverflow)
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_get_short_representation_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_version_get_short_representation(
            NonNullConst::dangling(),
            NonNull::dangling(),
        );
        assert_eq!(result, Result::new_err(VersionError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn version_get_short_representation_length() {
    let mut val = false;
    let mut closure = |_: NonNullConst<Version>| {
        val = true;
        0
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_get_short_representation_length_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_version_get_short_representation_length(NonNullConst::dangling());
        assert_eq!(result, 0);
        assert_eq!(val, true);
    }
}

#[test]
fn version_get_long_representation() {
    let mut val = false;
    let mut closure = |_: NonNullConst<Version>, _: NonNullConst<MutSpan<c_char>>| {
        val = true;
        Result::<usize, _>::new_err(VersionError::BufferOverflow)
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_get_long_representation_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_version_get_long_representation(
            NonNullConst::dangling(),
            NonNull::dangling(),
        );
        assert_eq!(result, Result::new_err(VersionError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn version_get_long_representation_length() {
    let mut val = false;
    let mut closure = |_: NonNullConst<Version>| {
        val = true;
        0
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_get_long_representation_length_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_version_get_long_representation_length(NonNullConst::dangling());
        assert_eq!(result, 0);
        assert_eq!(val, true);
    }
}

#[test]
fn version_get_full_representation() {
    let mut val = false;
    let mut closure = |_: NonNullConst<Version>, _: NonNullConst<MutSpan<c_char>>| {
        val = true;
        Result::<usize, _>::new_err(VersionError::BufferOverflow)
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_get_full_representation_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_version_get_full_representation(
            NonNullConst::dangling(),
            NonNull::dangling(),
        );
        assert_eq!(result, Result::new_err(VersionError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn version_get_full_representation_length() {
    let mut val = false;
    let mut closure = |_: NonNullConst<Version>| {
        val = true;
        0
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_get_full_representation_length_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_version_get_full_representation_length(NonNullConst::dangling());
        assert_eq!(result, 0);
        assert_eq!(val, true);
    }
}

#[test]
fn version_compare() {
    let mut val = false;
    let mut closure = |_: NonNullConst<Version>, _: NonNullConst<Version>| {
        val = true;
        0
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_compare_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_version_compare(NonNullConst::dangling(), NonNullConst::dangling());
        assert_eq!(result, 0);
        assert_eq!(val, true);
    }
}

#[test]
fn version_compare_weak() {
    let mut val = false;
    let mut closure = |_: NonNullConst<Version>, _: NonNullConst<Version>| {
        val = true;
        0
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_compare_weak_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result =
            emf_cbase_version_compare_weak(NonNullConst::dangling(), NonNullConst::dangling());
        assert_eq!(result, 0);
        assert_eq!(val, true);
    }
}

#[test]
fn version_compare_strong() {
    let mut val = false;
    let mut closure = |_: NonNullConst<Version>, _: NonNullConst<Version>| {
        val = true;
        0
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_compare_strong_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result =
            emf_cbase_version_compare_strong(NonNullConst::dangling(), NonNullConst::dangling());
        assert_eq!(result, 0);
        assert_eq!(val, true);
    }
}

#[test]
fn version_is_compatible() {
    let mut val = false;
    let mut closure = |_: NonNullConst<Version>, _: NonNullConst<Version>| {
        val = true;
        Bool::False
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.version_is_compatible_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result =
            emf_cbase_version_is_compatible(NonNullConst::dangling(), NonNullConst::dangling());
        assert_eq!(result, Bool::False);
        assert_eq!(val, true);
    }
}
