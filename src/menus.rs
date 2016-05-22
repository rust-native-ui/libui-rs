//! Functions and types related to menus.

use ffi::{self, uiMenu, uiMenuItem, uiWindow};
use libc::{c_int, c_void};
use std::ffi::CString;
use std::mem;
use windows::Window;

// NB: If there ever becomes a way to destroy menus and/or menu items, we'll need to reference
// count these for memory safety.
#[derive(Clone)]
pub struct MenuItem {
    ui_menu_item: *mut uiMenuItem,
}

impl MenuItem {
    #[inline]
    pub fn enable(&self) {
        unsafe {
            ffi::uiMenuItemEnable(self.ui_menu_item)
        }
    }

    #[inline]
    pub fn disable(&self) {
        unsafe {
            ffi::uiMenuItemDisable(self.ui_menu_item)
        }
    }

    #[inline]
    pub fn on_clicked(&self, callback: Box<FnMut(MenuItem, Window)>) {
        unsafe {
            let mut data: Box<Box<FnMut(MenuItem, Window)>> = Box::new(callback);
            ffi::uiMenuItemOnClicked(self.ui_menu_item,
                                     c_callback,
                                     &mut *data as *mut Box<FnMut(MenuItem,
                                                                  Window)> as *mut c_void);
            mem::forget(data);
        }

        extern "C" fn c_callback(menu_item: *mut uiMenuItem,
                                 window: *mut uiWindow,
                                 data: *mut c_void) {
            unsafe {
                let menu_item = MenuItem {
                    ui_menu_item: menu_item,
                };
                let window = Window::from_ui_window(window);
                mem::transmute::<*mut c_void, Box<Box<FnMut(MenuItem, Window)>>>(data)(menu_item,
                                                                                       window)
            }
        }
    }

    #[inline]
    pub fn checked(&self) -> bool {
        unsafe {
            ffi::uiMenuItemChecked(self.ui_menu_item) != 0
        }
    }

    #[inline]
    pub fn set_checked(&self, checked: bool) {
        unsafe {
            ffi::uiMenuItemSetChecked(self.ui_menu_item, checked as c_int)
        }
    }
}

// NB: If there ever becomes a way to destroy menus, we'll need to reference count these for memory
// safety.
#[derive(Clone)]
pub struct Menu {
    ui_menu: *mut uiMenu,
}

impl Menu {
    #[inline]
    pub fn append_item(&self, name: &str) -> MenuItem {
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            MenuItem {
                ui_menu_item: ffi::uiMenuAppendItem(self.ui_menu, c_string.as_ptr()),
            }
        }
    }

    #[inline]
    pub fn append_check_item(&self, name: &str) -> MenuItem {
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            MenuItem {
                ui_menu_item: ffi::uiMenuAppendCheckItem(self.ui_menu, c_string.as_ptr()),
            }
        }
    }

    #[inline]
    pub fn append_quit_item(&self) -> MenuItem {
        unsafe {
            MenuItem {
                ui_menu_item: ffi::uiMenuAppendQuitItem(self.ui_menu),
            }
        }
    }

    #[inline]
    pub fn append_preferences_item(&self) -> MenuItem {
        unsafe {
            MenuItem {
                ui_menu_item: ffi::uiMenuAppendPreferencesItem(self.ui_menu),
            }
        }
    }

    #[inline]
    pub fn append_about_item(&self) -> MenuItem {
        unsafe {
            MenuItem {
                ui_menu_item: ffi::uiMenuAppendAboutItem(self.ui_menu),
            }
        }
    }

    #[inline]
    pub fn append_separator(&self) {
        unsafe {
            ffi::uiMenuAppendSeparator(self.ui_menu)
        }
    }

    #[inline]
    pub fn new(name: &str) -> Menu {
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            Menu {
                ui_menu: ffi::uiNewMenu(c_string.as_ptr()),
            }
        }
    }
}

