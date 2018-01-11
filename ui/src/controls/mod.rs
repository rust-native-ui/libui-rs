//! Contains all available controls and related functionality.

use ffi_utils::{self, Text};
use libc::{c_int, c_void};
use std::ffi::CString;
use std::mem;
use std::ptr;
use ui_sys::{self, uiBox, uiButton, uiCheckbox, uiCombobox, uiControl,
             uiDateTimePicker, uiEditableCombobox};
use ui_sys::{uiEntry, uiGroup, uiLabel, uiMultilineEntry, uiProgressBar};
use ui_sys::{uiRadioButtons, uiSeparator, uiSlider, uiSpinbox, uiTab};

#[macro_use]
mod create_macro;
pub mod area;
pub use self::area::Area;
pub mod complex_selectors;
pub use self::complex_selectors::{ColorButton, FontButton};

/// A generic UI control. Any UI control can be dereferenced into this type.
pub struct Control {
    ui_control: *mut uiControl,
}

impl Drop for Control {
    #[inline]
    fn drop(&mut self) {
        // For now this does nothing, but in the future, when `libui` supports proper memory
        // management, this will likely need to twiddle reference counts.
    }
}

impl Clone for Control {
    #[inline]
    fn clone(&self) -> Control {
        Control {
            ui_control: self.ui_control,
        }
    }
}

impl Control {
    /// Creates a new `Control` object from an existing `uiControl`.
    #[inline]
    pub unsafe fn from_ui_control(ui_control: *mut uiControl) -> Control {
        Control {
            ui_control: ui_control,
        }
    }

    #[inline]
    pub fn as_ui_control(&self) -> *mut uiControl {
        self.ui_control
    }

    /// Destroys a control. Any use of the control after this is use-after-free; therefore, this
    /// is marked unsafe.
    #[inline]
    pub unsafe fn destroy(&self) {
        // Don't check for initialization here since this can be run during deinitialization.
        ui_sys::uiControlDestroy(self.ui_control)
    }

    #[inline]
    pub fn handle(&self) -> usize {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiControlHandle(self.ui_control) }
    }

    #[inline]
    /// Get the parent control of this control.
    pub fn parent(&self) -> Option<Control> {
        ffi_utils::ensure_initialized();
        unsafe {
            let ui_control = ui_sys::uiControlParent(self.ui_control);
            if ui_control.is_null() {
                None
            } else {
                Some(Control::from_ui_control(ui_control))
            }
        }
    }

    #[inline]
    /// Set the parent control of this control, "moving" it to a new place in
    /// the UI tree or, if passed `None`, removing it.
    pub unsafe fn set_parent(&self, parent: Option<&Control>) {
        ffi_utils::ensure_initialized();
        ui_sys::uiControlSetParent(
            self.ui_control,
            match parent {
                None => ptr::null_mut(),
                Some(parent) => parent.ui_control,
            },
        )
    }

    #[inline]
    /// Returns true if this control is a top-level control; the root of
    /// the UI tree.
    pub fn toplevel(&self) -> bool {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiControlToplevel(self.ui_control) != 0 }
    }

    #[inline]
    /// Returns true if this control is visible.
    pub fn visible(&self) -> bool {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiControlVisible(self.ui_control) != 0 }
    }

    #[inline]
    /// Shows the control and its sub-controls.
    pub fn show(&self) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiControlShow(self.ui_control) }
    }

    #[inline]
    /// Hides the control and its sub-controls.
    pub fn hide(&self) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiControlHide(self.ui_control) }
    }

    #[inline]
    /// Returns true if the control is enabled (can be interacted with).
    pub fn enabled(&self) -> bool {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiControlEnabled(self.ui_control) != 0 }
    }

    #[inline]
    /// Enable the control, so the user can interact with it.
    pub fn enable(&self) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiControlEnable(self.ui_control) }
    }

    #[inline]
    /// Disable the control, so the user cannot interact with it.
    pub fn disable(&self) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiControlDisable(self.ui_control) }
    }
}

define_control!{
    /// A labeled clickable control which animates when clicked.
    control(Button, uiButton, ui_button);
}

