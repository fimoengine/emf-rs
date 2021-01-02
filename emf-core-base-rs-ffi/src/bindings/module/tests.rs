use crate::bindings::closure_wrapper::{
    wrap_closure_0, wrap_closure_1, wrap_closure_2, wrap_closure_3,
};
use crate::bindings::module::{emf_cbase_module_initialize, emf_cbase_module_terminate};
use crate::bindings::BASE_INTERFACE;
use crate::containers::{MutSpan, NonNullConst, Result};
use crate::library::OsPathChar;
use crate::module::{
    emf_cbase_module_add_dependency, emf_cbase_module_add_module,
    emf_cbase_module_export_interface, emf_cbase_module_exported_interface_exists,
    emf_cbase_module_fetch_status, emf_cbase_module_get_exportable_interfaces,
    emf_cbase_module_get_exported_interface_handle, emf_cbase_module_get_exported_interfaces,
    emf_cbase_module_get_interface, emf_cbase_module_get_load_dependencies,
    emf_cbase_module_get_loader_handle, emf_cbase_module_get_module_info,
    emf_cbase_module_get_module_path, emf_cbase_module_get_module_types,
    emf_cbase_module_get_modules, emf_cbase_module_get_num_exported_interfaces,
    emf_cbase_module_get_num_loaders, emf_cbase_module_get_num_modules,
    emf_cbase_module_get_runtime_dependencies, emf_cbase_module_load,
    emf_cbase_module_module_exists, emf_cbase_module_register_loader,
    emf_cbase_module_remove_dependency, emf_cbase_module_remove_module,
    emf_cbase_module_type_exists, emf_cbase_module_unload, emf_cbase_module_unregister_loader,
    emf_cbase_module_unsafe_create_module_handle, emf_cbase_module_unsafe_get_loader,
    emf_cbase_module_unsafe_get_loader_handle, emf_cbase_module_unsafe_get_loader_module_handle,
    emf_cbase_module_unsafe_link_module, emf_cbase_module_unsafe_remove_module_handle,
    InterfaceDescriptor, LoaderHandle, LoaderModuleHandle, ModuleError, ModuleHandle, ModuleInfo,
    ModuleInterface, ModuleLoaderInterface, ModuleType,
};
use crate::{BaseInterface, BaseT, Bool};
use emf_core_base_rs_ffi_bare::containers::Optional;
use std::mem::MaybeUninit;
use std::ptr::NonNull;

