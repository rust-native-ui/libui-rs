//! Provides selection buttons with complex built-in interfaces, like color and font pickers.

use draw;
use ffi_utils;
use libc::c_void;
use std::mem;
use ui_sys::{self, uiColorButton, uiControl, uiFontButton};
use super::Control;

define_control!{
    /// A control which allows users to select a font.
    control(FontButton, uiFontButton, ui_font_button);
}

impl FontButton {
    /// Returns a new font.
    #[inline]
    pub fn font(&self) -> draw::text::Font {
        ffi_utils::ensure_initialized();
        unsafe {
            draw::text::Font::from_ui_draw_text_font(ui_sys::uiFontButtonFont(self.ui_font_button))
        }
    }

    #[inline]
    pub fn on_changed<F: FnMut(&FontButton)>(&self, callback: F) {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut data: Box<Box<FnMut(&FontButton)>> = Box::new(Box::new(callback));
            ui_sys::uiFontButtonOnChanged(
                self.ui_font_button,
                c_callback,
                &mut *data as *mut Box<FnMut(&FontButton)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(ui_font_button: *mut uiFontButton, data: *mut c_void) {
            unsafe {
                let font_button = FontButton::from_ui_control(ui_font_button);
                mem::transmute::<*mut c_void, &mut Box<FnMut(&FontButton)>>(data)(&font_button);
                mem::forget(font_button);
            }
        }
    }

    #[inline]
    pub fn new() -> FontButton {
        ffi_utils::ensure_initialized();
        unsafe { FontButton::from_ui_control(ui_sys::uiNewFontButton()) }
    }
}

define_control!{
    /// A control which allows the user to select a color.
    control(ColorButton, uiColorButton, ui_color_button);
}

impl ColorButton {
    #[inline]
    pub fn color(&self) -> Color {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut color: Color = mem::uninitialized();
            ui_sys::uiColorButtonColor(
                self.ui_color_button,
                &mut color.r,
                &mut color.g,
                &mut color.b,
                &mut color.a,
            );
            color
        }
    }

    #[inline]
    pub fn set_color(&self, color: &Color) {
        ffi_utils::ensure_initialized();
        unsafe {
            ui_sys::uiColorButtonSetColor(self.ui_color_button, color.r, color.g, color.b, color.a)
        }
    }

    #[inline]
    pub fn on_changed<F: FnMut(&ColorButton)>(&self, callback: F) {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut data: Box<Box<FnMut(&ColorButton)>> = Box::new(Box::new(callback));
            ui_sys::uiColorButtonOnChanged(
                self.ui_color_button,
                c_callback,
                &mut *data as *mut Box<FnMut(&ColorButton)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(ui_color_button: *mut uiColorButton, data: *mut c_void) {
            unsafe {
                let color_button = ColorButton::from_ui_control(ui_color_button);
                mem::transmute::<*mut c_void, &mut Box<FnMut(&ColorButton)>>(data)(&color_button);
                mem::forget(color_button)
            }
        }
    }

    #[inline]
    pub fn new() -> ColorButton {
        ffi_utils::ensure_initialized();
        unsafe { ColorButton::from_ui_control(ui_sys::uiNewColorButton()) }
    }
}

/// A RGBA color, returned by the color selector button.
#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Color {
    r: f64,
    g: f64,
    b: f64,
    a: f64,
}
