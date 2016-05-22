//! Functions and types related to widgets.

use draw;
use ffi::{self, uiArea, uiAreaDrawParams, uiAreaHandler, uiAreaKeyEvent, uiAreaMouseEvent, uiBox};
use ffi::{uiButton, uiCheckbox, uiCombobox, uiControl, uiDateTimePicker, uiEntry, uiGroup};
use ffi::{uiLabel, uiMultilineEntry, uiProgressBar, uiRadioButtons, uiSeparator, uiSlider};
use ffi::{uiSpinbox, uiTab};
use ffi_utils::Text;
use libc::{c_int, c_void};
use std::ffi::CString;
use std::mem;
use std::ops::Deref;
use std::ptr;

pub use ffi::uiExtKey as ExtKey;

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
    pub fn set_parent(&self, parent: Option<&Control>) {
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
    pub fn on_clicked(&self, callback: Box<FnMut(&Button)>) {
        unsafe {
            let mut data: Box<Box<FnMut(&Button)>> = Box::new(callback);
            ffi::uiButtonOnClicked(self.ui_button,
                                   c_callback,
                                   &mut *data as *mut Box<FnMut(&Button)> as *mut c_void);
            mem::forget(data);
        }

        extern "C" fn c_callback(button: *mut uiButton, data: *mut c_void) {
            unsafe {
                let button = Button {
                    ui_button: button,
                };
                mem::transmute::<*mut c_void, &mut Box<FnMut(&Button)>>(data)(&button)
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
    pub fn append(&self, child: &Control, stretchy: bool) {
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
    pub fn on_changed(&self, callback: Box<FnMut(&Entry)>) {
        unsafe {
            let mut data: Box<Box<FnMut(&Entry)>> = Box::new(callback);
            ffi::uiEntryOnChanged(self.ui_entry,
                                  c_callback,
                                  &mut *data as *mut Box<FnMut(&Entry)> as *mut c_void);
            mem::forget(data);
        }

        extern "C" fn c_callback(entry: *mut uiEntry, data: *mut c_void) {
            unsafe {
                let entry = Entry {
                    ui_entry: entry,
                };
                mem::transmute::<*mut c_void, &mut Box<FnMut(&Entry)>>(data)(&entry)
            }
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

#[derive(Clone)]
pub struct Checkbox {
    ui_checkbox: *mut uiCheckbox,
}

impl Deref for Checkbox {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&Checkbox, &Control>(self)
        }
    }
}

impl Checkbox {
    #[inline]
    pub fn text(&self) -> Text {
        unsafe {
            Text::new(ffi::uiCheckboxText(self.ui_checkbox))
        }
    }

    #[inline]
    pub fn set_text(&self, text: &str) {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            ffi::uiCheckboxSetText(self.ui_checkbox, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn on_toggled(&self, callback: Box<FnMut(&Checkbox)>) {
        unsafe {
            let mut data: Box<Box<FnMut(&Checkbox)>> = Box::new(callback);
            ffi::uiCheckboxOnToggled(self.ui_checkbox,
                                     c_callback,
                                     &mut *data as *mut Box<FnMut(&Checkbox)> as *mut c_void);
            mem::forget(data);
        }

        extern "C" fn c_callback(checkbox: *mut uiCheckbox, data: *mut c_void) {
            unsafe {
                let checkbox = Checkbox {
                    ui_checkbox: checkbox,
                };
                mem::transmute::<*mut c_void, &mut Box<FnMut(&Checkbox)>>(data)(&checkbox)
            }
        }
    }

    #[inline]
    pub fn checked(&self) -> bool {
        unsafe {
            ffi::uiCheckboxChecked(self.ui_checkbox) != 0
        }
    }

    #[inline]
    pub fn set_checked(&self, checked: bool) {
        unsafe {
            ffi::uiCheckboxSetChecked(self.ui_checkbox, checked as c_int)
        }
    }

    #[inline]
    pub fn new(text: &str) -> Checkbox {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            Checkbox {
                ui_checkbox: ffi::uiNewCheckbox(c_string.as_ptr()),
            }
        }
    }
}

#[derive(Clone)]
pub struct Label {
    ui_label: *mut uiLabel,
}

impl Deref for Label {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&Label, &Control>(self)
        }
    }
}

impl Label {
    #[inline]
    pub fn text(&self) -> Text {
        unsafe {
            Text::new(ffi::uiLabelText(self.ui_label))
        }
    }

    #[inline]
    pub fn set_text(&self, text: &str) {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            ffi::uiLabelSetText(self.ui_label, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn new(text: &str) -> Label {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            Label {
                ui_label: ffi::uiNewLabel(c_string.as_ptr()),
            }
        }
    }
}

#[derive(Clone)]
pub struct Tab {
    ui_tab: *mut uiTab,
}

impl Deref for Tab {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&Tab, &Control>(self)
        }
    }
}

impl Tab {
    #[inline]
    pub fn append(&self, name: &str, control: &Control) {
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            ffi::uiTabAppend(self.ui_tab, c_string.as_ptr(), control.ui_control)
        }
    }

    #[inline]
    pub fn insert_at(&self, name: &str, before: u64, control: &Control) {
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            ffi::uiTabInsertAt(self.ui_tab, c_string.as_ptr(), before, control.ui_control)
        }
    }

    #[inline]
    pub fn delete(&self, index: u64) {
        unsafe {
            ffi::uiTabDelete(self.ui_tab, index)
        }
    }

    #[inline]
    pub fn margined(&self, page: u64) -> bool {
        unsafe {
            ffi::uiTabMargined(self.ui_tab, page) != 0
        }
    }

    #[inline]
    pub fn set_margined(&self, page: u64, margined: bool) {
        unsafe {
            ffi::uiTabSetMargined(self.ui_tab, page, margined as c_int)
        }
    }

    #[inline]
    pub fn new() -> Tab {
        unsafe {
            Tab {
                ui_tab: ffi::uiNewTab(),
            }
        }
    }
}

#[derive(Clone)]
pub struct Group {
    ui_group: *mut uiGroup,
}

impl Deref for Group {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&Group, &Control>(self)
        }
    }
}