#[test]
fn module_register_loader() {
    let mut val = false;
    let mut closure = |_: NonNullConst<ModuleLoaderInterface>, _: NonNullConst<ModuleType>| {
        val = true;
        Result::new_err(ModuleError::BufferOverflow)
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_register_loader_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result =
            emf_cbase_module_register_loader(NonNullConst::dangling(), NonNullConst::dangling());
        assert_eq!(result, Result::new_err(ModuleError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn module_unregister_loader() {
    let mut val = false;
    let mut closure = |_: LoaderHandle| {
        val = true;
        Optional::none()
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_unregister_loader_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_unregister_loader(LoaderHandle { id: 0 });
        assert_eq!(result, Optional::none());
        assert_eq!(val, true);
    }
}

#[test]
fn module_get_num_loaders() {
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
        base.module_get_num_loaders_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_get_num_loaders();
        assert_eq!(result, 0);
        assert_eq!(val, true);
    }
}

#[test]
fn module_get_module_types() {
    let mut val = false;
    let mut closure = |_: NonNull<MutSpan<ModuleType>>| {
        val = true;
        Result::<_, ModuleError>::new_ok(0usize)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_get_module_types_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_get_module_types(NonNull::dangling());
        assert_eq!(result, Result::new_ok(0));
        assert_eq!(val, true);
    }
}

#[test]
fn module_get_num_modules() {
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
        base.module_get_num_modules_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_get_num_modules();
        assert_eq!(result, 0);
        assert_eq!(val, true);
    }
}

#[test]
fn module_get_modules() {
    let mut val = false;
    let mut closure = |_: NonNull<MutSpan<ModuleInfo>>| {
        val = true;
        Result::<_, ModuleError>::new_ok(0usize)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_get_modules_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_get_modules(NonNull::dangling());
        assert_eq!(result, Result::new_ok(0));
        assert_eq!(val, true);
    }
}

#[test]
fn module_get_num_exported_interfaces() {
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
        base.module_get_num_exported_interfaces_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_get_num_exported_interfaces();
        assert_eq!(result, 0);
        assert_eq!(val, true);
    }
}

#[test]
fn module_get_exported_interfaces() {
    let mut val = false;
    let mut closure = |_: NonNull<MutSpan<InterfaceDescriptor>>| {
        val = true;
        Result::<_, ModuleError>::new_ok(0usize)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_get_exported_interfaces_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_get_exported_interfaces(NonNull::dangling());
        assert_eq!(result, Result::new_ok(0));
        assert_eq!(val, true);
    }
}

#[test]
fn module_get_loader_handle() {
    let mut val = false;
    let mut closure = |_: NonNullConst<ModuleType>| {
        val = true;
        Result::new_err(ModuleError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_get_loader_handle_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_get_loader_handle(NonNullConst::dangling());
        assert_eq!(result, Result::new_err(ModuleError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn module_type_exists() {
    let mut val = false;
    let mut closure = |_: NonNullConst<ModuleType>| {
        val = true;
        Bool::False
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_type_exists_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_type_exists(NonNullConst::dangling());
        assert_eq!(result, Bool::False);
        assert_eq!(val, true);
    }
}

#[test]
fn module_module_exists() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Bool::False
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_module_exists_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_module_exists(ModuleHandle { id: 0 });
        assert_eq!(result, Bool::False);
        assert_eq!(val, true);
    }
}

#[test]
fn module_get_exported_interface_handle() {
    let mut val = false;
    let mut closure = |_: NonNullConst<InterfaceDescriptor>| {
        val = true;
        Result::<ModuleHandle, _>::new_err(ModuleError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_get_exported_interface_handle_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_get_exported_interface_handle(NonNullConst::dangling());
        assert_eq!(result, Result::new_err(ModuleError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn module_exported_interface_exists() {
    let mut val = false;
    let mut closure = |_: NonNullConst<InterfaceDescriptor>| {
        val = true;
        Bool::False
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_exported_interface_exists_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_exported_interface_exists(NonNullConst::dangling());
        assert_eq!(result, Bool::False);
        assert_eq!(val, true);
    }
}

#[test]
fn module_unsafe_create_module_handle() {
    let mut val = false;
    let mut closure = || {
        val = true;
        ModuleHandle { id: 0 }
    };
    let wrapper = wrap_closure_0(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_unsafe_create_module_handle_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_unsafe_create_module_handle();
        assert_eq!(result, ModuleHandle { id: 0 });
        assert_eq!(val, true);
    }
}

#[test]
fn module_unsafe_remove_module_handle() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Optional::none()
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_unsafe_remove_module_handle_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_unsafe_remove_module_handle(ModuleHandle { id: 0 });
        assert_eq!(result, Optional::none());
        assert_eq!(val, true);
    }
}

#[test]
fn module_unsafe_link_module() {
    let mut val = false;
    let mut closure = |_: ModuleHandle, _: LoaderHandle, _: LoaderModuleHandle| {
        val = true;
        Optional::none()
    };
    let wrapper = wrap_closure_3(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_unsafe_link_module_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_unsafe_link_module(
            ModuleHandle { id: 0 },
            LoaderHandle { id: 0 },
            LoaderModuleHandle { id: 0 },
        );
        assert_eq!(result, Optional::none());
        assert_eq!(val, true);
    }
}

#[test]
fn module_unsafe_get_loader_module_handle() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Result::new_err(ModuleError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_unsafe_get_loader_module_handle_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_unsafe_get_loader_module_handle(ModuleHandle { id: 0 });
        assert_eq!(result, Result::new_err(ModuleError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn module_unsafe_get_loader_handle() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Result::new_err(ModuleError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_unsafe_get_loader_handle_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_unsafe_get_loader_handle(ModuleHandle { id: 0 });
        assert_eq!(result, Result::new_err(ModuleError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn module_unsafe_get_loader() {
    let mut val = false;
    let mut closure = |_: LoaderHandle| {
        val = true;
        Result::new_err(ModuleError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_unsafe_get_loader_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_unsafe_get_loader(LoaderHandle { id: 0 });
        assert_eq!(result, Result::new_err(ModuleError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn module_add_module() {
    let mut val = false;
    let mut closure = |_: LoaderHandle, _: NonNullConst<OsPathChar>| {
        val = true;
        Result::new_err(ModuleError::BufferOverflow)
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_add_module_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_add_module(LoaderHandle { id: 0 }, NonNullConst::dangling());
        assert_eq!(result, Result::new_err(ModuleError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn module_remove_module() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Optional::none()
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_remove_module_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_remove_module(ModuleHandle { id: 0 });
        assert_eq!(result, Optional::none());
        assert_eq!(val, true);
    }
}

#[test]
fn module_get_load_dependencies() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Result::new_err(ModuleError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_get_load_dependencies_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_get_load_dependencies(ModuleHandle { id: 0 });
        assert_eq!(result, Result::new_err(ModuleError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn module_fetch_status() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Result::new_err(ModuleError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_fetch_status_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_fetch_status(ModuleHandle { id: 0 });
        assert_eq!(result, Result::new_err(ModuleError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn module_add_dependency() {
    let mut val = false;
    let mut closure = |_: ModuleHandle, _: NonNullConst<InterfaceDescriptor>| {
        val = true;
        Optional::<ModuleError>::none()
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_add_dependency_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result =
            emf_cbase_module_add_dependency(ModuleHandle { id: 0 }, NonNullConst::dangling());
        assert_eq!(result, Optional::none());
        assert_eq!(val, true);
    }
}

#[test]
fn module_remove_dependency() {
    let mut val = false;
    let mut closure = |_: ModuleHandle, _: NonNullConst<InterfaceDescriptor>| {
        val = true;
        Optional::<ModuleError>::none()
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_remove_dependency_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result =
            emf_cbase_module_remove_dependency(ModuleHandle { id: 0 }, NonNullConst::dangling());
        assert_eq!(result, Optional::none());
        assert_eq!(val, true);
    }
}

#[test]
fn module_export_interface() {
    let mut val = false;
    let mut closure = |_: ModuleHandle, _: NonNullConst<InterfaceDescriptor>| {
        val = true;
        Optional::<ModuleError>::none()
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_export_interface_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result =
            emf_cbase_module_export_interface(ModuleHandle { id: 0 }, NonNullConst::dangling());
        assert_eq!(result, Optional::none());
        assert_eq!(val, true);
    }
}

#[test]
fn module_load() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Optional::<ModuleError>::none()
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_load_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_load(ModuleHandle { id: 0 });
        assert_eq!(result, Optional::none());
        assert_eq!(val, true);
    }
}

#[test]
fn module_unload() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Optional::<ModuleError>::none()
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_unload_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_unload(ModuleHandle { id: 0 });
        assert_eq!(result, Optional::none());
        assert_eq!(val, true);
    }
}

#[test]
fn module_initialize() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Optional::<ModuleError>::none()
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_initialize_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_initialize(ModuleHandle { id: 0 });
        assert_eq!(result, Optional::none());
        assert_eq!(val, true);
    }
}

#[test]
fn module_terminate() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Optional::<ModuleError>::none()
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_terminate_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_terminate(ModuleHandle { id: 0 });
        assert_eq!(result, Optional::none());
        assert_eq!(val, true);
    }
}

#[test]
fn module_get_module_info() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Result::new_err(ModuleError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_get_module_info_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_get_module_info(ModuleHandle { id: 0 });
        assert_eq!(result, Result::new_err(ModuleError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn module_get_exportable_interfaces() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Result::new_err(ModuleError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_get_exportable_interfaces_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_get_exportable_interfaces(ModuleHandle { id: 0 });
        assert_eq!(result, Result::new_err(ModuleError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn module_get_runtime_dependencies() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Result::new_err(ModuleError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_get_runtime_dependencies_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_get_runtime_dependencies(ModuleHandle { id: 0 });
        assert_eq!(result, Result::new_err(ModuleError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn module_get_interface() {
    let mut val = false;
    let mut closure = |_: ModuleHandle, _: NonNullConst<InterfaceDescriptor>| {
        val = true;
        Result::<ModuleInterface, _>::new_err(ModuleError::BufferOverflow)
    };
    let wrapper = wrap_closure_2(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_get_interface_fn = std::mem::transmute(wrapper);
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result =
            emf_cbase_module_get_interface(ModuleHandle { id: 0 }, NonNullConst::dangling());
        assert_eq!(result, Result::new_err(ModuleError::BufferOverflow));
        assert_eq!(val, true);
    }
}

#[test]
fn module_get_module_path() {
    let mut val = false;
    let mut closure = |_: ModuleHandle| {
        val = true;
        Result::new_err(ModuleError::BufferOverflow)
    };
    let wrapper = wrap_closure_1(&closure);
    unsafe {
        #[allow(invalid_value)]
        let mut base: BaseInterface = MaybeUninit::zeroed().assume_init();
        base.cbase_module = &mut closure as *mut _ as *mut BaseT;
        base.module_get_module_path_fn = wrapper;
        BASE_INTERFACE = MaybeUninit::new(std::mem::transmute(&base));

        let result = emf_cbase_module_get_module_path(ModuleHandle { id: 0 });
        assert_eq!(result, Result::new_err(ModuleError::BufferOverflow));
        assert_eq!(val, true);
    }
}
