//! Low-level bindings to API-specific functions for interfacing with foreign controls on Unix
//! systems that use GTK+ to provide their UI (currently all except Mac OS X).

#![allow(non_camel_case_types, non_snake_case)]

use libc::{c_char, c_int, size_t};
use uiControl;

#[repr(C)]
pub struct uiUnixControl {
    pub c: uiControl,
    pub parent: *mut uiControl,
    pub addedBefore: gboolean,
    pub SetContainer: extern "C" fn(*mut uiUnixControl, *mut GtkContainer, gboolean),
}

#[link(name = "ui")]
extern {
    pub fn uiUnixControlSetContainer(control: *mut uiUnixControl,
                                     container: *mut GtkContainer,
                                     addBefore: gboolean);
    pub fn uiUnixAllocControl(n: size_t, typesig: u32, typenamestr: *const c_char)
                              -> *mut uiUnixControl;
    pub fn uiUnixStrdupText(text: *const c_char) -> *mut c_char;
}

pub type gboolean = gint;

pub type gint = c_int;

pub enum GtkContainer {}

