//! The `unwind_internal` extension.
use crate::ffi::collections::NonNullConst;
use crate::ffi::extensions::unwind_internal;
use crate::ffi::extensions::unwind_internal::{
    Context, PanicFn, ShutdownFn, UnwindInternalBinding,
};
use crate::sys::SysAPIMin;
use crate::CBaseAPI;
use std::any::Any;
use std::marker::PhantomData;
use std::panic::UnwindSafe;
use std::ptr::NonNull;

pub use unwind_internal::UNWIND_INTERNAL_INTERFACE_NAME;
pub use unwind_internal::UNWIND_INTERNAL_VERSION_BUILD;
pub use unwind_internal::UNWIND_INTERNAL_VERSION_MAJOR;
pub use unwind_internal::UNWIND_INTERNAL_VERSION_MINOR;
pub use unwind_internal::UNWIND_INTERNAL_VERSION_PATCH;
pub use unwind_internal::UNWIND_INTERNAL_VERSION_RELEASE_NUMBER;
pub use unwind_internal::UNWIND_INTERNAL_VERSION_RELEASE_TYPE;
pub use unwind_internal::UNWIND_INTERNAL_VERSION_STRING;

use crate::ffi::CBaseBinding;
use crate::CBaseInterfaceInfo;
pub use default_context::DefaultContext;
use std::ffi::CStr;

/// Possible signals
#[derive(Debug)]
pub enum Signal {
    /// A signal that requests the termination of the interface.
    Shutdown,
    /// A panic originating from the interface's `panic()` function.
    Panic(Box<dyn Any + Send + 'static>),
    /// A panic originating from an unknown source, including [panic!()].
    Other(Box<dyn Any + Send + 'static>),
}

/// Borrowed context of the unwind_internal api.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct UnwindInternalContextRef {
    /// Context pointer, that is passed to the shutdown and panic functions.
    pub _context: NonNull<Context>,
    /// Shutdown function.
    pub _shutdown: ShutdownFn,
    /// Panic function.
    pub _panic: PanicFn,
}

unsafe impl Send for UnwindInternalContextRef {}
unsafe impl Sync for UnwindInternalContextRef {}

/// Interface of the `unwind_internal` extension.
#[derive(Debug)]
pub struct UnwindInternalInterface<'interface> {
    _interface: NonNullConst<unwind_internal::UnwindInternalInterface>,
    _phantom: PhantomData<&'interface unwind_internal::UnwindInternalInterface>,
}

unsafe impl Send for UnwindInternalInterface<'_> {}
unsafe impl Sync for UnwindInternalInterface<'_> {}

