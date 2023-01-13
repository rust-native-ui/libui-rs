use super::Control;
use callback_helpers::{from_void_ptr, to_heap_ptr};
use std::i32;
use std::mem;
use std::os::raw::c_void;
use str_tools::to_toolkit_string;
use ui::UI;
use ui_sys::{self, uiCheckbox, uiControl};

define_control! {
    /// Boolean selection control which can be checked or unchecked.
    rust_type: Checkbox,
    sys_type: uiCheckbox
}

impl Checkbox {
    // Create a new Checkbox which can produce values from `min` to `max`.
    pub fn new(_ctx: &UI, text: &str) -> Self {
        let c_string = to_toolkit_string(text);
        unsafe { Checkbox::from_raw(ui_sys::uiNewCheckbox(c_string.as_ptr())) }
    }

    pub fn checked(&self, _ctx: &UI) -> bool {
        unsafe { ui_sys::uiCheckboxChecked(self.uiCheckbox) != 0 }
    }

    pub fn set_checked(&mut self, _ctx: &UI, checked: bool) {
        unsafe { ui_sys::uiCheckboxSetChecked(self.uiCheckbox, checked as i32) }
    }

    pub fn on_toggled<'ctx, F>(&mut self, _ctx: &'ctx UI, callback: F)
    where
        F: FnMut(bool) + 'static,
    {
        extern "C" fn c_callback<G>(checkbox: *mut uiCheckbox, data: *mut c_void)
        where
            G: FnMut(bool),
        {
            let val = unsafe { ui_sys::uiCheckboxChecked(checkbox) } != 0;
            unsafe { from_void_ptr::<G>(data)(val) }
        }

        unsafe {
            ui_sys::uiCheckboxOnToggled(
                self.uiCheckbox,
                Some(c_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }
}