impl Group {
    #[inline]
    pub fn title(&self) -> Text {
        unsafe {
            Text::new(ffi::uiGroupTitle(self.ui_group))
        }
    }

    #[inline]
    pub fn set_title(&self, title: &str) {
        unsafe {
            let c_string = CString::new(title.as_bytes().to_vec()).unwrap();
            ffi::uiGroupSetTitle(self.ui_group, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn set_child(&self, child: &Control) {
        unsafe {
            ffi::uiGroupSetChild(self.ui_group, child.ui_control)
        }
    }

    #[inline]
    pub fn margined(&self) -> bool {
        unsafe {
            ffi::uiGroupMargined(self.ui_group) != 0
        }
    }

    #[inline]
    pub fn set_margined(&self, margined: bool) {
        unsafe {
            ffi::uiGroupSetMargined(self.ui_group, margined as c_int)
        }
    }

    #[inline]
    pub fn new(title: &str) -> Group {
        unsafe {
            let c_string = CString::new(title.as_bytes().to_vec()).unwrap();
            Group {
                ui_group: ffi::uiNewGroup(c_string.as_ptr()),
            }
        }
    }
}

#[derive(Clone)]
pub struct Spinbox {
    ui_spinbox: *mut uiSpinbox,
}

impl Deref for Spinbox {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&Spinbox, &Control>(self)
        }
    }
}

impl Spinbox {
    #[inline]
    pub fn value(&self) -> i64 {
        unsafe {
            ffi::uiSpinboxValue(self.ui_spinbox)
        }
    }

    #[inline]
    pub fn set_value(&self, value: i64) {
        unsafe {
            ffi::uiSpinboxSetValue(self.ui_spinbox, value)
        }
    }

    #[inline]
    pub fn on_changed(&self, callback: Box<FnMut(&Spinbox)>) {
        unsafe {
            let mut data: Box<Box<FnMut(&Spinbox)>> = Box::new(callback);
            ffi::uiSpinboxOnChanged(self.ui_spinbox,
                                    c_callback,
                                    &mut *data as *mut Box<FnMut(&Spinbox)> as *mut c_void);
            mem::forget(data);
        }

        extern "C" fn c_callback(spinbox: *mut uiSpinbox, data: *mut c_void) {
            unsafe {
                let spinbox = Spinbox {
                    ui_spinbox: spinbox,
                };
                mem::transmute::<*mut c_void, &mut Box<FnMut(&Spinbox)>>(data)(&spinbox)
            }
        }
    }

