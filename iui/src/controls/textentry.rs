//! Text input mechanisms in various forms.
//!
//! All text buffers accept and return `\n` line endings; if on Windows, the appropriate
//! `\r\n` for display are added and removed by the controls.

use super::Control;
use callback_helpers::{from_void_ptr, to_heap_ptr};
use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::c_void;
use str_tools::{from_toolkit_string, to_toolkit_string};
use ui::UI;
use ui_sys::{self, uiControl, uiEntry, uiMultilineEntry};

pub trait TextEntry {
    fn value(&self, ctx: &UI) -> String;
    fn set_value(&mut self, ctx: &UI, value: &str);
    fn on_changed<'ctx, F: FnMut(String) + 'static>(&mut self, ctx: &'ctx UI, callback: F);
}

define_control! {
    /// Single-line editable text buffer.
    rust_type: Entry,
    sys_type: uiEntry
}

define_control! {
    /// Single-line editable text buffer.
    rust_type: PasswordEntry,
    sys_type: uiEntry
}

define_control! {
    /// Multi-line editable text buffer.
    rust_type: MultilineEntry,
    sys_type: uiMultilineEntry
}

impl Entry {
    pub fn new(_ctx: &UI) -> Entry {
        unsafe { Entry::from_raw(ui_sys::uiNewEntry()) }
    }
}

impl PasswordEntry {
    pub fn new(_ctx: &UI) -> PasswordEntry {
        unsafe { PasswordEntry::from_raw(ui_sys::uiNewPasswordEntry()) }
    }
}

impl MultilineEntry {
    pub fn new(_ctx: &UI) -> MultilineEntry {
        unsafe { MultilineEntry::from_raw(ui_sys::uiNewMultilineEntry()) }
    }
}

impl TextEntry for Entry {
    fn value(&self, _ctx: &UI) -> String {
        unsafe { from_toolkit_string(ui_sys::uiEntryText(self.uiEntry)) }
    }

    fn set_value(&mut self, _ctx: &UI, value: &str) {
        let cstring = to_toolkit_string(value);
        unsafe { ui_sys::uiEntrySetText(self.uiEntry, cstring.as_ptr()) }
    }

    fn on_changed<'ctx, F>(&mut self, _ctx: &'ctx UI, callback: F)
    where
        F: FnMut(String) + 'static,
    {
        extern "C" fn c_callback<G>(entry: *mut uiEntry, data: *mut c_void)
        where
            G: FnMut(String),
        {
            let string = unsafe { CStr::from_ptr(ui_sys::uiEntryText(entry)) }
                .to_string_lossy()
                .into_owned();
            unsafe { from_void_ptr::<G>(data)(string) }
        }

        unsafe {
            ui_sys::uiEntryOnChanged(self.uiEntry, Some(c_callback::<F>), to_heap_ptr(callback));
        }
    }
}

impl TextEntry for PasswordEntry {
    fn value(&self, _ctx: &UI) -> String {
        unsafe {
            CStr::from_ptr(ui_sys::uiEntryText(self.uiEntry))
                .to_string_lossy()
                .into_owned()
        }
    }
    fn set_value(&mut self, _ctx: &UI, value: &str) {
        let cstring = CString::new(value.as_bytes().to_vec()).unwrap();
        unsafe { ui_sys::uiEntrySetText(self.uiEntry, cstring.as_ptr()) }
    }

    fn on_changed<'ctx, F: FnMut(String) + 'static>(&mut self, _ctx: &'ctx UI, callback: F) {
        unsafe {
            let mut data: Box<Box<dyn FnMut(String)>> = Box::new(Box::new(callback));
            ui_sys::uiEntryOnChanged(
                self.uiEntry,
                Some(c_callback),
                &mut *data as *mut Box<dyn FnMut(String)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(entry: *mut uiEntry, data: *mut c_void) {
            unsafe {
                let string = from_toolkit_string(ui_sys::uiEntryText(entry));
                mem::transmute::<*mut c_void, &mut Box<dyn FnMut(String)>>(data)(string);
                mem::forget(entry);
            }
        }
    }
}

impl TextEntry for MultilineEntry {
    fn value(&self, _ctx: &UI) -> String {
        unsafe { from_toolkit_string(ui_sys::uiMultilineEntryText(self.uiMultilineEntry)) }
    }

    fn set_value(&mut self, _ctx: &UI, value: &str) {
        let cstring = to_toolkit_string(value);
        unsafe { ui_sys::uiMultilineEntrySetText(self.uiMultilineEntry, cstring.as_ptr()) }
    }

    fn on_changed<'ctx, F>(&mut self, _ctx: &'ctx UI, callback: F)
    where
        F: FnMut(String) + 'static,
    {
        extern "C" fn c_callback<G>(entry: *mut uiMultilineEntry, data: *mut c_void)
        where
            G: FnMut(String),
        {
            let string = unsafe { CStr::from_ptr(ui_sys::uiMultilineEntryText(entry)) }
                .to_string_lossy()
                .into_owned();
            unsafe { from_void_ptr::<G>(data)(string) }
        }

        unsafe {
            ui_sys::uiMultilineEntryOnChanged(
                self.uiMultilineEntry,
                Some(c_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }
}
