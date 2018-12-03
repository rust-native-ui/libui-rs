//! Functionality related to creating, managing, and destroying GUI windows.

use controls::Control;
use std::os::raw::{c_int, c_void};
use std::cell::RefCell;
use std::ffi::{CStr, CString};
use std::mem;
use std::path::PathBuf;
use ui::UI;
use ui_sys::{self, uiControl, uiWindow};

thread_local! {
    static WINDOWS: RefCell<Vec<Window>> = RefCell::new(Vec::new())
}

/// A `Window` can either have a menubar or not; this enum represents that decision.\
#[derive(Clone, Copy, Debug)]
pub enum WindowType {
    HasMenubar,
    NoMenubar,
}

define_control!{
    /// Contains a single child control and displays it and its children in a window on the screen.
    rust_type: Window,
    sys_type: uiWindow
}

impl Window {
    /// Create a new window with the given title, width, height, and type.
    /// By default, when a new window is created, it will cause the application to quit when closed.
    /// The user can prevent this by adding a custom `on_closing` behavior.
    pub fn new(_ctx: &UI, title: &str, width: c_int, height: c_int, t: WindowType) -> Window {
        let has_menubar = match t {
            WindowType::HasMenubar => true,
            WindowType::NoMenubar => false,
        };
        let mut window = unsafe {
            let c_string = CString::new(title.as_bytes().to_vec()).unwrap();
            let window = Window::from_raw(ui_sys::uiNewWindow(
                c_string.as_ptr(),
                width,
                height,
                has_menubar as c_int,
            ));

            WINDOWS.with(|windows| windows.borrow_mut().push(window.clone()));

            window
        };

        // Windows, by default, quit the application on closing.
        let ui = _ctx.clone();
        window.on_closing(_ctx, move |_| {
            ui.quit();
        });

        // Windows, by default, draw margins
        window.set_margined(_ctx, true);

        window
    }

    /// Get the current title of the window.
    pub fn title(&self, _ctx: &UI) -> String {
        unsafe {
            CStr::from_ptr(ui_sys::uiWindowTitle(self.uiWindow))
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Get a reference to the current title of the window.
    pub fn title_ref(&self, _ctx: &UI) -> &CStr {
        unsafe { &CStr::from_ptr(ui_sys::uiWindowTitle(self.uiWindow)) }
    }

    /// Set the window's title to the given string.
    pub fn set_title(&mut self, _ctx: &UI, title: &str) {
        unsafe {
            let c_string = CString::new(title.as_bytes().to_vec()).unwrap();
            ui_sys::uiWindowSetTitle(self.uiWindow, c_string.as_ptr())
        }
    }

    /// Set a callback to be run when the window closes.
    ///
    /// This is often used on the main window of an application to quit
    /// the application when the window is closed.
    pub fn on_closing<'ctx, F: FnMut(&mut Window) + 'ctx>(&mut self, _ctx: &'ctx UI, mut callback: F) {
        unsafe {
            let mut data: Box<Box<FnMut(&mut Window) -> bool>> = Box::new(Box::new(|window| {
                callback(window);
                false
            }));
            ui_sys::uiWindowOnClosing(
                self.uiWindow,
                Some(c_callback),
                &mut *data as *mut Box<FnMut(&mut Window) -> bool> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(window: *mut uiWindow, data: *mut c_void) -> i32 {
            unsafe {
                let mut window = Window { uiWindow: window };
                mem::transmute::<*mut c_void, Box<Box<FnMut(&mut Window) -> bool>>>(data)(
                    &mut window,
                ) as i32
            }
        }
    }

    /// Check whether or not this window has margins around the edges.
    pub fn margined(&self, _ctx: &UI) -> bool {
        unsafe { ui_sys::uiWindowMargined(self.uiWindow) != 0 }
    }

    /// Set whether or not the window has margins around the edges.
    pub fn set_margined(&mut self, _ctx: &UI, margined: bool) {
        unsafe { ui_sys::uiWindowSetMargined(self.uiWindow, margined as c_int) }
    }

    /// Sets the window's child widget. The window can only have one child widget at a time.
    pub fn set_child<T: Into<Control>>(&mut self, _ctx: &UI, child: T) {
        unsafe { ui_sys::uiWindowSetChild(self.uiWindow, child.into().as_ui_control()) }
    }

    /// Allow the user to select an existing file.
    pub fn open_file(&self, _ctx: &UI) -> Option<PathBuf> {
        let ptr = unsafe { ui_sys::uiOpenFile(self.uiWindow) };
        if ptr.is_null() {
            return None;
        };
        let path_string: String = unsafe { CStr::from_ptr(ptr).to_string_lossy().into() };
        Some(path_string.into())
    }

    /// Allow the user to select a new or existing file.
    pub fn save_file(&self, _ctx: &UI) -> Option<PathBuf> {
        let ptr = unsafe { ui_sys::uiSaveFile(self.uiWindow) };
        if ptr.is_null() {
            return None;
        };
        let path_string: String = unsafe { CStr::from_ptr(ptr).to_string_lossy().into() };
        Some(path_string.into())
    }

    /// Open a generic message box to show a message to the user.
    /// Returns when the user acknowledges the message.
    pub fn modal_msg(&self, _ctx: &UI, title: &str, description: &str) {
        unsafe {
            let c_title = CString::new(title.as_bytes().to_vec()).unwrap();
            let c_description = CString::new(description.as_bytes().to_vec()).unwrap();
            ui_sys::uiMsgBox(self.uiWindow, c_title.as_ptr(), c_description.as_ptr())
        }
    }

    /// Open an error-themed message box to show a message to the user.
    /// Returns when the user acknowledges the message.
    pub fn modal_err(&self, _ctx: &UI, title: &str, description: &str) {
        unsafe {
            let c_title = CString::new(title.as_bytes().to_vec()).unwrap();
            let c_description = CString::new(description.as_bytes().to_vec()).unwrap();
            ui_sys::uiMsgBoxError(self.uiWindow, c_title.as_ptr(), c_description.as_ptr())
        }
    }

    pub unsafe fn destroy_all_windows() {
        WINDOWS.with(|windows| {
            let mut windows = windows.borrow_mut();
            for window in windows.drain(..) {
                window.destroy();
            }
        })
    }

    /// Destroys a Window. Any use of the control after this is use-after-free; therefore, this
    /// is marked unsafe.
    pub unsafe fn destroy(&self) {
        // Don't check for initialization here since this can be run during deinitialization.
        ui_sys::uiControlDestroy(self.uiWindow as *mut ui_sys::uiControl)
    }
}
