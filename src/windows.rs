//! Functions and types related to windows.

use controls::Control;
use ffi::{self, uiWindow};
use ffi_utils::Text;
use libc::{c_int, c_void};
use std::ffi::CString;
use std::mem;
use std::ops::Deref;

/// FIXME(pcwalton): We need to reference count these for memory safety!
#[derive(Clone)]
pub struct Window {
    ui_window: *mut uiWindow,
}

impl Deref for Window {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&Window, &Control>(self)
        }
    }
}

impl Window {
    #[inline]
    pub fn title(&self) -> Text {
        unsafe {
            Text::new(ffi::uiWindowTitle(self.ui_window))
        }
    }

    #[inline]
    pub fn set_title(&self, title: &str) {
        unsafe {
            let c_string = CString::new(title.as_bytes().to_vec()).unwrap();
            ffi::uiWindowSetTitle(self.ui_window, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn on_closing(&self, callback: Box<FnMut(Window) -> bool>) {
        unsafe {
            let mut data: Box<Box<FnMut(Window) -> bool>> = Box::new(callback);
            ffi::uiWindowOnClosing(self.ui_window,
                                   c_callback,
                                   &mut *data as *mut Box<FnMut(Window) -> bool> as *mut c_void);
            mem::forget(data);
        }

        extern "C" fn c_callback(window: *mut uiWindow, data: *mut c_void) -> i32 {
            unsafe {
                let window = Window {
                    ui_window: window,
                };
                mem::transmute::<*mut c_void, Box<Box<FnMut(Window) -> bool>>>(data)(window) as i32
            }
        }
    }

    #[inline]
    pub fn set_child(&self, child: Control) {
        unsafe {
            ffi::uiWindowSetChild(self.ui_window, child.as_ui_control())
        }
    }

    #[inline]
    pub fn margined(&self) -> bool {
        unsafe {
            ffi::uiWindowMargined(self.ui_window) != 0
        }
    }

    #[inline]
    pub fn set_margined(&self, margined: bool) {
        unsafe {
            ffi::uiWindowSetMargined(self.ui_window, margined as c_int)
        }
    }

    #[inline]
    pub fn new(title: &str, width: c_int, height: c_int, has_menubar: bool) -> Window {
        unsafe {
            let c_string = CString::new(title.as_bytes().to_vec()).unwrap();
            Window {
                ui_window: ffi::uiNewWindow(c_string.as_ptr(),
                                            width,
                                            height,
                                            has_menubar as c_int),
            }
        }
    }
}