impl Button {
    #[inline]
    /// Create a new button with the given text as its label.
    pub fn new(text: &str) -> Button {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            Button::from_ui_control(ui_sys::uiNewButton(c_string.as_ptr()))
        }
    }

    #[inline]
    /// Get the existing text on the button.
    pub fn text(&self) -> Text {
        ffi_utils::ensure_initialized();
        unsafe { Text::new(ui_sys::uiButtonText(self.ui_button)) }
    }

    #[inline]
    /// Set the text on the button.
    pub fn set_text(&self, text: &str) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            ui_sys::uiButtonSetText(self.ui_button, c_string.as_ptr())
        }
    }

    #[inline]
    /// Run the given callback when the button is clicked.
    pub fn on_clicked(&self, callback: Box<FnMut(&Button)>) {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut data: Box<Box<FnMut(&Button)>> = Box::new(callback);
            ui_sys::uiButtonOnClicked(
                self.ui_button,
                c_callback,
                &mut *data as *mut Box<FnMut(&Button)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(button: *mut uiButton, data: *mut c_void) {
            unsafe {
                let button = Button { ui_button: button };
                mem::transmute::<*mut c_void, &mut Box<FnMut(&Button)>>(data)(&button)
            }
        }
    }
}

define_control!{
    /// An invisible, uninteractable control that arranges the widgets inside it.
    control(BoxControl, uiBox, ui_box);
}

impl BoxControl {
    #[inline]
    /// Create a new `BoxControl` that lays out its children horizontally.
    pub fn new_horizontal() -> BoxControl {
        ffi_utils::ensure_initialized();
        unsafe { BoxControl::from_ui_control(ui_sys::uiNewHorizontalBox()) }
    }

    #[inline]
    /// Create a new `BoxControl` that lays out its children vertically.
    pub fn new_vertical() -> BoxControl {
        ffi_utils::ensure_initialized();
        unsafe { BoxControl::from_ui_control(ui_sys::uiNewVerticalBox()) }
    }

    #[inline]
    /// Add the given widget to the `BoxControl`, at the end of the list of widgets.
    pub fn append(&self, child: Control, stretchy: bool) {
        ffi_utils::ensure_initialized();
        unsafe {
            assert!(child.parent().is_none());
            ui_sys::uiBoxAppend(self.ui_box, child.ui_control, stretchy as c_int)
        }
    }

    /// FIXME(pcwalton): This will leak the deleted control! We have no way of actually getting it
    /// to decrement its reference count per `libui`'s UI as of today, unless we maintain a
    /// separate list of children ourselves…
    #[inline]
    pub fn delete(&self, index: u64) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiBoxDelete(self.ui_box, index) }
    }

    #[inline]
    /// Returns `true` if the `BoxControl` provides padding for the widgets inside.
    pub fn padded(&self) -> bool {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiBoxPadded(self.ui_box) != 0 }
    }

    #[inline]
    /// Set whether or not the `BoxControl` provides padding for the widgets inside.
    pub fn set_padded(&self, padded: bool) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiBoxSetPadded(self.ui_box, padded as c_int) }
    }
}

define_control!{
    /// A control that allows the user to enter text.
    control(Entry, uiEntry, ui_entry);
}

impl Entry {
    #[inline]
    pub fn text(&self) -> Text {
        ffi_utils::ensure_initialized();
        unsafe { Text::new(ui_sys::uiEntryText(self.ui_entry)) }
    }

    #[inline]
    pub fn set_text(&self, text: &str) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            ui_sys::uiEntrySetText(self.ui_entry, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn on_changed(&self, callback: Box<FnMut(&Entry)>) {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut data: Box<Box<FnMut(&Entry)>> = Box::new(callback);
            ui_sys::uiEntryOnChanged(
                self.ui_entry,
                c_callback,
                &mut *data as *mut Box<FnMut(&Entry)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(entry: *mut uiEntry, data: *mut c_void) {
            unsafe {
                let entry = Entry::from_ui_control(entry);
                mem::transmute::<*mut c_void, &mut Box<FnMut(&Entry)>>(data)(&entry);
                mem::forget(entry);
            }
        }
    }

    #[inline]
    pub fn read_only(&self) -> bool {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiEntryReadOnly(self.ui_entry) != 0 }
    }

    #[inline]
    pub fn set_read_only(&self, readonly: bool) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiEntrySetReadOnly(self.ui_entry, readonly as c_int) }
    }

    #[inline]
    pub fn new() -> Entry {
        ffi_utils::ensure_initialized();
        unsafe { Entry::from_ui_control(ui_sys::uiNewEntry()) }
    }
}

define_control!{
    /// A togglable checkbox control.
    control(Checkbox, uiCheckbox, ui_checkbox);
}

impl Checkbox {
    #[inline]
    pub fn text(&self) -> Text {
        ffi_utils::ensure_initialized();
        unsafe { Text::new(ui_sys::uiCheckboxText(self.ui_checkbox)) }
    }

