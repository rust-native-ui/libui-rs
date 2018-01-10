//! General functions.

use ffi_utils;
use libc::{c_char, c_void};
use std::fmt::{self, Debug, Formatter};
use std::ffi::CStr;
use std::mem;
use std::ops::Deref;
use std::cell::RefCell;
use ui_sys::{self, uiInitOptions};
use windows::Window;

thread_local! {
    static IS_INIT: RefCell<bool> = RefCell::new(false)
}

pub struct UI;

impl UI {
    #[inline]
    /// Sets up the `libui` environment.
    /// 
    /// # Panics
    /// Will panic if two UIs are initialized simultaneously.
    /// ```
    /// use ui::UI;
    /// let ui1 = UI::init();
    /// let ui2 = UI::init();
    /// ```
    pub fn init() -> Result<Self, InitError> {
        IS_INIT.with(|isinit| {
            if *isinit.borrow() == true {
                panic!("Cannot initialize two libui UIs at the same time!");
            }
        });
        unsafe {
            let mut init_options = uiInitOptions { Size: mem::size_of::<uiInitOptions>() };
            let err = ui_sys::uiInit(&mut init_options);
            if err.is_null() {
                ffi_utils::set_initialized();
                IS_INIT.with(|isinit|{
                    *isinit.borrow_mut() = true;
                });
                Ok(Self {})
            } else {
                Err(InitError { ui_init_error: err })
            }
        }
    }

    #[inline]
    /// Hands control of this thread to the UI toolkit, allowing it to display the UI and respond to events. Does not return until the UI [quit](fn.quit.html)s.
    pub fn main(&self) {
        unsafe { ui_sys::uiMain() }
    }

    #[inline]
    /// Running this function causes the UI to quit, exiting from [main](fn.main.html) and no longer showing any widgets.
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

impl Drop for UI {
    fn drop(&mut self) {
        IS_INIT.with(|isinit|{
            *isinit.borrow_mut() = false;
        });
        unsafe {
            ffi_utils::unset_initialized();
            Window::destroy_all_windows();
            ui_sys::uiUninit();
        }
    }
}

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
