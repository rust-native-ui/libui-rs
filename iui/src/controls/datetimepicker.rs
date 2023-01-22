use super::Control;
use callback_helpers::{from_void_ptr, to_heap_ptr};
use std::mem;
use std::os::raw::c_void;
use ui::UI;
use ui_sys::{self, uiControl, uiDateTimePicker};

define_control! {
    /// Allows to enter a date and/or time.
    rust_type: DateTimePicker,
    sys_type: uiDateTimePicker
}

pub enum DateTimePickerKind {
    DateTime,
    Date,
    Time,
}

impl DateTimePicker {
    /// Create a new date and/or time picker.
    pub fn new(_ctx: &UI, mode: DateTimePickerKind) -> DateTimePicker {
        unsafe {
            DateTimePicker::from_raw(match mode {
                DateTimePickerKind::DateTime => ui_sys::uiNewDateTimePicker(),
                DateTimePickerKind::Date => ui_sys::uiNewDatePicker(),
                DateTimePickerKind::Time => ui_sys::uiNewTimePicker(),
            })
        }
    }

    /// Returns the date and/or time stored in the DateTimePicker.
    ///
    /// Depending on the `DateTimePickerKind` you created, the date or time fields
    /// will not be set and instead contain their unix epoch default.
    ///
    /// Warning: The `struct tm` member `tm_isdst` is unused on Windows and will be `-1`.
    pub fn datetime(&self, _ctx: &UI) -> libc::tm {
        unsafe {
            let mut datetime = libc::tm {
                tm_sec: 0,
                tm_min: 0,
                tm_hour: 0,
                tm_mday: 0,
                tm_mon: 0,
                tm_year: 0,
                tm_wday: 0,
                tm_yday: 0,
                tm_isdst: 0,
                tm_gmtoff: 0,
                tm_zone: std::ptr::null(),
            };
            let ptr = &mut datetime as *mut libc::tm;
            ui_sys::uiDateTimePickerTime(self.uiDateTimePicker, ptr as *mut ui_sys::tm);
            datetime
        }
    }

    /// Sets date and time of the DateTimePicker.
    ///
    /// Warning: The `struct tm` member `tm_isdst` is ignored on Windows and should be set to `-1`
    pub fn set_datetime(&self, _ctx: &UI, datetime: libc::tm) {
        unsafe {
            let ptr = &datetime as *const libc::tm;
            ui_sys::uiDateTimePickerSetTime(self.uiDateTimePicker, ptr as *const ui_sys::tm);
        }
    }

    /// Registers a callback for when the date time picker value is changed by the user.
    ///
    /// The callback is not triggered when calling `set_datetime()`.
    /// Only one callback can be registered at a time.
    pub fn on_changed<'ctx, F>(&mut self, _ctx: &'ctx UI, callback: F)
    where
        F: FnMut(&mut DateTimePicker) + 'static,
    {
        extern "C" fn c_callback<G>(picker: *mut uiDateTimePicker, data: *mut c_void)
        where
            G: FnMut(&mut DateTimePicker),
        {
            let mut picker = DateTimePicker {
                uiDateTimePicker: picker,
            };
            unsafe {
                from_void_ptr::<G>(data)(&mut picker);
            }
        }
        unsafe {
            ui_sys::uiDateTimePickerOnChanged(
                self.uiDateTimePicker,
                Some(c_callback::<F>),
                to_heap_ptr(callback),
            );
        }
    }
}