    #[inline]
    pub fn set_text(&self, text: &str) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            ui_sys::uiCheckboxSetText(self.ui_checkbox, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn on_toggled(&self, callback: Box<FnMut(&Checkbox)>) {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut data: Box<Box<FnMut(&Checkbox)>> = Box::new(callback);
            ui_sys::uiCheckboxOnToggled(
                self.ui_checkbox,
                c_callback,
                &mut *data as *mut Box<FnMut(&Checkbox)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(checkbox: *mut uiCheckbox, data: *mut c_void) {
            unsafe {
                let checkbox = Checkbox::from_ui_control(checkbox);
                mem::transmute::<*mut c_void, &mut Box<FnMut(&Checkbox)>>(data)(&checkbox);
                mem::forget(checkbox)
            }
        }
    }

    #[inline]
    pub fn checked(&self) -> bool {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiCheckboxChecked(self.ui_checkbox) != 0 }
    }

    #[inline]
    pub fn set_checked(&self, checked: bool) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiCheckboxSetChecked(self.ui_checkbox, checked as c_int) }
    }

    #[inline]
    pub fn new(text: &str) -> Checkbox {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            Checkbox::from_ui_control(ui_sys::uiNewCheckbox(c_string.as_ptr()))
        }
    }
}

define_control!{
    /// A control which simply displays some text.
    control(Label, uiLabel, ui_label);
}

impl Label {
    #[inline]
    pub fn new(text: &str) -> Label {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            Label::from_ui_control(ui_sys::uiNewLabel(c_string.as_ptr()))
        }
    }

    #[inline]
    pub fn text(&self) -> Text {
        ffi_utils::ensure_initialized();
        unsafe { Text::new(ui_sys::uiLabelText(self.ui_label)) }
    }

    #[inline]
    pub fn set_text(&self, text: &str) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            ui_sys::uiLabelSetText(self.ui_label, c_string.as_ptr())
        }
    }
}

define_control!{
    /// A control which simply displays some text.
    control(Tab, uiTab, ui_tab);
}

impl Tab {
    #[inline]
    pub fn append(&self, name: &str, control: Control) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            ui_sys::uiTabAppend(self.ui_tab, c_string.as_ptr(), control.ui_control)
        }
    }

    #[inline]
    pub fn insert_at(&self, name: &str, before: u64, control: Control) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            ui_sys::uiTabInsertAt(self.ui_tab, c_string.as_ptr(), before, control.ui_control)
        }
    }

    /// FIXME(pcwalton): This will leak the deleted control! We have no way of actually getting it
    /// to decrement its reference count per `libui`'s UI as of today, unless we maintain a
    /// separate list of children ourselves…
    #[inline]
    pub fn delete(&self, index: u64) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiTabDelete(self.ui_tab, index) }
    }

    #[inline]
    pub fn margined(&self, page: u64) -> bool {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiTabMargined(self.ui_tab, page) != 0 }
    }

    #[inline]
    pub fn set_margined(&self, page: u64, margined: bool) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiTabSetMargined(self.ui_tab, page, margined as c_int) }
    }

    #[inline]
    pub fn new() -> Tab {
        ffi_utils::ensure_initialized();
        unsafe { Tab::from_ui_control(ui_sys::uiNewTab()) }
    }
}

define_control!{
    /// A logical group of controls, which can collect them in one place and draw margins around them.
    control(Group, uiGroup, ui_group);
}

impl Group {
    #[inline]
    pub fn title(&self) -> Text {
        ffi_utils::ensure_initialized();
        unsafe { Text::new(ui_sys::uiGroupTitle(self.ui_group)) }
    }

    #[inline]
    pub fn set_title(&self, title: &str) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(title.as_bytes().to_vec()).unwrap();
            ui_sys::uiGroupSetTitle(self.ui_group, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn set_child(&self, child: Control) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiGroupSetChild(self.ui_group, child.ui_control) }
    }

    #[inline]
    pub fn margined(&self) -> bool {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiGroupMargined(self.ui_group) != 0 }
    }

    #[inline]
    pub fn set_margined(&self, margined: bool) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiGroupSetMargined(self.ui_group, margined as c_int) }
    }

    #[inline]
    pub fn new(title: &str) -> Group {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(title.as_bytes().to_vec()).unwrap();
            Group::from_ui_control(ui_sys::uiNewGroup(c_string.as_ptr()))
        }
    }
}

define_control!{
    /// A numerical entry control which allows users to set any value in a range.
    control(Spinbox, uiSpinbox, ui_spinbox);
}

