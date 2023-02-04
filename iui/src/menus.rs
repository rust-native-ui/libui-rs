//! Menus that appear at the top of windows, and the items that go in them.

use callback_helpers::{from_void_ptr, to_heap_ptr};
use controls::Window;
use std::ffi::CString;
use std::os::raw::{c_int, c_void};
use std::sync::atomic::{AtomicBool, Ordering};
use ui_sys::{self, uiMenu, uiMenuItem, uiWindow};
use UI;

pub static HAS_FINALIZED_MENUS: AtomicBool = AtomicBool::new(false);

/// A `MenuItem` represents an item that is shown in a `Menu`.
/// Note that, unlike many controls,
/// the text on `MenuItem`s cannot be changed after creation.
#[derive(Clone)]
pub struct MenuItem {
    ui_menu_item: *mut uiMenuItem,
}

/// A `Menu` represents one of the top-level menus at the top of a window.
/// That bar is unique per application, and creating a new `Menu` shows it
/// on all windows that support displaying menus.
///
/// Once windows have been created, no more menus can be created.
#[derive(Clone)]
pub struct Menu {
    ui_menu: *mut uiMenu,
}

impl MenuItem {
    /// Enables the item, allowing it to be selected. This is the default state of a menu item.
    pub fn enable(&self, _ctx: &UI) {
        unsafe { ui_sys::uiMenuItemEnable(self.ui_menu_item) }
    }

    /// Disables the item, preventing it from being selected and providing a visual cue to the
    /// user that it cannot be selected.
    pub fn disable(&self, _ctx: &UI) {
        unsafe { ui_sys::uiMenuItemDisable(self.ui_menu_item) }
    }

    /// Returns `true` if the menu item is checked, and false if it is not checked (or not checkable).
    pub fn checked(&self, _ctx: &UI) -> bool {
        unsafe { ui_sys::uiMenuItemChecked(self.ui_menu_item) != 0 }
    }

    /// Sets the menu item to either checked or unchecked based on the given value.
    ///
    /// Setting the checked value of a non-checkable menu item has no effect.
    pub fn set_checked(&self, _ctx: &UI, checked: bool) {
        unsafe { ui_sys::uiMenuItemSetChecked(self.ui_menu_item, checked as c_int) }
    }

    /// Sets the function to be executed when the item is clicked/selected.
    pub fn on_clicked<'ctx, F>(&self, _ctx: &'ctx UI, callback: F)
    where
        F: FnMut(&MenuItem, &Window) + 'static,
    {
        extern "C" fn c_callback<G: FnMut(&MenuItem, &Window)>(
            menu_item: *mut uiMenuItem,
            window: *mut uiWindow,
            data: *mut c_void,
        ) {
            let menu_item = unsafe { MenuItem::from_raw(menu_item) };
            let window = unsafe { Window::from_raw(window) };
            unsafe {
                from_void_ptr::<G>(data)(&menu_item, &window);
            }
        }
        unsafe {
            ui_sys::uiMenuItemOnClicked(
                self.ui_menu_item,
                Some(c_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }

    // Creates a `MenuItem` from a raw pointer
    pub unsafe fn from_raw(raw: *mut uiMenuItem) -> Self {
        MenuItem { ui_menu_item: raw }
    }
}

impl Menu {
    /// Creates a new menu with the given name to be displayed in the menubar
    /// at the top of all windows with a menubar.
    ///
    /// This is possible only if menus have not been finalized.
    pub fn new(_ctx: &UI, name: &str) -> Option<Menu> {
        if HAS_FINALIZED_MENUS.load(Ordering::SeqCst) { None }
        else {
            unsafe {
                let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
                Some(Menu {
                    ui_menu: ui_sys::uiNewMenu(c_string.as_ptr()),
                })
            }
        }
    }

    /// Adds a new item with the given name to the menu.
    ///
    /// This is possible only if menus have not been finalized.
    pub fn append_item(&self, name: &str) -> Option<MenuItem> {
        if HAS_FINALIZED_MENUS.load(Ordering::SeqCst) { None }
        else {
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            Some(MenuItem {
                ui_menu_item: ui_sys::uiMenuAppendItem(self.ui_menu, c_string.as_ptr()),
            })
        }
        }
    }

    /// Adds a new togglable (checkbox) item with the given name to the menu.
    ///
    /// This is possible only if menus have not been finalized.
    pub fn append_check_item(&self, name: &str) -> Option<MenuItem> {
        if HAS_FINALIZED_MENUS.load(Ordering::SeqCst) { None }
        else {
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            Some(MenuItem {
                ui_menu_item: ui_sys::uiMenuAppendCheckItem(self.ui_menu, c_string.as_ptr()),
            })
        }
        }
    }

    /// Adds a seperator to the menu.
    pub fn append_separator(&self) {
        unsafe { ui_sys::uiMenuAppendSeparator(self.ui_menu) }
    }
}

#[test]
fn cannot_change_menus_late() {
    use crate::prelude::*;
    let ui = UI::init().expect("failed to init");
    let mut menu = Menu::new(&ui, "menu").unwrap();
    assert!(menu.append_item("test item").is_some());
    let win = Window::new(&ui, "Test App", 200, 200, WindowType::HasMenubar);
    assert!(Menu::new(&ui, "menu2").is_none());
    assert!(menu.append_item("test item 2").is_none());
}