/// API of the `unwind_internal` interface.
pub trait UnwindInternalAPI<'interface> {
    /// Fetches the `unwind_internal` interface from the main interface.
    fn from_interface(interface: &(impl SysAPIMin<'interface> + CBaseInterfaceInfo)) -> Self;

    /// Retrieves a pointer to the native interface.
    fn to_interface(&self) -> NonNullConst<unwind_internal::UnwindInternalInterface>;

    /// Fetches the active context.
    ///
    /// # Return
    ///
    /// Active context.
    fn get_context(
        &self,
        interface: &impl CBaseAPI<'interface>,
    ) -> Option<UnwindInternalContextRef>;

    /// Sets the new active context.
    fn set_context(
        &mut self,
        interface: &mut impl CBaseAPI<'interface>,
        context: Option<UnwindInternalContextRef>,
    );
}

impl<'interface> UnwindInternalAPI<'interface> for UnwindInternalInterface<'interface> {
    fn from_interface(interface: &(impl SysAPIMin<'interface> + CBaseInterfaceInfo)) -> Self {
        use crate::fn_caster::extensions::unwind_internal as casters;
        if let Some(unw_int_fn) =
            SysAPIMin::get_function(interface, &casters::GetUnwindInternalInterfaceCaster {})
        {
            Self {
                _interface: unsafe { unw_int_fn(interface.internal_interface().base_module()) },
                _phantom: PhantomData,
            }
        } else {
            let error = unsafe {
                CStr::from_bytes_with_nul_unchecked(
                    b"Could not fetch the `unwind_internal` interface!\n",
                )
            };
            SysAPIMin::panic(interface, Some(error))
        }
    }

    fn to_interface(&self) -> NonNullConst<unwind_internal::UnwindInternalInterface> {
        self._interface
    }

    fn get_context(
        &self,
        interface: &impl CBaseAPI<'interface>,
    ) -> Option<UnwindInternalContextRef> {
        unsafe {
            if let Some(context) = self
                ._interface
                .as_ref()
                .get_context(interface.internal_interface())
            {
                if let Some(shutdown) = self
                    ._interface
                    .as_ref()
                    .get_shutdown_fn(interface.internal_interface())
                {
                    if let Some(panic) = self
                        ._interface
                        .as_ref()
                        .get_panic_fn(interface.internal_interface())
                    {
                        return Some(UnwindInternalContextRef {
                            _context: context,
                            _shutdown: shutdown,
                            _panic: panic,
                        });
                    }
                }
            }

            None
        }
    }

    fn set_context(
        &mut self,
        interface: &mut impl CBaseAPI<'interface>,
        context: Option<UnwindInternalContextRef>,
    ) {
        unsafe {
            if let Some(context) = context {
                self._interface
                    .into_mut()
                    .as_mut()
                    .set_context(interface.internal_interface(), Some(context._context));
                self._interface
                    .into_mut()
                    .as_mut()
                    .set_shutdown_fn(interface.internal_interface(), Some(context._shutdown));
                self._interface
                    .into_mut()
                    .as_mut()
                    .set_panic_fn(interface.internal_interface(), Some(context._panic));
            } else {
                self._interface
                    .into_mut()
                    .as_mut()
                    .set_context(interface.internal_interface(), None);
                self._interface
                    .into_mut()
                    .as_mut()
                    .set_shutdown_fn(interface.internal_interface(), None);
                self._interface
                    .into_mut()
                    .as_mut()
                    .set_panic_fn(interface.internal_interface(), None);
            }
        }
    }
}

/// Interface of a unwinding context.
pub trait UnwindInternalContextAPI<'interface> {
    /// Sets up the unwinding for the closure `f`
    ///
    /// Any panic or termination signal, that occurs within `f`, is propagated.
    ///
    /// # Return
    ///
    /// Return value from `f`.
    fn setup_unwind<T, U>(
        &self,
        extension: &mut impl UnwindInternalAPI<'interface>,
        interface: &mut T,
        f: impl FnOnce(&mut T) -> U + UnwindSafe,
    ) -> U
    where
        T: CBaseAPI<'interface>;

    /// Sets up the unwinding for the closure `f`
    ///
    /// Any panic or termination signal, that occurs within `f`, is caught and returned.
    ///
    /// # Return
    ///
    /// Return value from `f` or caught signal.
    fn catch_unwind<T, U>(
        &self,
        extension: &mut impl UnwindInternalAPI<'interface>,
        interface: &mut T,
        f: impl FnOnce(&mut T) -> U + UnwindSafe,
    ) -> Result<U, Signal>
    where
        T: CBaseAPI<'interface>;
}

pub mod default_context {
    //! Implementation of the default context.
    use crate::extensions::unwind_internal::{
        Signal, UnwindInternalAPI, UnwindInternalContextAPI, UnwindInternalContextRef,
    };
    use crate::ffi::collections::NonNullConst;
    use crate::ffi::extensions::unwind_internal::Context;
    use crate::ffi::TypeWrapper;
    use crate::CBaseAPI;
    use backtrace::Backtrace;
    use std::cell::UnsafeCell;
    use std::ffi::{CStr, CString};
    use std::panic::{AssertUnwindSafe, PanicInfo, UnwindSafe};
    use std::ptr::NonNull;

    /// Default context.
    #[derive(Debug)]
    pub struct DefaultContext {}

    /// A shutdown signal.
    #[derive(Debug)]
    pub struct ShutdownSignal {}

    /// A panic signal.
    #[derive(Debug)]
    pub struct PanicSignal {
        /// Error message of the panic.
        pub error: UnsafeCell<Option<CString>>,
    }

    extern "C-unwind" fn shutdown_fn(_context: Option<NonNull<Context>>) -> ! {
        std::panic::panic_any(ShutdownSignal {})
    }

    extern "C-unwind" fn panic_fn(
        _context: Option<NonNull<Context>>,
        err: Option<NonNullConst<u8>>,
    ) -> ! {
        let error = {
            if let Some(err) = err {
                let err_str = unsafe { CStr::from_ptr(err.cast().as_ptr()) };
                PanicSignal {
                    error: UnsafeCell::new(Some(err_str.to_owned())),
                }
            } else {
                PanicSignal {
                    error: UnsafeCell::new(None),
                }
            }
        };
        std::panic::panic_any(error)
    }

    fn panic_hook(info: &PanicInfo<'_>) {
        let backtrace = Backtrace::new();
        if let Some(payload) = info.payload().downcast_ref::<PanicSignal>() {
            let error = unsafe { &mut *payload.error.get() };
            if let Some(error) = error {
                let mut error_vec = Vec::from(error.as_bytes());
                error_vec.extend_from_slice("\n\nBacktrace:\n".as_bytes());
                error_vec.extend_from_slice(format!("{:?}", backtrace).as_bytes());

                *error = CString::new(error_vec).unwrap();
            } else {
                let trace = CString::new(format!("Backtrace:\n{:?}", backtrace)).unwrap();
                *error = Some(trace);
            }
        }
    }

