//! User input mechanisms: numbers, colors, and text in various forms.

use std::mem;
use std::ffi::{CString};
use std::i64;
use libc::c_void;
use ui_sys::{self, uiControl, uiSpinbox, uiSlider};
use super::Control;
use ui::UI;

pub trait NumericEntry {
    fn value(&self, ctx: &UI) -> i64;
    fn set_value(&self, ctx: &UI, value: i64);
    fn on_changed<F: FnMut(i64)>(&self, ctx: &UI, callback: F);
}

pub trait TextEntry {
    fn value(&self, ctx: &UI) -> String;
    fn set_value(&self, ctx: &UI, value: &str);
    fn on_changed<F: FnMut(String)>(&self, ctx: &UI, callback: F);
}

define_control!{
    /// Numerical entry control which allows users to set any value in a range.
    rust_type: Spinbox, 
    sys_type: uiSpinbox
}

define_control!{ 
    /// Allows users to select a value by picking a location along a line.
    rust_type: Slider, 
    sys_type: uiSlider
}

impl Spinbox {
    // Create a new Spinbox which can produce values from `min` to `max`.
    pub fn new(_ctx: &UI, min: i64, max: i64) -> Self {
        unsafe { Spinbox::from_raw(ui_sys::uiNewSpinbox(min, max)) }
    }

    // Create a new Spinbox with the maximum possible range.
    pub fn new_unlimited(_ctx: &UI) -> Self {
        Self::new(_ctx, i64::MIN, i64::MAX)
    }
}

impl Slider {
    // Create a new Spinbox which can produce values from `min` to `max`.
    pub fn new(_ctx: &UI, min: i64, max: i64) -> Self {
        unsafe { Slider::from_raw(ui_sys::uiNewSlider(min, max)) }
    }
}

impl NumericEntry for Spinbox {
    fn value(&self, _ctx: &UI) -> i64 {
        unsafe { ui_sys::uiSpinboxValue(self.uiSpinbox) }
    }

    fn set_value(&self, _ctx: &UI, value: i64) {
        unsafe { ui_sys::uiSpinboxSetValue(self.uiSpinbox, value) }
    }

    fn on_changed<F: FnMut(i64)>(&self, _ctx: &UI, callback: F) {
        unsafe {
            let mut data: Box<Box<FnMut(i64)>> = Box::new(Box::new(callback));
            ui_sys::uiSpinboxOnChanged(
                self.uiSpinbox,
                c_callback,
                &mut *data as *mut Box<FnMut(i64)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(spinbox: *mut uiSpinbox, data: *mut c_void) {
            unsafe {
                let val = ui_sys::uiSpinboxValue(spinbox);
                mem::transmute::<*mut c_void, &mut Box<FnMut(i64)>>(data)(val);
            }
        }
    }
}

impl NumericEntry for Slider {
    fn value(&self, _ctx: &UI) -> i64 {
        unsafe { ui_sys::uiSliderValue(self.uiSlider) }
    }

    fn set_value(&self, _ctx: &UI, value: i64) {
        unsafe { ui_sys::uiSliderSetValue(self.uiSlider, value) }
    }

    fn on_changed<F: FnMut(i64)>(&self, _ctx: &UI, callback: F) {
        unsafe {
            let mut data: Box<Box<FnMut(i64)>> = Box::new(Box::new(callback));
            ui_sys::uiSliderOnChanged(
                self.uiSlider,
                c_callback,
                &mut *data as *mut Box<FnMut(i64)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(slider: *mut uiSlider, data: *mut c_void) {
            unsafe {
                let val = ui_sys::uiSliderValue(slider);
                mem::transmute::<*mut c_void, &mut Box<FnMut(i64)>>(data)(val);
            }
        }
    }
}