    #[inline]
    pub fn new(min: i64, max: i64) -> Spinbox {
        unsafe {
            Spinbox {
                ui_spinbox: ffi::uiNewSpinbox(min, max),
            }
        }
    }
}

#[derive(Clone)]
pub struct ProgressBar {
    ui_progress_bar: *mut uiProgressBar,
}

impl Deref for ProgressBar {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&ProgressBar, &Control>(self)
        }
    }
}

impl ProgressBar {
    #[inline]
    pub fn set_value(&self, n: i32) {
        unsafe {
            ffi::uiProgressBarSetValue(self.ui_progress_bar, n)
        }
    }

    #[inline]
    pub fn new() -> ProgressBar {
        unsafe {
            ProgressBar {
                ui_progress_bar: ffi::uiNewProgressBar(),
            }
        }
    }
}

#[derive(Clone)]
pub struct Slider {
    ui_slider: *mut uiSlider,
}

impl Deref for Slider {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&Slider, &Control>(self)
        }
    }
}

impl Slider {
    #[inline]
    pub fn value(&self) -> i64 {
        unsafe {
            ffi::uiSliderValue(self.ui_slider)
        }
    }

    #[inline]
    pub fn set_value(&self, value: i64) {
        unsafe {
            ffi::uiSliderSetValue(self.ui_slider, value)
        }
    }

    #[inline]
    pub fn on_changed(&self, callback: Box<FnMut(&Slider)>) {
        unsafe {
            let mut data: Box<Box<FnMut(&Slider)>> = Box::new(callback);
            ffi::uiSliderOnChanged(self.ui_slider,
                                    c_callback,
                                    &mut *data as *mut Box<FnMut(&Slider)> as *mut c_void);
            mem::forget(data);
        }

        extern "C" fn c_callback(slider: *mut uiSlider, data: *mut c_void) {
            unsafe {
                let slider = Slider {
                    ui_slider: slider,
                };
                mem::transmute::<*mut c_void, &mut Box<FnMut(&Slider)>>(data)(&slider)
            }
        }
    }

    #[inline]
    pub fn new(min: i64, max: i64) -> Slider {
        unsafe {
            Slider {
                ui_slider: ffi::uiNewSlider(min, max),
            }
        }
    }
}

#[derive(Clone)]
pub struct Separator {
    ui_separator: *mut uiSeparator,
}

impl Deref for Separator {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&Separator, &Control>(self)
        }
    }
}

impl Separator {
    #[inline]
    pub fn new_horizontal() -> Separator {
        unsafe {
            Separator {
                ui_separator: ffi::uiNewHorizontalSeparator(),
            }
        }
    }
}

#[derive(Clone)]
pub struct Combobox {
    ui_combobox: *mut uiCombobox,
}

impl Deref for Combobox {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&Combobox, &Control>(self)
        }
    }
}

impl Combobox {
    #[inline]
    pub fn append(&self, name: &str) {
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            ffi::uiComboboxAppend(self.ui_combobox, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn selected(&self) -> i64 {
        unsafe {
            ffi::uiComboboxSelected(self.ui_combobox)
        }
    }

    #[inline]
    pub fn set_selected(&self, n: i64) {
        unsafe {
            ffi::uiComboboxSetSelected(self.ui_combobox, n)
        }
    }

    #[inline]
    pub fn on_selected(&self, callback: Box<FnMut(&Combobox)>) {
        unsafe {
            let mut data: Box<Box<FnMut(&Combobox)>> = Box::new(callback);
            ffi::uiComboboxOnSelected(self.ui_combobox,
                                      c_callback,
                                      &mut *data as *mut Box<FnMut(&Combobox)> as *mut c_void);
            mem::forget(data);
        }

        extern "C" fn c_callback(combobox: *mut uiCombobox, data: *mut c_void) {
            unsafe {
                let combobox = Combobox {
                    ui_combobox: combobox,
                };
                mem::transmute::<*mut c_void, &mut Box<FnMut(&Combobox)>>(data)(&combobox)
            }
        }
    }

    #[inline]
    pub fn new() -> Combobox {
        unsafe {
            Combobox {
                ui_combobox: ffi::uiNewCombobox(),
            }
        }
    }

    #[inline]
    pub fn new_editable() -> Combobox {
        unsafe {
            Combobox {
                ui_combobox: ffi::uiNewEditableCombobox(),
            }
        }
    }
}

/// FIXME(pcwalton): Are these supposed to be a subclass of something? They don't seem very usable
/// with just the `uiRadioButtons*` methods…
#[derive(Clone)]
pub struct RadioButtons {
    ui_radio_buttons: *mut uiRadioButtons,
}

impl Deref for RadioButtons {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&RadioButtons, &Control>(self)
        }
    }
}