    impl Default for DefaultContext {
        fn default() -> Self {
            DefaultContext {}
        }
    }

    impl<'interface> UnwindInternalContextAPI<'interface> for DefaultContext {
        fn setup_unwind<T, U>(
            &self,
            extension: &mut impl UnwindInternalAPI<'interface>,
            interface: &mut T,
            f: impl FnOnce(&mut T) -> U + UnwindSafe,
        ) -> U
        where
            T: CBaseAPI<'interface>,
        {
            let result = self.catch_unwind(extension, interface, f);

            match result {
                Ok(v) => v,
                Err(sig) => match sig {
                    Signal::Shutdown => interface.shutdown(),
                    Signal::Panic(err) => {
                        let error = err.downcast_ref::<PanicSignal>().unwrap();
                        interface.panic(unsafe { (&*error.error.get()).as_ref() })
                    }
                    Signal::Other(err) => {
                        if let Some(error) = err.downcast_ref::<&str>() {
                            if let Ok(error) = CString::new(*error) {
                                interface.panic(Some(error))
                            }
                        }
                        let error = unsafe {
                            CStr::from_bytes_with_nul_unchecked(b"Unknown error occurred!\0")
                        };
                        interface.panic(Some(error))
                    }
                },
            }
        }

        fn catch_unwind<T, U>(
            &self,
            extension: &mut impl UnwindInternalAPI<'interface>,
            interface: &mut T,
            f: impl FnOnce(&mut T) -> U + UnwindSafe,
        ) -> Result<U, Signal>
        where
            T: CBaseAPI<'interface>,
        {
            let saved_context = extension.get_context(interface);
            let saved_panic_hook = std::panic::take_hook();

            let context = UnwindInternalContextRef {
                _context: NonNull::dangling(),
                _shutdown: TypeWrapper(shutdown_fn),
                _panic: TypeWrapper(panic_fn),
            };

            extension.set_context(interface, Some(context));
            std::panic::set_hook(Box::new(panic_hook));

            let result = std::panic::catch_unwind(AssertUnwindSafe(|| f(interface)));

            std::panic::set_hook(saved_panic_hook);
            extension.set_context(interface, saved_context);

            result.map_err(|e| {
                if e.is::<ShutdownSignal>() {
                    Signal::Shutdown
                } else if e.is::<PanicSignal>() {
                    Signal::Panic(e)
                } else {
                    Signal::Other(e)
                }
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::extensions::unwind_internal::default_context::{
            panic_fn, panic_hook, shutdown_fn, PanicSignal, ShutdownSignal,
        };
        use crate::ffi::collections::NonNullConst;

        #[test]
        fn test_rust_panic() {
            let saved_panic_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(panic_hook));
            let result = std::panic::catch_unwind(|| panic!("My panic message!"));
            std::panic::set_hook(saved_panic_hook);

            assert_eq!(result.is_err(), true);
            let error = result.unwrap_err();
            assert_eq!(error.is::<ShutdownSignal>(), false);
            assert_eq!(error.is::<PanicSignal>(), false);
        }

        #[test]
        fn test_ext_shutdown() {
            let saved_panic_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(panic_hook));
            let result = std::panic::catch_unwind(|| shutdown_fn(None));
            std::panic::set_hook(saved_panic_hook);

            assert_eq!(result.is_err(), true);
            let error = result.unwrap_err();
            assert_eq!(error.is::<ShutdownSignal>(), true);
        }

        #[test]
        fn test_ext_panic() {
            let saved_panic_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(panic_hook));
            let result = std::panic::catch_unwind(|| {
                panic_fn(None, Some(NonNullConst::from(b"My panic message!\0")))
            });
            std::panic::set_hook(saved_panic_hook);

            assert_eq!(result.is_err(), true);
            let mut error = result.unwrap_err();
            assert_eq!(error.is::<PanicSignal>(), true);
            let signal = error.downcast_mut::<PanicSignal>().unwrap();
            let error = signal.error.get_mut();
            assert_eq!(error.is_some(), true);
            let error = error.as_ref().unwrap();
            let error = error.to_str().unwrap();
            print!("{}", error);
        }

        #[test]
        fn test_ext_panic_empty() {
            let saved_panic_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(panic_hook));
            let result = std::panic::catch_unwind(|| panic_fn(None, None));
            std::panic::set_hook(saved_panic_hook);

            assert_eq!(result.is_err(), true);
            let mut error = result.unwrap_err();
            assert_eq!(error.is::<PanicSignal>(), true);
            let signal = error.downcast_mut::<PanicSignal>().unwrap();
            let error = signal.error.get_mut();
            assert_eq!(error.is_some(), true);
            let error = error.as_ref().unwrap();
            let error = error.to_str().unwrap();
            print!("{}", error);
        }
    }
}
