//! Functions and types related to windows.

use controls::Control;
use ffi_utils::{self, Text};
use libc::{c_int, c_void};
use std::cell::RefCell;
use std::ffi::CString;
use std::mem;
use ui_sys::{self, uiControl, uiWindow};

thread_local! {
    static WINDOWS: RefCell<Vec<Window>> = RefCell::new(Vec::new())
}

define_control_alt!(Window, uiWindow, ui_window);

impl Window {
    #[inline]
    pub fn as_ui_window(&self) -> *mut uiWindow {
        self.ui_window
    }

    #[inline]
    pub fn title(&self) -> Text {
        ffi_utils::ensure_initialized();
        unsafe {
            Text::new(ui_sys::uiWindowTitle(self.ui_window))
        }
    }

    #[inline]
    pub fn set_title(&self, title: &str) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(title.as_bytes().to_vec()).unwrap();
            ui_sys::uiWindowSetTitle(self.ui_window, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn on_closing(&self, callback: Box<FnMut(&Window) -> bool>) {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut data: Box<Box<FnMut(&Window) -> bool>> = Box::new(callback);
            ui_sys::uiWindowOnClosing(self.ui_window,
                                      c_callback,
                                      &mut *data as *mut Box<FnMut(&Window) -> bool> as
                                      *mut c_void);
            mem::forget(data);
        }

        extern "C" fn c_callback(window: *mut uiWindow, data: *mut c_void) -> i32 {
            unsafe {
                let window = Window {
                    ui_window: window,
                };
                mem::transmute::<*mut c_void,
                                 Box<Box<FnMut(&Window) -> bool>>>(data)(&window) as i32
            }
        }
    }

    #[inline]
    pub fn set_child(&self, child: Control) {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiWindowSetChild(self.ui_window, child.as_ui_control())
        }
    }

    #[inline]
    pub fn margined(&self) -> bool {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiWindowMargined(self.ui_window) != 0
        }
    }

    #[inline]
    pub fn set_margined(&self, margined: bool) {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiWindowSetMargined(self.ui_window, margined as c_int)
        }
    }

    pub fn set_autosave(&self, name: &str) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            ui_sys::uiWindowSetAutosave(self.ui_window, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn new(title: &str, width: c_int, height: c_int, has_menubar: bool) -> Window {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(title.as_bytes().to_vec()).unwrap();
            let window = Window::from_ui_window(ui_sys::uiNewWindow(c_string.as_ptr(),
                                                                    width,
                                                                    height,
                                                                    has_menubar as c_int));

            WINDOWS.with(|windows| windows.borrow_mut().push(window.clone()));

            window
        }
    }

    #[inline]
    pub unsafe fn from_ui_window(window: *mut uiWindow) -> Window {
        Window {
            ui_window: window,
        }
    }

    pub unsafe fn destroy_all_windows() {
        WINDOWS.with(|windows| {
            let mut windows = windows.borrow_mut();
            for window in windows.drain(..) {
                window.destroy()
            }
        })
    }
}

// Defines a new control, creating a Rust wrapper, a `Deref` implementation, and a destructor.
// An example of use:
//
//     define_control!(Slider, uiSlider, ui_slider)
#[macro_export]
macro_rules! define_control_alt {
    ($rust_type:ident, $ui_type:ident, $ui_field:ident) => {
        pub struct $rust_type {
            $ui_field: *mut $ui_type,
        }

        impl ::std::ops::Deref for $rust_type {
            type Target = ::controls::Control;

            #[inline]
            fn deref(&self) -> &::controls::Control {
                // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
                unsafe {
                    mem::transmute::<&$rust_type, &::controls::Control>(self)
                }
            }
        }

        impl Drop for $rust_type {
            #[inline]
            fn drop(&mut self) {
                // For now this does nothing, but in the future, when `libui` supports proper
                // memory management, this will likely need to twiddle reference counts.
            }
        }

        impl Clone for $rust_type {
            #[inline]
            fn clone(&self) -> $rust_type {
                $rust_type {
                    $ui_field: self.$ui_field,
                }
            }
        }

        impl Into<Control> for $rust_type {
            #[inline]
            fn into(self) -> Control {
                unsafe {
                    let control = Control::from_ui_control(self.$ui_field as *mut uiControl);
                    mem::forget(self);
                    control
                }
            }
        }

        impl $rust_type {
            #[inline]
            pub unsafe fn from_ui_control($ui_field: *mut $ui_type) -> $rust_type {
                $rust_type {
                    $ui_field: $ui_field
                }
            }
        }
    }
}