impl RadioButtons {
    #[inline]
    pub fn append(&self, name: &str) {
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            ffi::uiRadioButtonsAppend(self.ui_radio_buttons, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn new() -> RadioButtons {
        unsafe {
            RadioButtons {
                ui_radio_buttons: ffi::uiNewRadioButtons(),
            }
        }
    }
}

/// FIXME(pcwalton): Are these supposed to be a subclass of something? They don't seem very usable
/// with just the `uiDatetimePicker*` methods…
#[derive(Clone)]
pub struct DateTimePicker {
    ui_date_time_picker: *mut uiDateTimePicker,
}

impl Deref for DateTimePicker {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&DateTimePicker, &Control>(self)
        }
    }
}

impl DateTimePicker {
    pub fn new_date_time_picker() -> DateTimePicker {
        unsafe {
            DateTimePicker {
                ui_date_time_picker: ffi::uiNewDateTimePicker(),
            }
        }
    }

    pub fn new_date_picker() -> DateTimePicker {
        unsafe {
            DateTimePicker {
                ui_date_time_picker: ffi::uiNewDatePicker(),
            }
        }
    }

    pub fn new_time_picker() -> DateTimePicker {
        unsafe {
            DateTimePicker {
                ui_date_time_picker: ffi::uiNewTimePicker(),
            }
        }
    }
}

#[derive(Clone)]
pub struct MultilineEntry {
    ui_multiline_entry: *mut uiMultilineEntry,
}

impl Deref for MultilineEntry {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&MultilineEntry, &Control>(self)
        }
    }
}

impl MultilineEntry {
    #[inline]
    pub fn text(&self) -> Text {
        unsafe {
            Text::new(ffi::uiMultilineEntryText(self.ui_multiline_entry))
        }
    }

