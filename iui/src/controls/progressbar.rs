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
    Determinate(i32),
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
            ProgressBarStyle::Determinate(value) => {
                // use !is_negative() because 0 is a valid value, but it
                // returns false for is_positive()
                assert!(
                    !value.is_negative(),
                    "determinate value for ProgressBar must not be negative"
                );
                value
            }
            ProgressBarStyle::Indeterminate => -1,
        };
        unsafe { ui_sys::uiProgressBarSetValue(self.uiProgressBar, sys_value) }
    }

    pub fn set_determinate(&mut self, value: i32) {
        self.set_value(ProgressBarStyle::Determinate(value));
    }

    pub fn value(&self) -> ProgressBarStyle {
        let sys_value = unsafe { ui_sys::uiProgressBarValue(self.uiProgressBar) };
        if sys_value.is_negative() {
            assert!(
                sys_value == -1,
                "if ProgressBar value is negative it should only be -1"
            );
            ProgressBarStyle::Indeterminate
        } else {
            ProgressBarStyle::Determinate(sys_value)
        }
    }
}