impl Spinbox {
    #[inline]
    pub fn value(&self) -> i64 {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiSpinboxValue(self.ui_spinbox) }
    }

    #[inline]
    pub fn set_value(&self, value: i64) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiSpinboxSetValue(self.ui_spinbox, value) }
    }

    #[inline]
    pub fn on_changed(&self, callback: Box<FnMut(&Spinbox)>) {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut data: Box<Box<FnMut(&Spinbox)>> = Box::new(callback);
            ui_sys::uiSpinboxOnChanged(
                self.ui_spinbox,
                c_callback,
                &mut *data as *mut Box<FnMut(&Spinbox)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(spinbox: *mut uiSpinbox, data: *mut c_void) {
            unsafe {
                let spinbox = Spinbox::from_ui_control(spinbox);
                mem::transmute::<*mut c_void, &mut Box<FnMut(&Spinbox)>>(data)(&spinbox);
                mem::forget(spinbox);
            }
        }
    }

    #[inline]
    pub fn new(min: i64, max: i64) -> Spinbox {
        ffi_utils::ensure_initialized();
        unsafe { Spinbox::from_ui_control(ui_sys::uiNewSpinbox(min, max)) }
    }
}

define_control!{
    /// A control that displays a given value as a partial fill of a bar.
    control(ProgressBar, uiProgressBar, ui_progress_bar);
}

impl ProgressBar {
    #[inline]
    pub fn set_value(&self, n: i32) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiProgressBarSetValue(self.ui_progress_bar, n) }
    }

    #[inline]
    pub fn new() -> ProgressBar {
        ffi_utils::ensure_initialized();
        unsafe { ProgressBar::from_ui_control(ui_sys::uiNewProgressBar()) }
    }
}

define_control!{ /// A control that allows users to select a value by picking a location along a line.
    control(Slider, uiSlider, ui_slider);
}

impl Slider {
    #[inline]
    pub fn value(&self) -> i64 {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiSliderValue(self.ui_slider) }
    }

    #[inline]
    pub fn set_value(&self, value: i64) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiSliderSetValue(self.ui_slider, value) }
    }

    #[inline]
    pub fn on_changed(&self, callback: Box<FnMut(&Slider)>) {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut data: Box<Box<FnMut(&Slider)>> = Box::new(callback);
            ui_sys::uiSliderOnChanged(
                self.ui_slider,
                c_callback,
                &mut *data as *mut Box<FnMut(&Slider)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(slider: *mut uiSlider, data: *mut c_void) {
            unsafe {
                let slider = Slider::from_ui_control(slider);
                mem::transmute::<*mut c_void, &mut Box<FnMut(&Slider)>>(data)(&slider);
                mem::forget(slider);
            }
        }
    }

    #[inline]
    pub fn new(min: i64, max: i64) -> Slider {
        ffi_utils::ensure_initialized();
        unsafe { Slider::from_ui_control(ui_sys::uiNewSlider(min, max)) }
    }
}

define_control!{
    /// A control which simply adds a horizontal line to seperate things.
    control(Separator, uiSeparator, ui_separator);
}

impl Separator {
    #[inline]
    pub fn new_horizontal() -> Separator {
        ffi_utils::ensure_initialized();
        unsafe { Separator::from_ui_control(ui_sys::uiNewHorizontalSeparator()) }
    }
}

define_control!{
    /// A control which allows the user to select any one of its options, from a list shown only when selected.
    control(Combobox, uiCombobox, ui_combobox);
}

impl Combobox {
    #[inline]
    pub fn append(&self, name: &str) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            ui_sys::uiComboboxAppend(self.ui_combobox, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn selected(&self) -> i64 {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiComboboxSelected(self.ui_combobox) }
    }

    #[inline]
    pub fn set_selected(&self, n: i64) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiComboboxSetSelected(self.ui_combobox, n) }
    }

    #[inline]
    pub fn on_selected(&self, callback: Box<FnMut(&Combobox)>) {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut data: Box<Box<FnMut(&Combobox)>> = Box::new(callback);
            ui_sys::uiComboboxOnSelected(
                self.ui_combobox,
                c_callback,
                &mut *data as *mut Box<FnMut(&Combobox)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(combobox: *mut uiCombobox, data: *mut c_void) {
            unsafe {
                let combobox = Combobox::from_ui_control(combobox);
                mem::transmute::<*mut c_void, &mut Box<FnMut(&Combobox)>>(data)(&combobox);
                mem::forget(combobox);
            }
        }
    }

    #[inline]
    pub fn new() -> Combobox {
        ffi_utils::ensure_initialized();
        unsafe { Combobox::from_ui_control(ui_sys::uiNewCombobox()) }
    }
}

