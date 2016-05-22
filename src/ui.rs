//! General functions.

use ffi::{self, uiInitOptions};
use ffi_utils;
use libc::{c_char, c_void};
use std::ffi::CStr;
use std::mem;
use std::ops::Deref;

#[derive(Clone)]
pub struct InitOptions;

#[inline]
pub fn init(_: InitOptions) -> Result<(),InitError> {
    let mut init_options = uiInitOptions {
        Size: mem::size_of::<uiInitOptions>(),
    };
    let err = unsafe {
        ffi::uiInit(&mut init_options)
    };
    if err.is_null() {
        Ok(())
    } else {
        Err(InitError {
            ui_init_error: err,
        })
    }
}

#[inline]
pub fn uninit() {
    unsafe {
        ffi::uiUninit();
    }
}

#[inline]
pub fn main() {
    unsafe {
        ffi::uiMain()
    }
}

#[inline]
pub fn quit() {
    unsafe {
        ffi::uiQuit()
    }
}

pub struct InitError {
    ui_init_error: *const c_char,
}

impl Drop for InitError {
    fn drop(&mut self) {
        unsafe {
            ffi::uiFreeInitError(self.ui_init_error)
        }
    }
}

impl Deref for InitError {
    type Target = str;
    fn deref(&self) -> &str {
        unsafe {
            CStr::from_ptr(self.ui_init_error).to_str().unwrap_or("")
        }
    }
}

#[inline]
pub fn queue_main(callback: Box<FnMut()>) {
    unsafe {
        let mut data: Box<Box<FnMut()>> = Box::new(callback);
        ffi::uiQueueMain(ffi_utils::void_void_callback,
                         &mut *data as *mut Box<FnMut()> as *mut c_void);
        mem::forget(data);
    }
}

#[inline]
pub fn on_should_quit(callback: Box<FnMut()>) {
    unsafe {
        let mut data: Box<Box<FnMut()>> = Box::new(callback);
        ffi::uiOnShouldQuit(ffi_utils::void_void_callback,
                            &mut *data as *mut Box<FnMut()> as *mut c_void);
        mem::forget(data);
    }
}

