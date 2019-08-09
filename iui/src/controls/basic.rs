use super::Control;
use std::os::raw::c_void;
use std::ffi::{CStr, CString};
use std::mem;
use ui::UI;
use ui_sys::{self, uiButton, uiColorButton, uiControl, uiLabel};

#[derive(Debug)]
pub struct Rgba {
    pub r: f64,
    pub g: f64,
    pub b: f64,
    pub a: f64,
}

define_control!{
    /// A non-interactable piece of text.
    rust_type: Label,
    sys_type: uiLabel
}

define_control!{
    /// A textual button which users can click on, causing a callback to run.
    rust_type: Button,
    sys_type: uiButton
}

define_control!{
    /// A color chooser button which users can click on, causing a callback to run.
    rust_type: ColorButton,
    sys_type: uiColorButton
}

impl Button {
    /// Create a new button with the given text as its label.
    pub fn new(_ctx: &UI, text: &str) -> Button {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            Button::from_raw(ui_sys::uiNewButton(c_string.as_ptr()))
        }
    }

    /// Get a copy of the existing text on the button.
    pub fn text(&self, _ctx: &UI) -> String {
        unsafe {
            CStr::from_ptr(ui_sys::uiButtonText(self.uiButton))
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Get a reference to the existing text on the button.
    pub fn text_ref(&self, _ctx: &UI) -> &CStr {
        unsafe { CStr::from_ptr(ui_sys::uiButtonText(self.uiButton)) }
    }

    /// Set the text on the button.
    pub fn set_text(&mut self, _ctx: &UI, text: &str) {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            ui_sys::uiButtonSetText(self.uiButton, c_string.as_ptr())
        }
    }

    /// Run the given callback when the button is clicked.
    pub fn on_clicked<'ctx, F: FnMut(&mut Button) + 'ctx>(&mut self, _ctx: &'ctx UI, callback: F) {
        unsafe {
            let mut data: Box<Box<FnMut(&mut Button)>> = Box::new(Box::new(callback));
            ui_sys::uiButtonOnClicked(
                self.uiButton,
                Some(c_callback),
                &mut *data as *mut Box<FnMut(&mut Button)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(button: *mut uiButton, data: *mut c_void) {
            unsafe {
                let mut button = Button { uiButton: button };
                mem::transmute::<*mut c_void, &mut Box<FnMut(&mut Button)>>(data)(&mut button)
            }
        }
    }
}

impl ColorButton {
    /// Create a new color button
    pub fn new(_ctx: &UI) -> ColorButton {
        unsafe { ColorButton::from_raw(ui_sys::uiNewColorButton()) }
    }

    pub fn color(&self, _ctx: &UI) -> Rgba {
        let mut r: f64 = 0.0;
        let mut g: f64 = 0.0;
        let mut b: f64 = 0.0;
        let mut a: f64 = 0.0;

        unsafe {
            ui_sys::uiColorButtonColor(self.uiColorButton, &mut r, &mut g, &mut b, &mut a);
            Rgba { r, g, b, a }
        }
    }

    /// Set the text on the button.
    pub fn set_color(&mut self, _ctx: &UI, color: Rgba) {
        unsafe {
            ui_sys::uiColorButtonSetColor(self.uiColorButton, color.r, color.g, color.b, color.a)
        }
    }

    /// Run the given callback when the color button is clicked.
    pub fn on_changed<'ctx, F: FnMut(&mut ColorButton) + 'ctx>(
        &mut self,
        _ctx: &'ctx UI,
        callback: F,
    ) {
        unsafe {
            let mut data: Box<Box<FnMut(&mut ColorButton)>> = Box::new(Box::new(callback));
            ui_sys::uiColorButtonOnChanged(
                self.uiColorButton,
                Some(c_callback),
                &mut *data as *mut Box<FnMut(&mut ColorButton)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(button: *mut uiColorButton, data: *mut c_void) {
            unsafe {
                let mut button = ColorButton {
                    uiColorButton: button,
                };
                mem::transmute::<*mut c_void, &mut Box<FnMut(&mut ColorButton)>>(data)(&mut button)
            }
        }
    }
}

impl Label {
    /// Create a new label with the given string as its text.
    /// Note that labels do not auto-wrap their text; they will expand as far as needed
    /// to fit.
    pub fn new(_ctx: &UI, text: &str) -> Label {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            Label::from_raw(ui_sys::uiNewLabel(c_string.as_ptr()))
        }
    }

    /// Get a copy of the existing text on the label.
    pub fn text(&self, _ctx: &UI) -> String {
        unsafe {
            CStr::from_ptr(ui_sys::uiLabelText(self.uiLabel))
                .to_string_lossy()
                .into_owned()
        }
    }

    /// Get a reference to the existing text on the label.
    pub fn text_ref(&self, _ctx: &UI) -> &CStr {
        unsafe { CStr::from_ptr(ui_sys::uiLabelText(self.uiLabel)) }
    }

    /// Set the text on the label.
    pub fn set_text(&mut self, _ctx: &UI, text: &str) {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            ui_sys::uiLabelSetText(self.uiLabel, c_string.as_ptr())
        }
    }
}