define_control!{
    /// A control which allows the user to select an option from a list shown when selected and/or enter arbitrary text.
    control(EditableCombobox, uiEditableCombobox, ui_editable_combobox);
}

impl EditableCombobox {
    #[inline]
    pub fn new() -> EditableCombobox {
        ffi_utils::ensure_initialized();
        unsafe { EditableCombobox::from_ui_control(ui_sys::uiNewEditableCombobox()) }
    }
    pub fn append(&self, name: &str) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            ui_sys::uiEditableComboboxAppend(self.ui_editable_combobox, c_string.as_ptr())
        }
    }
}

// FIXME(pcwalton): Are these supposed to be a subclass of something? They don't seem very usable
// with just the `uiRadioButtons*` methods…
define_control!{
    /// A control which allows a user to select any one of its options, displayed as radio buttons.
    control(RadioButtons, uiRadioButtons, ui_radio_buttons);
}

impl RadioButtons {
    #[inline]
    pub fn append(&self, name: &str) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(name.as_bytes().to_vec()).unwrap();
            ui_sys::uiRadioButtonsAppend(self.ui_radio_buttons, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn new() -> RadioButtons {
        ffi_utils::ensure_initialized();
        unsafe { RadioButtons::from_ui_control(ui_sys::uiNewRadioButtons()) }
    }
}

// FIXME(pcwalton): Are these supposed to be a subclass of something? They don't seem very usable
// with just the `uiDatetimePicker*` methods…
define_control!{
    /// A control which allows the user to pick a date, time, or both.
    control(DateTimePicker, uiDateTimePicker, ui_date_time_picker);
}

impl DateTimePicker {
    pub fn new_date_time_picker() -> DateTimePicker {
        ffi_utils::ensure_initialized();
        unsafe { DateTimePicker::from_ui_control(ui_sys::uiNewDateTimePicker()) }
    }

    pub fn new_date_picker() -> DateTimePicker {
        ffi_utils::ensure_initialized();
        unsafe { DateTimePicker::from_ui_control(ui_sys::uiNewDatePicker()) }
    }

    pub fn new_time_picker() -> DateTimePicker {
        ffi_utils::ensure_initialized();
        unsafe { DateTimePicker::from_ui_control(ui_sys::uiNewTimePicker()) }
    }
}

define_control!{
    /// A control which allows a user to enter text over multiple lines.
    control(MultilineEntry, uiMultilineEntry, ui_multiline_entry);
}

impl MultilineEntry {
    #[inline]
    pub fn text(&self) -> Text {
        ffi_utils::ensure_initialized();
        unsafe { Text::new(ui_sys::uiMultilineEntryText(self.ui_multiline_entry)) }
    }

    #[inline]
    pub fn set_text(&self, text: &str) {
        ffi_utils::ensure_initialized();
        unsafe {
            let c_string = CString::new(text.as_bytes().to_vec()).unwrap();
            ui_sys::uiMultilineEntrySetText(self.ui_multiline_entry, c_string.as_ptr())
        }
    }

    #[inline]
    pub fn on_changed(&self, callback: Box<FnMut(&MultilineEntry)>) {
        ffi_utils::ensure_initialized();
        unsafe {
            let mut data: Box<Box<FnMut(&MultilineEntry)>> = Box::new(callback);
            ui_sys::uiMultilineEntryOnChanged(
                self.ui_multiline_entry,
                c_callback,
                &mut *data as *mut Box<FnMut(&MultilineEntry)> as *mut c_void,
            );
            mem::forget(data);
        }

        extern "C" fn c_callback(multiline_entry: *mut uiMultilineEntry, data: *mut c_void) {
            unsafe {
                let multiline_entry = MultilineEntry::from_ui_control(multiline_entry);
                mem::transmute::<*mut c_void, &mut Box<FnMut(&MultilineEntry)>>(data)(
                    &multiline_entry,
                );
                mem::forget(multiline_entry);
            }
        }
    }

    #[inline]
    pub fn read_only(&self) -> bool {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiMultilineEntryReadOnly(self.ui_multiline_entry) != 0 }
    }

    #[inline]
    pub fn set_read_only(&self, readonly: bool) {
        ffi_utils::ensure_initialized();
        unsafe { ui_sys::uiMultilineEntrySetReadOnly(self.ui_multiline_entry, readonly as c_int) }
    }

    #[inline]
    pub fn new() -> MultilineEntry {
        ffi_utils::ensure_initialized();
        unsafe { MultilineEntry::from_ui_control(ui_sys::uiNewMultilineEntry()) }
    }
}
