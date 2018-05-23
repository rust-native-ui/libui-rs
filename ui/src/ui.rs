//! General functions.

use ffi_utils;
use libc::{c_char, c_void};
use std::fmt::{self, Debug, Formatter};
use std::ffi::CStr;
use std::mem;
use std::ops::Deref;
use std::cell::RefCell;
use std::marker::PhantomData;
use ui_sys::{self, uiInitOptions};
use window::Window;

thread_local! {
    static IS_INIT: RefCell<bool> = RefCell::new(false)
}

/// An initialized UI environment. You need to create one of these to do anything with this library.
///
/// Only one `UI` can be active at a time. When dropped, it will automatically deinitialize the UI environment.
/// A common pattern involves placing the `UI` in a [std::rc::Rc](https://doc.rust-lang.org/std/rc/struct.Rc.html)
/// so it can be shared with UI closures.
pub struct UI {
    pd: PhantomData<*const ()>,
}

impl UI {
    #[inline]
    /// Sets up the `libui` environment.
    ///
    /// # Panics
    /// Will panic if two UIs are initialized simultaneously.
    ///
    /// # Examples
    ///
    /// This will cause a panic:
    ///
    /// ```should_panic
    /// use ui::UI;
    /// let ui1 = UI::init();
    /// let ui2 = UI::init();
    /// ```
    ///
    /// This, however, will not, as `UI` is `Drop`.
    ///
    /// ```
    /// use ui::UI;
    /// {
    ///     let ui1 = UI::init();
    /// }
    /// let ui2 = UI::init();
    /// ```
    ///
    pub fn init() -> Result<Self, InitError> {
        IS_INIT.with(|isinit| {
            if *isinit.borrow() == true {
                panic!("Cannot initialize two libui UIs at the same time!");
            }
        });
        unsafe {
            let mut init_options = uiInitOptions {
                Size: mem::size_of::<uiInitOptions>(),
            };
            let err = ui_sys::uiInit(&mut init_options);
            if err.is_null() {
                ffi_utils::set_initialized();
                IS_INIT.with(|isinit| {
                    *isinit.borrow_mut() = true;
                });
                Ok(Self { pd: PhantomData })
            } else {
                Err(InitError { ui_init_error: err })
            }
        }
    }

    #[inline]
    /// Hands control of this thread to the UI toolkit, allowing it to display the UI and respond to events. Does not return until the UI [quit](struct.UI.html#method.quit)s.
    pub fn main(&self) {
        unsafe { ui_sys::uiMain() }
    }

    #[inline]
    /// Running this function causes the UI to quit, exiting from [main](struct.UI.html#method.main) and no longer showing any widgets.
    pub fn quit(&self) {
        unsafe { ui_sys::uiQuit() }
    }

    #[inline]
    pub fn queue_main(&self, callback: Box<FnMut()>) {
        unsafe {
            let mut data: Box<Box<FnMut()>> = Box::new(callback);
            ui_sys::uiQueueMain(
                ffi_utils::void_void_callback,
                &mut *data as *mut Box<FnMut()> as *mut c_void,
            );
            mem::forget(data);
        }
    }

    #[inline]
    /// Set a callback to be run when the application quits.
    pub fn on_should_quit(&self, callback: Box<FnMut()>) {
        unsafe {
            let mut data: Box<Box<FnMut()>> = Box::new(callback);
            ui_sys::uiOnShouldQuit(
                ffi_utils::void_void_callback,
                &mut *data as *mut Box<FnMut()> as *mut c_void,
            );
            mem::forget(data);
        }
    }
}

/// start manual control over event loop
#[inline]
pub fn main_steps() {
    unsafe {
        ui_sys::uiMainSteps()
    }
}

/// advance event loop
#[inline]
pub fn main_step(wait: bool) -> bool {
    unsafe {
        ui_sys::uiMainStep(if wait { 1 } else { 0 }) > 0
    }
}

/// call to start event loop handled by libui
#[inline]
pub fn main() {
    unsafe {
        ui_sys::uiMain()
    }
}

impl Drop for UI {
    fn drop(&mut self) {
        IS_INIT.with(|isinit| {
            *isinit.borrow_mut() = false;
        });
        unsafe {
            ffi_utils::unset_initialized();
            Window::destroy_all_windows();
            ui_sys::uiUninit();
        }
    }
}

/// An error that occurred during the initialization of the UI library.
pub struct InitError {
    ui_init_error: *const c_char,
}

impl Drop for InitError {
    fn drop(&mut self) {
        unsafe { ui_sys::uiFreeInitError(self.ui_init_error) }
    }
}

impl Debug for InitError {
    fn fmt(&self, f: &mut Formatter) -> Result<(), fmt::Error> {
        (**self).fmt(f)
    }
}

impl Deref for InitError {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe { CStr::from_ptr(self.ui_init_error).to_str().unwrap_or("") }
    }
}