    #[inline]
    pub fn set_text(&self, text: &str) {
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            ffi::uiMultilineEntrySetText(self.ui_multiline_entry, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn on_changed(&self, callback: Box<FnMut(&MultilineEntry)>) {
        unsafe {
            let mut data: Box<Box<FnMut(&MultilineEntry)>> = Box::new(callback);
            ffi::uiMultilineEntryOnChanged(self.ui_multiline_entry,
                                           c_callback,
                                           &mut *data as *mut Box<FnMut(&MultilineEntry)> as
                                           *mut c_void);
            mem::forget(data);
        }

        extern "C" fn c_callback(multiline_entry: *mut uiMultilineEntry, data: *mut c_void) {
            unsafe {
                let multiline_entry = MultilineEntry {
                    ui_multiline_entry: multiline_entry,
                };
                mem::transmute::<*mut c_void,
                                 &mut Box<FnMut(&MultilineEntry)>>(data)(&multiline_entry)
            }
        }
    }

    #[inline]
    pub fn read_only(&self) -> bool {
        unsafe {
            ffi::uiMultilineEntryReadOnly(self.ui_multiline_entry) != 0
        }
    }

    #[inline]
    pub fn set_read_only(&self, readonly: bool) {
        unsafe {
            ffi::uiMultilineEntrySetReadOnly(self.ui_multiline_entry, readonly as c_int)
        }
    }

    #[inline]
    pub fn new() -> MultilineEntry {
        unsafe {
            MultilineEntry {
                ui_multiline_entry: ffi::uiNewMultilineEntry(),
            }
        }
    }
}

pub trait AreaHandler {
    fn draw(&mut self, area: &Area, area_draw_params: &AreaDrawParams);
    fn mouse_event(&mut self, area: &Area, area_mouse_event: &AreaMouseEvent);
    fn mouse_crossed(&mut self, area: &Area, left: bool);
    fn drag_broken(&mut self, area: &Area);
    fn key_event(&mut self, area: &Area, area_key_event: &AreaKeyEvent) -> bool;
}

#[repr(C)]
struct RustAreaHandler {
    ui_area_handler: uiAreaHandler,
    trait_object: Box<AreaHandler>,
}

impl RustAreaHandler {
    #[inline]
    fn new(trait_object: Box<AreaHandler>) -> Box<RustAreaHandler> {
        return Box::new(RustAreaHandler {
            ui_area_handler: uiAreaHandler {
                Draw: draw,
                MouseEvent: mouse_event,
                MouseCrossed: mouse_crossed,
                DragBroken: drag_broken,
                KeyEvent: key_event,
            },
            trait_object: trait_object,
        });

        extern "C" fn draw(ui_area_handler: *mut uiAreaHandler,
                           ui_area: *mut uiArea,
                           ui_area_draw_params: *mut uiAreaDrawParams) {
            unsafe {
                let area = Area::from_ui_area(ui_area);
                let area_draw_params =
                    AreaDrawParams::from_ui_area_draw_params(&*ui_area_draw_params);
                (*(ui_area_handler as *mut RustAreaHandler)).trait_object.draw(&area,
                                                                               &area_draw_params);
                mem::forget(area_draw_params);
                mem::forget(area);
            }
        }

        extern "C" fn mouse_event(ui_area_handler: *mut uiAreaHandler,
                                  ui_area: *mut uiArea,
                                  ui_area_mouse_event: *mut uiAreaMouseEvent) {
            unsafe {
                let area = Area::from_ui_area(ui_area);
                let area_mouse_event =
                    AreaMouseEvent::from_ui_area_mouse_event(&*ui_area_mouse_event);
                (*(ui_area_handler as *mut RustAreaHandler)).trait_object
                                                            .mouse_event(&area, &area_mouse_event);
                mem::forget(area_mouse_event);
                mem::forget(area);
            }
        }

        extern "C" fn mouse_crossed(ui_area_handler: *mut uiAreaHandler,
                                    ui_area: *mut uiArea,
                                    left: c_int) {
            unsafe {
                let area = Area::from_ui_area(ui_area);
                (*(ui_area_handler as *mut RustAreaHandler)).trait_object.mouse_crossed(&area,
                                                                                        left != 0);
                mem::forget(area);
            }
        }

        extern "C" fn drag_broken(ui_area_handler: *mut uiAreaHandler, ui_area: *mut uiArea) {
            unsafe {
                let area = Area::from_ui_area(ui_area);
                (*(ui_area_handler as *mut RustAreaHandler)).trait_object.drag_broken(&area);
                mem::forget(area);
            }
        }

        extern "C" fn key_event(ui_area_handler: *mut uiAreaHandler,
                                ui_area: *mut uiArea,
                                ui_area_key_event: *mut uiAreaKeyEvent)
                                -> c_int {
            unsafe {
                let area = Area::from_ui_area(ui_area);
                let area_key_event = AreaKeyEvent::from_ui_area_key_event(&*ui_area_key_event);
                let result =
                    (*(ui_area_handler as *mut RustAreaHandler)).trait_object
                                                                .key_event(&area, &area_key_event);
                mem::forget(area_key_event);
                mem::forget(area);
                result as c_int
            }
        }
    }
}

#[derive(Clone)]
pub struct Area {
    ui_area: *mut uiArea,
}

impl Deref for Area {
    type Target = Control;

    #[inline]
    fn deref(&self) -> &Control {
        // FIXME(pcwalton): $10 says this is undefined behavior. How do I make it not so?
        unsafe {
            mem::transmute::<&Area, &Control>(self)
        }
    }
}

impl Area {
    #[inline]
    pub unsafe fn from_ui_area(ui_area: *mut uiArea) -> Area {
        Area {
            ui_area: ui_area,
        }
    }

    #[inline]
    pub fn set_size(&self, width: i64, height: i64) {
        unsafe {
            ffi::uiAreaSetSize(self.ui_area, width, height)
        }
    }

    #[inline]
    pub fn queue_redraw_all(&self) {
        unsafe {
            ffi::uiAreaQueueRedrawAll(self.ui_area)
        }
    }

    #[inline]
    pub fn scroll_to(&self, x: f64, y: f64, width: f64, height: f64) {
        unsafe {
            ffi::uiAreaScrollTo(self.ui_area, x, y, width, height)
        }
    }

    #[inline]
    pub fn new(&self, area_handler: Box<AreaHandler>) -> Area {
        unsafe {
            let mut rust_area_handler = RustAreaHandler::new(area_handler);
            let area = Area {
                ui_area: ffi::uiNewArea(&mut *rust_area_handler as *mut RustAreaHandler as
                                        *mut uiAreaHandler),
            };
            mem::forget(rust_area_handler);
            area
        }
    }

    #[inline]
    pub fn new_scrolling(&self, area_handler: Box<AreaHandler>, width: i64, height: i64) -> Area {
        unsafe {
            let mut rust_area_handler = RustAreaHandler::new(area_handler);
            let area = Area {
                ui_area: ffi::uiNewScrollingArea(&mut *rust_area_handler as *mut RustAreaHandler as
                                                 *mut uiAreaHandler,
                                                 width,
                                                 height),
            };
            mem::forget(rust_area_handler);
            area
        }
    }
}

pub struct AreaDrawParams {
    pub context: draw::Context,

    pub area_width: f64,
    pub area_height: f64,

    pub clip_x: f64,
    pub clip_y: f64,
    pub clip_width: f64,
    pub clip_height: f64,
}

impl AreaDrawParams {
    #[inline]
    unsafe fn from_ui_area_draw_params(ui_area_draw_params: &uiAreaDrawParams) -> AreaDrawParams {
        AreaDrawParams {
            context: draw::Context::from_ui_draw_context(ui_area_draw_params.Context),
            area_width: ui_area_draw_params.AreaWidth,
            area_height: ui_area_draw_params.AreaHeight,
            clip_x: ui_area_draw_params.ClipX,
            clip_y: ui_area_draw_params.ClipY,
            clip_width: ui_area_draw_params.ClipWidth,
            clip_height: ui_area_draw_params.ClipHeight,
        }
    }
}

bitflags! {
    pub flags Modifiers: u8 {
        const MODIFIER_CTRL = 1 << 0,
        const MODIFIER_ALT = 1 << 1,
        const MODIFIER_SHIFT = 1 << 2,
        const MODIFIER_SUPER = 1 << 3,
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AreaMouseEvent {
    pub x: f64,
    pub y: f64,

    pub area_width: f64,
    pub area_height: f64,

    pub down: u64,
    pub up: u64,

    pub count: u64,

    pub modifiers: Modifiers,

    pub held_1_to_64: u64,
}

impl AreaMouseEvent {
    #[inline]
    pub fn from_ui_area_mouse_event(ui_area_mouse_event: &uiAreaMouseEvent) -> AreaMouseEvent {
        AreaMouseEvent {
            x: ui_area_mouse_event.X,
            y: ui_area_mouse_event.Y,
            area_width: ui_area_mouse_event.AreaWidth,
            area_height: ui_area_mouse_event.AreaHeight,
            down: ui_area_mouse_event.Down,
            up: ui_area_mouse_event.Up,
            count: ui_area_mouse_event.Count,
            modifiers: Modifiers::from_bits(ui_area_mouse_event.Modifiers as u8).unwrap(),
            held_1_to_64: ui_area_mouse_event.Held1To64,
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct AreaKeyEvent {
    pub key: u8,
    pub ext_key: ExtKey,
    pub modifier: Modifiers,
    pub modifiers: Modifiers,
    pub up: bool,
}

impl AreaKeyEvent {
    #[inline]
    pub fn from_ui_area_key_event(ui_area_key_event: &uiAreaKeyEvent) -> AreaKeyEvent {
        AreaKeyEvent {
            key: ui_area_key_event.Key as u8,
            ext_key: ui_area_key_event.ExtKey,
            modifier: Modifiers::from_bits(ui_area_key_event.Modifier as u8).unwrap(),
            modifiers: Modifiers::from_bits(ui_area_key_event.Modifiers as u8).unwrap(),
            up: ui_area_key_event.Up != 0,
        }
    }
}

