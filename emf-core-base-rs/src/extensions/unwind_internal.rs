//! The `unwind_internal` extension.
use crate::ffi::collections::NonNullConst;
use crate::ffi::errors::StaticError;
use crate::ffi::extensions::unwind_internal;
use crate::ffi::extensions::unwind_internal::{
    Context, PanicFn, ShutdownFn, UnwindInternalBinding,
};
use crate::ffi::CBaseBinding;
use crate::ownership::Owned;
use crate::sys::SysAPIMin;
use crate::CBaseInterfaceInfo;
use crate::{CBaseAPI, Error};
use std::any::Any;
use std::marker::PhantomData;
use std::panic::UnwindSafe;
use std::ptr::NonNull;

pub use default_context::DefaultContext;
pub use unwind_internal::UNWIND_INTERNAL_INTERFACE_NAME;
pub use unwind_internal::UNWIND_INTERNAL_VERSION_BUILD;
pub use unwind_internal::UNWIND_INTERNAL_VERSION_MAJOR;
pub use unwind_internal::UNWIND_INTERNAL_VERSION_MINOR;
pub use unwind_internal::UNWIND_INTERNAL_VERSION_PATCH;
pub use unwind_internal::UNWIND_INTERNAL_VERSION_RELEASE_NUMBER;
pub use unwind_internal::UNWIND_INTERNAL_VERSION_RELEASE_TYPE;
pub use unwind_internal::UNWIND_INTERNAL_VERSION_STRING;

/// Possible signals
#[derive(Debug)]
pub enum Signal {
    /// A signal that requests the termination of the interface.
    Shutdown,
    /// A panic originating from the interface's `panic()` function.
    Panic(Option<Error<Owned>>),
    /// A panic originating from an unknown source, including [panic!()].
    Other(Box<dyn Any + Send + 'static>),
}

/// Borrowed context of the unwind_internal api.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
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
#[derive(Debug, Hash)]
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
            let error = Error::from(StaticError::new(
                "Could not fetch the `unwind_internal` interface!",
            ));
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
    use crate::ffi::collections::Optional;
    use crate::ffi::errors::{Error, SimpleError};
    use crate::ffi::extensions::unwind_internal::Context;
    use crate::ffi::TypeWrapper;
    use crate::CBaseAPI;
    use std::any::Any;
    use std::fmt::{Debug, Display, Formatter};
    use std::panic::{AssertUnwindSafe, UnwindSafe};
    use std::ptr::NonNull;

    /// An unknown error.
    #[derive(Debug)]
    struct UnknownError {
        error: Box<dyn Any + Send>,
    }

    impl Display for UnknownError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            Debug::fmt(&*self.error, f)
        }
    }

    impl std::error::Error for UnknownError {}

    /// Default context.
    #[derive(Debug, Hash)]
    pub struct DefaultContext {}

    /// A shutdown signal.
    #[derive(Debug, Hash)]
    pub struct ShutdownSignal {}

    /// A panic signal.
    #[derive(Debug, Hash)]
    pub struct PanicSignal {
        /// Error message of the panic.
        pub error: Option<Error>,
    }

    extern "C-unwind" fn shutdown_fn(_context: Option<NonNull<Context>>) -> ! {
        std::panic::panic_any(ShutdownSignal {})
    }

    extern "C-unwind" fn panic_fn(_context: Option<NonNull<Context>>, err: Optional<Error>) -> ! {
        std::panic::panic_any(PanicSignal {
            error: err.into_rust(),
        })
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
                    Signal::Panic(err) => interface.panic(err),
                    Signal::Other(err) => interface.panic(Some(From::from(SimpleError::new(
                        format!("Unknown error: {:?}", err),
                    )))),
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
            std::panic::set_hook(Box::new(|_| {}));

            let result = std::panic::catch_unwind(AssertUnwindSafe(|| f(interface)));

            std::panic::set_hook(saved_panic_hook);
            extension.set_context(interface, saved_context);

            result.map_err(|e| {
                if e.is::<ShutdownSignal>() {
                    Signal::Shutdown
                } else if e.is::<PanicSignal>() {
                    let err = e.downcast::<PanicSignal>().unwrap();
                    Signal::Panic(err.error.map(From::from))
                } else {
                    Signal::Other(e)
                }
            })
        }
    }

    #[cfg(test)]
    mod tests {
        use crate::extensions::unwind_internal::default_context::{
            panic_fn, shutdown_fn, PanicSignal, ShutdownSignal,
        };
        use crate::ffi::collections::Optional;
        use crate::ffi::errors::StaticError;

        #[test]
        fn test_rust_panic() {
            let saved_panic_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
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
            std::panic::set_hook(Box::new(|_| {}));
            let result = std::panic::catch_unwind(|| shutdown_fn(None));
            std::panic::set_hook(saved_panic_hook);

            assert_eq!(result.is_err(), true);
            let error = result.unwrap_err();
            assert_eq!(error.is::<ShutdownSignal>(), true);
        }

        #[test]
        fn test_ext_panic() {
            let saved_panic_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            let result = std::panic::catch_unwind(|| {
                let error = StaticError::new("My panic message!");
                panic_fn(None, Optional::Some(From::from(error)))
            });
            std::panic::set_hook(saved_panic_hook);

            assert_eq!(result.is_err(), true);
            let mut error = result.unwrap_err();
            assert_eq!(error.is::<PanicSignal>(), true);
            let signal = error.downcast_mut::<PanicSignal>().unwrap();
            let error = signal.error.take();
            assert_eq!(error.is_some(), true);

            let error = error.as_ref().unwrap();
            let error_info_dbg = error.debug_info();
            let error_info_dis = error.display_info();
            assert_eq!("\"My panic message!\"", error_info_dbg.as_ref());
            assert_eq!("My panic message!", error_info_dis.as_ref());
        }

        #[test]
        fn test_ext_panic_empty() {
            let saved_panic_hook = std::panic::take_hook();
            std::panic::set_hook(Box::new(|_| {}));
            let result = std::panic::catch_unwind(|| panic_fn(None, Optional::None));
            std::panic::set_hook(saved_panic_hook);

            assert_eq!(result.is_err(), true);
            let mut error = result.unwrap_err();
            assert_eq!(error.is::<PanicSignal>(), true);
            let signal = error.downcast_mut::<PanicSignal>().unwrap();
            let error = signal.error.take();
            assert_eq!(error.is_some(), false);
        }
    }
}
