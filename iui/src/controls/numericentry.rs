//! Numeric input mechanisms in various forms.
//!

use super::Control;
use callback_helpers::{from_void_ptr, to_heap_ptr};
use std::i32;
use std::mem;
use std::os::raw::c_void;
use ui::UI;
use ui_sys::{self, uiControl, uiSlider, uiSpinbox};

pub trait NumericEntry {
    fn value(&self, ctx: &UI) -> i32;
    fn set_value(&mut self, ctx: &UI, value: i32);
    fn on_changed<'ctx, F: FnMut(i32) + 'static>(&mut self, ctx: &'ctx UI, callback: F);
}

define_control! {
    /// Numerical entry control which allows users to set any value in a range by typing or incrementing/decrementing.
    rust_type: Spinbox,
    sys_type: uiSpinbox
}

define_control! {
    /// Numerical entry which allows users to select a value by picking a location along a line.
    rust_type: Slider,
    sys_type: uiSlider
}

impl Spinbox {
    // Create a new Spinbox which can produce values from `min` to `max`.
    pub fn new(_ctx: &UI, min: i32, max: i32) -> Self {
        unsafe { Spinbox::from_raw(ui_sys::uiNewSpinbox(min, max)) }
    }

    // Create a new Spinbox with the maximum possible range.
    pub fn new_unlimited(_ctx: &UI) -> Self {
        Self::new(_ctx, i32::MIN, i32::MAX)
    }
}

impl Slider {
    // Create a new Spinbox which can produce values from `min` to `max`.
    pub fn new(_ctx: &UI, min: i32, max: i32) -> Self {
        unsafe { Slider::from_raw(ui_sys::uiNewSlider(min, max)) }
    }
}

impl NumericEntry for Spinbox {
    fn value(&self, _ctx: &UI) -> i32 {
        unsafe { ui_sys::uiSpinboxValue(self.uiSpinbox) }
    }

    fn set_value(&mut self, _ctx: &UI, value: i32) {
        unsafe { ui_sys::uiSpinboxSetValue(self.uiSpinbox, value) }
    }

    fn on_changed<'ctx, F>(&mut self, _ctx: &'ctx UI, callback: F)
    where
        F: FnMut(i32) + 'static,
    {
        extern "C" fn c_callback<G>(spinbox: *mut uiSpinbox, data: *mut c_void)
        where
            G: FnMut(i32),
        {
            let val = unsafe { ui_sys::uiSpinboxValue(spinbox) };
            unsafe {
                from_void_ptr::<G>(data)(val);
            }
        }

        unsafe {
            ui_sys::uiSpinboxOnChanged(
                self.uiSpinbox,
                Some(c_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }
}

impl NumericEntry for Slider {
    fn value(&self, _ctx: &UI) -> i32 {
        unsafe { ui_sys::uiSliderValue(self.uiSlider) }
    }

    fn set_value(&mut self, _ctx: &UI, value: i32) {
        unsafe { ui_sys::uiSliderSetValue(self.uiSlider, value) }
    }

    fn on_changed<'ctx, F>(&mut self, _ctx: &'ctx UI, callback: F)
    where
        F: FnMut(i32) + 'static,
    {
        extern "C" fn c_callback<G>(slider: *mut uiSlider, data: *mut c_void)
        where
            G: FnMut(i32),
        {
            let val = unsafe { ui_sys::uiSliderValue(slider) };
            unsafe {
                from_void_ptr::<G>(data)(val);
            }
        }

        unsafe {
            ui_sys::uiSliderOnChanged(self.uiSlider, Some(c_callback::<F>), to_heap_ptr(callback));
        }
    }
}
