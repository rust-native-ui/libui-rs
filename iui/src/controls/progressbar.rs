use super::Control;
use std::mem;
use ui::UI;
use ui_sys::{self, uiControl, uiProgressBar};

define_control! {
  /// A bar that fills up with a set percentage, used to show completion of a task
  rust_type: ProgressBar,
  sys_type: uiProgressBar,
}

pub enum ProgressBarStyle {
    Determinate(u32),
    Indeterminate,
}

impl ProgressBar {
    pub fn new() -> ProgressBar {
        unsafe { ProgressBar::from_raw(ui_sys::uiNewProgressBar()) }
    }

    pub fn indeterminate() -> ProgressBar {
        let mut pb = ProgressBar::new();
        pb.set_value(ProgressBarStyle::Indeterminate);
        pb
    }

    pub fn set_value(&mut self, value: ProgressBarStyle) {
        let sys_value = match value {
            ProgressBarStyle::Determinate(value) => value as i32,
            ProgressBarStyle::Indeterminate => -1,
        };
        unsafe { ui_sys::uiProgressBarSetValue(self.uiProgressBar, sys_value) }
    }

    pub fn set_determinate(&mut self, value: u32) {
        self.set_value(ProgressBarStyle::Determinate(value));
    }

    pub fn value(&self) -> ProgressBarStyle {
        let sys_value = unsafe { ui_sys::uiProgressBarValue(self.uiProgressBar) };
        if sys_value.is_negative() {
            assert!(
                sys_value == -1,
                "if ProgressBar value is negative it can only be -1"
            );
            ProgressBarStyle::Indeterminate
        } else {
            ProgressBarStyle::Determinate(sys_value as u32)
        }
    }
}
