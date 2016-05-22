//! Functions and types related to widgets.

use ffi::{self, uiBox, uiButton, uiControl, uiEntry};
use ffi_utils::Text;
use libc::{c_int, c_void};
use std::ffi::CString;
use std::mem;
use std::ops::Deref;
use std::ptr;

/// FIXME(pcwalton): We need to reference count these for memory safety!
#[derive(Clone)]
pub struct Control {
    ui_control: *mut uiControl,
}

impl Control {
    #[inline]
    pub fn as_ui_control(&self) -> *mut uiControl {
        self.ui_control
    }

    /// FIXME(pcwalton): Offer a safe way to destroy controls.
    #[inline]
    pub unsafe fn destroy(&self) {
        ffi::uiControlDestroy(self.ui_control)
    }

    #[inline]
    pub fn handle(&self) -> usize {
        unsafe {
            ffi::uiControlHandle(self.ui_control)
        }
    }

    #[inline]
    pub fn parent(&self) -> Option<Control> {
        let ui_control = unsafe {
            ffi::uiControlParent(self.ui_control)
        };
        if ui_control.is_null() {
            None
        } else {
            Some(Control {
                ui_control: ui_control,
            })
        }
    }

    #[inline]
    pub fn set_parent(&self, parent: Option<Control>) {
        unsafe {
            ffi::uiControlSetParent(self.ui_control,
                                    match parent {
                                        None => ptr::null_mut(),
                                        Some(parent) => parent.ui_control,
                                    })
        }
    }

    #[inline]
    pub fn toplevel(&self) -> bool {
        unsafe {
            ffi::uiControlToplevel(self.ui_control) != 0
        }
    }

    #[inline]
    pub fn visible(&self) -> bool {
        unsafe {
            ffi::uiControlVisible(self.ui_control) != 0
        }
    }

    #[inline]
    pub fn show(&self) {
        unsafe {
            ffi::uiControlShow(self.ui_control)
        }
    }

    #[inline]
    pub fn hide(&self) {
        unsafe {
            ffi::uiControlHide(self.ui_control)
        }
    }

    #[inline]
    pub fn enabled(&self) -> bool {
        unsafe {
            ffi::uiControlEnabled(self.ui_control) != 0
        }
    }

    #[inline]
    pub fn enable(&self) {
        unsafe {
            ffi::uiControlEnable(self.ui_control)
        }
    }

    #[inline]
    pub fn disable(&self) {
        unsafe {
            ffi::uiControlDisable(self.ui_control)
        }
    }
}

#[derive(Clone)]
pub struct Button {
    ui_button: *mut uiButton,
}

impl Deref for Button {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&Button, &Control>(self)
        }
    }
}

impl Button {
    #[inline]
    pub fn text(&self) -> Text {
        unsafe {
            Text::new(ffi::uiButtonText(self.ui_button))
        }
    }

    #[inline]
    pub fn set_text(&self, text: &str) {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            ffi::uiButtonSetText(self.ui_button, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn on_clicked(&self, callback: Box<FnMut(Button)>) {
        unsafe {
            let mut data: Box<Box<FnMut(Button)>> = Box::new(callback);
            ffi::uiButtonOnClicked(self.ui_button,
                                   c_callback,
                                   &mut *data as *mut Box<FnMut(Button)> as *mut c_void);
            mem::forget(data);
        }

        extern "C" fn c_callback(button: *mut uiButton, data: *mut c_void) {
            unsafe {
                let button = Button {
                    ui_button: button,
                };
                mem::transmute::<*mut c_void, Box<Box<FnMut(Button)>>>(data)(button)
            }
        }
    }

    #[inline]
    pub fn new(text: &str) -> Button {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            Button {
                ui_button: ffi::uiNewButton(c_string.as_ptr()),
            }
        }
    }
}

#[derive(Clone)]
pub struct BoxControl {
    ui_box: *mut uiBox,
}

impl Deref for BoxControl {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&BoxControl, &Control>(self)
        }
    }
}

impl BoxControl {
    #[inline]
    pub fn append(&self, child: Control, stretchy: bool) {
        unsafe {
            ffi::uiBoxAppend(self.ui_box, child.ui_control, stretchy as c_int)
        }
    }

    #[inline]
    pub fn delete(&self, index: u64) {
        unsafe {
            ffi::uiBoxDelete(self.ui_box, index)
        }
    }

    #[inline]
    pub fn padded(&self) -> bool {
        unsafe {
            ffi::uiBoxPadded(self.ui_box) != 0
        }
    }

    #[inline]
    pub fn set_padded(&self, padded: bool) {
        unsafe {
            ffi::uiBoxSetPadded(self.ui_box, padded as c_int)
        }
    }

    #[inline]
    pub fn new_horizontal() -> BoxControl {
        unsafe {
            BoxControl {
                ui_box: ffi::uiNewHorizontalBox(),
            }
        }
    }

    #[inline]
    pub fn new_vertical() -> BoxControl {
        unsafe {
            BoxControl {
                ui_box: ffi::uiNewVerticalBox(),
            }
        }
    }
}

#[derive(Clone)]
pub struct Entry {
    ui_entry: *mut uiEntry,
}

impl Deref for Entry {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&Entry, &Control>(self)
        }
    }
}

impl Entry {
    #[inline]
    pub fn text(&self) -> Text {
        unsafe {
            Text::new(ffi::uiEntryText(self.ui_entry))
        }
    }

    #[inline]
    pub fn set_text(&self, text: &str) {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            ffi::uiEntrySetText(self.ui_entry, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn read_only(&self) -> bool {
        unsafe {
            ffi::uiEntryReadOnly(self.ui_entry) != 0
        }
    }

    #[inline]
    pub fn set_read_only(&self, readonly: bool) {
        unsafe {
            ffi::uiEntrySetReadOnly(self.ui_entry, readonly as c_int)
        }
    }

    #[inline]
    pub fn new() -> Entry {
        unsafe {
            Entry {
                ui_entry: ffi::uiNewEntry(),
            }
        }
    }
}

