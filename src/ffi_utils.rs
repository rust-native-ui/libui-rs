//! Useful utility functions for calling the `libui` C bindings.

use ffi;
use libc::{c_char, c_void};
use std::ffi::CStr;
use std::mem;
use std::ops::Deref;

pub struct Text {
    ui_text: *mut c_char,
}

impl Drop for Text {
    fn drop(&mut self) {
        unsafe {
            ffi::uiFreeText(self.ui_text)
        }
    }
}

impl Deref for Text {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.ui_text).to_str().unwrap_or("")
        }
    }
}

impl Text {
    pub unsafe fn new(text: *mut c_char) -> Text {
        Text {
            ui_text: text,
        }
    }
}

pub extern "C" fn void_void_callback(data: *mut c_void) {
    unsafe {
        mem::transmute::<*mut c_void, Box<Box<FnMut()>>>(data)()
    }
}

