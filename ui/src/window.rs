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

define_control!{
    /// Contains a single child control and displays it and its children in a window on the screen.
    control(Window, uiWindow, ui_window);
}

impl Window {
    #[inline]
    /// Create a new window with the given title, width, and height.
    /// If `has_menubar` is true, you can add items to the window's menu bar with
    /// the [`Menu`](struct.Menu.html) struct.
    pub fn new(title: &str, width: c_int, height: c_int, has_menubar: bool) -> Window {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(title.as_bytes().to_vec()).unwrap();
            let window = Window::from_ui_window(ui_sys::uiNewWindow(
                c_string.as_ptr(),
                width,
                height,
                has_menubar as c_int,
            ));

            WINDOWS.with(|windows| windows.borrow_mut().push(window.clone()));

            window
        }
    }

    #[inline]
    /// Return the inner `libui` pointer.
    pub fn as_ui_window(&self) -> *mut uiWindow {
        self.ui_window
    }

    #[inline]
    /// Get the current title of the window.
    pub fn title(&self) -> Text {
        ffi_utils::ensure_initialized();
        unsafe { Text::new(ui_sys::uiWindowTitle(self.ui_window)) }
    }

    #[inline]
    /// Set the window's title to the given string.
    pub fn set_title(&self, title: &str) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(title.as_bytes().to_vec()).unwrap();
            ui_sys::uiWindowSetTitle(self.ui_window, c_string.as_ptr())
        }
    }

    #[inline]
    /// Set a callback to be run when the window closes.
    ///
    /// This is often used on the main window of an application to quit
    /// the application when the window is closed.
    pub fn on_closing<F: FnMut(&Window) -> bool>(&self, callback: F) {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut data: Box<Box<FnMut(&Window) -> bool>> = Box::new(Box::new(callback));
            ui_sys::uiWindowOnClosing(
                self.ui_window,
                c_callback,
                &mut *data as *mut Box<FnMut(&Window) -> bool> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(window: *mut uiWindow, data: *mut c_void) -> i32 {
            unsafe {
                let window = Window { ui_window: window };
                mem::transmute::<*mut c_void, Box<Box<FnMut(&Window) -> bool>>>(data)(&window)
                    as i32
            }
        }
    }

    #[inline]
    /// Sets the window's child widget. The window can only have one child widget at a time.
    pub fn set_child(&self, child: Control) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiWindowSetChild(self.ui_window, child.as_ui_control()) }
    }

    #[inline]
    /// Check whether or not this window has margins around the edges.
    pub fn margined(&self) -> bool {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiWindowMargined(self.ui_window) != 0 }
    }

    #[inline]
    /// Set whether or not the window has margins around the edges.
    pub fn set_margined(&self, margined: bool) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiWindowSetMargined(self.ui_window, margined as c_int) }
    }

    #[inline]
    /// Allow the user to select an existing file.
    pub fn open_file(&self) -> Option<Text> {
        unsafe { Text::optional(ui_sys::uiOpenFile(self.as_ui_window())) }
    }

    #[inline]
    /// Allow the user to select a new or existing file.
    pub fn save_file(&self) -> Option<Text> {
        unsafe { Text::optional(ui_sys::uiSaveFile(self.as_ui_window())) }
    }

    #[inline]
    /// Open a generic message box to show a message to the user.
    pub fn msg_box(&self, title: &str, description: &str) {
        unsafe {
            let c_title = CString::new(title.as_bytes().to_vec()).unwrap();
            let c_description = CString::new(description.as_bytes().to_vec()).unwrap();
            ui_sys::uiMsgBox(
                self.as_ui_window(),
                c_title.as_ptr(),
                c_description.as_ptr(),
            )
        }
    }

    #[inline]
    /// Open an error-themed message box to show a message to the user.
    pub fn msg_box_error(&self, title: &str, description: &str) {
        unsafe {
            let c_title = CString::new(title.as_bytes().to_vec()).unwrap();
            let c_description = CString::new(description.as_bytes().to_vec()).unwrap();
            ui_sys::uiMsgBoxError(
                self.as_ui_window(),
                c_title.as_ptr(),
                c_description.as_ptr(),
            )
        }
    }

    #[inline]
    pub unsafe fn from_ui_window(window: *mut uiWindow) -> Window {
        Window { ui_window: window }
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
