use crate::bindings::closure_wrapper::{
    wrap_closure_0, wrap_closure_1, wrap_closure_2, wrap_closure_3,
};
use crate::bindings::library::{
    emf_cbase_library_get_library_types, emf_cbase_library_get_num_loaders,
    emf_cbase_library_unregister_loader,
};
use crate::bindings::BASE_INTERFACE;
use crate::containers::{MutSpan, NonNullConst, Optional, Result};
use crate::library::{
    emf_cbase_library_get_data_symbol, emf_cbase_library_get_function_symbol,
    emf_cbase_library_get_loader_handle, emf_cbase_library_library_exists, emf_cbase_library_load,
    emf_cbase_library_register_loader, emf_cbase_library_type_exists, emf_cbase_library_unload,
    emf_cbase_library_unsafe_create_library_handle, emf_cbase_library_unsafe_get_loader_handle,
    emf_cbase_library_unsafe_get_loader_interface,
    emf_cbase_library_unsafe_get_loader_library_handle, emf_cbase_library_unsafe_link_library,
    emf_cbase_library_unsafe_remove_library_handle, DataSymbol, FnSymbol, LibraryError,
    LibraryHandle, LibraryType, LoaderHandle, LoaderInterface, LoaderLibraryHandle, OsPathChar,
};
use crate::{BaseInterface, BaseT, Bool};
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use std::ptr::NonNull;

#[test]
fn library_register_loader() {
    let mut val = false;
    let mut closure = |_: NonNullConst<LoaderInterface>, _: NonNullConst<LibraryType>| {
        val = true;
        Result::<LoaderHandle, LibraryError>::new_err(LibraryError::BufferOverflow)
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_register_loader_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res =
            emf_cbase_library_register_loader(NonNullConst::dangling(), NonNullConst::dangling());
        assert_eq!(res, Result::new_err(LibraryError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn library_unregister_loader() {
    let mut val = false;
    let mut closure = |_: LoaderHandle| {
        val = true;
        Optional::some(LibraryError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_unregister_loader_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_unregister_loader(LoaderHandle { id: 0 });
        assert_eq!(res, Optional::some(LibraryError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn library_get_num_loaders() {
    let mut val = false;
    let mut closure = || {
        val = true;
        0usize
    };
    let wrapper = wrap_closure_0(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_get_num_loaders_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_get_num_loaders();
        assert_eq!(res, 0);
        assert_eq!(val, true);
    }
}

#[test]
fn library_get_library_types() {
    let mut val = false;
    let mut closure = |_: NonNull<MutSpan<LibraryType>>| {
        val = true;
        Result::<usize, LibraryError>::new_ok(0)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_get_library_types_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_get_library_types(NonNull::dangling());
        assert_eq!(res, Result::new_ok(0));
        assert_eq!(val, true);
    }
}

#[test]
fn library_get_loader_handle() {
    let mut val = false;
    let mut closure = |_: NonNullConst<LibraryType>| {
        val = true;
        Result::new_err(LibraryError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_get_loader_handle_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_get_loader_handle(NonNullConst::dangling());
        assert_eq!(res, Result::new_err(LibraryError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn library_type_exists() {
    let mut val = false;
    let mut closure = |_: NonNullConst<LibraryType>| {
        val = true;
        Bool::False
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_type_exists_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_type_exists(NonNullConst::dangling());
        assert_eq!(res, Bool::False);
        assert_eq!(val, true);
    }
}

#[test]
fn library_library_exists() {
    let mut val = false;
    let mut closure = |_: LibraryHandle| {
        val = true;
        Bool::False
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_library_exists_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_library_exists(LibraryHandle { id: 0 });
        assert_eq!(res, Bool::False);
        assert_eq!(val, true);
    }
}

#[test]
fn library_unsafe_create_library_handle() {
    let mut val = false;
    let mut closure = || {
        val = true;
        LibraryHandle { id: 0 }
    };
    let wrapper = wrap_closure_0(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_unsafe_create_library_handle_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_unsafe_create_library_handle();
        assert_eq!(res, LibraryHandle { id: 0 });
        assert_eq!(val, true);
    }
}

#[test]
fn library_unsafe_remove_library_handle() {
    let mut val = false;
    let mut closure = |_: LibraryHandle| {
        val = true;
        Optional::none()
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_unsafe_remove_library_handle_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_unsafe_remove_library_handle(LibraryHandle { id: 0 });
        assert_eq!(res, Optional::none());
        assert_eq!(val, true);
    }
}

#[test]
fn library_unsafe_link_library() {
    let mut val = false;
    let mut closure = |_: LibraryHandle, _: LoaderHandle, _: LoaderLibraryHandle| {
        val = true;
        Optional::none()
    };
    let wrapper = wrap_closure_3(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_unsafe_link_library_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_unsafe_link_library(
            LibraryHandle { id: 0 },
            LoaderHandle { id: 0 },
            LoaderLibraryHandle { id: 0 },
        );
        assert_eq!(res, Optional::none());
        assert_eq!(val, true);
    }
}

#[test]
fn library_unsafe_get_loader_library_handle() {
    let mut val = false;
    let mut closure = |_: LibraryHandle| {
        val = true;
        Result::new_err(LibraryError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_unsafe_get_loader_library_handle_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_unsafe_get_loader_library_handle(LibraryHandle { id: 0 });
        assert_eq!(res, Result::new_err(LibraryError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn library_unsafe_get_loader_handle() {
    let mut val = false;
    let mut closure = |_: LibraryHandle| {
        val = true;
        Result::new_err(LibraryError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_unsafe_get_loader_handle_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_unsafe_get_loader_handle(LibraryHandle { id: 0 });
        assert_eq!(res, Result::new_err(LibraryError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn library_unsafe_get_loader_interface() {
    let mut val = false;
    let mut closure = |_: LoaderHandle| {
        val = true;
        Result::new_err(LibraryError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_unsafe_get_loader_interface_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_unsafe_get_loader_interface(LoaderHandle { id: 0 });
        assert_eq!(res, Result::new_err(LibraryError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn library_load() {
    let mut val = false;
    let mut closure = |_: LoaderHandle, _: NonNullConst<OsPathChar>| {
        val = true;
        Result::new_err(LibraryError::BufferOverflow)
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_load_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_load(LoaderHandle { id: 0 }, NonNullConst::dangling());
        assert_eq!(res, Result::new_err(LibraryError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn library_unload() {
    let mut val = false;
    let mut closure = |_: LibraryHandle| {
        val = true;
        Optional::none()
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_unload_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_unload(LibraryHandle { id: 0 });
        assert_eq!(res, Optional::none());
        assert_eq!(val, true);
    }
}

fn library_get_data_symbol() {
    let mut val = false;
    let mut closure = |_: LibraryHandle, _: NonNullConst<c_char>| {
        val = true;
        Result::new_err(LibraryError::BufferOverflow)
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_get_data_symbol_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res =
            emf_cbase_library_get_data_symbol(LibraryHandle { id: 0 }, NonNullConst::dangling());
        assert_eq!(res, Result::new_err(LibraryError::BufferOverflow));
        assert_eq!(val, true);
    }
}

fn library_get_function_symbol() {
    let mut val = false;
    let mut closure = |_: LibraryHandle, _: NonNullConst<c_char>| {
        val = true;
        Result::new_err(LibraryError::BufferOverflow)
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.library_get_function_symbol_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let res = emf_cbase_library_get_function_symbol(
            LibraryHandle { id: 0 },
            NonNullConst::dangling(),
        );
        assert_eq!(res, Result::new_err(LibraryError::BufferOverflow));
        assert_eq!(val, true);
    }
}
